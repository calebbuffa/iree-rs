//! Laws: compound types preserve the order and identity of their constituents.
use super::{Type, count, same_context};
use crate::{Context, Error, Result};
use iree_native as n;
typed!(ComplexType, mlirTypeIsAComplex, "A complex number type.");
typed!(TupleType, mlirTypeIsATuple, "An ordered tuple of types.");
typed!(
    FunctionType,
    mlirTypeIsAFunction,
    "A function's input and result types."
);
impl<'c> ComplexType<'c> {
    /// Construct a complex type over an integer or floating point element.
    pub fn new(element: Type<'c>) -> Result<Self> {
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirTypeIsAInteger`.
        if !unsafe { n::mlirTypeIsAInteger(element.raw) || n::mlirTypeIsAFloat(element.raw) } {
            return Err(Error::Invalid(
                "complex element must be integer or float".into(),
            ));
        }
        // SAFETY: `element` is a live native handle; their wrappers keep them valid throughout `mlirComplexTypeGet`.
        Ok(Self(Type::checked(unsafe {
            n::mlirComplexTypeGet(element.raw)
        })?))
    }
    /// The complex component type.
    pub fn element_type(&self) -> Type<'c> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirComplexTypeGetElementType`.
        Type::borrowed(unsafe { n::mlirComplexTypeGetElementType(self.raw) })
    }
}
impl<'c> TupleType<'c> {
    /// Construct a tuple, rejecting constituent types from other contexts.
    pub fn new<O>(context: &'c Context<'_, O>, elements: &[Type<'c>]) -> Result<Self> {
        for ty in elements {
            same_context(context.raw, ty.context_raw())?;
        }
        let raw: Vec<_> = elements.iter().map(|ty| ty.raw).collect();
        // SAFETY: `raw` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Ok(Self(Type::checked(unsafe {
            n::mlirTupleTypeGet(context.raw, count(raw.len())?, raw.as_ptr())
        })?))
    }
    /// Tuple elements in order.
    pub fn elements(&self) -> Vec<Type<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTupleTypeGetNumTypes`.
        let count = unsafe { n::mlirTupleTypeGetNumTypes(self.raw) };
        (0..count)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirTupleTypeGetType`.
            .map(|i| Type::borrowed(unsafe { n::mlirTupleTypeGetType(self.raw, i) }))
            .collect()
    }
}
impl<'c> FunctionType<'c> {
    /// Construct a function type with ordered inputs and results.
    pub fn new<O>(
        context: &'c Context<'_, O>,
        inputs: &[Type<'c>],
        results: &[Type<'c>],
    ) -> Result<Self> {
        for ty in inputs.iter().chain(results) {
            same_context(context.raw, ty.context_raw())?;
        }
        let inputs: Vec<_> = inputs.iter().map(|ty| ty.raw).collect();
        let results: Vec<_> = results.iter().map(|ty| ty.raw).collect();
        // SAFETY: `inputs` and `results` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            n::mlirFunctionTypeGet(
                context.raw,
                count(inputs.len())?,
                inputs.as_ptr(),
                count(results.len())?,
                results.as_ptr(),
            )
        };
        Ok(Self(Type::checked(raw)?))
    }
    /// Input types in argument order.
    pub fn inputs(&self) -> Vec<Type<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirFunctionTypeGetNumInputs`.
        let count = unsafe { n::mlirFunctionTypeGetNumInputs(self.raw) };
        (0..count)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirFunctionTypeGetInput`.
            .map(|i| Type::borrowed(unsafe { n::mlirFunctionTypeGetInput(self.raw, i) }))
            .collect()
    }
    /// Result types in result order.
    pub fn results(&self) -> Vec<Type<'c>> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirFunctionTypeGetNumResults`.
        let count = unsafe { n::mlirFunctionTypeGetNumResults(self.raw) };
        (0..count)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirFunctionTypeGetResult`.
            .map(|i| Type::borrowed(unsafe { n::mlirFunctionTypeGetResult(self.raw, i) }))
            .collect()
    }
}
