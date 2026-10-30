//! Public VM module, context, reflection, and invocation integration coverage.
// Internal stack-frame and bytecode-verifier mutation cases have no public C API surface.

use ireec::{Compiler, Device, InputConversion, Options, Output, Pipeline, Target};
use ireert::vm::{
    BytecodeModule, BytecodeModuleFlags, Context, ContextFlags, InlineLoop, Instance, Invocation,
    InvocationFlags, List, Loop, TypeDef, Value,
};
use ireert::{Code, Error, Result};
use std::sync::{Arc, Mutex, MutexGuard};

type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn lock<T>(mutex: &Mutex<T>) -> Result<MutexGuard<'_, T>> {
    mutex.lock().map_err(|error| {
        Error::new(
            Code::FailedPrecondition,
            format!("test mutex poisoned: {error}"),
        )
    })
}

fn compiled_module() -> ireec::Result<Vec<u8>> {
    let source = br#"
      func.func @add(%a: i32, %b: i32) -> i32 {
        %0 = arith.addi %a, %b : i32
        return %0 : i32
      }
      func.func @empty() { return }
      func.func @identity(%arg: i32) -> i32 { return %arg : i32 }
      func.func @integer_conversions(%value: i32) -> (i8, i16, i32, i32) {
        %narrow8 = arith.trunci %value : i32 to i8
        %narrow16 = arith.trunci %value : i32 to i16
        %unsigned = arith.extui %narrow8 : i8 to i32
        %signed = arith.extsi %narrow8 : i8 to i32
        return %narrow8, %narrow16, %unsigned, %signed : i8, i16, i32, i32
      }
      func.func @integer_i64_truncation(%value: i64) -> i32 {
        %narrow = arith.trunci %value : i64 to i32
        return %narrow : i32
      }
      func.func @integer_i32_extension(%signed: i32, %unsigned: i32) -> (i64, i64) {
        %sign_extended = arith.extsi %signed : i32 to i64
        %zero_extended = arith.extui %unsigned : i32 to i64
        return %sign_extended, %zero_extended : i64, i64
      }
      func.func @float_bitcasts(%f32_bits: i32, %f64_bits: i64)
          -> (f32, i32, f64, i64) {
        %f32 = arith.bitcast %f32_bits : i32 to f32
        %roundtrip_f32 = arith.bitcast %f32 : f32 to i32
        %f64 = arith.bitcast %f64_bits : i64 to f64
        %roundtrip_f64 = arith.bitcast %f64 : f64 to i64
        return %f32, %roundtrip_f32, %f64, %roundtrip_f64 : f32, i32, f64, i64
      }
      func.func @integer_comparisons(%a: i32, %b: i32)
          -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        %signed_less = arith.cmpi slt, %a, %b : i32
        %unsigned_less = arith.cmpi ult, %a, %b : i32
        %signed_less_equal = arith.cmpi sle, %a, %b : i32
        %unsigned_greater = arith.cmpi ugt, %a, %b : i32
        %signed_greater_equal = arith.cmpi sge, %a, %b : i32
        %equal = arith.cmpi eq, %a, %b : i32
        %not_equal = arith.cmpi ne, %a, %b : i32
        %unsigned_greater_equal = arith.cmpi uge, %a, %b : i32
        %signed_less_i32 = arith.extui %signed_less : i1 to i32
        %unsigned_less_i32 = arith.extui %unsigned_less : i1 to i32
        %signed_less_equal_i32 = arith.extui %signed_less_equal : i1 to i32
        %unsigned_greater_i32 = arith.extui %unsigned_greater : i1 to i32
        %signed_greater_equal_i32 = arith.extui %signed_greater_equal : i1 to i32
        %equal_i32 = arith.extui %equal : i1 to i32
        %not_equal_i32 = arith.extui %not_equal : i1 to i32
        %unsigned_greater_equal_i32 = arith.extui %unsigned_greater_equal : i1 to i32
        return %signed_less_i32, %unsigned_less_i32, %signed_less_equal_i32,
            %unsigned_greater_i32, %signed_greater_equal_i32, %equal_i32,
            %not_equal_i32, %unsigned_greater_equal_i32
            : i32, i32, i32, i32, i32, i32, i32, i32
      }
      func.func @integer_i64_ops(%a: i64, %b: i64)
          -> (i64, i64, i64, i64, i64, i64, i64, i64) {
        %sum = arith.addi %a, %b : i64
        %difference = arith.subi %a, %b : i64
        %product = arith.muli %a, %b : i64
        %signed_quotient = arith.divsi %a, %b : i64
        %unsigned_quotient = arith.divui %a, %b : i64
        %signed_remainder = arith.remsi %a, %b : i64
        %left_shift = arith.shli %a, %b : i64
        %right_shift = arith.shrui %a, %b : i64
        return %sum, %difference, %product, %signed_quotient,
            %unsigned_quotient, %signed_remainder, %left_shift, %right_shift
            : i64, i64, i64, i64, i64, i64, i64, i64
      }
      func.func @integer_i64_comparisons(%a: i64, %b: i64)
          -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        %signed_less = arith.cmpi slt, %a, %b : i64
        %unsigned_less = arith.cmpi ult, %a, %b : i64
        %signed_less_equal = arith.cmpi sle, %a, %b : i64
        %unsigned_greater = arith.cmpi ugt, %a, %b : i64
        %signed_greater_equal = arith.cmpi sge, %a, %b : i64
        %equal = arith.cmpi eq, %a, %b : i64
        %not_equal = arith.cmpi ne, %a, %b : i64
        %unsigned_greater_equal = arith.cmpi uge, %a, %b : i64
        %signed_less_i32 = arith.extui %signed_less : i1 to i32
        %unsigned_less_i32 = arith.extui %unsigned_less : i1 to i32
        %signed_less_equal_i32 = arith.extui %signed_less_equal : i1 to i32
        %unsigned_greater_i32 = arith.extui %unsigned_greater : i1 to i32
        %signed_greater_equal_i32 = arith.extui %signed_greater_equal : i1 to i32
        %equal_i32 = arith.extui %equal : i1 to i32
        %not_equal_i32 = arith.extui %not_equal : i1 to i32
        %unsigned_greater_equal_i32 = arith.extui %unsigned_greater_equal : i1 to i32
        return %signed_less_i32, %unsigned_less_i32, %signed_less_equal_i32,
            %unsigned_greater_i32, %signed_greater_equal_i32, %equal_i32,
            %not_equal_i32, %unsigned_greater_equal_i32
            : i32, i32, i32, i32, i32, i32, i32, i32
      }
      func.func @floating_ops(%a: f32, %b: f32, %c: f64, %d: f64)
          -> (f32, f32, f32, f32, f64, f64, f64, f64) {
        %f32_sum = arith.addf %a, %b : f32
        %f32_difference = arith.subf %a, %b : f32
        %f32_product = arith.mulf %a, %b : f32
        %f32_quotient = arith.divf %a, %b : f32
        %f64_sum = arith.addf %c, %d : f64
        %f64_difference = arith.subf %c, %d : f64
        %f64_product = arith.mulf %c, %d : f64
        %f64_quotient = arith.divf %c, %d : f64
        return %f32_sum, %f32_difference, %f32_product, %f32_quotient,
            %f64_sum, %f64_difference, %f64_product, %f64_quotient
            : f32, f32, f32, f32, f64, f64, f64, f64
      }
      func.func @floating_comparisons_f32(%a: f32, %b: f32)
          -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        %olt = arith.cmpf olt, %a, %b : f32
        %ole = arith.cmpf ole, %a, %b : f32
        %ogt = arith.cmpf ogt, %a, %b : f32
        %oge = arith.cmpf oge, %a, %b : f32
        %oeq = arith.cmpf oeq, %a, %b : f32
        %one = arith.cmpf one, %a, %b : f32
        %ueq = arith.cmpf ueq, %a, %b : f32
        %uno = arith.cmpf uno, %a, %b : f32
        %olt_i32 = arith.extui %olt : i1 to i32
        %ole_i32 = arith.extui %ole : i1 to i32
        %ogt_i32 = arith.extui %ogt : i1 to i32
        %oge_i32 = arith.extui %oge : i1 to i32
        %oeq_i32 = arith.extui %oeq : i1 to i32
        %one_i32 = arith.extui %one : i1 to i32
        %ueq_i32 = arith.extui %ueq : i1 to i32
        %uno_i32 = arith.extui %uno : i1 to i32
        return %olt_i32, %ole_i32, %ogt_i32, %oge_i32,
            %oeq_i32, %one_i32, %ueq_i32, %uno_i32
            : i32, i32, i32, i32, i32, i32, i32, i32
      }
      func.func @floating_comparisons_f64(%a: f64, %b: f64)
          -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        %olt = arith.cmpf olt, %a, %b : f64
        %ole = arith.cmpf ole, %a, %b : f64
        %ogt = arith.cmpf ogt, %a, %b : f64
        %oge = arith.cmpf oge, %a, %b : f64
        %oeq = arith.cmpf oeq, %a, %b : f64
        %one = arith.cmpf one, %a, %b : f64
        %ueq = arith.cmpf ueq, %a, %b : f64
        %uno = arith.cmpf uno, %a, %b : f64
        %olt_i32 = arith.extui %olt : i1 to i32
        %ole_i32 = arith.extui %ole : i1 to i32
        %ogt_i32 = arith.extui %ogt : i1 to i32
        %oge_i32 = arith.extui %oge : i1 to i32
        %oeq_i32 = arith.extui %oeq : i1 to i32
        %one_i32 = arith.extui %one : i1 to i32
        %ueq_i32 = arith.extui %ueq : i1 to i32
        %uno_i32 = arith.extui %uno : i1 to i32
        return %olt_i32, %ole_i32, %ogt_i32, %oge_i32,
            %oeq_i32, %one_i32, %ueq_i32, %uno_i32
            : i32, i32, i32, i32, i32, i32, i32, i32
      }
      func.func @integer_ops(%a: i32, %b: i32)
          -> (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32,
              i32, i32, i32, i32) {
        %sum = arith.addi %a, %b : i32
        %difference = arith.subi %a, %b : i32
        %product = arith.muli %a, %b : i32
        %signed_quotient = arith.divsi %a, %b : i32
        %unsigned_quotient = arith.divui %a, %b : i32
        %signed_remainder = arith.remsi %a, %b : i32
        %unsigned_remainder = arith.remui %a, %b : i32
        %minimum = arith.minsi %a, %b : i32
        %maximum = arith.maxsi %a, %b : i32
        %and = arith.andi %a, %b : i32
        %or = arith.ori %a, %b : i32
        %xor = arith.xori %a, %b : i32
        %left_shift = arith.shli %a, %b : i32
        %signed_right_shift = arith.shrsi %a, %b : i32
        %unsigned_right_shift = arith.shrui %a, %b : i32
        return %sum, %difference, %product, %signed_quotient,
            %unsigned_quotient, %signed_remainder, %unsigned_remainder,
            %minimum, %maximum, %and, %or, %xor, %left_shift,
            %signed_right_shift, %unsigned_right_shift
            : i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32,
              i32, i32, i32, i32
      }
      func.func @fpowi_f32(%base: f32, %exponent: i64) -> f32 {
        %result = math.fpowi %base, %exponent : f32, i64
        return %result : f32
      }
      func.func @fpowi_f64(%base: f64, %exponent: i64) -> f64 {
        %result = math.fpowi %base, %exponent : f64, i64
        return %result : f64
      }
      func.func @eight(%a0: i32, %a1: i32, %a2: i32, %a3: i32,
                       %a4: i32, %a5: i32, %a6: i32, %a7: i32)
          -> (i32, i32, i32, i32, i32, i32, i32, i32) {
        return %a0, %a1, %a2, %a3, %a4, %a5, %a6, %a7
            : i32, i32, i32, i32, i32, i32, i32, i32
      }
    "#;
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.configure(
        &Options::default()
            .with_input_conversion(InputConversion::DemoteI64ToI32, false)
            .with_input_conversion(InputConversion::DemoteF64ToF32, false)
            .with_device(Device::Alias("local".into()))
            .with_target(Target::Vmvx),
    )?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(session.source_buffer("vm_module_test.mlir", source)?)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

