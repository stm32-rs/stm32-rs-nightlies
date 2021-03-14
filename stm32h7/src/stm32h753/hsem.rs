#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSEM register HSEM_R0 HSEM_R31"]
    pub r0: R0,
    #[doc = "0x04 - HSEM register HSEM_R0 HSEM_R31"]
    pub r1: R1,
    #[doc = "0x08 - HSEM register HSEM_R0 HSEM_R31"]
    pub r2: R2,
    #[doc = "0x0c - HSEM register HSEM_R0 HSEM_R31"]
    pub r3: R3,
    #[doc = "0x10 - HSEM register HSEM_R0 HSEM_R31"]
    pub r4: R4,
    #[doc = "0x14 - HSEM register HSEM_R0 HSEM_R31"]
    pub r5: R5,
    #[doc = "0x18 - HSEM register HSEM_R0 HSEM_R31"]
    pub r6: R6,
    #[doc = "0x1c - HSEM register HSEM_R0 HSEM_R31"]
    pub r7: R7,
    #[doc = "0x20 - HSEM register HSEM_R0 HSEM_R31"]
    pub r8: R8,
    #[doc = "0x24 - HSEM register HSEM_R0 HSEM_R31"]
    pub r9: R9,
    #[doc = "0x28 - HSEM register HSEM_R0 HSEM_R31"]
    pub r10: R10,
    #[doc = "0x2c - HSEM register HSEM_R0 HSEM_R31"]
    pub r11: R11,
    #[doc = "0x30 - HSEM register HSEM_R0 HSEM_R31"]
    pub r12: R12,
    #[doc = "0x34 - HSEM register HSEM_R0 HSEM_R31"]
    pub r13: R13,
    #[doc = "0x38 - HSEM register HSEM_R0 HSEM_R31"]
    pub r14: R14,
    #[doc = "0x3c - HSEM register HSEM_R0 HSEM_R31"]
    pub r15: R15,
    #[doc = "0x40 - HSEM register HSEM_R0 HSEM_R31"]
    pub r16: R16,
    #[doc = "0x44 - HSEM register HSEM_R0 HSEM_R31"]
    pub r17: R17,
    #[doc = "0x48 - HSEM register HSEM_R0 HSEM_R31"]
    pub r18: R18,
    #[doc = "0x4c - HSEM register HSEM_R0 HSEM_R31"]
    pub r19: R19,
    #[doc = "0x50 - HSEM register HSEM_R0 HSEM_R31"]
    pub r20: R20,
    #[doc = "0x54 - HSEM register HSEM_R0 HSEM_R31"]
    pub r21: R21,
    #[doc = "0x58 - HSEM register HSEM_R0 HSEM_R31"]
    pub r22: R22,
    #[doc = "0x5c - HSEM register HSEM_R0 HSEM_R31"]
    pub r23: R23,
    #[doc = "0x60 - HSEM register HSEM_R0 HSEM_R31"]
    pub r24: R24,
    #[doc = "0x64 - HSEM register HSEM_R0 HSEM_R31"]
    pub r25: R25,
    #[doc = "0x68 - HSEM register HSEM_R0 HSEM_R31"]
    pub r26: R26,
    #[doc = "0x6c - HSEM register HSEM_R0 HSEM_R31"]
    pub r27: R27,
    #[doc = "0x70 - HSEM register HSEM_R0 HSEM_R31"]
    pub r28: R28,
    #[doc = "0x74 - HSEM register HSEM_R0 HSEM_R31"]
    pub r29: R29,
    #[doc = "0x78 - HSEM register HSEM_R0 HSEM_R31"]
    pub r30: R30,
    #[doc = "0x7c - HSEM register HSEM_R0 HSEM_R31"]
    pub r31: R31,
    #[doc = "0x80 - HSEM Read lock register"]
    pub rlr0: RLR0,
    #[doc = "0x84 - HSEM Read lock register"]
    pub rlr1: RLR1,
    #[doc = "0x88 - HSEM Read lock register"]
    pub rlr2: RLR2,
    #[doc = "0x8c - HSEM Read lock register"]
    pub rlr3: RLR3,
    #[doc = "0x90 - HSEM Read lock register"]
    pub rlr4: RLR4,
    #[doc = "0x94 - HSEM Read lock register"]
    pub rlr5: RLR5,
    #[doc = "0x98 - HSEM Read lock register"]
    pub rlr6: RLR6,
    #[doc = "0x9c - HSEM Read lock register"]
    pub rlr7: RLR7,
    #[doc = "0xa0 - HSEM Read lock register"]
    pub rlr8: RLR8,
    #[doc = "0xa4 - HSEM Read lock register"]
    pub rlr9: RLR9,
    #[doc = "0xa8 - HSEM Read lock register"]
    pub rlr10: RLR10,
    #[doc = "0xac - HSEM Read lock register"]
    pub rlr11: RLR11,
    #[doc = "0xb0 - HSEM Read lock register"]
    pub rlr12: RLR12,
    #[doc = "0xb4 - HSEM Read lock register"]
    pub rlr13: RLR13,
    #[doc = "0xb8 - HSEM Read lock register"]
    pub rlr14: RLR14,
    #[doc = "0xbc - HSEM Read lock register"]
    pub rlr15: RLR15,
    #[doc = "0xc0 - HSEM Read lock register"]
    pub rlr16: RLR16,
    #[doc = "0xc4 - HSEM Read lock register"]
    pub rlr17: RLR17,
    #[doc = "0xc8 - HSEM Read lock register"]
    pub rlr18: RLR18,
    #[doc = "0xcc - HSEM Read lock register"]
    pub rlr19: RLR19,
    #[doc = "0xd0 - HSEM Read lock register"]
    pub rlr20: RLR20,
    #[doc = "0xd4 - HSEM Read lock register"]
    pub rlr21: RLR21,
    #[doc = "0xd8 - HSEM Read lock register"]
    pub rlr22: RLR22,
    #[doc = "0xdc - HSEM Read lock register"]
    pub rlr23: RLR23,
    #[doc = "0xe0 - HSEM Read lock register"]
    pub rlr24: RLR24,
    #[doc = "0xe4 - HSEM Read lock register"]
    pub rlr25: RLR25,
    #[doc = "0xe8 - HSEM Read lock register"]
    pub rlr26: RLR26,
    #[doc = "0xec - HSEM Read lock register"]
    pub rlr27: RLR27,
    #[doc = "0xf0 - HSEM Read lock register"]
    pub rlr28: RLR28,
    #[doc = "0xf4 - HSEM Read lock register"]
    pub rlr29: RLR29,
    #[doc = "0xf8 - HSEM Read lock register"]
    pub rlr30: RLR30,
    #[doc = "0xfc - HSEM Read lock register"]
    pub rlr31: RLR31,
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub ier: IER,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub misr: MISR,
    _reserved68: [u8; 48usize],
    #[doc = "0x140 - HSEM Clear register"]
    pub cr: CR,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub keyr: KEYR,
}
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r0](r0) module"]
pub type R0 = crate::Reg<u32, _R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R0;
#[doc = "`read()` method returns [r0::R](r0::R) reader structure"]
impl crate::Readable for R0 {}
#[doc = "`write(|w| ..)` method takes [r0::W](r0::W) writer structure"]
impl crate::Writable for R0 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r0;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1](r1) module"]
pub type R1 = crate::Reg<u32, _R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1;
#[doc = "`read()` method returns [r1::R](r1::R) reader structure"]
impl crate::Readable for R1 {}
#[doc = "`write(|w| ..)` method takes [r1::W](r1::W) writer structure"]
impl crate::Writable for R1 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r1;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2](r2) module"]
pub type R2 = crate::Reg<u32, _R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2;
#[doc = "`read()` method returns [r2::R](r2::R) reader structure"]
impl crate::Readable for R2 {}
#[doc = "`write(|w| ..)` method takes [r2::W](r2::W) writer structure"]
impl crate::Writable for R2 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r2;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3](r3) module"]
pub type R3 = crate::Reg<u32, _R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3;
#[doc = "`read()` method returns [r3::R](r3::R) reader structure"]
impl crate::Readable for R3 {}
#[doc = "`write(|w| ..)` method takes [r3::W](r3::W) writer structure"]
impl crate::Writable for R3 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r3;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4](r4) module"]
pub type R4 = crate::Reg<u32, _R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4;
#[doc = "`read()` method returns [r4::R](r4::R) reader structure"]
impl crate::Readable for R4 {}
#[doc = "`write(|w| ..)` method takes [r4::W](r4::W) writer structure"]
impl crate::Writable for R4 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r4;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5](r5) module"]
pub type R5 = crate::Reg<u32, _R5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5;
#[doc = "`read()` method returns [r5::R](r5::R) reader structure"]
impl crate::Readable for R5 {}
#[doc = "`write(|w| ..)` method takes [r5::W](r5::W) writer structure"]
impl crate::Writable for R5 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r5;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r6](r6) module"]
pub type R6 = crate::Reg<u32, _R6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R6;
#[doc = "`read()` method returns [r6::R](r6::R) reader structure"]
impl crate::Readable for R6 {}
#[doc = "`write(|w| ..)` method takes [r6::W](r6::W) writer structure"]
impl crate::Writable for R6 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r6;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r7](r7) module"]
pub type R7 = crate::Reg<u32, _R7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R7;
#[doc = "`read()` method returns [r7::R](r7::R) reader structure"]
impl crate::Readable for R7 {}
#[doc = "`write(|w| ..)` method takes [r7::W](r7::W) writer structure"]
impl crate::Writable for R7 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r7;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8](r8) module"]
pub type R8 = crate::Reg<u32, _R8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R8;
#[doc = "`read()` method returns [r8::R](r8::R) reader structure"]
impl crate::Readable for R8 {}
#[doc = "`write(|w| ..)` method takes [r8::W](r8::W) writer structure"]
impl crate::Writable for R8 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r8;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r9](r9) module"]
pub type R9 = crate::Reg<u32, _R9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R9;
#[doc = "`read()` method returns [r9::R](r9::R) reader structure"]
impl crate::Readable for R9 {}
#[doc = "`write(|w| ..)` method takes [r9::W](r9::W) writer structure"]
impl crate::Writable for R9 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r9;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r10](r10) module"]
pub type R10 = crate::Reg<u32, _R10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R10;
#[doc = "`read()` method returns [r10::R](r10::R) reader structure"]
impl crate::Readable for R10 {}
#[doc = "`write(|w| ..)` method takes [r10::W](r10::W) writer structure"]
impl crate::Writable for R10 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r10;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r11](r11) module"]
pub type R11 = crate::Reg<u32, _R11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R11;
#[doc = "`read()` method returns [r11::R](r11::R) reader structure"]
impl crate::Readable for R11 {}
#[doc = "`write(|w| ..)` method takes [r11::W](r11::W) writer structure"]
impl crate::Writable for R11 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r11;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r12](r12) module"]
pub type R12 = crate::Reg<u32, _R12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R12;
#[doc = "`read()` method returns [r12::R](r12::R) reader structure"]
impl crate::Readable for R12 {}
#[doc = "`write(|w| ..)` method takes [r12::W](r12::W) writer structure"]
impl crate::Writable for R12 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r12;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r13](r13) module"]
pub type R13 = crate::Reg<u32, _R13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R13;
#[doc = "`read()` method returns [r13::R](r13::R) reader structure"]
impl crate::Readable for R13 {}
#[doc = "`write(|w| ..)` method takes [r13::W](r13::W) writer structure"]
impl crate::Writable for R13 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r13;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r14](r14) module"]
pub type R14 = crate::Reg<u32, _R14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R14;
#[doc = "`read()` method returns [r14::R](r14::R) reader structure"]
impl crate::Readable for R14 {}
#[doc = "`write(|w| ..)` method takes [r14::W](r14::W) writer structure"]
impl crate::Writable for R14 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r14;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r15](r15) module"]
pub type R15 = crate::Reg<u32, _R15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R15;
#[doc = "`read()` method returns [r15::R](r15::R) reader structure"]
impl crate::Readable for R15 {}
#[doc = "`write(|w| ..)` method takes [r15::W](r15::W) writer structure"]
impl crate::Writable for R15 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r15;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16](r16) module"]
pub type R16 = crate::Reg<u32, _R16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R16;
#[doc = "`read()` method returns [r16::R](r16::R) reader structure"]
impl crate::Readable for R16 {}
#[doc = "`write(|w| ..)` method takes [r16::W](r16::W) writer structure"]
impl crate::Writable for R16 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r16;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r17](r17) module"]
pub type R17 = crate::Reg<u32, _R17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R17;
#[doc = "`read()` method returns [r17::R](r17::R) reader structure"]
impl crate::Readable for R17 {}
#[doc = "`write(|w| ..)` method takes [r17::W](r17::W) writer structure"]
impl crate::Writable for R17 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r17;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r18](r18) module"]
pub type R18 = crate::Reg<u32, _R18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R18;
#[doc = "`read()` method returns [r18::R](r18::R) reader structure"]
impl crate::Readable for R18 {}
#[doc = "`write(|w| ..)` method takes [r18::W](r18::W) writer structure"]
impl crate::Writable for R18 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r18;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r19](r19) module"]
pub type R19 = crate::Reg<u32, _R19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R19;
#[doc = "`read()` method returns [r19::R](r19::R) reader structure"]
impl crate::Readable for R19 {}
#[doc = "`write(|w| ..)` method takes [r19::W](r19::W) writer structure"]
impl crate::Writable for R19 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r19;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r20](r20) module"]
pub type R20 = crate::Reg<u32, _R20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R20;
#[doc = "`read()` method returns [r20::R](r20::R) reader structure"]
impl crate::Readable for R20 {}
#[doc = "`write(|w| ..)` method takes [r20::W](r20::W) writer structure"]
impl crate::Writable for R20 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r20;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r21](r21) module"]
pub type R21 = crate::Reg<u32, _R21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R21;
#[doc = "`read()` method returns [r21::R](r21::R) reader structure"]
impl crate::Readable for R21 {}
#[doc = "`write(|w| ..)` method takes [r21::W](r21::W) writer structure"]
impl crate::Writable for R21 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r21;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r22](r22) module"]
pub type R22 = crate::Reg<u32, _R22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R22;
#[doc = "`read()` method returns [r22::R](r22::R) reader structure"]
impl crate::Readable for R22 {}
#[doc = "`write(|w| ..)` method takes [r22::W](r22::W) writer structure"]
impl crate::Writable for R22 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r22;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r23](r23) module"]
pub type R23 = crate::Reg<u32, _R23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R23;
#[doc = "`read()` method returns [r23::R](r23::R) reader structure"]
impl crate::Readable for R23 {}
#[doc = "`write(|w| ..)` method takes [r23::W](r23::W) writer structure"]
impl crate::Writable for R23 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r23;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r24](r24) module"]
pub type R24 = crate::Reg<u32, _R24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R24;
#[doc = "`read()` method returns [r24::R](r24::R) reader structure"]
impl crate::Readable for R24 {}
#[doc = "`write(|w| ..)` method takes [r24::W](r24::W) writer structure"]
impl crate::Writable for R24 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r24;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r25](r25) module"]
pub type R25 = crate::Reg<u32, _R25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R25;
#[doc = "`read()` method returns [r25::R](r25::R) reader structure"]
impl crate::Readable for R25 {}
#[doc = "`write(|w| ..)` method takes [r25::W](r25::W) writer structure"]
impl crate::Writable for R25 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r25;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r26](r26) module"]
pub type R26 = crate::Reg<u32, _R26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R26;
#[doc = "`read()` method returns [r26::R](r26::R) reader structure"]
impl crate::Readable for R26 {}
#[doc = "`write(|w| ..)` method takes [r26::W](r26::W) writer structure"]
impl crate::Writable for R26 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r26;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r27](r27) module"]
pub type R27 = crate::Reg<u32, _R27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R27;
#[doc = "`read()` method returns [r27::R](r27::R) reader structure"]
impl crate::Readable for R27 {}
#[doc = "`write(|w| ..)` method takes [r27::W](r27::W) writer structure"]
impl crate::Writable for R27 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r27;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r28](r28) module"]
pub type R28 = crate::Reg<u32, _R28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R28;
#[doc = "`read()` method returns [r28::R](r28::R) reader structure"]
impl crate::Readable for R28 {}
#[doc = "`write(|w| ..)` method takes [r28::W](r28::W) writer structure"]
impl crate::Writable for R28 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r28;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r29](r29) module"]
pub type R29 = crate::Reg<u32, _R29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R29;
#[doc = "`read()` method returns [r29::R](r29::R) reader structure"]
impl crate::Readable for R29 {}
#[doc = "`write(|w| ..)` method takes [r29::W](r29::W) writer structure"]
impl crate::Writable for R29 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r29;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r30](r30) module"]
pub type R30 = crate::Reg<u32, _R30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R30;
#[doc = "`read()` method returns [r30::R](r30::R) reader structure"]
impl crate::Readable for R30 {}
#[doc = "`write(|w| ..)` method takes [r30::W](r30::W) writer structure"]
impl crate::Writable for R30 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r30;
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r31](r31) module"]
pub type R31 = crate::Reg<u32, _R31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R31;
#[doc = "`read()` method returns [r31::R](r31::R) reader structure"]
impl crate::Readable for R31 {}
#[doc = "`write(|w| ..)` method takes [r31::W](r31::W) writer structure"]
impl crate::Writable for R31 {}
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r31;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr0](rlr0) module"]
pub type RLR0 = crate::Reg<u32, _RLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR0;
#[doc = "`read()` method returns [rlr0::R](rlr0::R) reader structure"]
impl crate::Readable for RLR0 {}
#[doc = "HSEM Read lock register"]
pub mod rlr0;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr1](rlr1) module"]
pub type RLR1 = crate::Reg<u32, _RLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR1;
#[doc = "`read()` method returns [rlr1::R](rlr1::R) reader structure"]
impl crate::Readable for RLR1 {}
#[doc = "HSEM Read lock register"]
pub mod rlr1;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr2](rlr2) module"]
pub type RLR2 = crate::Reg<u32, _RLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR2;
#[doc = "`read()` method returns [rlr2::R](rlr2::R) reader structure"]
impl crate::Readable for RLR2 {}
#[doc = "HSEM Read lock register"]
pub mod rlr2;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr3](rlr3) module"]
pub type RLR3 = crate::Reg<u32, _RLR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR3;
#[doc = "`read()` method returns [rlr3::R](rlr3::R) reader structure"]
impl crate::Readable for RLR3 {}
#[doc = "HSEM Read lock register"]
pub mod rlr3;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr4](rlr4) module"]
pub type RLR4 = crate::Reg<u32, _RLR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR4;
#[doc = "`read()` method returns [rlr4::R](rlr4::R) reader structure"]
impl crate::Readable for RLR4 {}
#[doc = "HSEM Read lock register"]
pub mod rlr4;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr5](rlr5) module"]
pub type RLR5 = crate::Reg<u32, _RLR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR5;
#[doc = "`read()` method returns [rlr5::R](rlr5::R) reader structure"]
impl crate::Readable for RLR5 {}
#[doc = "HSEM Read lock register"]
pub mod rlr5;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr6](rlr6) module"]
pub type RLR6 = crate::Reg<u32, _RLR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR6;
#[doc = "`read()` method returns [rlr6::R](rlr6::R) reader structure"]
impl crate::Readable for RLR6 {}
#[doc = "HSEM Read lock register"]
pub mod rlr6;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr7](rlr7) module"]
pub type RLR7 = crate::Reg<u32, _RLR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR7;
#[doc = "`read()` method returns [rlr7::R](rlr7::R) reader structure"]
impl crate::Readable for RLR7 {}
#[doc = "HSEM Read lock register"]
pub mod rlr7;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr8](rlr8) module"]
pub type RLR8 = crate::Reg<u32, _RLR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR8;
#[doc = "`read()` method returns [rlr8::R](rlr8::R) reader structure"]
impl crate::Readable for RLR8 {}
#[doc = "HSEM Read lock register"]
pub mod rlr8;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr9](rlr9) module"]
pub type RLR9 = crate::Reg<u32, _RLR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR9;
#[doc = "`read()` method returns [rlr9::R](rlr9::R) reader structure"]
impl crate::Readable for RLR9 {}
#[doc = "HSEM Read lock register"]
pub mod rlr9;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr10](rlr10) module"]
pub type RLR10 = crate::Reg<u32, _RLR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR10;
#[doc = "`read()` method returns [rlr10::R](rlr10::R) reader structure"]
impl crate::Readable for RLR10 {}
#[doc = "HSEM Read lock register"]
pub mod rlr10;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr11](rlr11) module"]
pub type RLR11 = crate::Reg<u32, _RLR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR11;
#[doc = "`read()` method returns [rlr11::R](rlr11::R) reader structure"]
impl crate::Readable for RLR11 {}
#[doc = "HSEM Read lock register"]
pub mod rlr11;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr12](rlr12) module"]
pub type RLR12 = crate::Reg<u32, _RLR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR12;
#[doc = "`read()` method returns [rlr12::R](rlr12::R) reader structure"]
impl crate::Readable for RLR12 {}
#[doc = "HSEM Read lock register"]
pub mod rlr12;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr13](rlr13) module"]
pub type RLR13 = crate::Reg<u32, _RLR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR13;
#[doc = "`read()` method returns [rlr13::R](rlr13::R) reader structure"]
impl crate::Readable for RLR13 {}
#[doc = "HSEM Read lock register"]
pub mod rlr13;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr14](rlr14) module"]
pub type RLR14 = crate::Reg<u32, _RLR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR14;
#[doc = "`read()` method returns [rlr14::R](rlr14::R) reader structure"]
impl crate::Readable for RLR14 {}
#[doc = "HSEM Read lock register"]
pub mod rlr14;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr15](rlr15) module"]
pub type RLR15 = crate::Reg<u32, _RLR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR15;
#[doc = "`read()` method returns [rlr15::R](rlr15::R) reader structure"]
impl crate::Readable for RLR15 {}
#[doc = "HSEM Read lock register"]
pub mod rlr15;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr16](rlr16) module"]
pub type RLR16 = crate::Reg<u32, _RLR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR16;
#[doc = "`read()` method returns [rlr16::R](rlr16::R) reader structure"]
impl crate::Readable for RLR16 {}
#[doc = "HSEM Read lock register"]
pub mod rlr16;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr17](rlr17) module"]
pub type RLR17 = crate::Reg<u32, _RLR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR17;
#[doc = "`read()` method returns [rlr17::R](rlr17::R) reader structure"]
impl crate::Readable for RLR17 {}
#[doc = "HSEM Read lock register"]
pub mod rlr17;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr18](rlr18) module"]
pub type RLR18 = crate::Reg<u32, _RLR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR18;
#[doc = "`read()` method returns [rlr18::R](rlr18::R) reader structure"]
impl crate::Readable for RLR18 {}
#[doc = "HSEM Read lock register"]
pub mod rlr18;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr19](rlr19) module"]
pub type RLR19 = crate::Reg<u32, _RLR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR19;
#[doc = "`read()` method returns [rlr19::R](rlr19::R) reader structure"]
impl crate::Readable for RLR19 {}
#[doc = "HSEM Read lock register"]
pub mod rlr19;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr20](rlr20) module"]
pub type RLR20 = crate::Reg<u32, _RLR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR20;
#[doc = "`read()` method returns [rlr20::R](rlr20::R) reader structure"]
impl crate::Readable for RLR20 {}
#[doc = "HSEM Read lock register"]
pub mod rlr20;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr21](rlr21) module"]
pub type RLR21 = crate::Reg<u32, _RLR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR21;
#[doc = "`read()` method returns [rlr21::R](rlr21::R) reader structure"]
impl crate::Readable for RLR21 {}
#[doc = "HSEM Read lock register"]
pub mod rlr21;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr22](rlr22) module"]
pub type RLR22 = crate::Reg<u32, _RLR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR22;
#[doc = "`read()` method returns [rlr22::R](rlr22::R) reader structure"]
impl crate::Readable for RLR22 {}
#[doc = "HSEM Read lock register"]
pub mod rlr22;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr23](rlr23) module"]
pub type RLR23 = crate::Reg<u32, _RLR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR23;
#[doc = "`read()` method returns [rlr23::R](rlr23::R) reader structure"]
impl crate::Readable for RLR23 {}
#[doc = "HSEM Read lock register"]
pub mod rlr23;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr24](rlr24) module"]
pub type RLR24 = crate::Reg<u32, _RLR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR24;
#[doc = "`read()` method returns [rlr24::R](rlr24::R) reader structure"]
impl crate::Readable for RLR24 {}
#[doc = "HSEM Read lock register"]
pub mod rlr24;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr25](rlr25) module"]
pub type RLR25 = crate::Reg<u32, _RLR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR25;
#[doc = "`read()` method returns [rlr25::R](rlr25::R) reader structure"]
impl crate::Readable for RLR25 {}
#[doc = "HSEM Read lock register"]
pub mod rlr25;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr26](rlr26) module"]
pub type RLR26 = crate::Reg<u32, _RLR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR26;
#[doc = "`read()` method returns [rlr26::R](rlr26::R) reader structure"]
impl crate::Readable for RLR26 {}
#[doc = "HSEM Read lock register"]
pub mod rlr26;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr27](rlr27) module"]
pub type RLR27 = crate::Reg<u32, _RLR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR27;
#[doc = "`read()` method returns [rlr27::R](rlr27::R) reader structure"]
impl crate::Readable for RLR27 {}
#[doc = "HSEM Read lock register"]
pub mod rlr27;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr28](rlr28) module"]
pub type RLR28 = crate::Reg<u32, _RLR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR28;
#[doc = "`read()` method returns [rlr28::R](rlr28::R) reader structure"]
impl crate::Readable for RLR28 {}
#[doc = "HSEM Read lock register"]
pub mod rlr28;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr29](rlr29) module"]
pub type RLR29 = crate::Reg<u32, _RLR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR29;
#[doc = "`read()` method returns [rlr29::R](rlr29::R) reader structure"]
impl crate::Readable for RLR29 {}
#[doc = "HSEM Read lock register"]
pub mod rlr29;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr30](rlr30) module"]
pub type RLR30 = crate::Reg<u32, _RLR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR30;
#[doc = "`read()` method returns [rlr30::R](rlr30::R) reader structure"]
impl crate::Readable for RLR30 {}
#[doc = "HSEM Read lock register"]
pub mod rlr30;
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr31](rlr31) module"]
pub type RLR31 = crate::Reg<u32, _RLR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR31;
#[doc = "`read()` method returns [rlr31::R](rlr31::R) reader structure"]
impl crate::Readable for RLR31 {}
#[doc = "HSEM Read lock register"]
pub mod rlr31;
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "HSEM Interrupt enable register"]
pub mod ier;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "HSEM Interrupt clear register"]
pub mod icr;
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "HSEM Interrupt status register"]
pub mod isr;
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "HSEM Masked interrupt status register"]
pub mod misr;
#[doc = "HSEM Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr](keyr) module"]
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
#[doc = "`read()` method returns [keyr::R](keyr::R) reader structure"]
impl crate::Readable for KEYR {}
#[doc = "`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure"]
impl crate::Writable for KEYR {}
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
