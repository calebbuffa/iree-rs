//! Feature-gated ports of upstream format parser tests.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

#[cfg(any(
    feature = "io-format-gguf",
    feature = "io-format-irpa",
    feature = "io-format-safetensors"
))]
#[test]
fn parsers_reject_write_only_handles_before_reading() -> ireert::Result<()> {
    use ireert::io::{FileAccess, FileHandle, ParameterIndex};
    let handle = FileHandle::from_bytes(vec![0; 64].into_boxed_slice(), FileAccess::WRITE)?;
    let mut index = ParameterIndex::new()?;
    #[cfg(feature = "io-format-gguf")]
    assert_eq!(
        index.parse_gguf(&handle).unwrap_err().code(),
        ireert::Code::PermissionDenied
    );
    #[cfg(feature = "io-format-irpa")]
    assert_eq!(
        index.parse_irpa(&handle).unwrap_err().code(),
        ireert::Code::PermissionDenied
    );
    #[cfg(feature = "io-format-safetensors")]
    assert_eq!(
        index.parse_safetensors(&handle).unwrap_err().code(),
        ireert::Code::PermissionDenied
    );
    Ok(())
}

#[cfg(any(
    feature = "io-format-gguf",
    feature = "io-format-irpa",
    feature = "io-format-safetensors"
))]
#[test]
fn parsers_reject_truncated_headers() -> ireert::Result<()> {
    use ireert::io::{FileAccess, FileHandle, ParameterIndex};

    #[cfg(feature = "io-format-gguf")]
    {
        let handle = FileHandle::from_bytes(vec![0; 7].into_boxed_slice(), FileAccess::READ)?;
        assert!(ParameterIndex::new()?.parse_gguf(&handle).is_err());
    }
    #[cfg(feature = "io-format-irpa")]
    {
        let handle = FileHandle::from_bytes(vec![0; 7].into_boxed_slice(), FileAccess::READ)?;
        assert!(ParameterIndex::new()?.parse_irpa(&handle).is_err());
    }
    #[cfg(feature = "io-format-safetensors")]
    {
        let handle = FileHandle::from_bytes(vec![0; 7].into_boxed_slice(), FileAccess::READ)?;
        assert!(ParameterIndex::new()?.parse_safetensors(&handle).is_err());
    }
    Ok(())
}

#[cfg(feature = "io-format-gguf")]
mod gguf {
    use ireert::io::{FileAccess, FileHandle, ParameterIndex, ParameterStorage};

    fn parse(bytes: &'static [u8]) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_owner(bytes)?;
        let mut index = ParameterIndex::new()?;
        index.parse_gguf(&handle)?;
        Ok(index)
    }