fn compiled_yield_module() -> ireec::Result<Vec<u8>> {
    let source = br#"
      vm.module @async_ops {
        vm.import private @yieldable_test.yield_n(%value: i32, %count: i32) -> i32
            attributes {vm.yield}
        vm.export @test_call_yieldable_import_yields_3
        vm.func @test_call_yieldable_import_yields_3() -> i32 {
          %value = vm.const.i32 100
          %count = vm.const.i32 3
          %result = vm.call @yieldable_test.yield_n(%value, %count)
              : (i32, i32) -> i32
          vm.return %result : i32
        }
      }
    "#;
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.configure(
        &Options::default()
            .with_device(Device::Alias("local".into()))
            .with_target(Target::Vmvx),
    )?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(session.source_buffer("vm_yield_module.mlir", source)?)?;
    invocation
        .run_pass_pipeline("builtin.module(vm.module(iree-vm-convert-to-yieldable-calls))")?;
    invocation.pipeline(Pipeline::Vm)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

fn compiled_vm_operator_module() -> ireec::Result<Vec<u8>> {
    let source = br#"
      vm.module @upstream_vm_ops {
        vm.export @signed_to_f32
        vm.func @signed_to_f32(%value: i32) -> f32 {
          %converted = vm.cast.si32.f32 %value : i32 -> f32
          vm.return %converted : f32
        }
        vm.export @unsigned_to_f32
        vm.func @unsigned_to_f32(%value: i32) -> f32 {
          %converted = vm.cast.ui32.f32 %value : i32 -> f32
          vm.return %converted : f32
        }
        vm.export @f32_to_signed
        vm.func @f32_to_signed(%value: f32) -> i32 {
          %converted = vm.cast.f32.si32 %value : f32 -> i32
          vm.return %converted : i32
        }
        vm.export @f32_to_unsigned
        vm.func @f32_to_unsigned(%value: f32) -> i32 {
          %converted = vm.cast.f32.ui32 %value : f32 -> i32
          vm.return %converted : i32
        }
        vm.export @truncate_i64_to_i32
        vm.func @truncate_i64_to_i32(%value: i64) -> i32 {
          %converted = vm.trunc.i64.i32 %value : i64 -> i32
          vm.return %converted : i32
        }
        vm.export @f32_arithmetic
        vm.func @f32_arithmetic(%lhs: f32, %rhs: f32, %mul_lhs: f32,
                                %mul_rhs: f32, %addend: f32, %value: f32,
                                %rounding: f32)
            -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
          %remainder = vm.rem.f32 %lhs, %rhs : f32
          %fma = vm.fma.f32 %mul_lhs, %mul_rhs, %addend : f32
          %absolute = vm.abs.f32 %value : f32
          %negative = vm.neg.f32 %value : f32
          %ceiling = vm.ceil.f32 %rounding : f32
          %floor = vm.floor.f32 %rounding : f32
          %rounded = vm.round.f32 %rounding : f32
          %rounded_even = vm.round.f32.even %rounding : f32
          %minimum = vm.min.f32 %lhs, %rhs : f32
          %maximum = vm.max.f32 %lhs, %rhs : f32
          vm.return %remainder, %fma, %absolute, %negative, %ceiling, %floor,
              %rounded, %rounded_even, %minimum, %maximum
              : f32, f32, f32, f32, f32, f32, f32, f32, f32, f32
        }
        vm.export @f64_arithmetic
        vm.func @f64_arithmetic(%lhs: f64, %rhs: f64, %mul_lhs: f64,
                                %mul_rhs: f64, %addend: f64, %value: f64,
                                %rounding: f64)
            -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
          %remainder = vm.rem.f64 %lhs, %rhs : f64
          %fma = vm.fma.f64 %mul_lhs, %mul_rhs, %addend : f64
          %absolute = vm.abs.f64 %value : f64
          %negative = vm.neg.f64 %value : f64
          %ceiling = vm.ceil.f64 %rounding : f64
          %floor = vm.floor.f64 %rounding : f64
          %rounded = vm.round.f64 %rounding : f64
          %rounded_even = vm.round.f64.even %rounding : f64
          %minimum = vm.min.f64 %lhs, %rhs : f64
          %maximum = vm.max.f64 %lhs, %rhs : f64
          vm.return %remainder, %fma, %absolute, %negative, %ceiling, %floor,
              %rounded, %rounded_even, %minimum, %maximum
              : f64, f64, f64, f64, f64, f64, f64, f64, f64, f64
        }
        vm.export @f32_math
        vm.func @f32_math(%angle: f32, %half: f32, %zero: f32, %two: f32,
                          %ten: f32, %three: f32, %four: f32)
            -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32,
                f32, f32, f32) {
          %atan = vm.atan.f32 %angle : f32
          %atan2 = vm.atan2.f32 %angle, %zero : f32
          %cos = vm.cos.f32 %half : f32
          %sin = vm.sin.f32 %half : f32
          %exp = vm.exp.f32 %angle : f32
          %exp2 = vm.exp2.f32 %two : f32
          %expm1 = vm.expm1.f32 %two : f32
          %log = vm.log.f32 %ten : f32
          %log10 = vm.log10.f32 %ten : f32
          %log1p = vm.log1p.f32 %ten : f32
          %log2 = vm.log2.f32 %ten : f32
          %pow = vm.pow.f32 %three, %two : f32
          %rsqrt = vm.rsqrt.f32 %four : f32
          %sqrt = vm.sqrt.f32 %four : f32
          %tanh = vm.tanh.f32 %half : f32
          vm.return %atan, %atan2, %cos, %sin, %exp, %exp2, %expm1, %log,
              %log10, %log1p, %log2, %pow, %rsqrt, %sqrt, %tanh
              : f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32,
                f32, f32, f32
        }
        vm.export @f64_math
        vm.func @f64_math(%angle: f64, %half: f64, %zero: f64, %two: f64,
                          %ten: f64, %three: f64, %four: f64)
            -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,
                f64, f64, f64) {
          %atan = vm.atan.f64 %angle : f64
          %atan2 = vm.atan2.f64 %angle, %zero : f64
          %cos = vm.cos.f64 %half : f64
          %sin = vm.sin.f64 %half : f64
          %exp = vm.exp.f64 %angle : f64
          %exp2 = vm.exp2.f64 %two : f64
          %expm1 = vm.expm1.f64 %two : f64
          %log = vm.log.f64 %ten : f64
          %log10 = vm.log10.f64 %ten : f64
          %log1p = vm.log1p.f64 %ten : f64
          %log2 = vm.log2.f64 %ten : f64
          %pow = vm.pow.f64 %three, %two : f64
          %rsqrt = vm.rsqrt.f64 %four : f64
          %sqrt = vm.sqrt.f64 %four : f64
          %tanh = vm.tanh.f64 %half : f64
          vm.return %atan, %atan2, %cos, %sin, %exp, %exp2, %expm1, %log,
              %log10, %log1p, %log2, %pow, %rsqrt, %sqrt, %tanh
              : f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64,
                f64, f64, f64
        }
      }
    "#;
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.configure(
        &Options::default()
            .with_device(Device::Alias("local".into()))
            .with_target(Target::Vmvx),
    )?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(session.source_buffer("vm_operator_test.mlir", source)?)?;
    invocation.pipeline(Pipeline::Vm)?;
    let mut output = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut output)?;
    Ok(output.bytes()?.to_vec())
}

