//! Read-only view of a device group's topology: the directed device-to-device
//! edges that describe synchronization, buffer interop and transfer costs.
//!
//! Topologies are built natively when a [`DeviceGroup`](super::DeviceGroup) is
//! created; applications only query them.
use super::bitfield::{bitfield, scalar_value};
use core::marker::PhantomData;
use iree_native as n;

scalar_value! {
    /// How one cross-device operation is performed (`iree_hal_topology_interop_mode_t`).
    TopologyInteropMode: n::iree_hal_topology_interop_mode_t {
        /// Native access without interop.
        NATIVE = n::IREE_HAL_TOPOLOGY_INTEROP_MODE_NATIVE,
        /// Import through an external handle.
        IMPORT = n::IREE_HAL_TOPOLOGY_INTEROP_MODE_IMPORT,
        /// Copy between devices.
        COPY = n::IREE_HAL_TOPOLOGY_INTEROP_MODE_COPY,
        /// The operation is unsupported.
        NONE = n::IREE_HAL_TOPOLOGY_INTEROP_MODE_NONE,
    }
}

scalar_value! {
    /// Physical link between two devices (`iree_hal_topology_link_class_t`).
    TopologyLinkClass: n::iree_hal_topology_link_class_t {
        /// Devices share a die.
        SAME_DIE = n::IREE_HAL_TOPOLOGY_LINK_CLASS_SAME_DIE,
        /// NVLink, Infinity Fabric or an equivalent interconnect.
        NVLINK_IF = n::IREE_HAL_TOPOLOGY_LINK_CLASS_NVLINK_IF,
        /// Devices share a PCIe root complex.
        PCIE_SAME_ROOT = n::IREE_HAL_TOPOLOGY_LINK_CLASS_PCIE_SAME_ROOT,
        /// Devices are on separate PCIe roots.
        PCIE_CROSS_ROOT = n::IREE_HAL_TOPOLOGY_LINK_CLASS_PCIE_CROSS_ROOT,
        /// Transfers are staged through host memory.
        HOST_STAGED = n::IREE_HAL_TOPOLOGY_LINK_CLASS_HOST_STAGED,
        /// A fabric interconnect.
        FABRIC = n::IREE_HAL_TOPOLOGY_LINK_CLASS_FABRIC,
        /// Another link class.
        OTHER = n::IREE_HAL_TOPOLOGY_LINK_CLASS_OTHER,
        /// Devices cannot communicate.
        ISOLATED = n::IREE_HAL_TOPOLOGY_LINK_CLASS_ISOLATED,
    }
}

bitfield! {
    /// Edge capabilities (`iree_hal_topology_capability_t`).
    TopologyCapabilities: n::iree_hal_topology_capability_t {
        /// Devices share a runtime domain.
        SAME_RUNTIME_DOMAIN = n::IREE_HAL_TOPOLOGY_CAPABILITY_SAME_RUNTIME_DOMAIN,
        /// Devices use unified memory.
        UNIFIED_MEMORY = n::IREE_HAL_TOPOLOGY_CAPABILITY_UNIFIED_MEMORY,
        /// Peer memory is coherent.
        PEER_COHERENT = n::IREE_HAL_TOPOLOGY_CAPABILITY_PEER_COHERENT,
        /// Host memory is coherent.
        HOST_COHERENT = n::IREE_HAL_TOPOLOGY_CAPABILITY_HOST_COHERENT,
        /// Direct peer-to-peer copies are supported.
        P2P_COPY = n::IREE_HAL_TOPOLOGY_CAPABILITY_P2P_COPY,
        /// Concurrent access is safe.
        CONCURRENT_SAFE = n::IREE_HAL_TOPOLOGY_CAPABILITY_CONCURRENT_SAFE,
        /// Device-scope atomics are supported.
        ATOMIC_DEVICE = n::IREE_HAL_TOPOLOGY_CAPABILITY_ATOMIC_DEVICE,
        /// System-scope atomics are supported.
        ATOMIC_SYSTEM = n::IREE_HAL_TOPOLOGY_CAPABILITY_ATOMIC_SYSTEM,
        /// Timeline semaphores are supported.
        TIMELINE_SEMAPHORE = n::IREE_HAL_TOPOLOGY_CAPABILITY_TIMELINE_SEMAPHORE,
        /// Only binary semaphores are supported.
        BINARY_SEMAPHORE_ONLY = n::IREE_HAL_TOPOLOGY_CAPABILITY_BINARY_SEMAPHORE_ONLY,
        /// Remote DMA is supported.
        REMOTE_DMA = n::IREE_HAL_TOPOLOGY_CAPABILITY_REMOTE_DMA,
        /// A shared virtual address space is supported.
        SHARED_VIRTUAL_ADDRESS = n::IREE_HAL_TOPOLOGY_CAPABILITY_SHARED_VIRTUAL_ADDRESS,
        /// Peer access requires an explicit grant.
        PEER_ACCESS_REQUIRES_GRANT = n::IREE_HAL_TOPOLOGY_CAPABILITY_PEER_ACCESS_REQUIRES_GRANT,
    }
}

