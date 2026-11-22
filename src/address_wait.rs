/// The word is 8 bits. Reserved; refused.
pub const WIDTH_8_BITS: u64 = 0;
/// The word is 16 bits. Reserved; refused.
pub const WIDTH_16_BITS: u64 = 1;
/// The word is 32 bits: the one size admitted, and the one `std`'s address wait locks use.
pub const WIDTH_32_BITS: u64 = 2;
/// The word is 64 bits. Reserved; refused.
pub const WIDTH_64_BITS: u64 = 3;
/// Where the size lives.
pub const WIDTH_MASK: u64 = 3;
/// Keyed by (address space, address): only threads of one space meet. Without it the address wait is
/// shared, keyed by (memory object, offset), which is reserved and which this kernel refuses.
pub const PRIVATE: u64 = 1 << 7;

/// **Is this a `flags` word the kernel answers?** Exactly one today.
pub const fn is_admitted(flags: u64) -> bool {
    flags == PRIVATE | WIDTH_32_BITS
}
