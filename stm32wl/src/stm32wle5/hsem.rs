#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r0: HSEM_R0,
    #[doc = "0x04 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r1: HSEM_R1,
    #[doc = "0x08 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r2: HSEM_R2,
    #[doc = "0x0c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r3: HSEM_R3,
    #[doc = "0x10 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r4: HSEM_R4,
    #[doc = "0x14 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r5: HSEM_R5,
    #[doc = "0x18 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r6: HSEM_R6,
    #[doc = "0x1c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r7: HSEM_R7,
    #[doc = "0x20 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r8: HSEM_R8,
    #[doc = "0x24 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r9: HSEM_R9,
    #[doc = "0x28 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r10: HSEM_R10,
    #[doc = "0x2c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r11: HSEM_R11,
    #[doc = "0x30 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r12: HSEM_R12,
    #[doc = "0x34 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r13: HSEM_R13,
    #[doc = "0x38 - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r14: HSEM_R14,
    #[doc = "0x3c - HSEM register HSEM_R0 HSEM_R31"]
    pub hsem_r15: HSEM_R15,
    _reserved16: [u8; 64usize],
    #[doc = "0x80 - HSEM Read lock register"]
    pub hsem_rlr0: HSEM_RLR0,
    #[doc = "0x84 - HSEM Read lock register"]
    pub hsem_rlr1: HSEM_RLR1,
    #[doc = "0x88 - HSEM Read lock register"]
    pub hsem_rlr2: HSEM_RLR2,
    #[doc = "0x8c - HSEM Read lock register"]
    pub hsem_rlr3: HSEM_RLR3,
    #[doc = "0x90 - HSEM Read lock register"]
    pub hsem_rlr4: HSEM_RLR4,
    #[doc = "0x94 - HSEM Read lock register"]
    pub hsem_rlr5: HSEM_RLR5,
    #[doc = "0x98 - HSEM Read lock register"]
    pub hsem_rlr6: HSEM_RLR6,
    #[doc = "0x9c - HSEM Read lock register"]
    pub hsem_rlr7: HSEM_RLR7,
    #[doc = "0xa0 - HSEM Read lock register"]
    pub hsem_rlr8: HSEM_RLR8,
    #[doc = "0xa4 - HSEM Read lock register"]
    pub hsem_rlr9: HSEM_RLR9,
    #[doc = "0xa8 - HSEM Read lock register"]
    pub hsem_rlr10: HSEM_RLR10,
    #[doc = "0xac - HSEM Read lock register"]
    pub hsem_rlr11: HSEM_RLR11,
    #[doc = "0xb0 - HSEM Read lock register"]
    pub hsem_rlr12: HSEM_RLR12,
    #[doc = "0xb4 - HSEM Read lock register"]
    pub hsem_rlr13: HSEM_RLR13,
    #[doc = "0xb8 - HSEM Read lock register"]
    pub hsem_rlr14: HSEM_RLR14,
    #[doc = "0xbc - HSEM Read lock register"]
    pub hsem_rlr15: HSEM_RLR15,
    _reserved32: [u8; 64usize],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub hsem_ier: HSEM_IER,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub hsem_icr: HSEM_ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub hsem_isr: HSEM_ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub hsem_misr: HSEM_MISR,
    _reserved36: [u8; 48usize],
    #[doc = "0x140 - HSEM Clear register"]
    pub hsem_cr: HSEM_CR,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub hsem_keyr: HSEM_KEYR,
}
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r0](hsem_r0) module"]
pub type HSEM_R0 = crate::Reg<u32, _HSEM_R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R0;
#[doc = "`read()` method returns [hsem_r0::R](hsem_r0::R) reader structure"]
impl crate::Readable for HSEM_R0 {}
#[doc = "`write(|w| ..)` method takes [hsem_r0::W](hsem_r0::W) writer structure"]
impl crate::Writable for HSEM_R0 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r0;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r1](hsem_r1) module"]
pub type HSEM_R1 = crate::Reg<u32, _HSEM_R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R1;
#[doc = "`read()` method returns [hsem_r1::R](hsem_r1::R) reader structure"]
impl crate::Readable for HSEM_R1 {}
#[doc = "`write(|w| ..)` method takes [hsem_r1::W](hsem_r1::W) writer structure"]
impl crate::Writable for HSEM_R1 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r1;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r2](hsem_r2) module"]
pub type HSEM_R2 = crate::Reg<u32, _HSEM_R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R2;
#[doc = "`read()` method returns [hsem_r2::R](hsem_r2::R) reader structure"]
impl crate::Readable for HSEM_R2 {}
#[doc = "`write(|w| ..)` method takes [hsem_r2::W](hsem_r2::W) writer structure"]
impl crate::Writable for HSEM_R2 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r2;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r3](hsem_r3) module"]
pub type HSEM_R3 = crate::Reg<u32, _HSEM_R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R3;
#[doc = "`read()` method returns [hsem_r3::R](hsem_r3::R) reader structure"]
impl crate::Readable for HSEM_R3 {}
#[doc = "`write(|w| ..)` method takes [hsem_r3::W](hsem_r3::W) writer structure"]
impl crate::Writable for HSEM_R3 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r3;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r4](hsem_r4) module"]
pub type HSEM_R4 = crate::Reg<u32, _HSEM_R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R4;
#[doc = "`read()` method returns [hsem_r4::R](hsem_r4::R) reader structure"]
impl crate::Readable for HSEM_R4 {}
#[doc = "`write(|w| ..)` method takes [hsem_r4::W](hsem_r4::W) writer structure"]
impl crate::Writable for HSEM_R4 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r4;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r5](hsem_r5) module"]
pub type HSEM_R5 = crate::Reg<u32, _HSEM_R5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R5;
#[doc = "`read()` method returns [hsem_r5::R](hsem_r5::R) reader structure"]
impl crate::Readable for HSEM_R5 {}
#[doc = "`write(|w| ..)` method takes [hsem_r5::W](hsem_r5::W) writer structure"]
impl crate::Writable for HSEM_R5 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r5;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r6](hsem_r6) module"]
pub type HSEM_R6 = crate::Reg<u32, _HSEM_R6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R6;
#[doc = "`read()` method returns [hsem_r6::R](hsem_r6::R) reader structure"]
impl crate::Readable for HSEM_R6 {}
#[doc = "`write(|w| ..)` method takes [hsem_r6::W](hsem_r6::W) writer structure"]
impl crate::Writable for HSEM_R6 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r6;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r7](hsem_r7) module"]
pub type HSEM_R7 = crate::Reg<u32, _HSEM_R7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R7;
#[doc = "`read()` method returns [hsem_r7::R](hsem_r7::R) reader structure"]
impl crate::Readable for HSEM_R7 {}
#[doc = "`write(|w| ..)` method takes [hsem_r7::W](hsem_r7::W) writer structure"]
impl crate::Writable for HSEM_R7 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r7;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r8](hsem_r8) module"]
pub type HSEM_R8 = crate::Reg<u32, _HSEM_R8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R8;
#[doc = "`read()` method returns [hsem_r8::R](hsem_r8::R) reader structure"]
impl crate::Readable for HSEM_R8 {}
#[doc = "`write(|w| ..)` method takes [hsem_r8::W](hsem_r8::W) writer structure"]
impl crate::Writable for HSEM_R8 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r8;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r9](hsem_r9) module"]
pub type HSEM_R9 = crate::Reg<u32, _HSEM_R9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R9;
#[doc = "`read()` method returns [hsem_r9::R](hsem_r9::R) reader structure"]
impl crate::Readable for HSEM_R9 {}
#[doc = "`write(|w| ..)` method takes [hsem_r9::W](hsem_r9::W) writer structure"]
impl crate::Writable for HSEM_R9 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r9;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r10](hsem_r10) module"]
pub type HSEM_R10 = crate::Reg<u32, _HSEM_R10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R10;
#[doc = "`read()` method returns [hsem_r10::R](hsem_r10::R) reader structure"]
impl crate::Readable for HSEM_R10 {}
#[doc = "`write(|w| ..)` method takes [hsem_r10::W](hsem_r10::W) writer structure"]
impl crate::Writable for HSEM_R10 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r10;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r11](hsem_r11) module"]
pub type HSEM_R11 = crate::Reg<u32, _HSEM_R11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R11;
#[doc = "`read()` method returns [hsem_r11::R](hsem_r11::R) reader structure"]
impl crate::Readable for HSEM_R11 {}
#[doc = "`write(|w| ..)` method takes [hsem_r11::W](hsem_r11::W) writer structure"]
impl crate::Writable for HSEM_R11 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r11;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r12](hsem_r12) module"]
pub type HSEM_R12 = crate::Reg<u32, _HSEM_R12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R12;
#[doc = "`read()` method returns [hsem_r12::R](hsem_r12::R) reader structure"]
impl crate::Readable for HSEM_R12 {}
#[doc = "`write(|w| ..)` method takes [hsem_r12::W](hsem_r12::W) writer structure"]
impl crate::Writable for HSEM_R12 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r12;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r13](hsem_r13) module"]
pub type HSEM_R13 = crate::Reg<u32, _HSEM_R13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R13;
#[doc = "`read()` method returns [hsem_r13::R](hsem_r13::R) reader structure"]
impl crate::Readable for HSEM_R13 {}
#[doc = "`write(|w| ..)` method takes [hsem_r13::W](hsem_r13::W) writer structure"]
impl crate::Writable for HSEM_R13 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r13;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r14](hsem_r14) module"]
pub type HSEM_R14 = crate::Reg<u32, _HSEM_R14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R14;
#[doc = "`read()` method returns [hsem_r14::R](hsem_r14::R) reader structure"]
impl crate::Readable for HSEM_R14 {}
#[doc = "`write(|w| ..)` method takes [hsem_r14::W](hsem_r14::W) writer structure"]
impl crate::Writable for HSEM_R14 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r14;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_r15](hsem_r15) module"]
pub type HSEM_R15 = crate::Reg<u32, _HSEM_R15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_R15;
#[doc = "`read()` method returns [hsem_r15::R](hsem_r15::R) reader structure"]
impl crate::Readable for HSEM_R15 {}
#[doc = "`write(|w| ..)` method takes [hsem_r15::W](hsem_r15::W) writer structure"]
impl crate::Writable for HSEM_R15 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r15;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr0](hsem_rlr0) module"]
pub type HSEM_RLR0 = crate::Reg<u32, _HSEM_RLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR0;
#[doc = "`read()` method returns [hsem_rlr0::R](hsem_rlr0::R) reader structure"]
impl crate::Readable for HSEM_RLR0 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr0;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr1](hsem_rlr1) module"]
pub type HSEM_RLR1 = crate::Reg<u32, _HSEM_RLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR1;
#[doc = "`read()` method returns [hsem_rlr1::R](hsem_rlr1::R) reader structure"]
impl crate::Readable for HSEM_RLR1 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr1;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr2](hsem_rlr2) module"]
pub type HSEM_RLR2 = crate::Reg<u32, _HSEM_RLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR2;
#[doc = "`read()` method returns [hsem_rlr2::R](hsem_rlr2::R) reader structure"]
impl crate::Readable for HSEM_RLR2 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr2;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr3](hsem_rlr3) module"]
pub type HSEM_RLR3 = crate::Reg<u32, _HSEM_RLR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR3;
#[doc = "`read()` method returns [hsem_rlr3::R](hsem_rlr3::R) reader structure"]
impl crate::Readable for HSEM_RLR3 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr3;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr4](hsem_rlr4) module"]
pub type HSEM_RLR4 = crate::Reg<u32, _HSEM_RLR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR4;
#[doc = "`read()` method returns [hsem_rlr4::R](hsem_rlr4::R) reader structure"]
impl crate::Readable for HSEM_RLR4 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr4;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr5](hsem_rlr5) module"]
pub type HSEM_RLR5 = crate::Reg<u32, _HSEM_RLR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR5;
#[doc = "`read()` method returns [hsem_rlr5::R](hsem_rlr5::R) reader structure"]
impl crate::Readable for HSEM_RLR5 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr5;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr6](hsem_rlr6) module"]
pub type HSEM_RLR6 = crate::Reg<u32, _HSEM_RLR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR6;
#[doc = "`read()` method returns [hsem_rlr6::R](hsem_rlr6::R) reader structure"]
impl crate::Readable for HSEM_RLR6 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr6;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr7](hsem_rlr7) module"]
pub type HSEM_RLR7 = crate::Reg<u32, _HSEM_RLR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR7;
#[doc = "`read()` method returns [hsem_rlr7::R](hsem_rlr7::R) reader structure"]
impl crate::Readable for HSEM_RLR7 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr7;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr8](hsem_rlr8) module"]
pub type HSEM_RLR8 = crate::Reg<u32, _HSEM_RLR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR8;
#[doc = "`read()` method returns [hsem_rlr8::R](hsem_rlr8::R) reader structure"]
impl crate::Readable for HSEM_RLR8 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr8;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr9](hsem_rlr9) module"]
pub type HSEM_RLR9 = crate::Reg<u32, _HSEM_RLR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR9;
#[doc = "`read()` method returns [hsem_rlr9::R](hsem_rlr9::R) reader structure"]
impl crate::Readable for HSEM_RLR9 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr9;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr10](hsem_rlr10) module"]
pub type HSEM_RLR10 = crate::Reg<u32, _HSEM_RLR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR10;
#[doc = "`read()` method returns [hsem_rlr10::R](hsem_rlr10::R) reader structure"]
impl crate::Readable for HSEM_RLR10 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr10;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr11](hsem_rlr11) module"]
pub type HSEM_RLR11 = crate::Reg<u32, _HSEM_RLR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR11;
#[doc = "`read()` method returns [hsem_rlr11::R](hsem_rlr11::R) reader structure"]
impl crate::Readable for HSEM_RLR11 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr11;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr12](hsem_rlr12) module"]
pub type HSEM_RLR12 = crate::Reg<u32, _HSEM_RLR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR12;
#[doc = "`read()` method returns [hsem_rlr12::R](hsem_rlr12::R) reader structure"]
impl crate::Readable for HSEM_RLR12 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr12;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr13](hsem_rlr13) module"]
pub type HSEM_RLR13 = crate::Reg<u32, _HSEM_RLR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR13;
#[doc = "`read()` method returns [hsem_rlr13::R](hsem_rlr13::R) reader structure"]
impl crate::Readable for HSEM_RLR13 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr13;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr14](hsem_rlr14) module"]
pub type HSEM_RLR14 = crate::Reg<u32, _HSEM_RLR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR14;
#[doc = "`read()` method returns [hsem_rlr14::R](hsem_rlr14::R) reader structure"]
impl crate::Readable for HSEM_RLR14 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr14;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_rlr15](hsem_rlr15) module"]
pub type HSEM_RLR15 = crate::Reg<u32, _HSEM_RLR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_RLR15;
#[doc = "`read()` method returns [hsem_rlr15::R](hsem_rlr15::R) reader structure"]
impl crate::Readable for HSEM_RLR15 {}
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr15;
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_ier](hsem_ier) module"]
pub type HSEM_IER = crate::Reg<u32, _HSEM_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_IER;
#[doc = "`read()` method returns [hsem_ier::R](hsem_ier::R) reader structure"]
impl crate::Readable for HSEM_IER {}
#[doc = "`write(|w| ..)` method takes [hsem_ier::W](hsem_ier::W) writer structure"]
impl crate::Writable for HSEM_IER {}
#[doc = "HSEM Interrupt enable register"]
pub mod hsem_ier;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_icr](hsem_icr) module"]
pub type HSEM_ICR = crate::Reg<u32, _HSEM_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_ICR;
#[doc = "`read()` method returns [hsem_icr::R](hsem_icr::R) reader structure"]
impl crate::Readable for HSEM_ICR {}
#[doc = "`write(|w| ..)` method takes [hsem_icr::W](hsem_icr::W) writer structure"]
impl crate::Writable for HSEM_ICR {}
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_icr;
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_isr](hsem_isr) module"]
pub type HSEM_ISR = crate::Reg<u32, _HSEM_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_ISR;
#[doc = "`read()` method returns [hsem_isr::R](hsem_isr::R) reader structure"]
impl crate::Readable for HSEM_ISR {}
#[doc = "HSEM Interrupt status register"]
pub mod hsem_isr;
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_misr](hsem_misr) module"]
pub type HSEM_MISR = crate::Reg<u32, _HSEM_MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_MISR;
#[doc = "`read()` method returns [hsem_misr::R](hsem_misr::R) reader structure"]
impl crate::Readable for HSEM_MISR {}
#[doc = "HSEM Masked interrupt status register"]
pub mod hsem_misr;
#[doc = "HSEM Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_cr](hsem_cr) module"]
pub type HSEM_CR = crate::Reg<u32, _HSEM_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_CR;
#[doc = "`write(|w| ..)` method takes [hsem_cr::W](hsem_cr::W) writer structure"]
impl crate::Writable for HSEM_CR {}
#[doc = "HSEM Clear register"]
pub mod hsem_cr;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_keyr](hsem_keyr) module"]
pub type HSEM_KEYR = crate::Reg<u32, _HSEM_KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEM_KEYR;
#[doc = "`read()` method returns [hsem_keyr::R](hsem_keyr::R) reader structure"]
impl crate::Readable for HSEM_KEYR {}
#[doc = "`write(|w| ..)` method takes [hsem_keyr::W](hsem_keyr::W) writer structure"]
impl crate::Writable for HSEM_KEYR {}
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_keyr;
