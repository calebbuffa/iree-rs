//! Reusable module storage, ordered import resolution, and isolated VM state.
#![cfg(all(feature = "compiler", feature = "cpu"))]

use iree::{Compiler, Instance, SessionSignal, Target, Value};

const LIBRARY: &str = r#"
vm.module @library attributes {
  iree.reflection = {purpose = "module composition test"}
} {
  vm.global.i32 mutable @counter : i32
  vm.export @next
  vm.func @next() -> i32 {
    %current = vm.global.load.i32 @counter : i32
    %one = vm.const.i32 1
    %next = vm.add.i32 %current, %one : i32
    vm.global.store.i32 %next, @counter : i32
    vm.return %next : i32
  }
  vm.export @double
  vm.func @double(%value: i32) -> i32 attributes {
    iree.reflection = {empty = "", role = "twice"}
  } {
    %result = vm.add.i32 %value, %value : i32
    vm.return %result : i32
  }
}
"#;

const APPLICATION: &str = r#"
vm.module @application {
  vm.import private @library.double(i32) -> i32
  vm.export @main
  vm.func @main(%value: i32) -> i32 {
    %result = vm.call @library.double(%value) : (i32) -> i32
    vm.return %result : i32
  }
}
"#;

fn scalar(session: &mut iree::Session, name: &str, inputs: &[Value]) -> iree::Result<i32> {
    let outputs = session.call_values(name, inputs)?;
    let [Value::I32(value)] = outputs.as_slice() else {
        panic!("expected one i32 result");
    };
    Ok(*value)
}

#[test]
fn modules_compose_and_outlive_original_handles() -> iree::Result<()> {
    let mut compiler = Compiler::new()?;
    let instance = Instance::new()?;
    let library_artifact = compiler.compile(LIBRARY, Target::Vmvx)?;
    let library = instance.module(&library_artifact)?;
    let application_artifact = compiler.compile(APPLICATION, Target::Vmvx)?;
    let application = instance.module(&application_artifact)?;
    let metadata = library.info()?;
    assert_eq!(metadata.name, "library");
    assert_eq!(metadata.exports.len(), 2);
    assert!(metadata.attributes.iter().any(|attr| attr.key == "purpose"));
    assert!(
        metadata
            .exports
            .iter()
            .any(|function| function.name == "library.double")
    );
    assert_eq!(application.info()?.import_count, 1);
    let device = instance.device("local-task")?;
    assert!(
        device
            .session_builder()
            .module(&application)
            .build()
            .is_err()
    );
    assert!(
        device
            .session_builder()
            .module(&application)
            .module(&library)
            .build()
            .is_err()
    );
    let builder = device
        .session_builder()
        .module(&library)
        .module(&application);
    let mut independent = device.session_builder().module(&library).build()?;
    drop(library_artifact);
    drop(application_artifact);
    drop(library);
    drop(application);
    drop(device);
    drop(instance);
    let mut session = builder.build()?;
    assert_eq!(
        session.function_attribute("library.double", "empty")?,
        Some(String::new())
    );
    assert_eq!(
        session.function_attribute("library.double", "absent")?,
        None
    );
    assert_eq!(
        scalar(&mut session, "application.main", &[Value::I32(21)])?,
        42
    );
    assert_eq!(scalar(&mut session, "library.next", &[])?, 1);
    assert_eq!(scalar(&mut session, "library.next", &[])?, 2);
    assert_eq!(scalar(&mut independent, "library.next", &[])?, 1);
    assert_eq!(session.modules()?.len(), 3);
    session.notify(SessionSignal::Suspend)?;
    session.notify(SessionSignal::Resume)?;
    session.notify(SessionSignal::LowMemory)?;
    assert_eq!(scalar(&mut session, "library.next", &[])?, 3);
    drop(session);
    assert_eq!(metadata.name, "library");
    assert_eq!(scalar(&mut independent, "library.next", &[])?, 2);
    Ok(())
}

#[test]
fn modules_reject_foreign_instances_and_invalid_bytecode() -> iree::Result<()> {
    let instance = Instance::new()?;
    let artifact = Compiler::new()?.compile(LIBRARY, Target::Vmvx)?;
    let module = instance.module(&artifact)?;
    let foreign_device = Instance::new()?.device("local-task")?;
    let result = foreign_device.session_builder().module(&module).build();
    let Err(error) = result else {
        panic!("a module from a different VM instance must be rejected");
    };
    assert!(error.message().contains("different runtime instance"));
    assert!(unsafe { instance.module_from_bytes(vec![0; 16]) }.is_err());
    Ok(())
}
