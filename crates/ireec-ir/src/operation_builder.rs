//! Incremental construction of MLIR operations.
use crate::{
    Block, Context, Error, Location, Operation, Region, Result, Type, Value,
    attribute::NamedAttribute, string_ref::string_ref,
};
use iree_native as n;
use std::marker::PhantomData;

/// Incrementally constructs an operation using MLIR's operation-state API.
pub struct OperationBuilder<'ctx> {
    context: n::MlirContext,
    name: String,
    state: n::MlirOperationState,
    results: Vec<n::MlirType>,
    operands: Vec<n::MlirValue>,
    attributes: Vec<n::MlirNamedAttribute>,
    regions: Vec<Region<'ctx>>,
    infer_result_types: bool,
    _context: PhantomData<&'ctx ()>,
}

impl<'ctx> OperationBuilder<'ctx> {
    /// Start constructing a named operation at `location`.
    pub fn new<Ownership>(
        context: &'ctx Context<'_, Ownership>,
        name: &str,
        location: &Location<'ctx>,
    ) -> Result<OperationBuilder<'ctx>> {
        if name.is_empty() || name.as_bytes().contains(&0) {
            return Err(Error::Invalid(
                "operation name must be nonempty and contain no NUL".into(),
            ));
        }
        // SAFETY: `location` is a live native handle; their wrappers keep them valid throughout `mlirLocationGetContext`.
        if unsafe { n::mlirLocationGetContext(location.raw) }.ptr != context.raw.ptr {
            return Err(Error::Invalid(
                "operation location belongs to another MLIR context".into(),
            ));
        }
        let name = name.to_owned();
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        let state = unsafe { n::mlirOperationStateGet(string_ref(&name), location.raw) };
        Ok(OperationBuilder {
            context: context.raw,
            name,
            state,
            results: Vec::new(),
            operands: Vec::new(),
            attributes: Vec::new(),
            regions: Vec::new(),
            infer_result_types: false,
            _context: PhantomData,
        })
    }

    /// Add a result type from the builder's context.
    pub fn add_result(&mut self, ty: &Type<'ctx>) -> Result<&mut OperationBuilder<'ctx>> {
        // SAFETY: `ty` is a live native handle; their wrappers keep them valid throughout `mlirTypeGetContext`.
        if unsafe { n::mlirTypeGetContext(ty.raw) }.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "operation result type belongs to another MLIR context".into(),
            ));
        }
        self.results.push(ty.raw);
        Ok(self)
    }

    /// Add an SSA operand from the builder's context.
    ///
    /// The wrapper is borrowed only for this call; its owner lifetime still
    /// ensures the SSA value remains valid while the builder is in use.
    pub fn add_operand(&mut self, value: &Value<'ctx>) -> Result<&mut OperationBuilder<'ctx>> {
        if value.context.ptr != self.context.ptr {
            return Err(Error::Invalid(
                "operation operand belongs to another MLIR context".into(),
            ));
        }

        self.operands.push(value.raw);
        Ok(self)
    }

    /// Add a named attribute from the builder's context.
    pub fn add_attribute(
        &mut self,
        attribute: &NamedAttribute<'ctx>,
    ) -> Result<&mut OperationBuilder<'ctx>> {
        // SAFETY: `attribute` and `value` are live native handles; their wrappers keep them valid throughout `mlirAttributeGetContext`.
        if unsafe { n::mlirAttributeGetContext(attribute.value().as_raw()) }.ptr != self.context.ptr
        {
            return Err(Error::Invalid(
                "operation attribute belongs to another MLIR context".into(),
            ));
        }
        self.attributes.push(attribute.as_raw());
        Ok(self)
    }

    /// Add a region, transferring its ownership if this builder is built.
    pub fn add_region(&mut self, region: Region<'ctx>) -> Result<&mut OperationBuilder<'ctx>> {
        if region.context_raw().ptr != self.context.ptr {
            return Err(Error::Invalid(
                "operation region belongs to another MLIR context".into(),
            ));
        }
        self.regions.push(region);
        Ok(self)
    }

    /// Ask MLIR to infer result types when the operation supports it.
    pub fn enable_result_type_inference(&mut self) -> &mut OperationBuilder<'ctx> {
        self.infer_result_types = true;
        self
    }

    /// Create an operation without successors.
    pub fn build(self) -> Result<Operation<'ctx>> {
        self.build_with_successor_handles(&[])
    }

    /// Create the operation with successors borrowed for the duration of construction.
    pub fn build_with_successors<Ownership>(
        self,
        successors: &[&Block<'ctx, Ownership>],
    ) -> Result<Operation<'ctx>> {
        let mut raw_successors = Vec::with_capacity(successors.len());
        for block in successors {
            if block.context.ptr != self.context.ptr {
                return Err(Error::Invalid(
                    "operation successor belongs to another MLIR context".into(),
                ));
            }
            raw_successors.push(block.raw);
        }
        self.build_with_successor_handles(&raw_successors)
    }

    fn build_with_successor_handles(
        mut self,
        successors: &[n::MlirBlock],
    ) -> Result<Operation<'ctx>> {
        let _keep_name_alive = &self.name;
        let result_count = isize::try_from(self.results.len())
            .map_err(|_| Error::Invalid("too many operation results".into()))?;
        let operand_count = isize::try_from(self.operands.len())
            .map_err(|_| Error::Invalid("too many operation operands".into()))?;
        let attribute_count = isize::try_from(self.attributes.len())
            .map_err(|_| Error::Invalid("too many operation attributes".into()))?;
        let region_count = isize::try_from(self.regions.len())
            .map_err(|_| Error::Invalid("too many operation regions".into()))?;
        let successor_count = isize::try_from(successors.len())
            .map_err(|_| Error::Invalid("too many operation successors".into()))?;
        let regions: Vec<_> = self
            .regions
            .drain(..)
            // SAFETY: `into_raw` consumes this uniquely owned region and transfers the handle into the operation state.
            .map(|region| unsafe { region.into_raw() })
            .collect();
        // SAFETY: `results`, `operands`, `attributes`, `regions` and `successors` remain allocated for the synchronous C call and are read only during the call.
        unsafe {
            n::mlirOperationStateAddResults(&mut self.state, result_count, self.results.as_ptr());
            n::mlirOperationStateAddOperands(
                &mut self.state,
                operand_count,
                self.operands.as_ptr(),
            );
            n::mlirOperationStateAddAttributes(
                &mut self.state,
                attribute_count,
                self.attributes.as_ptr(),
            );
            n::mlirOperationStateAddOwnedRegions(&mut self.state, region_count, regions.as_ptr());
            n::mlirOperationStateAddSuccessors(
                &mut self.state,
                successor_count,
                successors.as_ptr(),
            );
            if self.infer_result_types {
                n::mlirOperationStateEnableResultTypeInference(&mut self.state);
            }
            let raw = n::mlirOperationCreate(&mut self.state);
            Operation::<'ctx, crate::Owned>::owned(raw, self.context)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_operand_does_not_require_value_wrapper_to_live_as_long_as_context() -> Result<()> {
        let context = Context::new()?;
        let ty = context.integer_type(32)?;
        let location = context.unknown_location();
        let block = Block::new(&context, &[ty], &[location])?;
        let mut builder =
            OperationBuilder::new(&context, "builtin.unrealized_conversion_cast", &location)?;

        builder.add_result(&ty)?;
        builder.add_operand(&block.argument(0)?.ok_or(Error::Null("block argument"))?)?;
        let _operation = builder.build()?;
        Ok(())
    }
}
