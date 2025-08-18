use std::arch::asm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    if std::is_x86_feature_detected!("avx") {
        unsafe { dot_product_avx(a, b) }
    } else if std::is_x86_feature_detected!("sse3") {
        unsafe { dot_product_sse3(a, b) }
    } else {
        dot_product_scalar(a, b)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    dot_product_scalar(a, b)
}

#[inline]
fn dot_product_scalar(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), 12);
    assert_eq!(b.len(), 12);

    let mut sum = [0.0f64; 4];
    let pa = a.as_ptr();
    let pb = b.as_ptr();

    unsafe {
        sum[0] += *pa * *pb;
        sum[1] += *pa.add(1) * *pb.add(1);
        sum[2] += *pa.add(2) * *pb.add(2);
        sum[3] += *pa.add(3) * *pb.add(3);
        sum[0] += *pa.add(4) * *pb.add(4);
        sum[1] += *pa.add(5) * *pb.add(5);
        sum[2] += *pa.add(6) * *pb.add(6);
        sum[3] += *pa.add(7) * *pb.add(7);
        sum[0] += *pa.add(8) * *pb.add(8);
        sum[1] += *pa.add(9) * *pb.add(9);
        sum[2] += *pa.add(10) * *pb.add(10);
        sum[3] += *pa.add(11) * *pb.add(11);
    }

    sum[0] + sum[1] + sum[2] + sum[3]
}

#[inline]
#[target_feature(enable = "sse3")]
unsafe fn dot_product_sse3(a: &[f64], b: &[f64]) -> f64 {
    let mut result: f64;
    unsafe {
        asm!(
            "xorpd xmm0, xmm0",
            "xor rcx, rcx",

            "2:",
            "movupd xmm1, [{a} + rcx]",
            "movupd xmm2, [{b} + rcx]",
            "mulpd xmm1, xmm2",
            "addpd xmm0, xmm1",
            "add rcx, 16",
            "cmp rcx, 96",
            "jb 2b",

            "haddpd xmm0, xmm0", // somma orizzontale

            out("xmm0") result,
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),

            out("xmm1") _,
            out("xmm2") _,

            options(nostack, preserves_flags),
        );
    }
    result
}

#[inline]
#[target_feature(enable = "avx")]
unsafe fn dot_product_avx(a: &[f64], b: &[f64]) -> f64 {
    let mut result: f64;
    unsafe {
        asm!(
           "vxorpd ymm0, ymm0, ymm0",
           "xor rcx, rcx",
           
           "2:",
           "vmovupd ymm1, [{a} + rcx]",
           "vmovupd ymm2, [{b} + rcx]",
           "vmulpd  ymm1, ymm1, ymm2",
           "vaddpd  ymm0, ymm0, ymm1",
           "add rcx, 32",
           "cmp rcx, 96",
           "jb 2b",

           "vextractf128 xmm1, ymm0, 1",
           "vaddpd xmm0, xmm0, xmm1",
           "vhaddpd xmm0, xmm0, xmm0",

            out("xmm0") result,
            a = in(reg) a.as_ptr(),
            b = in(reg) b.as_ptr(),

            out("ymm1") _,
            out("ymm2") _,

            options(nostack, preserves_flags),
        );
    }
    result
}