fn schedule_resume<'a>(loop_: Loop<'a>, invocation: Arc<Mutex<Option<Invocation>>>) -> Result<()> {
    loop_.call(move |loop_, status| {
        status?;
        let mut current = lock(&invocation)?
            .take()
            .ok_or_else(|| Error::new(Code::FailedPrecondition, "invocation already resumed"))?;
        current.resume()?;
        let pending = current.is_pending();
        *lock(&invocation)? = Some(current);
        if pending {
            schedule_resume(loop_, invocation)
        } else {
            Ok(())
        }
    })
}

fn setup() -> TestResult<(Instance, BytecodeModule, Context)> {
    let instance = Instance::new()?;
    let archive = compiled_module()?;
    // SAFETY: this archive is produced by `compiled_module` immediately above.
    let bytecode =
        unsafe { BytecodeModule::new(&instance, BytecodeModuleFlags::default(), archive)? };
    let context = Context::with_modules(
        &instance,
        ContextFlags::default(),
        &[bytecode.module().clone()],
    )?;
    Ok((instance, bytecode, context))
}

fn setup_vm_operator_module() -> TestResult<(Instance, BytecodeModule, Context)> {
    let instance = Instance::new()?;
    let archive = compiled_vm_operator_module()?;
    // SAFETY: this archive is produced by `compiled_vm_operator_module` immediately above.
    let bytecode =
        unsafe { BytecodeModule::new(&instance, BytecodeModuleFlags::default(), archive)? };
    let context = Context::with_modules(
        &instance,
        ContextFlags::default(),
        &[bytecode.module().clone()],
    )?;
    Ok((instance, bytecode, context))
}

