//! VM lists.
use super::{Ref, RefType, TypeDef, Value, ValueType};
use crate::{Code, Error, Result, check, hal};
use iree_native as n;

/// One list element: a scalar or an owned reference.
pub enum Element {
    /// A scalar value.
    Value(Value),
    /// An owned reference.
    Ref(Ref),
}

impl From<Value> for Element {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

impl From<Ref> for Element {
    fn from(value: Ref) -> Self {
        Self::Ref(value)
    }
}

/// A reference-counted VM list.
///
/// Clones share one native list.
pub struct List {
    raw: *mut n::iree_vm_list_t,
}

impl List {
    /// Creates a list with the given element type and initial capacity.
    pub fn new(element_type: TypeDef, capacity: usize) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the output is writable.
        check(unsafe {
            n::iree_vm_list_create(element_type.raw(), capacity, crate::system(), &mut raw)
        })?;
        Ok(Self { raw })
    }

    /// Adopts an owned list pointer.
    ///
    /// # Safety
    /// `raw` must be a live, owned IREE VM list pointer.
    pub unsafe fn from_raw(raw: *mut n::iree_vm_list_t) -> Self {
        Self { raw }
    }

    /// Borrows the native list pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_list_t {
        self.raw
    }

    /// Transfers the owned pointer to the caller.
    pub fn into_raw(self) -> *mut n::iree_vm_list_t {
        std::mem::ManuallyDrop::new(self).raw
    }

    /// Returns the declared element type.
    pub fn element_type(&self) -> Result<TypeDef> {
        // SAFETY: the list is live.
        TypeDef::from_raw(unsafe { n::iree_vm_list_element_type(self.raw) })
            .ok_or_else(|| Error::new(Code::Internal, "invalid VM list element type"))
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        // SAFETY: the list is live.
        unsafe { n::iree_vm_list_size(self.raw) }
    }

    /// Returns whether the list has no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the element capacity.
    pub fn capacity(&self) -> usize {
        // SAFETY: the list is live.
        unsafe { n::iree_vm_list_capacity(self.raw) }
    }

    /// Ensures capacity for at least `capacity` elements.
    pub fn reserve(&mut self, capacity: usize) -> Result<()> {
        // SAFETY: the list is live.
        check(unsafe { n::iree_vm_list_reserve(self.raw, capacity) })
    }

    /// Resizes the list, zero-initializing new elements and releasing removed ones.
    pub fn resize(&mut self, len: usize) -> Result<()> {
        // SAFETY: the list is live.
        check(unsafe { n::iree_vm_list_resize(self.raw, len) })
    }

    /// Removes and releases every element.
    pub fn clear(&mut self) {
        // SAFETY: the list is live.
        unsafe { n::iree_vm_list_clear(self.raw) }
    }

    /// Reads the scalar at `index`.
    pub fn get_value(&self, index: usize) -> Result<Value> {
        let mut raw = n::iree_vm_value_t::default();
        // SAFETY: the list is live and the output is writable.
        check(unsafe { n::iree_vm_list_get_value(self.raw, index, &mut raw) })?;
        Value::from_raw(raw)
    }

    /// Reads the scalar at `index`, converting it to `value_type`.
    pub fn get_value_as(&self, index: usize, value_type: ValueType) -> Result<Value> {
        let mut raw = n::iree_vm_value_t::default();
        // SAFETY: the list is live and the output is writable.
        check(unsafe {
            n::iree_vm_list_get_value_as(self.raw, index, value_type.raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    /// Replaces the element at `index` with a scalar.
    fn set_value(&mut self, index: usize, value: Value) -> Result<()> {
        let raw = value.into_raw();
        // SAFETY: the list is live and native copies the value.
        check(unsafe { n::iree_vm_list_set_value(self.raw, index, &raw) })
    }

    /// Appends a scalar.
    fn push_value(&mut self, value: Value) -> Result<()> {
        let raw = value.into_raw();
        // SAFETY: the list is live and native copies the value.
        check(unsafe { n::iree_vm_list_push_value(self.raw, &raw) })
    }

    /// Returns a new reference to the element at `index`.
    pub fn get_ref(&self, index: usize) -> Result<Ref> {
        let mut raw = n::iree_vm_ref_t::default();
        // SAFETY: the list is live; the output receives a retained reference.
        check(unsafe { n::iree_vm_list_get_ref_retain(self.raw, index, &mut raw) })?;
        // SAFETY: the reference was retained for us.
        Ok(unsafe { Ref::from_raw(raw) })
    }

    /// Moves the reference at `index` out of the list, leaving null in its place.
    pub fn take_ref(&mut self, index: usize) -> Result<Ref> {
        let mut raw = n::iree_vm_ref_t::default();
        // SAFETY: the list is live; the output receives the list's reference.
        check(unsafe { n::iree_vm_list_get_ref_move(self.raw, index, &mut raw) })?;
        // SAFETY: ownership moved out of the list.
        Ok(unsafe { Ref::from_raw(raw) })
    }

    /// Replaces the element at `index` with `value`.
    fn set_ref(&mut self, index: usize, value: Ref) -> Result<()> {
        let mut raw = value.into_raw();
        // SAFETY: the list is live; on success the list takes the reference.
        let status = unsafe { n::iree_vm_list_set_ref_move(self.raw, index, &mut raw) };
        // SAFETY: on failure the reference is still ours.
        unsafe { restore_ref(status, raw) }
    }

    /// Appends `value`.
    fn push_ref(&mut self, value: Ref) -> Result<()> {
        let mut raw = value.into_raw();
        // SAFETY: the list is live; on success the list takes the reference.
        let status = unsafe { n::iree_vm_list_push_ref_move(self.raw, &mut raw) };
        // SAFETY: on failure the reference is still ours.
        unsafe { restore_ref(status, raw) }
    }

    /// Removes and returns the first element, or `None` if the list is empty.
    pub fn pop_front_ref(&mut self) -> Result<Option<Ref>> {
        if self.is_empty() {
            return Ok(None);
        }
        let mut raw = n::iree_vm_ref_t::default();
        // SAFETY: the list is live; the output receives the list's reference.
        check(unsafe { n::iree_vm_list_pop_front_ref_move(self.raw, &mut raw) })?;
        // SAFETY: ownership moved out of the list.
        Ok(Some(unsafe { Ref::from_raw(raw) }))
    }

    /// Returns the element at `index`, or `None` for an empty variant slot.
    pub fn get(&self, index: usize) -> Result<Option<Element>> {
        let mut raw = n::iree_vm_variant_t::default();
        // SAFETY: the list is live; references are retained for us.
        check(unsafe { n::iree_vm_list_get_variant_retain(self.raw, index, &mut raw) })?;
        // SAFETY: the variant's reference, if any, is owned.
        unsafe { element_from_variant(raw) }
    }

    /// Moves the element at `index` out of the list, or `None` for an empty slot.
    pub fn take(&mut self, index: usize) -> Result<Option<Element>> {
        let mut raw = n::iree_vm_variant_t::default();
        // SAFETY: the list is live; references move out of the list.
        check(unsafe { n::iree_vm_list_get_variant_move(self.raw, index, &mut raw) })?;
        // SAFETY: the variant's reference, if any, is owned.
        unsafe { element_from_variant(raw) }
    }

    /// Replaces the element at `index`.
    pub fn set(&mut self, index: usize, element: impl Into<Element>) -> Result<()> {
        match element.into() {
            Element::Value(value) => self.set_value(index, value),
            Element::Ref(value) => self.set_ref(index, value),
        }
    }

    /// Appends an element.
    pub fn push(&mut self, element: impl Into<Element>) -> Result<()> {
        match element.into() {
            Element::Value(value) => self.push_value(value),
            Element::Ref(value) => self.push_ref(value),
        }
    }

    /// Copies `count` elements starting at `source_index` into `target` at `target_index`.
    pub fn copy_to(
        &self,
        source_index: usize,
        target: &mut List,
        target_index: usize,
        count: usize,
    ) -> Result<()> {
        // Upstream range checks add without overflow checking.
        if source_index.checked_add(count).is_none() || target_index.checked_add(count).is_none() {
            return Err(Error::new(Code::OutOfRange, "VM list range overflows"));
        }
        // SAFETY: both lists are live.
        check(unsafe {
            n::iree_vm_list_copy(self.raw, source_index, target.raw, target_index, count)
        })
    }

    /// Swaps the contents and element types of two lists.
    pub fn swap_storage(&mut self, other: &mut List) {
        // SAFETY: both lists are live; native handles `self == other`.
        unsafe { n::iree_vm_list_swap_storage(self.raw, other.raw) }
    }

    /// Creates a new list holding new references to the same elements.
    pub fn try_clone(&self) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the list is live and the output is writable.
        check(unsafe { n::iree_vm_list_clone(self.raw, crate::system(), &mut raw) })?;
        Ok(Self { raw })
    }

    /// Returns the reference type of VM lists.
    pub fn ref_type() -> RefType {
        // SAFETY: IREE's built-in list type is process-global and has no pointer preconditions.
        unsafe { RefType::from_raw(n::iree_vm_list_type()) }
    }

    /// Returns `self` as a VM reference.
    pub fn into_ref(self) -> Ref {
        // SAFETY: moves our list reference into the returned VM reference.
        unsafe { Ref::from_raw(n::iree_vm_list_move_ref(self.into_raw())) }
    }

    /// Returns the list held by `reference`.
    pub fn from_ref(reference: &Ref) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the reference is valid; the result is type-checked.
        check(unsafe { n::iree_vm_list_check_deref(reference.raw(), &mut raw) })?;
        // SAFETY: `raw` is live; retaining gives the result its own reference.
        unsafe { n::iree_vm_list_retain(raw) };
        Ok(Self { raw })
    }

    /// Returns a new reference to the HAL buffer at `index`, or `None` if it is not one.
    pub fn get_buffer(&self, index: usize) -> Option<hal::Buffer> {
        // SAFETY: the list is live; the result is retained or null.
        let raw = unsafe { n::iree_vm_list_get_buffer_retain(self.raw, index) };
        // SAFETY: the buffer was retained for us.
        (!raw.is_null()).then(|| unsafe { hal::Buffer::from_raw(raw) })
    }

    /// Replaces the element at `index` with a new reference to `buffer`.
    pub fn set_buffer(&mut self, index: usize, buffer: &hal::Buffer) -> Result<()> {
        // SAFETY: the list and buffer are live; native retains the buffer.
        check(unsafe { n::iree_vm_list_set_buffer_retain(self.raw, index, buffer.as_raw()) })
    }

    /// Returns a new reference to the HAL buffer view at `index`, or `None` if it is not one.
    pub fn get_buffer_view(&self, index: usize) -> Option<hal::BufferView> {
        // SAFETY: the list is live; the result is retained or null.
        let raw = unsafe { n::iree_vm_list_get_buffer_view_retain(self.raw, index) };
        // SAFETY: the view was retained for us.
        (!raw.is_null()).then(|| unsafe { hal::BufferView::from_raw(raw) })
    }

    /// Replaces the element at `index` with a new reference to `view`.
    pub fn set_buffer_view(&mut self, index: usize, view: &hal::BufferView) -> Result<()> {
        // SAFETY: the list and view are live; native retains the view.
        check(unsafe { n::iree_vm_list_set_buffer_view_retain(self.raw, index, view.as_raw()) })
    }

    /// Returns a new reference to the HAL fence at `index`, or `None` if it is not one.
    pub fn get_fence(&self, index: usize) -> Option<hal::Fence> {
        // SAFETY: the list is live; the result is retained or null.
        let raw = unsafe { n::iree_vm_list_get_fence_retain(self.raw, index) };
        // SAFETY: the fence was retained for us.
        (!raw.is_null()).then(|| unsafe { hal::Fence::from_raw(raw) })
    }

    /// Replaces the element at `index` with a new reference to `fence`.
    pub fn set_fence(&mut self, index: usize, fence: &hal::Fence) -> Result<()> {
        // SAFETY: the list and fence are live; native retains the fence.
        check(unsafe { n::iree_vm_list_set_fence_retain(self.raw, index, fence.as_raw()) })
    }
}

/// # Safety
/// On failure, `raw` must still be owned by the caller.
unsafe fn restore_ref(status: n::iree_status_t, raw: n::iree_vm_ref_t) -> Result<()> {
    if !status.is_null() {
        // SAFETY: a failed move left the reference with us.
        drop(unsafe { Ref::from_raw(raw) });
    }
    check(status)
}

/// # Safety
/// The variant's reference, if any, must be owned by the caller.
unsafe fn element_from_variant(raw: n::iree_vm_variant_t) -> Result<Option<Element>> {
    match TypeDef::from_raw(raw.type_) {
        Some(TypeDef::Any) => Ok(None),
        // SAFETY: a reference variant's active member is `ref_`, owned per the caller.
        Some(TypeDef::Ref(_)) => Ok(Some(Element::Ref(unsafe {
            Ref::from_raw(raw.__bindgen_anon_1.ref_)
        }))),
        Some(TypeDef::Value(value_type)) => {
            let mut value = n::iree_vm_value_t {
                type_: value_type.raw(),
                ..Default::default()
            };
            // SAFETY: both unions share the same scalar storage member.
            value.__bindgen_anon_1.value_storage = unsafe { raw.__bindgen_anon_1.value_storage };
            Value::from_raw(value).map(|value| Some(Element::Value(value)))
        }
        None => Err(Error::new(Code::Internal, "invalid VM variant type")),
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        // SAFETY: `self.raw` is a live ref-counted list.
        unsafe { n::iree_vm_list_retain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one list reference.
        unsafe { n::iree_vm_list_release(self.raw) }
    }
}
