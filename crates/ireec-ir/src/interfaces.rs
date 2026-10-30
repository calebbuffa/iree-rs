//! # Laws
//! Interface enum values round-trip to the pinned C enum; inference callbacks
//! only copy borrowed results and never unwind through MLIR.

use crate::{
    Attribute, Context, Error, Location, Result, Type, Value, callback::contain,
    string_ref::string_ref,
};
use iree_native as n;
use std::{ffi::c_void, ptr};

/// The operation interfaces with TypeIDs exposed by the matched MLIR C API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationInterface {
    /// `InferTypeOpInterface`.
    InferType,
    /// `InferShapedTypeOpInterface`.
    InferShapedType,
    /// Conditional speculatability.
    ConditionallySpeculatable,
    /// Memory effects.
    MemoryEffects,
}

/// Result of querying `mlirConditionallySpeculatableOpInterfaceGetSpeculatability`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speculatability {
    /// The operation cannot be speculated.
    No,
    /// The operation is speculatable without considering nested operations.
    Yes,
    /// The operation is speculatable only when all nested operations are.
    Recursively,
}

impl Speculatability {
    fn from_native(value: n::MlirSpeculatability) -> Result<Self> {
        match value {
            n::MlirSpeculatabilityNotSpeculatable => Ok(Self::No),
            n::MlirSpeculatabilitySpeculatable => Ok(Self::Yes),
            n::MlirSpeculatabilityRecursivelySpeculatable => Ok(Self::Recursively),
            _ => Err(Error::Invalid(
                "MLIR returned an unknown speculatability value".into(),
            )),
        }
    }
}

/// A copied effect reported by the MLIR memory-effects interface.
pub struct MemoryEffect<'a> {
    /// The builtin effect kind, or `Other` for an unrecognized native effect.
    pub kind: MemoryEffectKind,
    /// The side-effect resource classification.
    pub resource: MemoryEffectResource,
    /// The stage reported by MLIR.
    pub stage: i32,
    /// Whether this effect applies to the full resource region.
    pub on_full_region: bool,
    /// Optional SSA value associated with this effect, borrowed from the operation context.
    pub value: Option<Value<'a>>,
    /// Optional symbol-reference attribute associated with this effect.
    pub symbol: Option<Attribute<'a>>,
    /// Optional effect-parameter attribute.
    pub parameters: Option<Attribute<'a>>,
}

/// Builtin memory-effect kinds exposed by MLIR's C API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryEffectKind {
    /// Allocates a resource.
    Allocate,
    /// Frees a resource.
    Free,
    /// Reads a resource.
    Read,
    /// Writes a resource.
    Write,
    /// An effect kind not recognized by the matched compiler image.
    Other,
}

/// Memory-effect resource classification exposed by the matched MLIR C API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryEffectResource {
    /// The singleton default side-effect resource.
    Default,
    /// A non-default resource; this API version exposes no name getter.
    Other,
}

impl OperationInterface {
    /// The process-local native TypeID for this interface.
    pub fn type_id(self) -> TypeId {
        TypeId {
            raw: self.raw_type_id(),
        }
    }

    fn raw_type_id(self) -> n::MlirTypeID {
        // SAFETY: these parameterless getters return process-stable interface TypeIDs by value.
        unsafe {
            match self {
                Self::InferType => n::mlirInferTypeOpInterfaceTypeID(),
                Self::InferShapedType => n::mlirInferShapedTypeOpInterfaceTypeID(),
                Self::ConditionallySpeculatable => {
                    n::mlirConditionallySpeculatableOpInterfaceTypeID()
                }
                Self::MemoryEffects => n::mlirMemoryEffectsOpInterfaceTypeID(),
            }
        }
    }
}

/// A process-local MLIR TypeID. Its hash is not persistent across executions.
#[derive(Clone, Copy)]
pub struct TypeId {
    raw: n::MlirTypeID,
}

impl TypeId {
    /// Wrap a TypeID from the exact MLIR library used by this crate.
    ///
    /// # Safety
    ///
    /// `raw` must be a live TypeID from the matched compiler image.
    pub unsafe fn from_raw(raw: n::MlirTypeID) -> Result<Self> {
        // SAFETY: `mlirTypeIDIsNull` accepts a nullable value handle and only tests its null sentinel.
        if unsafe { n::mlirTypeIDIsNull(raw) } {
            return Err(Error::Null("MLIR TypeID"));
        }
        Ok(Self { raw })
    }

    /// The borrowed native TypeID.
    pub fn as_raw(self) -> n::MlirTypeID {
        self.raw
    }
}

impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIDEqual`.
        unsafe { n::mlirTypeIDEqual(self.raw, other.raw) }
    }
}

impl Eq for TypeId {}

impl std::hash::Hash for TypeId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirTypeIDHashValue`.
        state.write_usize(unsafe { n::mlirTypeIDHashValue(self.raw) });
    }
}

/// Values copied out of a synchronous MLIR callback; `invalid` records bad input or a panic.
struct Collector<T> {
    values: Vec<T>,
    invalid: bool,
}

impl<T> Default for Collector<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            invalid: false,
        }
    }
}

impl<T> Collector<T> {
    /// Runs `collect` on the collector behind `data`, marking it invalid on `None` or panic.
    ///
    /// # Safety
    /// `data` must point to a live `Collector<T>` exclusively borrowed by the caller.
    unsafe fn update(data: *mut c_void, collect: impl FnOnce(&mut Vec<T>) -> Option<()>) {
        // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
        let collector = unsafe { &mut *data.cast::<Self>() };
        let values = &mut collector.values;
        if contain(|| collect(values)).flatten().is_none() {
            collector.invalid = true;
        }
    }
}

/// Borrows a native array, rejecting a negative count or a null non-empty array.
///
/// # Safety
/// A non-null `items` must point to `count` initialized values live for `'a`.
unsafe fn native_slice<'a, T>(items: *const T, count: isize) -> Option<&'a [T]> {
    let count = usize::try_from(count).ok()?;
    if count == 0 {
        return Some(&[]);
    }
    if items.is_null() {
        return None;
    }
    // SAFETY: the interface callback supplies initialized values for this count.
    Some(unsafe { n::slice(items, count) })
}

unsafe extern "C" fn collect_types(count: isize, types: *mut n::MlirType, data: *mut c_void) {
    // SAFETY: `data` points to the collector passed by the enclosing synchronous interface query.
    unsafe {
        Collector::update(data, |values| {
            values.extend_from_slice(native_slice(types, count)?);
            Some(())
        })
    };
}

unsafe extern "C" fn collect_shaped_types(
    has_rank: bool,
    rank: isize,
    shape: *const i64,
    element_type: n::MlirType,
    attribute: n::MlirAttribute,
    data: *mut c_void,
) {
    // SAFETY: `data` points to the collector passed by the enclosing synchronous interface query.
    unsafe {
        Collector::update(data, |values| {
            let shape: Option<Box<[i64]>> = if has_rank {
                Some(native_slice(shape, rank)?.into())
            } else {
                None
            };
            values.push((shape, element_type, attribute));
            Some(())
        })
    };
}

type RawShapedComponents = (Option<Box<[i64]>>, n::MlirType, n::MlirAttribute);

/// One copied result from `InferShapedTypeOpInterface`.
pub struct ShapedTypeComponents<'ctx> {
    /// Inferred shape; `None` represents an unranked result.
    pub shape: Option<Box<[i64]>>,
    /// Inferred element type, when supplied by the interface.
    pub element_type: Option<Type<'ctx>>,
    /// Optional interface-provided type attribute.
    pub attribute: Option<Attribute<'ctx>>,
}

/// The builtin effect kinds and default resource, looked up once per query.
struct KnownEffects {
    kinds: [n::MlirTypeID; 4],
    default_resource: n::MlirSideEffectResource,
}

impl KnownEffects {
    fn get() -> Self {
        // SAFETY: these parameterless getters return process-stable effect IDs by value.
        unsafe {
            Self {
                kinds: [
                    n::mlirMemoryEffectGetEffectID(n::mlirMemoryEffectsAllocateGet()),
                    n::mlirMemoryEffectGetEffectID(n::mlirMemoryEffectsFreeGet()),
                    n::mlirMemoryEffectGetEffectID(n::mlirMemoryEffectsReadGet()),
                    n::mlirMemoryEffectGetEffectID(n::mlirMemoryEffectsWriteGet()),
                ],
                default_resource: n::mlirSideEffectsDefaultResourceGet(),
            }
        }
    }