fn invoke_values(context: &Context, name: &str, values: &[Value]) -> TestResult<Vec<Value>> {
    let function = context.resolve_function(name)?;
    let (argument_count, result_count) = function.argument_result_counts()?;
    assert_eq!(values.len(), argument_count);
    let mut inputs = List::new(TypeDef::Any, argument_count)?;
    for &value in values {
        inputs.push(value)?;
    }
    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, result_count)?;
    invocation.end(&mut outputs)?;
    (0..result_count)
        .map(|index| Ok(outputs.get_value(index)?))
        .collect()
}

fn assert_float_outputs(actual: &[Value], expected: &[f64], tolerance: f64) -> TestResult<()> {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let actual = match actual {
            Value::F32(value) => f64::from(*value),
            Value::F64(value) => *value,
            value => {
                return Err(
                    format!("expected floating-point output at {index}, got {value:?}").into(),
                );
            }
        };
        assert!(
            (actual - expected).abs() <= tolerance,
            "output {index}: expected {expected}, got {actual}"
        );
    }
    Ok(())
}

#[test]
fn required_features_accepts_unaligned_archive_bytes() -> TestResult {
    let archive = compiled_module()?;
    let mut unaligned = Vec::with_capacity(archive.len() + 1);
    unaligned.push(0);
    unaligned.extend_from_slice(&archive);
    assert!(BytecodeModule::required_features(&unaligned[1..]).is_ok());
    Ok(())
}

