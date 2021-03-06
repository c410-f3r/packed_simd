//! Vertical floating-point `fma`
#![allow(unused)]
use crate::*;

// FIXME: 64-bit 1 element mul_add

crate trait Fma {
    fn fma(self, y: Self, z: Self) -> Self;
}

#[cfg(not(target_arch = "s390x"))]
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.fma.v2f32"]
    fn fma_v2f32(x: f32x2, y: f32x2, z: f32x2) -> f32x2;
    #[link_name = "llvm.fma.v4f32"]
    fn fma_v4f32(x: f32x4, y: f32x4, z: f32x4) -> f32x4;
    #[link_name = "llvm.fma.v8f32"]
    fn fma_v8f32(x: f32x8, y: f32x8, z: f32x8) -> f32x8;
    #[link_name = "llvm.fma.v16f32"]
    fn fma_v16f32(x: f32x16, y: f32x16, z: f32x16) -> f32x16;
    /* FIXME 64-bit single elem vectors
    #[link_name = "llvm.fma.v1f64"]
    fn fma_v1f64(x: f64x1, y: f64x1, z: f64x1) -> f64x1;
    */
    #[link_name = "llvm.fma.v2f64"]
    fn fma_v2f64(x: f64x2, y: f64x2, z: f64x2) -> f64x2;
    #[link_name = "llvm.fma.v4f64"]
    fn fma_v4f64(x: f64x4, y: f64x4, z: f64x4) -> f64x4;
    #[link_name = "llvm.fma.v8f64"]
    fn fma_v8f64(x: f64x8, y: f64x8, z: f64x8) -> f64x8;
}

macro_rules! impl_fma {
    ($id:ident : $fn:ident) => {
        impl Fma for $id {
            #[inline]
            fn fma(self, y: Self, z: Self) -> Self {
                #[cfg(not(target_arch = "s390x"))]
                {
                    unsafe {
                        mem::transmute($fn(
                            mem::transmute(self),
                            mem::transmute(y),
                            mem::transmute(z),
                        ))
                    }
                }
                #[cfg(target_arch = "s390x")]
                {
                    // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
                    self * y + z
                }
            }
        }
    };
}

cfg_if! {
    if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_fma!(f32x2: fma_v2f32);
        impl_fma!(f32x16: fma_v16f32);
        impl_fma!(f64x8: fma_v8f64);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_fma!(f32x4: Sleef_fmaf4_avx2128);
                impl_fma!(f32x8: Sleef_fmaf8_avx2);
                impl_fma!(f64x2: Sleef_fmad2_avx2128);
                impl_fma!(f64x4: Sleef_fmad4_avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_fma!(f32x4: Sleef_fmaf4_sse4);
                impl_fma!(f32x8: Sleef_fmaf8_avx);
                impl_fma!(f64x2: Sleef_fmad2_sse4);
                impl_fma!(f64x4: Sleef_fmad4_avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_fma!(f32x4: Sleef_fmaf4_sse4);
                impl_fma!(f32x8: fma_v8f32);
                impl_fma!(f64x2: Sleef_fmad2_sse4);
                impl_fma!(f64x4: fma_v4f64);
            } else {
                impl_fma!(f32x4: fma_v4f32);
                impl_fma!(f32x8: fma_v8f32);
                impl_fma!(f64x2: fma_v2f64);
                impl_fma!(f64x4: fma_v4f64);
            }
        }
    } else {
        impl_fma!(f32x2: fma_v2f32);
        impl_fma!(f32x4: fma_v4f32);
        impl_fma!(f32x8: fma_v8f32);
        impl_fma!(f32x16: fma_v16f32);
        // impl_fma!(f64x1: fma_v1f64); // FIXME 64-bit fmagle elem vectors
        impl_fma!(f64x2: fma_v2f64);
        impl_fma!(f64x4: fma_v4f64);
        impl_fma!(f64x8: fma_v8f64);
    }
}
