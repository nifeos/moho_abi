// Generated from method-table.toml by tests/method_table: edit the table, then run
// `MOHO_ABI_REGENERATE=1 cargo test --test method_table`.
/// `invoke(capability, REVOKE, _, _, _)` -> 0. **Take the ports back from everyone else.** Delete every
/// `PortRange` capability naming this range from every other thread's table and clear the TSS
/// bitmap that granted it, so those holders fault on their next `in`/`out`, while the caller
/// keeps its own. Needs `GRANT` (you were trusted to lend the ports on, so you may take them
/// back), the same rule and the same asymmetry `DeviceFrame`'s take-back uses: the kernel mints
/// a port capability once, at boot, so a symmetric revoke would strand the device forever.
pub const REVOKE: u64 = 1;
// End of the part generated from method-table.toml.