#[test]
fn context_module_resolution_and_reflection() -> TestResult {
    let (_instance, bytecode, context) = setup()?;
    assert_eq!(context.module_count(), 1);
    assert!(context.module_at(0).is_some());
    assert!(context.module_at(1).is_none());
    let function = context.resolve_function("module.add")?;
    assert_eq!(function.name(), "add");
    assert_eq!(function.argument_result_counts()?, (2, 1));
    assert!(context.resolve_function("module.not_found").is_err());
    assert!(
        bytecode
            .disassemble(&function, ireert::vm::DisassemblyFormat::default())
            .is_ok()
    );
    assert!(context.resolve_module_state(function.module()).is_ok());
    assert!(context.fork().is_ok());
    Ok(())
}

#[test]
fn stepwise_invocation_completes_and_returns_results() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.add")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(19))?;
    inputs.push(Value::I32(23))?;
    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    assert!(!invocation.is_pending());
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(42));
    Ok(())
}

#[test]
fn compiled_integer_operations_match_upstream_vm_arithmetic_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.integer_ops")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(-7))?;
    inputs.push(Value::I32(3))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 15)?;
    invocation.end(&mut outputs)?;

    let expected = [
        -4,
        -10,
        -21,
        -2,
        1_431_655_763,
        -1,
        0,
        -7,
        3,
        1,
        -5,
        -6,
        -56,
        -1,
        536_870_911,
    ];
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I32(value));
    }
    Ok(())
}