bitfield! {
    /// External handle types usable across an edge (`iree_hal_topology_handle_type_t`).
    TopologyHandleTypes: n::iree_hal_topology_handle_type_t {
        /// Native handles.
        NATIVE = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_NATIVE,
        /// Opaque file descriptors.
        OPAQUE_FD = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_OPAQUE_FD,
        /// Opaque Win32 handles.
        OPAQUE_WIN32 = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_OPAQUE_WIN32,
        /// DMA-BUF handles.
        DMA_BUF = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_DMA_BUF,
        /// RDMA memory registrations.
        RDMA_MR = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_RDMA_MR,
        /// Shared-memory handles.
        SHM = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_SHM,
        /// Metal IOSurface handles.
        METAL_IOSURFACE = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_METAL_IOSURFACE,
        /// Android hardware buffers.
        ANDROID_HARDWARE_BUFFER = n::IREE_HAL_TOPOLOGY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER,
    }
}

bitfield! {
    /// Device-wide capabilities used to build topologies (`iree_hal_device_capability_bits_t`).
    DeviceCapabilityFlags: n::iree_hal_device_capability_bits_t {
        /// Timeline semaphores are supported.
        TIMELINE_SEMAPHORES = n::IREE_HAL_DEVICE_CAPABILITY_TIMELINE_SEMAPHORES,
        /// Memory is unified with the host.
        UNIFIED_MEMORY = n::IREE_HAL_DEVICE_CAPABILITY_UNIFIED_MEMORY,
        /// Host memory is coherent.
        HOST_COHERENT = n::IREE_HAL_DEVICE_CAPABILITY_HOST_COHERENT,
        /// Peer memory is coherent.
        PEER_COHERENT = n::IREE_HAL_DEVICE_CAPABILITY_PEER_COHERENT,
        /// Direct peer-to-peer copies are supported.
        P2P_COPY = n::IREE_HAL_DEVICE_CAPABILITY_P2P_COPY,
        /// Host memory can be used without copies.
        HOST_ZERO_COPY_OK = n::IREE_HAL_DEVICE_CAPABILITY_HOST_ZERO_COPY_OK,
        /// Concurrent access is safe.
        CONCURRENT_SAFE = n::IREE_HAL_DEVICE_CAPABILITY_CONCURRENT_SAFE,
        /// Device-scope atomics are supported.
        ATOMIC_SCOPE_DEVICE = n::IREE_HAL_DEVICE_CAPABILITY_ATOMIC_SCOPE_DEVICE,
        /// System-scope atomics are supported.
        ATOMIC_SCOPE_SYSTEM = n::IREE_HAL_DEVICE_CAPABILITY_ATOMIC_SCOPE_SYSTEM,
        /// The device cannot interoperate with peers.
        ISOLATED = n::IREE_HAL_DEVICE_CAPABILITY_ISOLATED,
        /// Peers can address this device's memory.
        PEER_ADDRESSABLE = n::IREE_HAL_DEVICE_CAPABILITY_PEER_ADDRESSABLE,
        /// The device shares the host virtual address space.
        SHARED_VIRTUAL_ADDRESS = n::IREE_HAL_DEVICE_CAPABILITY_SHARED_VIRTUAL_ADDRESS,
    }
}

