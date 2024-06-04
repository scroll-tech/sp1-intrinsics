# SP1 Intrinsics

This crate wraps the syscall and precompile intrinsics for the SP1 zkVM.

## For Developers

To add a new syscall or precompile, follow these steps to implement it in `sp1`:

1. **Update `SyscallCode` Enum:**
    - Add a new entry to the `SyscallCode` enum.
    - Update `SyscallCode::from_u32` and `test_syscall_consistency_zkvm` in `core/src/runtime/syscall.rs`.

2. **Add ffi interface:**
    - Add the new function name with `syscall_` prefix in `zkvm/entrypoint/src/syscalls`.
    - Mark the function as `#[no_mangle]`

3. **Implement the Syscall Chip:**
    - Add your syscall chip in `sp1/src/syscall/mod.rs`.
    - Implement the `Syscall` trait for your syscall chip.

4. **Add to Default Syscall Map:**
    - Include the new chip in the `default_syscall_map` in `core/src/runtime/syscall.rs`.

5. **Verify the Syscall:**
    - Add the corresponding `Cols` struct.
    - Implement `MachineAir<F>`, `BaseAir<F>`, and `Air<AB>` for the syscall chip.

6. **Update `RiscvAir`:**
    - Add the chip type as a new variant of `RiscvAir`.
    - Update `RiscvAir::get_all` in `core/src/stark/air.rs`.

After implementing the syscall in `sp1`, you can add the wrapper in this crate.

### Safety Considerations

Almost all syscalls involve raw pointer dereferencing and casting.
Therefore, the wrapper function **SHOULD** be marked as `unsafe` 
unless the safety is guaranteed by the function signature 
(e.g., no raw pointers in the signature, and the syscall does not violate safety).

- If marking the function as safe, explain and prove in the comments why it does not need to be marked as `unsafe`.
- If the function is marked as `unsafe`, write a detailed safety section in the comments, listing all assumptions and invariants.

#### Example of a safe wrapper

```
/// `BN254_SCALAR_MUL` syscall implements as `*a = *a * b`,
/// where `a` and `b` are `Fr` elements.
/// The signature ensures write access to `a` and read access to `b`.
/// Also the values of `a` and `b` are `Fr` valid.
fn bn256_scalar_mul(a: &mut Fr, b: &Fr) {
    syscall!(SYSCALL_ID_BN254_SCALAR_MUL, a, b)
}
```