    fn parse_owned(bytes: Vec<u8>) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_bytes(bytes.into_boxed_slice(), FileAccess::READ)?;
        let mut index = ParameterIndex::new()?;
        index.parse_gguf(&handle)?;
        Ok(index)
    }

    #[test]
    fn empty() {
        assert!(
            parse(include_bytes!("fixtures/gguf/empty.gguf"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn single_tensor_and_v2() {
        for bytes in [
            &include_bytes!("fixtures/gguf/single.gguf")[..],
            &include_bytes!("fixtures/gguf/single_v2.gguf")[..],
        ] {
            let index = parse(bytes).unwrap();
            let entry = index.lookup("tensor0").unwrap();
            assert_eq!(entry.key, "tensor0");
            assert!(entry.metadata.is_empty());
            assert_eq!(entry.storage, ParameterStorage::File { offset: 384 });
            assert_eq!(entry.length, 16);
        }
    }

    #[test]
    fn multiple_tensors() {
        let index = parse(include_bytes!("fixtures/gguf/multiple.gguf")).unwrap();
        for (key, offset, length) in [
            ("tensor0", 448, 16),
            ("tensor1", 512, 8),
            ("tensor2", 576, 48),
        ] {
            let entry = index.lookup(key).unwrap();
            assert_eq!(entry.key, key);
            assert!(entry.metadata.is_empty());
            assert_eq!(entry.storage, ParameterStorage::File { offset });
            assert_eq!(entry.length, length);
        }
    }

    #[test]
    fn rejects_invalid_magic_version_and_truncated_tensor_table() {
        let fixture = include_bytes!("fixtures/gguf/single.gguf");

        let mut invalid_magic = fixture.to_vec();
        invalid_magic[..4].copy_from_slice(b"NOPE");
        assert_eq!(
            parse_owned(invalid_magic).err().unwrap().code(),
            ireert::Code::InvalidArgument
        );

        let mut unsupported_version = fixture.to_vec();
        unsupported_version[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            parse_owned(unsupported_version).err().unwrap().code(),
            ireert::Code::Unimplemented
        );

        let mut truncated_table = fixture.to_vec();
        truncated_table[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(parse_owned(truncated_table).is_err());

        let mut truncated_metadata = fixture.to_vec();
        truncated_metadata[16..24].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(parse_owned(truncated_metadata).is_err());

        let mut unsupported_tensor_type = fixture.to_vec();
        unsupported_tensor_type[318..322].copy_from_slice(&u32::MAX.to_le_bytes());
        assert_eq!(
            parse_owned(unsupported_tensor_type).err().unwrap().code(),
            ireert::Code::Unimplemented
        );
    }
}

#[cfg(feature = "io-format-irpa")]
mod irpa {
    use ireert::io::{FileAccess, FileHandle, ParameterIndex, ParameterStorage};

    fn parse(bytes: &'static [u8]) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_owner(bytes)?;
        let mut index = ParameterIndex::new()?;
        index.parse_irpa(&handle)?;
        Ok(index)
    }

    fn parse_owned(bytes: Vec<u8>) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_bytes(bytes.into_boxed_slice(), FileAccess::READ)?;
        let mut index = ParameterIndex::new()?;
        index.parse_irpa(&handle)?;
        Ok(index)
    }

    #[test]
    fn empty() {
        assert!(
            parse(include_bytes!("fixtures/irpa/empty.irpa"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn single_and_multiple_parameters() {
        let single = parse(include_bytes!("fixtures/irpa/single.irpa")).unwrap();
        let entry = single.lookup("key0").unwrap();
        assert_eq!(entry.key, "key0");
        assert!(entry.metadata.is_empty());
        assert_eq!(entry.storage, ParameterStorage::File { offset: 192 });
        assert_eq!(entry.length, 16);

        let multiple = parse(include_bytes!("fixtures/irpa/multiple.irpa")).unwrap();
        for (key, offset, length) in [("key0", 320, 16), ("key1", 384, 5)] {
            let entry = multiple.lookup(key).unwrap();
            assert_eq!(entry.key, key);
            assert!(entry.metadata.is_empty());
            assert_eq!(entry.storage, ParameterStorage::File { offset });
            assert_eq!(entry.length, length);
        }
    }

    #[test]
    fn mixed_file_and_splat_entries() {
        let index = parse(include_bytes!("fixtures/irpa/mixed.irpa")).unwrap();
        for (key, offset, length) in [("key0", 512, 16), ("key1", 576, 5)] {
            let entry = index.lookup(key).unwrap();
            assert!(entry.metadata.is_empty());
            assert_eq!(entry.storage, ParameterStorage::File { offset });
            assert_eq!(entry.length, length);
        }
        assert_eq!(
            index.lookup("key2").unwrap().storage,
            ParameterStorage::Splat(&[102])
        );
        assert_eq!(index.lookup("key2").unwrap().length, 1);
        assert_eq!(
            index.lookup("key3").unwrap().storage,
            ParameterStorage::Splat(&i64::MAX.to_ne_bytes())
        );
        assert_eq!(index.lookup("key3").unwrap().length, 33_554_432);
    }

    #[test]
    fn rejects_invalid_magic_and_segment_ranges() {
        let fixture = include_bytes!("fixtures/irpa/single.irpa");

        let mut invalid_magic = fixture.to_vec();
        invalid_magic[..4].copy_from_slice(b"NOPE");
        assert_eq!(
            parse_owned(invalid_magic).err().unwrap().code(),
            ireert::Code::InvalidArgument
        );

        let mut out_of_range_segment = fixture.to_vec();
        out_of_range_segment[40..48].copy_from_slice(&u64::MAX.to_le_bytes());
        assert_eq!(
            parse_owned(out_of_range_segment).err().unwrap().code(),
            ireert::Code::OutOfRange
        );
    }
}

#[cfg(feature = "io-format-safetensors")]
mod safetensors {
    use ireert::io::{FileAccess, FileHandle, ParameterIndex, ParameterStorage};

    fn parse(bytes: &'static [u8]) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_owner(bytes)?;
        let mut index = ParameterIndex::new()?;
        index.parse_safetensors(&handle)?;
        Ok(index)
    }

    fn parse_owned(bytes: Vec<u8>) -> ireert::Result<ParameterIndex> {
        let handle = FileHandle::from_bytes(bytes.into_boxed_slice(), FileAccess::READ)?;
        let mut index = ParameterIndex::new()?;
        index.parse_safetensors(&handle)?;
        Ok(index)
    }

    fn with_header(header: &str, data: &[u8]) -> Vec<u8> {
        let mut bytes = (header.len() as u64).to_le_bytes().to_vec();
        bytes.extend_from_slice(header.as_bytes());
        bytes.extend_from_slice(data);
        bytes
    }

    #[test]
    fn empty() {
        assert!(
            parse(include_bytes!("fixtures/safetensors/empty.safetensors"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn single_and_multiple_tensors() {
        let single = parse(include_bytes!("fixtures/safetensors/single.safetensors")).unwrap();
        let entry = single.lookup("tensor0").unwrap();
        assert_eq!(entry.key, "tensor0");
        assert!(entry.metadata.is_empty());
        assert_eq!(entry.storage, ParameterStorage::File { offset: 72 });
        assert_eq!(entry.length, 16);

        let multiple = parse(include_bytes!("fixtures/safetensors/multiple.safetensors")).unwrap();
        for (key, offset, length) in [
            ("tensor0", 200, 16),
            ("tensor1", 216, 8),
            ("tensor2", 224, 48),
        ] {
            let entry = multiple.lookup(key).unwrap();
            assert_eq!(entry.key, key);
            assert!(entry.metadata.is_empty());
            assert_eq!(entry.storage, ParameterStorage::File { offset });
            assert_eq!(entry.length, length);
        }
    }

    #[test]
    fn rejects_malformed_and_out_of_range_data_offsets() {
        let malformed = with_header(r#"{"tensor":{"data_offsets":"[0, 1]"}}"#, &[]);
        assert_eq!(
            parse_owned(malformed).err().unwrap().code(),
            ireert::Code::InvalidArgument
        );

        for header in [
            r#"{"tensor":{"data_offsets":[1,0]}}"#,
            r#"{"tensor":{"data_offsets":[0,1]}}"#,
        ] {
            assert_eq!(
                parse_owned(with_header(header, &[])).err().unwrap().code(),
                ireert::Code::InvalidArgument
            );
        }
    }

    #[test]
    fn metadata_empty_ranges_and_ignored_tensor_fields() {
        let header = r#"{"__metadata__":{"format":"pt"},"tensor":{"dtype":"F32","shape":[2],"data_offsets":[0,8]},"empty":{"data_offsets":[8,8]}}"#;
        let bytes = with_header(header, &[0; 8]);
        let index = parse_owned(bytes).unwrap();
        assert_eq!(index.len(), 2);

        let tensor = index.lookup("tensor").unwrap();
        assert_eq!(tensor.length, 8);
        assert_eq!(
            tensor.storage,
            ParameterStorage::File {
                offset: 8 + header.len() as u64
            }
        );
        let empty = index.lookup("empty").unwrap();
        assert_eq!(empty.length, 0);
        assert_eq!(
            empty.storage,
            ParameterStorage::File {
                offset: 8 + header.len() as u64 + 8
            }
        );
        assert_eq!(
            index.lookup("__metadata__").unwrap_err().code(),
            ireert::Code::NotFound
        );
    }

    #[test]
    fn rejects_invalid_header_lengths_and_offset_arrays() {
        let mut truncated_header = 16_u64.to_le_bytes().to_vec();
        truncated_header.extend_from_slice(b"{}");
        assert!(parse_owned(truncated_header).is_err());

        for offsets in ["[]", "[0]", "[-1,0]", "[0, 18446744073709551616]"] {
            let header = format!(r#"{{"tensor":{{"data_offsets":{offsets}}}}}"#);
            assert!(
                parse_owned(with_header(&header, &[0; 4])).is_err(),
                "accepted invalid offsets {offsets}"
            );
        }

        let missing_offsets = with_header(r#"{"tensor":{"dtype":"F32","shape":[1]}}"#, &[0; 4]);
        assert!(parse_owned(missing_offsets).is_err());
    }
}