/// Capabilities a device reports for topology construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceCapabilities {
    /// Capability flags.
    pub flags: DeviceCapabilityFlags,
    /// Semaphore handle types the device can export.
    pub semaphore_export_types: TopologyHandleTypes,
    /// Semaphore handle types the device can import.
    pub semaphore_import_types: TopologyHandleTypes,
    /// Buffer handle types the device can export.
    pub buffer_export_types: TopologyHandleTypes,
    /// Buffer handle types the device can import.
    pub buffer_import_types: TopologyHandleTypes,
    /// NUMA node the device is attached to.
    pub numa_node: u8,
    /// Physical device UUID, when the driver reports one.
    pub physical_device_uuid: Option<[u8; 16]>,
    /// Opaque driver handle for alias detection; zero when unset.
    pub driver_device_handle: usize,
    /// Same-driver device group index, when the device belongs to one.
    pub device_group_index: Option<u32>,
}
impl DeviceCapabilities {
    pub(crate) fn from_raw(raw: &n::iree_hal_device_capabilities_t) -> Self {
        Self {
            flags: DeviceCapabilityFlags::from_bits_retain(raw.flags),
            semaphore_export_types: TopologyHandleTypes::from_bits_retain(
                raw.semaphore_export_types,
            ),
            semaphore_import_types: TopologyHandleTypes::from_bits_retain(
                raw.semaphore_import_types,
            ),
            buffer_export_types: TopologyHandleTypes::from_bits_retain(raw.buffer_export_types),
            buffer_import_types: TopologyHandleTypes::from_bits_retain(raw.buffer_import_types),
            numa_node: raw.numa_node,
            physical_device_uuid: raw
                .has_physical_device_uuid
                .then_some(raw.physical_device_uuid),
            driver_device_handle: raw.driver_device_handle,
            device_group_index: raw.has_device_group.then_some(raw.device_group_index),
        }
    }
}

/// A device's position and peer reachability within its group topology.
#[derive(Clone, Copy)]
pub struct DeviceTopologyInfo<'a> {
    raw: &'a n::iree_hal_device_topology_info_t,
}
impl<'a> DeviceTopologyInfo<'a> {
    pub(crate) fn new(raw: &'a n::iree_hal_device_topology_info_t) -> Self {
        Self { raw }
    }

    /// Index of this device in the topology.
    pub fn index(&self) -> u32 {
        self.raw.topology_index
    }

    /// The group topology, absent for standalone devices.
    pub fn topology(&self) -> Option<Topology<'a>> {
        // SAFETY: the topology outlives every device in its group.
        unsafe { self.raw.topology.as_ref() }.map(Topology::new)
    }

    /// Peers whose semaphores this device can wait on, as a device bitmap.
    pub fn can_wait_from(&self) -> u64 {
        self.raw.can_wait_from.into()
    }

    /// Peers that can observe semaphores this device signals.
    pub fn can_signal_to(&self) -> u64 {
        self.raw.can_signal_to.into()
    }

    /// Peers this device can import buffers from.
    pub fn can_import_from(&self) -> u64 {
        self.raw.can_import_from.into()
    }

    /// Peers this device can access directly or copy with peer-to-peer.
    pub fn can_p2p_with(&self) -> u64 {
        self.raw.can_p2p_with.into()
    }

    /// Queue-domain frontier axis assigned to this device.
    pub fn frontier_axis(&self) -> u64 {
        self.raw.frontier.base_axis
    }
}

/// Extracts one field from an edge word using the native shift and mask.
const fn field(word: u64, shift: u32, mask: u32) -> u64 {
    (word >> shift) & mask as u64
}

/// One directed edge of a device topology (`iree_hal_topology_edge_t`).
#[derive(Clone, Copy, Debug, Default)]
pub struct TopologyEdge(n::iree_hal_topology_edge_t);

