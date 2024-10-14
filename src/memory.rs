//! Memory intrinsics for SP1 zkVM.

/// `MEMCPY_32` syscall ID.
pub const SYSCALL_ID_MEMCPY_32: u32 = 0x00_01_01_90;
/// `MEMCPY_64` syscall ID.
pub const SYSCALL_ID_MEMCPY_64: u32 = 0x00_01_01_91;

/// Create 32 bytes bitwise copy from `src` to `dst`.
/// The source and destination may overlap.
///
/// Like `core::ptr::copy`, the safety requirements are the same.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be [valid] for reads of 32 bytes, and must remain valid even
///   when `dst` is written for 32 bytes. (This means if the memory ranges overlap,
///   the two pointers must not be subject to aliasing restrictions relative to each other.)
///
/// * `dst` must be [valid] for writes of 32 bytes, and must remain valid even
///   when `src` is read for 32 bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// If type be copied is not [`Copy`], using both the values in
/// the region beginning at `*src` and the region beginning at `*dst` can
/// [violate memory safety][read-ownership].
///
/// [valid]: core::ptr#safety
/// [`read`]: core::ptr::read
/// [`Copy`]: core::marker::Copy
/// [read-ownership]: core::ptr::read#ownership-of-the-returned-value
#[inline(always)]
pub unsafe fn memcpy32<T>(src: *const T, dst: *mut T) {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "disable-memcpy-syscalls"))] {
                crate::syscall!(SYSCALL_ID_MEMCPY_32, src, dst)
            } else {
                core::ptr::copy(src as *const u8, dst as *mut u8, 64);
            }
        }
    }
}

/// Create 64 bytes bitwise copy from `src` to `dst`.
/// The source and destination may overlap.
///
/// Like `core::ptr::copy`, the safety requirements are the same.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be [valid] for reads of 64 bytes, and must remain valid even
///   when `dst` is written for 64 bytes. (This means if the memory ranges overlap,
///   the two pointers must not be subject to aliasing restrictions relative to each other.)
///
/// * `dst` must be [valid] for writes of 64 bytes, and must remain valid even
///   when `src` is read for 64 bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// If type be copied is not [`Copy`], using both the values in
/// the region beginning at `*src` and the region beginning at `*dst` can
/// [violate memory safety][read-ownership].
///
/// [valid]: core::ptr#safety
/// [`read`]: core::ptr::read
/// [`Copy`]: core::marker::Copy
/// [read-ownership]: core::ptr::read#ownership-of-the-returned-value
#[inline(always)]
pub unsafe fn memcpy64<T>(src: *const T, dst: *mut T) {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "disable-memcpy-syscalls"))] {
                crate::syscall!(SYSCALL_ID_MEMCPY_64, src, dst)
            } else {
                core::ptr::copy(src as *const u8, dst as *mut u8, 64);
            }
        }
    }
}
