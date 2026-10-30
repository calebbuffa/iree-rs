//! Bytecode VM modules loaded from compiled `.vmfb` archives.
use super::{Function, Instance, Module};
use crate::{Code, Error, Result, check};
use core::ffi::c_void;
use iree_native as n;
use n::const_span;

/// Bytecode module loading flags.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BytecodeModuleFlags {
    /// Allows unresolved reference types to resolve to placeholders.
    pub allow_placeholder_types: bool,
}

impl BytecodeModuleFlags {
    fn raw(self) -> n::iree_vm_bytecode_module_flags_t {
        if self.allow_placeholder_types {
            n::IREE_VM_BYTECODE_MODULE_FLAG_ALLOW_PLACEHOLDER_TYPES
        } else {
            n::IREE_VM_BYTECODE_MODULE_FLAG_NONE
        }
    }
}

/// Opaque bytecode feature bits required by an archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeatureBits(n::iree_vm_FeatureBits_enum_t);

impl FeatureBits {
    /// Returns the native feature bits.
    pub const fn bits(self) -> n::iree_vm_FeatureBits_enum_t {
        self.0
    }
}

/// Bytecode disassembly formatting options.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DisassemblyFormat {
    /// Instructions only.
    #[default]
    Default,
    /// Annotates instructions with register values.
    InlineValues,
    /// Prefixes instructions with their bytecode offsets.
    BytecodeOffsets,
}

impl DisassemblyFormat {
    fn raw(self) -> n::iree_vm_bytecode_disassembly_format_t {
        match self {
            Self::Default => n::IREE_VM_BYTECODE_DISASSEMBLY_FORMAT_DEFAULT,
            Self::InlineValues => n::IREE_VM_BYTECODE_DISASSEMBLY_FORMAT_INLINE_VALUES,
            Self::BytecodeOffsets => n::IREE_VM_BYTECODE_DISASSEMBLY_FORMAT_BYTECODE_OFFSETS,
        }
    }
}

/// A VM module executing a compiled bytecode archive.
///
/// The archive stays owned by the native module and is dropped with its final
/// reference. Loading copies it only when its byte buffer is not sufficiently
/// aligned for the VMFB format.
#[derive(Clone)]
pub struct BytecodeModule(Module);

