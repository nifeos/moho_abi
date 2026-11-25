// Generated from method-table.toml by tests/method_table: edit the table, then run
// `MOHO_ABI_REGENERATE=1 cargo test --test method_table`.
/// `invoke(capability, READ_REGISTER, off, _, _)` -> register value. Reads are DMA-safe, so any register
/// in the device's own slot; an offset outside the slot, or not word-aligned, is `WrongObject`. The
/// kernel offers only modern (version 2) devices on either bus, so the `Version` register always
/// reads 2 here.
pub const READ_REGISTER: u64 = 0;
/// `invoke(capability, WRITE_REGISTER, off, val, _)` -> 0. Only DMA-*safe* registers (status, features,
/// interrupt-ack); the queue-address and notify registers are refused (they go through the
/// validated paths below).
pub const WRITE_REGISTER: u64 = 1;
/// `invoke(capability, SETUP_QUEUE, num, queue, _)` -> 0. The kernel programs the given queue's ring
/// addresses to the fixed offsets of that queue's ring block in the driver's DMA region, so the
/// driver never chooses them. `queue` selects the virtqueue (a virtio-net device uses receive =
/// 0, transmit = 1; the disk uses only queue 0, and passing 0 keeps its ABI byte-identical).
/// `WrongObject` when `queue` is out of range, the block does not fit the region, or the queue was never set up.
pub const SETUP_QUEUE: u64 = 2;
/// `invoke(capability, NOTIFY, queue, _, _)` -> 0, or `DeviceRefused` if a newly-published descriptor on
/// that queue points outside the driver's DMA region. On refusal the device is NOT told to go.
/// `queue` selects the virtqueue, as for `SETUP_QUEUE`; each queue keeps its own validated
/// high-water mark, so receive and transmit submits never interfere.
pub const NOTIFY: u64 = 3;
// End of the part generated from method-table.toml.

/// **Where a boot virtio device's DMA region says where it is**: the byte offset, inside the
/// region's first page, at which the kernel writes the region's own physical base as a
/// little-endian `u64` before it grants the region to anyone.
///
/// No method translates a virtual address and a supervisor building a driver has no spare `START`
/// word, so the page carries the base, in its last eight bytes because no driver's rings or buffers
/// reach them. The kernel writes it and the drivers that map the region read it, so it is one constant.
pub const DIRECT_MEMORY_ACCESS_REGION_BASE_PHYSICAL_ADDRESS_WORD_OFFSET: u64 = 4096 - 8;

// The word is the page's last eight bytes; a wrong operator in the subtraction would put it past
// the page or into a driver's rings, and nothing else runs the arithmetic.
const _: () = assert!(DIRECT_MEMORY_ACCESS_REGION_BASE_PHYSICAL_ADDRESS_WORD_OFFSET + 8 == 4096);

// The ring layout `SETUP_QUEUE` builds. The kernel places every queue's rings at these offsets
// and its confinement proves them, and each driver reads and writes its rings at the same
// offsets, so the numbers are the agreement between the two and live here once.

/// **How many entries each queue has**: the largest `num` that `SETUP_QUEUE` takes, and the
/// size every offset below is laid out for. `SETUP_QUEUE` refuses 0 and anything larger with
/// `WrongObject`.
///
/// *(Name provisional.)*
#[doc(alias = "queue size")]
pub const QUEUE_ENTRIES: u16 = 8;

/// **Where a queue's descriptor table starts**, relative to that queue's ring block: 16 bytes
/// for each of the `QUEUE_ENTRIES` descriptors.
///
/// *(Name provisional.)*
#[doc(alias = "descriptor area")]
pub const DESCRIPTOR_TABLE_OFFSET_BYTES: u64 = 0x000;

/// **Where a queue's available ring starts**, relative to its ring block: just past the
/// descriptor table, which is `16 * QUEUE_ENTRIES` bytes long. The ring is `6 + 2 * QUEUE_ENTRIES`
/// bytes (flags, index, one entry per descriptor, the used event word).
///
/// *(Name provisional.)*
#[doc(alias = "driver area")]
pub const AVAILABLE_RING_OFFSET_BYTES: u64 = 0x080;

/// **Where a queue's used ring starts**, relative to its ring block: past the available ring, on
/// the 4-byte alignment the used ring needs. The ring is `6 + 8 * QUEUE_ENTRIES` bytes (flags,
/// index, an 8-byte element per descriptor, the available event word).
///
/// *(Name provisional.)*
#[doc(alias = "device area")]
pub const USED_RING_OFFSET_BYTES: u64 = 0x100;

/// **Where a queue's rings end**, relative to its ring block: the first byte past the used ring.
/// A driver's buffers for that queue start at or above it.
///
/// *(Name provisional.)*
pub const RINGS_END_OFFSET_BYTES: u64 = USED_RING_OFFSET_BYTES + 6 + 8 * QUEUE_ENTRIES as u64;

/// **The stride between successive queues' ring blocks** in the driver's DMA region: queue `q`'s
/// rings are at `q * RING_BLOCK_BYTES` plus the offsets above.
///
/// *(Name provisional.)*
pub const RING_BLOCK_BYTES: u64 = 0x200;

// The layout follows from the queue size and the ring alignments the virtio specification
// requires (descriptor table 16, available ring 2, used ring 4), and each queue's rings fit in
// its block. A wrong edit to any number above breaks the build here.
const _: () = assert!(DESCRIPTOR_TABLE_OFFSET_BYTES.is_multiple_of(16));
const _: () = assert!(
    DESCRIPTOR_TABLE_OFFSET_BYTES + 16 * QUEUE_ENTRIES as u64 <= AVAILABLE_RING_OFFSET_BYTES
);
const _: () = assert!(AVAILABLE_RING_OFFSET_BYTES.is_multiple_of(2));
const _: () =
    assert!(AVAILABLE_RING_OFFSET_BYTES + 6 + 2 * QUEUE_ENTRIES as u64 <= USED_RING_OFFSET_BYTES);
const _: () = assert!(USED_RING_OFFSET_BYTES.is_multiple_of(4));
const _: () = assert!(RINGS_END_OFFSET_BYTES <= RING_BLOCK_BYTES);
// A free-running 16-bit ring index taken modulo the queue size wraps cleanly only when the size
// is a power of two.
const _: () = assert!(QUEUE_ENTRIES.is_power_of_two());
