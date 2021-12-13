use pairing_bn256::bn256::Fr;

pub fn add(a: &[u64; 4], b: &[u64; 4]) -> [u64; 4] {
    let modulus: &[u64; 4] = &[
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];
    let mut r0: u64;
    let mut r1: u64;
    let mut r2: u64;
    let mut r3: u64;
    let mut r4: u64;
    let mut r5: u64;
    let mut r6: u64;
    let mut r7: u64;
    unsafe {
        asm!(
            // load a array to former register
            "mov r8, qword ptr [{a_ptr} + 0]",
            "mov r9, qword ptr [{a_ptr} + 8]",
            "mov r10, qword ptr [{a_ptr} + 16]",
            "mov r11, qword ptr [{a_ptr} + 24]",

            // // add a array and b array with carry
            "add r8, qword ptr [{b_ptr} + 0]",
            "adc r9, qword ptr [{b_ptr} + 8]",
            "adc r10, qword ptr [{b_ptr} + 16]",
            "adc r11, qword ptr [{b_ptr} + 24]",

            // copy result array to latter register
            "mov r12, r8",
            "mov r13, r9",
            "mov r14, r10",
            "mov r15, r11",

            // mod reduction
            "sub r12, qword ptr [{m_ptr} + 0]",
            "sbb r13, qword ptr [{m_ptr} + 8]",
            "sbb r14, qword ptr [{m_ptr} + 16]",
            "sbb r15, qword ptr [{m_ptr} + 24]",

            // if sum >= mod return latter register
            // if sum <= mod return former register
            "cmovc r12, r8",
            "cmovc r13, r9",
            "cmovc r14, r10",
            "cmovc r15, r11",

            m_ptr = in(reg) modulus.as_ptr(),
            a_ptr = in(reg) a.as_ptr(),
            b_ptr = in(reg) b.as_ptr(),
            out("r8") r4,
            out("r9") r5,
            out("r10") r6,
            out("r11") r7,
            out("r12") r0,
            out("r13") r1,
            out("r14") r2,
            out("r15") r3,
            options(pure, readonly, nostack)
        );
    }
    [r0, r1, r2, r3]
}

#[cfg(test)]
mod asembly_tests {
    use super::*;

    #[test]
    fn add_test() {
        let a: [u64; 4] = [
            0x7e7140b5196b9e6f,
            0x9abac9e4157b6172,
            0xf04bc41062fd7322,
            0x1185fa9c9fef6326,
        ];
        let zero: [u64; 4] = [0; 4];
        assert_eq!(
            add(&a, &zero),
            [
                0x7e7140b5196b9e6f,
                0x9abac9e4157b6172,
                0xf04bc41062fd7322,
                0x1185fa9c9fef6326,
            ]
        );
        let prime: [u64; 4] = [
            0x43e1f593f0000001,
            0x2833e84879b97091,
            0xb85045b68181585d,
            0x30644e72e131a029,
        ];
        assert_eq!(
            add(&a, &prime),
            [
                0x7e7140b5196b9e6f,
                0x9abac9e4157b6172,
                0xf04bc41062fd7322,
                0x1185fa9c9fef6326,
            ]
        );
        let one: [u64; 4] = [
            0x0000000000000001,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
        ];
        assert_eq!(
            add(&a, &one),
            [
                0x7e7140b5196b9e70,
                0x9abac9e4157b6172,
                0xf04bc41062fd7322,
                0x1185fa9c9fef6326,
            ]
        );
    }
}