#[test]
fn compiled_integer_conversions_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.integer_conversions")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::I32(i32::MAX))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 4)?;
    invocation.end(&mut outputs)?;

    assert_eq!(outputs.get_value(0)?, Value::I32(255));
    assert_eq!(outputs.get_value(1)?, Value::I32(65_535));
    assert_eq!(outputs.get_value(2)?, Value::I32(255));
    assert_eq!(outputs.get_value(3)?, Value::I32(-1));
    Ok(())
}

#[test]
fn compiled_integer_width_conversions_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let truncation = context.resolve_function("module.integer_i64_truncation")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::I64(0x1_0000_0001))?;
    let invocation = context.begin_invoke(&truncation, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(1));

    let extension = context.resolve_function("module.integer_i32_extension")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(-1))?;
    inputs.push(Value::I32(-1))?;
    let invocation = context.begin_invoke(&extension, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 2)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I64(-1));
    assert_eq!(outputs.get_value(1)?, Value::I64(i64::from(u32::MAX)));
    Ok(())
}

#[test]
fn compiled_float_bitcasts_preserve_upstream_bit_patterns() -> TestResult<()> {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.float_bitcasts")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(0x3f80_0000))?;
    inputs.push(Value::I64(0x3ff0_0000_0000_0000))?;
    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 4)?;
    invocation.end(&mut outputs)?;

    assert_eq!(outputs.get_value(0)?, Value::F32(1.0));
    assert_eq!(outputs.get_value(1)?, Value::I32(0x3f80_0000));
    assert_eq!(outputs.get_value(2)?, Value::F64(1.0));
    assert_eq!(outputs.get_value(3)?, Value::I64(0x3ff0_0000_0000_0000));
    Ok(())
}

#[test]
fn compiled_integer_and_float_conversions_match_upstream_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup_vm_operator_module()?;

    let signed_to_float = context.resolve_function("upstream_vm_ops.signed_to_f32")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::I32(-7))?;
    let invocation = context.begin_invoke(&signed_to_float, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::F32(-7.0));

    let unsigned_to_float = context.resolve_function("upstream_vm_ops.unsigned_to_f32")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::I32(-1))?;
    let invocation =
        context.begin_invoke(&unsigned_to_float, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::F32(4_294_967_296.0));

    let f32_to_signed = context.resolve_function("upstream_vm_ops.f32_to_signed")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::F32(-2.5))?;
    let invocation = context.begin_invoke(&f32_to_signed, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(-3));

    let f32_to_unsigned = context.resolve_function("upstream_vm_ops.f32_to_unsigned")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::F32(2.5))?;
    let invocation = context.begin_invoke(&f32_to_unsigned, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(3));

    let truncate = context.resolve_function("upstream_vm_ops.truncate_i64_to_i32")?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.push(Value::I64(0x1_0000_0001))?;
    let invocation = context.begin_invoke(&truncate, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(1));
    Ok(())
}

#[test]
fn compiled_integer_exponentiation_matches_upstream_fpowi_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    for (name, base, expected) in [
        ("module.fpowi_f32", Value::F32(2.0), Value::F32(8.0)),
        ("module.fpowi_f64", Value::F64(2.0), Value::F64(8.0)),
    ] {
        let function = context.resolve_function(name)?;
        let mut inputs = List::new(TypeDef::Any, 2)?;
        inputs.push(base)?;
        inputs.push(Value::I64(3))?;
        let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
        let mut outputs = List::new(TypeDef::Any, 1)?;
        invocation.end(&mut outputs)?;
        assert_eq!(outputs.get_value(0)?, expected, "function {name}");
    }

    let function = context.resolve_function("module.fpowi_f32")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::F32(2.0))?;
    inputs.push(Value::I64(-2))?;
    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::F32(0.25));
    Ok(())
}

