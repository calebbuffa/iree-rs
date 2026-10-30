//! HAL buffer CTS ports; test panics surface assertion and setup failures.
// Test-only panics surface CTS assertion and setup failures; they are not library behavior.

#[path = "support\\cts.rs"]
mod cts;
#[path = "support\\cts_buffer.rs"]
mod cts_buffer;

use cts::devices;
use cts_buffer::{buffer, read};
use ireert::hal::{BufferParams, BufferUsage, MemoryAccess, MemoryType};

const DEFAULT_SIZE: ireert::hal::DeviceSize = 1024;

#[test]
fn allocator_supports_buffer_mapping() {
    for (_, _, device) in devices().unwrap() {
        let allocator = device.allocator();
        let params = BufferParams {
            memory_type: MemoryType::HOST_VISIBLE,
            usage: BufferUsage::MAPPING,
            ..BufferParams::default()
        };
        let (compatibility, _, allocation_size) =
            allocator.query_buffer_compatibility(params, DEFAULT_SIZE);
        assert!(compatibility.contains(ireert::hal::allocator::BufferCompatibility::ALLOCATABLE));
        let b = allocator
            .allocate_buffer(
                BufferParams {
                    memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
                    usage: BufferUsage::TRANSFER | BufferUsage::MAPPING,
                    ..params
                },
                allocation_size,
            )
            .unwrap();
        assert!(b.memory_type().contains(params.memory_type));
        assert!(b.allowed_usage().contains(params.usage));
        assert!(b.allocation_size() >= allocation_size);
    }
}

