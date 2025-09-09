use std::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn dot_product(left: &[f64], right: &[f64]) -> f64 {
    if std::is_x86_feature_detected!("avx") {
        unsafe { dot_product_avx(left, right) }
    } else if std::is_x86_feature_detected!("sse3") {
        unsafe { dot_product_sse3(left, right) }
    } else {
        dot_product_scalar(left, right)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn dot_product(left: &[f64], right: &[f64]) -> f64 {
    dot_product_scalar(left, right)
}

#[inline]
fn dot_product_scalar(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), 12);
    assert_eq!(right.len(), 12);

    let mut sum = [0.0f64; 4];
    let letf_ptr = left.as_ptr();
    let right_ptr = right.as_ptr();

    unsafe {
        sum[0] += *letf_ptr * *right_ptr;
        sum[1] += *letf_ptr.add(1) * *right_ptr.add(1);
        sum[2] += *letf_ptr.add(2) * *right_ptr.add(2);
        sum[3] += *letf_ptr.add(3) * *right_ptr.add(3);
        sum[0] += *letf_ptr.add(4) * *right_ptr.add(4);
        sum[1] += *letf_ptr.add(5) * *right_ptr.add(5);
        sum[2] += *letf_ptr.add(6) * *right_ptr.add(6);
        sum[3] += *letf_ptr.add(7) * *right_ptr.add(7);
        sum[0] += *letf_ptr.add(8) * *right_ptr.add(8);
        sum[1] += *letf_ptr.add(9) * *right_ptr.add(9);
        sum[2] += *letf_ptr.add(10) * *right_ptr.add(10);
        sum[3] += *letf_ptr.add(11) * *right_ptr.add(11);
    }

    sum[0] + sum[1] + sum[2] + sum[3]
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn dot_product_sse3(left: &[f64], right: &[f64]) -> f64 {
    let mut result: f64;
    unsafe {
        asm!(
            "xorpd xmm0, xmm0",
            "xor rcx, rcx",

            "2:",
            "vmovupd xmm1, [{left_ptr} + rcx]",
            "vfmadd231pd xmm0, xmm1, [{right_ptr} + rcx]",
            "add rcx, 16",
            "cmp rcx, 96",
            "jb 2b",

            "haddpd xmm0, xmm0", // somma orizzontale

            out("xmm0") result,
            left_ptr = in(reg) left.as_ptr(),
            right_ptr = in(reg) right.as_ptr(),

            out("xmm1") _,
            out("xmm2") _,

            options(nostack, preserves_flags),
        );
    }
    result
}

#[inline]
#[target_feature(enable = "avx")]
unsafe fn dot_product_avx(left: &[f64], right: &[f64]) -> f64 {
    let mut result: f64;
    unsafe {
        asm!(
           "vxorpd ymm0, ymm0, ymm0",
           "xor rcx, rcx",
           
           "2:",
           "vmovupd ymm1, [{left_ptr} + rcx]",
           "vfmadd231pd ymm0, ymm1, [{right_ptr} + rcx]",
           "add rcx, 32",
           "cmp rcx, 96",
           "jb 2b",

           "vextractf128 xmm1, ymm0, 1",
           "vaddpd xmm0, xmm0, xmm1",
           "movhlps xmm1, xmm0",
           "addsd xmm0, xmm1", 

            out("xmm0") result,
            left_ptr = in(reg) left.as_ptr(),
            right_ptr = in(reg) right.as_ptr(),

            out("ymm1") _,
            out("ymm2") _,

            options(nostack, preserves_flags),
        );
    }
    result
}