#[test]
fn compiled_f32_and_f64_arithmetic_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup_vm_operator_module()?;
    let f32_outputs = invoke_values(
        &context,
        "upstream_vm_ops.f32_arithmetic",
        &[
            Value::F32(-3.0),
            Value::F32(-2.0),
            Value::F32(2.0),
            Value::F32(3.0),
            Value::F32(5.0),
            Value::F32(-1.0),
            Value::F32(2.5),
        ],
    )?;
    let f32_expected = [
        Value::F32(1.0),
        Value::F32(11.0),
        Value::F32(1.0),
        Value::F32(1.0),
        Value::F32(3.0),
        Value::F32(2.0),
        Value::F32(3.0),
        Value::F32(2.0),
        Value::F32(-3.0),
        Value::F32(-2.0),
    ];
    assert_eq!(f32_outputs, f32_expected);

    let f64_outputs = invoke_values(
        &context,
        "upstream_vm_ops.f64_arithmetic",
        &[
            Value::F64(-3.0),
            Value::F64(-2.0),
            Value::F64(2.0),
            Value::F64(3.0),
            Value::F64(5.0),
            Value::F64(-1.0),
            Value::F64(2.5),
        ],
    )?;
    let f64_expected = [
        Value::F64(1.0),
        Value::F64(11.0),
        Value::F64(1.0),
        Value::F64(1.0),
        Value::F64(3.0),
        Value::F64(2.0),
        Value::F64(3.0),
        Value::F64(2.0),
        Value::F64(-3.0),
        Value::F64(-2.0),
    ];
    assert_eq!(f64_outputs, f64_expected);
    Ok(())
}

#[test]
fn compiled_f32_and_f64_math_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup_vm_operator_module()?;
    let f32_outputs = invoke_values(
        &context,
        "upstream_vm_ops.f32_math",
        &[
            Value::F32(1.0),
            Value::F32(0.5),
            Value::F32(0.0),
            Value::F32(2.0),
            Value::F32(10.0),
            Value::F32(3.0),
            Value::F32(4.0),
        ],
    )?;
    assert_float_outputs(
        &f32_outputs,
        &[
            f64::from(std::f32::consts::FRAC_PI_4),
            f64::from(std::f32::consts::FRAC_PI_2),
            0.877_582_55,
            0.479_425_55,
            f64::from(std::f32::consts::E),
            4.0,
            6.389_056_21,
            f64::from(std::f32::consts::LN_10),
            1.0,
            2.397_895_34,
            f64::from(std::f32::consts::LOG2_10),
            9.0,
            0.5,
            2.0,
            0.462_117_165,
        ],
        0.000_01,
    )?;

    let f64_outputs = invoke_values(
        &context,
        "upstream_vm_ops.f64_math",
        &[
            Value::F64(1.0),
            Value::F64(0.5),
            Value::F64(0.0),
            Value::F64(2.0),
            Value::F64(10.0),
            Value::F64(3.0),
            Value::F64(4.0),
        ],
    )?;
    assert_float_outputs(
        &f64_outputs,
        &[
            std::f64::consts::FRAC_PI_4,
            std::f64::consts::FRAC_PI_2,
            0.877_582_561_890_372_8,
            0.479_425_538_604_203,
            std::f64::consts::E,
            4.0,
            6.389_056_098_930_65,
            std::f64::consts::LN_10,
            1.0,
            2.397_895_272_798_370_7,
            std::f64::consts::LOG2_10,
            9.0,
            0.5,
            2.0,
            0.462_117_157_260_009_74,
        ],
        1e-12,
    )?;
    Ok(())
}

#[test]
fn compiled_integer_comparisons_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.integer_comparisons")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(-2))?;
    inputs.push(Value::I32(2))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;

    let expected = [1, 0, 1, 1, 0, 0, 1, 1];
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I32(value));
    }
    Ok(())
}

#[test]
fn compiled_i64_operations_preserve_full_width_upstream_semantics() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.integer_i64_ops")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I64(4_294_967_296))?;
    inputs.push(Value::I64(3))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;

    let expected = [
        4_294_967_299,
        4_294_967_293,
        12_884_901_888,
        1_431_655_765,
        1_431_655_765,
        1,
        34_359_738_368,
        536_870_912,
    ];
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I64(value));
    }
    Ok(())
}

#[test]
fn compiled_i64_comparisons_match_upstream_signed_and_unsigned_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.integer_i64_comparisons")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I64(-2))?;
    inputs.push(Value::I64(2))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;

    for (index, expected) in [1, 0, 1, 1, 0, 0, 1, 1].into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I32(expected));
    }
    Ok(())
}

#[test]
fn compiled_floating_operations_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.floating_ops")?;
    let mut inputs = List::new(TypeDef::Any, 4)?;
    inputs.push(Value::F32(6.0))?;
    inputs.push(Value::F32(2.0))?;
    inputs.push(Value::F64(-9.0))?;
    inputs.push(Value::F64(3.0))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;

    let expected = [
        Value::F32(8.0),
        Value::F32(4.0),
        Value::F32(12.0),
        Value::F32(3.0),
        Value::F64(-6.0),
        Value::F64(-12.0),
        Value::F64(-27.0),
        Value::F64(-3.0),
    ];
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, value);
    }
    Ok(())
}