macro_rules! edge_field {
    ($(#[$meta:meta])* $name:ident, $word:ident, $shift:ident, $mask:ident, $out:ty, $wrap:expr) => {
        $(#[$meta])*
        pub const fn $name(self) -> $out {
            // The mask bounds the value to the output width.
            $wrap(field(self.0.$word, n::$shift, n::$mask) as _)
        }
    };
}

impl TopologyEdge {
    /// Returns whether the edge carries no information (devices are not in the same topology).
    pub const fn is_empty(self) -> bool {
        self.0.lo == 0 && self.0.hi == 0
    }
    edge_field!(
        /// How the destination waits on the source's semaphores.
        wait_mode, lo, IREE_HAL_TOPOLOGY_EDGE_WAIT_MODE_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_WAIT_MODE_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// How the source signals semaphores observed by the destination.
        signal_mode, lo, IREE_HAL_TOPOLOGY_EDGE_SIGNAL_MODE_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_SIGNAL_MODE_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// How the destination reads noncoherent source buffers.
        buffer_read_mode_noncoherent, lo, IREE_HAL_TOPOLOGY_EDGE_BUFFER_READ_MODE_NONCOHERENT_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_READ_MODE_NONCOHERENT_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// How the destination writes noncoherent source buffers.
        buffer_write_mode_noncoherent, lo, IREE_HAL_TOPOLOGY_EDGE_BUFFER_WRITE_MODE_NONCOHERENT_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_WRITE_MODE_NONCOHERENT_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// How the destination reads coherent source buffers.
        buffer_read_mode_coherent, lo, IREE_HAL_TOPOLOGY_EDGE_BUFFER_READ_MODE_COHERENT_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_READ_MODE_COHERENT_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// How the destination writes coherent source buffers.
        buffer_write_mode_coherent, lo, IREE_HAL_TOPOLOGY_EDGE_BUFFER_WRITE_MODE_COHERENT_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_WRITE_MODE_COHERENT_MASK, TopologyInteropMode, TopologyInteropMode::from_bits_retain
    );
    edge_field!(
        /// Capabilities of the edge.
        capabilities, lo, IREE_HAL_TOPOLOGY_EDGE_CAPABILITY_FLAGS_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_CAPABILITY_FLAGS_MASK, TopologyCapabilities, TopologyCapabilities::from_bits_retain
    );
    edge_field!(
        /// Relative semaphore-wait cost (0-15).
        wait_cost, lo, IREE_HAL_TOPOLOGY_EDGE_WAIT_COST_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_WAIT_COST_MASK, u8, core::convert::identity
    );
    edge_field!(
        /// Relative semaphore-signal cost (0-15).
        signal_cost, lo, IREE_HAL_TOPOLOGY_EDGE_SIGNAL_COST_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_SIGNAL_COST_MASK, u8, core::convert::identity
    );
    edge_field!(
        /// Relative copy cost (0-15).
        copy_cost, lo, IREE_HAL_TOPOLOGY_EDGE_COPY_COST_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_COPY_COST_MASK, u8, core::convert::identity
    );
    edge_field!(
        /// Latency class (0-15).
        latency_class, lo, IREE_HAL_TOPOLOGY_EDGE_LATENCY_CLASS_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_LATENCY_CLASS_MASK, u8, core::convert::identity
    );
    edge_field!(
        /// NUMA distance (0-15).
        numa_distance, lo, IREE_HAL_TOPOLOGY_EDGE_NUMA_DISTANCE_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_NUMA_DISTANCE_MASK, u8, core::convert::identity
    );
    edge_field!(
        /// Physical link class.
        link_class, lo, IREE_HAL_TOPOLOGY_EDGE_LINK_CLASS_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_LINK_CLASS_MASK, TopologyLinkClass, TopologyLinkClass::from_bits_retain
    );
    edge_field!(
        /// Semaphore handle types the destination can import.
        semaphore_import_types, hi, IREE_HAL_TOPOLOGY_EDGE_SEMAPHORE_IMPORT_TYPES_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_SEMAPHORE_IMPORT_TYPES_MASK, TopologyHandleTypes, TopologyHandleTypes::from_bits_retain
    );
    edge_field!(
        /// Semaphore handle types the source can export.
        semaphore_export_types, hi, IREE_HAL_TOPOLOGY_EDGE_SEMAPHORE_EXPORT_TYPES_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_SEMAPHORE_EXPORT_TYPES_MASK, TopologyHandleTypes, TopologyHandleTypes::from_bits_retain
    );
    edge_field!(
        /// Buffer handle types the destination can import.
        buffer_import_types, hi, IREE_HAL_TOPOLOGY_EDGE_BUFFER_IMPORT_TYPES_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_IMPORT_TYPES_MASK, TopologyHandleTypes, TopologyHandleTypes::from_bits_retain
    );
    edge_field!(
        /// Buffer handle types the source can export.
        buffer_export_types, hi, IREE_HAL_TOPOLOGY_EDGE_BUFFER_EXPORT_TYPES_SHIFT,
        IREE_HAL_TOPOLOGY_EDGE_BUFFER_EXPORT_TYPES_MASK, TopologyHandleTypes, TopologyHandleTypes::from_bits_retain
    );
}

/// The immutable topology of a [`DeviceGroup`](super::DeviceGroup), borrowed for its lifetime.
#[derive(Clone, Copy)]
pub struct Topology<'a> {
    raw: &'a n::iree_hal_topology_t,
    _group: PhantomData<&'a super::DeviceGroup>,
}
impl<'a> Topology<'a> {
    /// Borrows a native topology.
    pub(crate) fn new(raw: &'a n::iree_hal_topology_t) -> Self {
        Self {
            raw,
            _group: PhantomData,
        }
    }

    /// Number of devices in the topology.
    pub fn device_count(&self) -> u32 {
        self.raw.device_count
    }

    /// The directed edge from device `source` to device `destination`, or `None`
    /// when either ordinal is outside the topology.
    pub fn edge(&self, source: u32, destination: u32) -> Option<TopologyEdge> {
        let count = self.device_count() as usize;
        let (source, destination) = (source as usize, destination as usize);
        if source >= count || destination >= count {
            return None;
        }
        self.raw
            .edges
            .get(source * count + destination)
            .copied()
            .map(TopologyEdge)
    }
}