impl BytecodeModule {
    /// Loads an archive held by any byte owner (`Vec<u8>`, `Arc<[u8]>`,
    /// `memmap2::Mmap`, ...). The bytes are copied only when the owner's
    /// buffer does not satisfy the bytecode format's alignment requirement.
    ///
    /// # Safety
    /// The archive must be trusted: its executables may contain native code.
    /// Its contents must not be modified while the module is alive.
    pub unsafe fn new<T: AsRef<[u8]> + Send + 'static>(
        instance: &Instance,
        flags: BytecodeModuleFlags,
        archive: T,
    ) -> Result<Self> {
        let owner = ArchiveOwner::new(archive);
        let bytes = const_span(owner.as_bytes());
        let owner = Box::into_raw(owner);
        let allocator = n::iree_allocator_t {
            self_: owner.cast(),
            ctl: Some(release::<ArchiveOwner<T>>),
        };
        // SAFETY: the archive bytes live inside `owner`, which native frees
        // through `allocator` when the module is destroyed.
        let result = unsafe { Self::create(instance, flags, bytes, allocator) };
        if result.is_err() {
            // SAFETY: failed creation does not take ownership of the allocator.
            drop(unsafe { Box::from_raw(owner) });
        }
        result
    }

    /// Memory-maps and loads an archive file.
    ///
    /// # Safety
    /// The archive must be trusted: its executables may contain native code.
    /// The file must not be modified or truncated while the module is alive.
    pub unsafe fn from_file(
        instance: &Instance,
        flags: BytecodeModuleFlags,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        // SAFETY: the caller guarantees the file is not mutated while mapped.
        let mapping = unsafe { memmap2::Mmap::map(&file) }?;
        // SAFETY: the caller guarantees the archive is trusted.
        unsafe { Self::new(instance, flags, mapping) }
    }

    unsafe fn create(
        instance: &Instance,
        flags: BytecodeModuleFlags,
        archive: n::iree_const_byte_span_t,
        archive_allocator: n::iree_allocator_t,
    ) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the caller keeps the archive valid until `archive_allocator`
        // is invoked; native verifies the archive before using it.
        check(unsafe {
            n::iree_vm_bytecode_module_create(
                instance.as_raw(),
                flags.raw(),
                archive,
                archive_allocator,
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: successful creation returns one owned module reference whose
        // types are registered in `instance`.
        Ok(Self(unsafe { Module::from_parts(raw, instance.clone()) }))
    }

    /// Returns the bytecode features `archive` requires without loading it.
    pub fn required_features(archive: &[u8]) -> Result<FeatureBits> {
        let aligned = (!is_aligned(archive)).then(|| AlignedArchive::copy_from(archive));
        let archive = aligned.as_ref().map_or(archive, AlignedArchive::as_bytes);
        let mut features = 0;
        // SAFETY: the archive and output are valid for the query.
        check(unsafe {
            n::iree_vm_bytecode_module_query_required_features(const_span(archive), &mut features)
        })?;
        Ok(FeatureBits(features))
    }

    /// Borrows the underlying VM module.
    pub fn module(&self) -> &Module {
        &self.0
    }

    /// Returns the underlying VM module.
    pub fn into_module(self) -> Module {
        self.0
    }

    /// Disassembles one function of this module.
    pub fn disassemble(&self, function: &Function, format: DisassemblyFormat) -> Result<String> {
        if function.module_ptr() != self.0.as_raw() {
            return Err(Error::new(
                Code::InvalidArgument,
                "function belongs to another module",
            ));
        }
        // SAFETY: the function ordinal belongs to this live bytecode module.
        crate::base::string_builder::format(|builder| unsafe {
            n::iree_vm_bytecode_module_disassemble_function(
                self.0.as_raw(),
                function.ordinal(),
                format.raw(),
                builder,
            )
        })
    }
}

impl From<BytecodeModule> for Module {
    fn from(module: BytecodeModule) -> Self {
        module.0
    }
}

struct ArchiveOwner<T> {
    source: T,
    aligned: Option<AlignedArchive>,
}

impl<T: AsRef<[u8]>> ArchiveOwner<T> {
    fn new(source: T) -> Box<Self> {
        let mut owner = Box::new(Self {
            source,
            aligned: None,
        });
        // Inline byte owners must reach their final address before alignment
        // is checked or any pointer into them is handed to native code.
        let bytes = owner.source.as_ref();
        owner.aligned = (!is_aligned(bytes)).then(|| AlignedArchive::copy_from(bytes));
        owner
    }

    fn as_bytes(&self) -> &[u8] {
        match &self.aligned {
            Some(aligned) => aligned.as_bytes(),
            None => self.source.as_ref(),
        }
    }
}

#[repr(align(16))]
#[derive(Clone, Copy)]
struct AlignedWord {
    _bytes: [u8; 16],
}

struct AlignedArchive {
    words: Box<[AlignedWord]>,
    len: usize,
}

impl AlignedArchive {
    fn copy_from(bytes: &[u8]) -> Self {
        let word_count = bytes.len().div_ceil(core::mem::size_of::<AlignedWord>());
        let mut words = vec![AlignedWord { _bytes: [0; 16] }; word_count].into_boxed_slice();
        // SAFETY: `words` is initialized and has enough storage for `bytes`;
        // byte access is valid for any initialized object representation.
        let storage = unsafe {
            core::slice::from_raw_parts_mut(
                words.as_mut_ptr().cast::<u8>(),
                words.len() * core::mem::size_of::<AlignedWord>(),
            )
        };
        storage[..bytes.len()].copy_from_slice(bytes);
        Self {
            words,
            len: bytes.len(),
        }
    }

    fn as_bytes(&self) -> &[u8] {
        // SAFETY: the initialized word storage contains at least `len` bytes.
        unsafe { core::slice::from_raw_parts(self.words.as_ptr().cast::<u8>(), self.len) }
    }
}

fn is_aligned(bytes: &[u8]) -> bool {
    bytes
        .as_ptr()
        .align_offset(core::mem::align_of::<AlignedWord>())
        == 0
}

unsafe extern "C" fn release<T>(
    self_: *mut c_void,
    command: n::iree_allocator_command_t,
    _params: *const c_void,
    _inout_ptr: *mut *mut c_void,
) -> n::iree_status_t {
    if command != n::IREE_ALLOCATOR_COMMAND_FREE {
        return n::abi::status::from_code(n::IREE_STATUS_UNIMPLEMENTED);
    }
    n::callback_status(|| {
        // SAFETY: native frees the archive exactly once, on module destruction.
        drop(unsafe { Box::from_raw(self_.cast::<T>()) });
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::{AlignedArchive, AlignedWord, ArchiveOwner, is_aligned};

    struct OffsetArchive(Vec<u8>);

    impl AsRef<[u8]> for OffsetArchive {
        fn as_ref(&self) -> &[u8] {
            &self.0[1..]
        }
    }

    #[test]
    fn inline_archive_is_checked_at_its_stable_address() {
        #[repr(align(16))]
        struct InlineArchive([u8; 16]);
        impl AsRef<[u8]> for InlineArchive {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        let owner = ArchiveOwner::new(InlineArchive([7; 16]));
        assert!(owner.aligned.is_none());
        assert!(is_aligned(owner.as_bytes()));
        assert_eq!(owner.as_bytes().as_ptr(), owner.source.0.as_ptr());
        let pointer = owner.as_bytes().as_ptr();
        let moved_owner = owner;
        assert_eq!(pointer, moved_owner.as_bytes().as_ptr());
        assert_eq!(moved_owner.as_bytes(), [7; 16]);
    }

    #[test]
    fn aligns_unaligned_owned_archives() {
        let owner = ArchiveOwner::new(OffsetArchive(vec![0, 1, 2, 3]));
        assert!(owner.aligned.is_some());
        assert!(is_aligned(owner.as_bytes()));
        assert_eq!(owner.as_bytes(), [1, 2, 3]);
    }

    #[test]
    fn aligned_copy_preserves_bytes_and_alignment() {
        let archive = AlignedArchive::copy_from(&[1, 2, 3, 4, 5]);
        assert!(is_aligned(archive.as_bytes()));
        assert_eq!(archive.as_bytes(), [1, 2, 3, 4, 5]);
        assert_eq!(core::mem::align_of::<AlignedWord>(), 16);
    }
}