#[test]
fn compiled_f32_comparisons_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.floating_comparisons_f32")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::F32(-2.0))?;
    inputs.push(Value::F32(2.0))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;
    for (index, expected) in [1, 1, 0, 0, 0, 1, 0, 0].into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I32(expected));
    }

    Ok(())
}

#[test]
fn compiled_f64_comparisons_match_upstream_vm_cases() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.floating_comparisons_f64")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::F64(-2.0))?;
    inputs.push(Value::F64(2.0))?;

    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    let mut outputs = List::new(TypeDef::Any, 8)?;
    invocation.end(&mut outputs)?;
    for (index, expected) in [1, 1, 0, 0, 0, 1, 0, 0].into_iter().enumerate() {
        assert_eq!(outputs.get_value(index)?, Value::I32(expected));
    }

    Ok(())
}

#[test]
fn async_invocation_runs_completion_callback() -> TestResult {
    let (_instance, _bytecode, context) = setup()?;
    let function = context.resolve_function("module.add")?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.push(Value::I32(6))?;
    inputs.push(Value::I32(7))?;
    let outputs = List::new(TypeDef::Any, 1)?;
    let completion = Arc::new(Mutex::new(None));
    let captured = completion.clone();
    let mut loop_ = InlineLoop::new();
    context.invoke_async(
        loop_.as_loop(),
        &function,
        InvocationFlags::default(),
        &inputs,
        outputs,
        move |_loop, result| {
            *lock(&captured)? = Some(result.and_then(|list| list.get_value(0)));
            Ok(())
        },
    )?;
    loop_.take_error()?;
    let value = lock(&completion)?
        .take()
        .ok_or("completion callback did not run")??;
    assert_eq!(value, Value::I32(13));
    Ok(())
}

#[test]
fn test_call_yieldable_import_yields_3() -> TestResult {
    use ireert::vm::{NativeCallPhase, NativeModule};
    use ireert::{Code, Error};
    use std::sync::Mutex;

    let instance = Instance::new()?;
    let phases = Arc::new(Mutex::new(Vec::new()));
    let callback_phases = phases.clone();
    let module = NativeModule::builder("yieldable_test")
        .state(
            || Ok(Mutex::new((0_i32, 0_i32, 0_i32))),
            |state| Ok(Mutex::new(*lock(state)?)),
        )
        .export("yield_n", "0ii_i", move |call| {
            let phase = call.phase();
            lock(&callback_phases)?.push(phase);
            let state = call.state::<Mutex<(i32, i32, i32)>>().ok_or_else(|| {
                Error::new(Code::FailedPrecondition, "yieldable module state missing")
            })?;
            let (mut value, mut count, mut remaining) = *lock(state)?;
            if phase == NativeCallPhase::Begin {
                value = call.argument(0)?;
                count = call.argument(4)?;
                remaining = count;
            }
            if remaining > 0 {
                remaining -= 1;
                *lock(state)? = (value, count, remaining);
                return Err(Error::new(Code::Deferred, "yield for test"));
            }
            call.set_result(0, value + count)
        })
        .build(&instance)?;
    let archive = compiled_yield_module()?;
    // SAFETY: this test compiles a VM module that imports the yieldable native export.
    let bytecode =
        unsafe { BytecodeModule::new(&instance, BytecodeModuleFlags::default(), archive)? };
    let context = Context::with_modules(
        &instance,
        ContextFlags::default(),
        &[module, bytecode.module().clone()],
    )?;
    let function = context.resolve_function("async_ops.test_call_yieldable_import_yields_3")?;
    let inputs = List::new(TypeDef::Any, 0)?;
    let invocation = context.begin_invoke(&function, InvocationFlags::default(), &inputs)?;
    assert!(invocation.is_pending());
    let invocation = Arc::new(Mutex::new(Some(invocation)));
    let mut loop_ = InlineLoop::new();
    schedule_resume(loop_.as_loop(), invocation.clone())?;
    loop_.take_error()?;
    let invocation = lock(&invocation)?
        .take()
        .ok_or("resumed invocation missing")?;
    assert!(!invocation.is_pending());
    assert_eq!(
        *lock(&phases)?,
        [
            NativeCallPhase::Begin,
            NativeCallPhase::Resume,
            NativeCallPhase::Resume,
            NativeCallPhase::Resume
        ]
    );
    let mut outputs = List::new(TypeDef::Any, 1)?;
    invocation.end(&mut outputs)?;
    assert_eq!(outputs.get_value(0)?, Value::I32(103));
    Ok(())
}