    /// Copies one native effect instance.
    ///
    /// # Safety
    /// `instance` must be a live effect instance from the operation's context.
    unsafe fn copy<'a>(&self, instance: n::MlirMemoryEffectInstance) -> Option<MemoryEffect<'a>> {
        if instance.ptr.is_null() {
            return None;
        }
        // SAFETY: `instance` was checked non-null and must be a live effect instance under this unsafe method's contract.
        unsafe {
            let effect = n::mlirMemoryEffectInstanceGetEffect(instance);
            let effect_id = n::mlirMemoryEffectGetEffectID(effect);
            let kind = match self
                .kinds
                .iter()
                .position(|known| known.ptr == effect_id.ptr)
            {
                Some(0) => MemoryEffectKind::Allocate,
                Some(1) => MemoryEffectKind::Free,
                Some(2) => MemoryEffectKind::Read,
                Some(3) => MemoryEffectKind::Write,
                _ => MemoryEffectKind::Other,
            };
            let resource = n::mlirMemoryEffectInstanceGetResource(instance);
            let value = n::mlirMemoryEffectInstanceGetValue(instance);
            let symbol = n::mlirMemoryEffectInstanceGetSymbolRef(instance);
            let parameters = n::mlirMemoryEffectInstanceGetParameters(instance);
            Some(MemoryEffect {
                kind,
                resource: if resource.ptr == self.default_resource.ptr {
                    MemoryEffectResource::Default
                } else {
                    MemoryEffectResource::Other
                },
                stage: n::mlirMemoryEffectInstanceGetStage(instance),
                on_full_region: n::mlirMemoryEffectInstanceGetEffectOnFullRegion(instance),
                value: (!value.ptr.is_null()).then(|| {
                    Value::borrowed(value, n::mlirTypeGetContext(n::mlirValueGetType(value)))
                }),
                symbol: (!symbol.ptr.is_null()).then(|| Attribute::borrowed(symbol)),
                parameters: (!parameters.ptr.is_null()).then(|| Attribute::borrowed(parameters)),
            })
        }
    }
}

unsafe extern "C" fn collect_effects(
    count: isize,
    instances: *mut n::MlirMemoryEffectInstance,
    data: *mut c_void,
) {
    // SAFETY: `data` points to the collector supplied by the synchronous memory-effect query.
    unsafe {
        Collector::update(data, |values| {
            let known = KnownEffects::get();
            for instance in native_slice(instances, count)? {
                values.push(known.copy(*instance)?);
            }
            Some(())
        })
    };
}

impl<'ctx, Ownership> Context<'ctx, Ownership> {
    /// Query whether a registered operation name implements `interface`.
    pub fn operation_implements_interface(
        &self,
        operation_name: &str,
        interface: OperationInterface,
    ) -> bool {
        // SAFETY: `string_ref` borrows bytes that remain live until this synchronous C call returns.
        unsafe {
            n::mlirOperationImplementsInterfaceStatic(
                string_ref(operation_name),
                self.raw,
                interface.raw_type_id(),
            )
        }
    }

    /// Checks that inference inputs belong to this context and lowers them to raw handles.
    fn inference_inputs(
        &self,
        location: &Location<'_>,
        operands: &[Value<'_>],
        attributes: Option<&Attribute<'_>>,
    ) -> Result<(Vec<n::MlirValue>, n::MlirAttribute)> {
        // SAFETY: `location` is a live native handle; their wrappers keep them valid throughout `mlirLocationGetContext`.
        let foreign = unsafe {
            n::mlirLocationGetContext(location.raw).ptr != self.raw.ptr
                || operands.iter().any(|operand| {
                    n::mlirTypeGetContext(n::mlirValueGetType(operand.raw)).ptr != self.raw.ptr
                })
                || attributes
                    .is_some_and(|attr| n::mlirAttributeGetContext(attr.raw).ptr != self.raw.ptr)
        };
        if foreign {
            return Err(Error::Invalid(
                "inference input belongs to another context".into(),
            ));
        }
        Ok((
            operands.iter().map(|operand| operand.raw).collect(),
            attributes.map_or_else(n::MlirAttribute::default, |attr| attr.raw),
        ))
    }

    /// Infer operation result types using `InferTypeOpInterface`.
    ///
    /// Pass empty operands and `None` attributes when the operation has none.
    pub fn infer_return_types<'a>(
        &'a self,
        operation_name: &str,
        location: &Location<'_>,
        operands: &[Value<'_>],
        attributes: Option<&Attribute<'_>>,
    ) -> Result<Vec<Type<'a>>> {
        let (mut operands, attributes) = self.inference_inputs(location, operands, attributes)?;
        let mut collector = Collector::<n::MlirType>::default();
        // SAFETY: `operands` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s); the callback collector also outlives the call.
        let result = unsafe {
            n::mlirInferTypeOpInterfaceInferReturnTypes(
                string_ref(operation_name),
                self.raw,
                location.raw,
                count(operands.len())?,
                operands.as_mut_ptr(),
                attributes,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                Some(collect_types),
                ptr::from_mut(&mut collector).cast(),
            )
        };
        if collector.invalid || !n::compiler::logical_result::is_success(result) {
            return Err(Error::Invalid(format!(
                "MLIR failed to infer result types for '{operation_name}'"
            )));
        }
        collector
            .values
            .into_iter()
            .map(|raw| {
                if raw.ptr.is_null() {
                    return Err(Error::Null("inferred MLIR type"));
                }
                Ok(Type::borrowed(raw))
            })
            .collect()
    }