#[test]
fn zero_whole_buffer() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, DEFAULT_SIZE).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        assert_eq!(read(&b, 0, DEFAULT_SIZE).unwrap(), vec![0; DEFAULT_SIZE]);
    }
}
#[test]
fn zero_with_offset() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.fill(0, ireert::hal::DeviceSize::MAX, &[0xff]).unwrap();
        b.zero(4, 8).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0xff; 4], vec![0; 8], vec![0xff; 4]].concat()
        );
    }
}
#[test]
fn zero_subspan() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.fill(0, ireert::hal::DeviceSize::MAX, &[0xff]).unwrap();
        let s = b.subspan(4, 8).unwrap();
        s.zero(4, 4).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0xff; 8], vec![0; 4], vec![0xff; 4]].concat()
        );
        assert_eq!(
            read(&s, 0, 8).unwrap(),
            [vec![0xff; 4], vec![0; 4]].concat()
        );
    }
}
#[test]
fn fill_empty() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, DEFAULT_SIZE).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        b.fill(0, 0, &[0xff]).unwrap();
        assert_eq!(read(&b, 0, DEFAULT_SIZE).unwrap(), vec![0; DEFAULT_SIZE]);
    }
}
#[test]
fn fill_whole_buffer() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, DEFAULT_SIZE).unwrap();
        b.fill(0, ireert::hal::DeviceSize::MAX, &[0xff]).unwrap();
        assert_eq!(read(&b, 0, DEFAULT_SIZE).unwrap(), vec![0xff; DEFAULT_SIZE]);
    }
}
#[test]
fn fill_with_offset() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        b.fill(4, 8, &[0xff]).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0; 4], vec![0xff; 8], vec![0; 4]].concat()
        );
    }
}
#[test]
fn fill_subspan() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        let s = b.subspan(4, 8).unwrap();
        s.fill(4, 4, &[0xff]).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0; 8], vec![0xff; 4], vec![0; 4]].concat()
        );
        assert_eq!(
            read(&s, 0, 8).unwrap(),
            [vec![0; 4], vec![0xff; 4]].concat()
        );
    }
}
#[test]
fn read_data() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.zero(0, 8).unwrap();
        b.fill(8, 8, &[0xff]).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0; 8], vec![0xff; 8]].concat()
        );
        assert_eq!(
            read(&b, 4, 8).unwrap(),
            [vec![0; 4], vec![0xff; 4]].concat()
        );
    }
}
#[test]
fn read_data_subspan() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        for (offset, value) in [(0, 0xaa), (4, 0xbb), (8, 0xcc), (12, 0xdd)] {
            b.fill(offset, 4, &[value]).unwrap();
        }
        let s = b.subspan(4, 8).unwrap();
        assert_eq!(
            read(&s, 0, 8).unwrap(),
            [vec![0xbb; 4], vec![0xcc; 4]].concat()
        );
        assert_eq!(read(&s, 4, 4).unwrap(), vec![0xcc; 4]);
    }
}
#[test]
fn write_data_whole_buffer() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        let expected = vec![0xff; 16];
        b.write(0, &expected).unwrap();
        assert_eq!(read(&b, 0, 16).unwrap(), expected);
    }
}
#[test]
fn write_data_with_offset() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        b.write(4, &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88])
            .unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [
                vec![0; 4],
                vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
                vec![0; 4]
            ]
            .concat()
        );
    }
}
#[test]
fn write_data_subspan() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.zero(0, ireert::hal::DeviceSize::MAX).unwrap();
        let s = b.subspan(4, 8).unwrap();
        s.write(4, &[0x11, 0x22, 0x33, 0x44]).unwrap();
        assert_eq!(
            read(&b, 0, 16).unwrap(),
            [vec![0; 8], vec![0x11, 0x22, 0x33, 0x44], vec![0; 4]].concat()
        );
        assert_eq!(
            read(&s, 0, 8).unwrap(),
            [vec![0; 4], vec![0x11, 0x22, 0x33, 0x44]].concat()
        );
    }
}
#[test]
fn copy_data() {
    for (_, _, d) in devices().unwrap() {
        let a = buffer(&d, DEFAULT_SIZE).unwrap();
        let b = buffer(&d, DEFAULT_SIZE).unwrap();
        a.fill(0, DEFAULT_SIZE, &[7]).unwrap();
        b.copy_from(0, &a, 0, DEFAULT_SIZE).unwrap();
        assert_eq!(read(&b, 0, DEFAULT_SIZE).unwrap(), vec![7; DEFAULT_SIZE]);
    }
}
#[test]
fn map_range_read() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap();
        b.fill(0, ireert::hal::DeviceSize::MAX, &[0xef]).unwrap(); // SAFETY: this test is the only access while the scoped mapping is live.
        let mapping = unsafe { b.map_range(0, 16, MemoryAccess::READ).unwrap() };
        assert!(mapping.as_bytes().unwrap().len() >= 16);
        assert_eq!(&mapping.as_bytes().unwrap()[..16], &[0xef; 16]);
        mapping.unmap().unwrap();
    }
}
#[test]
fn map_range_write() {
    for (_, _, d) in devices().unwrap() {
        let b = buffer(&d, 16).unwrap(); // SAFETY: this test is the only access while the scoped mapping is live.
        let mut mapping = unsafe { b.map_range(0, 16, MemoryAccess::DISCARD_WRITE).unwrap() };
        assert!(mapping.as_bytes_mut().is_err());
        assert!(mapping.as_uninit_bytes_mut().unwrap().len() >= 16);
        for byte in &mut mapping.as_uninit_bytes_mut().unwrap()[..16] {
            byte.write(0x12);
        }
        mapping.flush(0, 16).unwrap();
        mapping.unmap().unwrap();
        assert_eq!(read(&b, 0, 16).unwrap(), vec![0x12; 16]);
    }
}
#[test]
fn map_range_multiple_mappings() {
    for (_, _, d) in devices().unwrap() {
        let b = d
            .allocator()
            .allocate_buffer(
                BufferParams {
                    memory_type: MemoryType::HOST_LOCAL | MemoryType::DEVICE_VISIBLE,
                    usage: BufferUsage::TRANSFER | BufferUsage::MAPPING_SCOPED,
                    ..BufferParams::default()
                },
                32,
            )
            .unwrap(); // SAFETY: mappings are disjoint or read-only and the test keeps their accesses ordered.
        let mut first = unsafe { b.map_range(0, 32, MemoryAccess::WRITE).unwrap() };
        for byte in &mut first.as_uninit_bytes_mut().unwrap()[..32] {
            byte.write(0x12);
        }
        // SAFETY: the read-only mapping may overlap the live write mapping; accesses are ordered.
        let second = unsafe { b.map_range(0, 16, MemoryAccess::READ).unwrap() };
        assert_eq!(second.as_bytes().unwrap()[0], 0x12);
        first.unmap().unwrap();
        assert_eq!(second.as_bytes().unwrap()[0], 0x12);
        // SAFETY: this write mapping is disjoint from the first and follows the completed read.
        let mut third = unsafe { b.map_range(16, 16, MemoryAccess::WRITE).unwrap() };
        second.unmap().unwrap();
        for byte in &mut third.as_uninit_bytes_mut().unwrap()[..16] {
            byte.write(0x34);
        }
        third.unmap().unwrap();
    }
}
