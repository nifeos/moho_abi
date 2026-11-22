/// `invoke(capability, WAIT, _, _, _)` -> 1. **Blocks until the interrupt fires.** The kernel masks
/// the interrupt when it fires and hands it to us as a message; nothing device-specific
/// happens in the kernel.
pub const WAIT: u64 = 0;

/// `invoke(capability, ACKNOWLEDGE, _, _, _)` -> 0. Re-enable the interrupt at the generic interrupt controller (GIC), once we
/// have quieted the device. Until we call this, the interrupt stays masked and cannot storm.
pub const ACKNOWLEDGE: u64 = 1;
