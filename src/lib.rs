#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_camel_case_types)]

cfg_if::cfg_if! {
    if #[cfg(target_env = "msvc")] {
        #[doc = "Type binding to C's `long double`"]
        pub type c_longdouble = core::ffi::c_double;
    } else if #[cfg(target_arch = "arm")] {
        #[doc = "Type binding to C's `long double`"]
        pub type c_longdouble = f64;
    } else if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        compile_error!("`long double` on x86 is 80-bit extended precision; not yet supported");
    } else if #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))] {
        compile_error!("`long double` on PowerPC is either f128 or doubledouble; not supported");
    } else if #[cfg(any(
        target_arch = "aarch64",
        target_arch = "loongarch64",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64gc",
        target_arch = "s390x"
    ))] {
        compile_error!("long double is f128; this will not be supported until f128 is available in `std`");
    } else {
        compile_error!("Long double type is not known for this target. Please file an issue!");
    }
}
