#[cfg(not(all(target_os = "zkvm", target_vendor = "succinct")))]
compile_error!("This crate is only meant to be compiled for sp1 zkvm.");

pub mod bn254;
pub mod memory;

#[macro_export]
macro_rules! syscall {
    ($syscall_id:expr, $arg0:expr, $arg1:expr) => {
        ::core::arch::asm!(
            "ecall",
            in("t0") $syscall_id,
            in("a0") $arg0,
            in("a1") $arg1,
        )
    };
}