    /// Infer shaped type components using `InferShapedTypeOpInterface`.
    pub fn infer_shaped_return_types<'a>(
        &'a self,
        operation_name: &str,
        location: &Location<'_>,
        operands: &[Value<'_>],
        attributes: Option<&Attribute<'_>>,
    ) -> Result<Vec<ShapedTypeComponents<'a>>> {
        let (mut operands, attributes) = self.inference_inputs(location, operands, attributes)?;
        let mut collector = Collector::<RawShapedComponents>::default();
        // SAFETY: `operands` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s); the callback collector also outlives the call.
        let result = unsafe {
            n::mlirInferShapedTypeOpInterfaceInferReturnTypes(
                string_ref(operation_name),
                self.raw,
                location.raw,
                count(operands.len())?,
                operands.as_mut_ptr(),
                attributes,
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                Some(collect_shaped_types),
                ptr::from_mut(&mut collector).cast(),
            )
        };
        if collector.invalid || !n::compiler::logical_result::is_success(result) {
            return Err(Error::Invalid(format!(
                "MLIR failed to infer shaped result types for '{operation_name}'"
            )));
        }
        Ok(collector
            .values
            .into_iter()
            .map(|(shape, element_type, attribute)| ShapedTypeComponents {
                shape,
                element_type: (!element_type.ptr.is_null()).then(|| Type::borrowed(element_type)),
                attribute: (!attribute.ptr.is_null()).then(|| Attribute::borrowed(attribute)),
            })
            .collect())
    }
}

fn count(len: usize) -> Result<isize> {
    isize::try_from(len).map_err(|_| Error::Invalid("too many inference operands".into()))
}

impl<'ctx, Ownership> crate::Operation<'ctx, Ownership> {
    /// Query whether this operation implements a public MLIR interface.
    pub fn implements_interface(&self, interface: OperationInterface) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirOperationImplementsInterface`.
        unsafe { n::mlirOperationImplementsInterface(self.raw, interface.raw_type_id()) }
    }

    /// Query conditional speculatability when the operation implements that interface.
    pub fn speculatability(&self) -> Result<Option<Speculatability>> {
        if !self.implements_interface(OperationInterface::ConditionallySpeculatable) {
            return Ok(None);
        }
        // SAFETY: the live operation implements the queried interface.
        let value =
// SAFETY: `self` retains this valid native handle and its context throughout `mlirConditionallySpeculatableOpInterfaceGetSpeculatability`.
            unsafe { n::mlirConditionallySpeculatableOpInterfaceGetSpeculatability(self.raw) };
        Speculatability::from_native(value).map(Some)
    }

    /// Copy the effects reported by this operation's memory-effects interface.
    ///
    /// The returned values and attributes are borrowed from the operation's
    /// context and cannot outlive this borrow of the operation.
    pub fn memory_effects(&self) -> Result<Option<Vec<MemoryEffect<'_>>>> {
        if !self.implements_interface(OperationInterface::MemoryEffects) {
            return Ok(None);
        }
        let mut collector = Collector::default();
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirMemoryEffectsOpInterfaceGetEffects`.
        unsafe {
            n::mlirMemoryEffectsOpInterfaceGetEffects(
                self.raw,
                Some(collect_effects),
                ptr::from_mut(&mut collector).cast(),
            );
        }
        if collector.invalid {
            return Err(Error::Invalid(
                "MLIR returned invalid memory-effect data".into(),
            ));
        }
        Ok(Some(collector.values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn unknown_speculatability_values_return_errors(value in any::<i32>()) {
            let known = value == n::MlirSpeculatabilityNotSpeculatable
                || value == n::MlirSpeculatabilitySpeculatable
                || value == n::MlirSpeculatabilityRecursivelySpeculatable;
            prop_assert_eq!(Speculatability::from_native(value).is_ok(), known);
        }
    }

    #[test]
    fn effect_callback_rejects_invalid_spans() {
        let mut collector = Collector::<MemoryEffect<'_>>::default();
        // SAFETY: the test keeps `collector` alive and supplies invalid spans that are rejected before any item read.
        unsafe {
            collect_effects(
                -1,
                std::ptr::null_mut(),
                std::ptr::from_mut(&mut collector).cast(),
            );
        }
        assert!(collector.invalid);
        collector.invalid = false;
        // SAFETY: the test keeps `collector` alive and supplies invalid spans that are rejected before any item read.
        unsafe {
            collect_effects(
                1,
                std::ptr::null_mut(),
                std::ptr::from_mut(&mut collector).cast(),
            );
        }
        assert!(collector.invalid);
    }
}
