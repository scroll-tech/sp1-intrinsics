//! bn254 scalar operation

/// `BN254_SCALAR_MUL` syscall ID.
pub const BN254_SCALAR_MUL: u32 = 0x00_01_01_80;

/// `BN254_SCALAR_MAC` syscall ID.
pub const BN254_SCALAR_MAC: u32 = 0x00_01_01_81;

/// Perform in-place scalar multiplication `p *= q`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `p` must be [valid] for writes of [`bn254::Fr`], and must remain valid even
///   when `q` is read for [`bn254::Fr`].
///
/// * `q` must be [valid] for reads of [`bn254::Fr`].
///
/// * Both `p` and `q` must be properly aligned and not overlap.
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_mul<P, Q>(p: *mut P, q: *const Q) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MUL, p, q)
    }
}

/// Perform in-place scalar multiplication and addition `ret += a + b`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `ret` must be [valid] for writes of [`bn254::Fr`], and must remain valid even
///   when `a` and `b` are read for [`bn254::Fr`].
///
/// * `a` and `b` must be [valid] for reads of [`bn254::Fr`].
///
/// * Both `ret`, `a`, and `b` must be properly aligned and not overlap.
#[inline(always)]
pub unsafe fn syscall_bn254_scalar_mac<R, T>(ret: *mut R, a: *const T, b: *const T) {
    unsafe {
        crate::syscall!(BN254_SCALAR_MAC, ret, &[a, b])
    }
}
