#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphore 0 register"]
    pub r0: R0,
    #[doc = "0x04 - Semaphore 1 register"]
    pub r1: R1,
    #[doc = "0x08 - Semaphore 2 register"]
    pub r2: R2,
    #[doc = "0x0c - Semaphore 3 register"]
    pub r3: R3,
    #[doc = "0x10 - Semaphore 4 register"]
    pub r4: R4,
    #[doc = "0x14 - Semaphore 5 register"]
    pub r5: R5,
    #[doc = "0x18 - Semaphore 6 register"]
    pub r6: R6,
    #[doc = "0x1c - Semaphore 7 register"]
    pub r7: R7,
    #[doc = "0x20 - Semaphore 8 register"]
    pub r8: R8,
    #[doc = "0x24 - Semaphore 9 register"]
    pub r9: R9,
    #[doc = "0x28 - Semaphore 10 register"]
    pub r10: R10,
    #[doc = "0x2c - Semaphore 11 register"]
    pub r11: R11,
    #[doc = "0x30 - Semaphore 12 register"]
    pub r12: R12,
    #[doc = "0x34 - Semaphore 13 register"]
    pub r13: R13,
    #[doc = "0x38 - Semaphore 14 register"]
    pub r14: R14,
    #[doc = "0x3c - Semaphore 15 register"]
    pub r15: R15,
    #[doc = "0x40 - Semaphore 16 register"]
    pub r16: R16,
    #[doc = "0x44 - Semaphore 17 register"]
    pub r17: R17,
    #[doc = "0x48 - Semaphore 18 register"]
    pub r18: R18,
    #[doc = "0x4c - Semaphore 19 register"]
    pub r19: R19,
    #[doc = "0x50 - Semaphore 20 register"]
    pub r20: R20,
    #[doc = "0x54 - Semaphore 21 register"]
    pub r21: R21,
    #[doc = "0x58 - Semaphore 22 register"]
    pub r22: R22,
    #[doc = "0x5c - Semaphore 23 register"]
    pub r23: R23,
    #[doc = "0x60 - Semaphore 24 register"]
    pub r24: R24,
    #[doc = "0x64 - Semaphore 25 register"]
    pub r25: R25,
    #[doc = "0x68 - Semaphore 26 register"]
    pub r26: R26,
    #[doc = "0x6c - Semaphore 27 register"]
    pub r27: R27,
    #[doc = "0x70 - Semaphore 28 register"]
    pub r28: R28,
    #[doc = "0x74 - Semaphore 29 register"]
    pub r29: R29,
    #[doc = "0x78 - Semaphore 30 register"]
    pub r30: R30,
    #[doc = "0x7c - Semaphore 31 register"]
    pub r31: R31,
    #[doc = "0x80 - Semaphore 0 read lock register"]
    pub rlr0: RLR0,
    #[doc = "0x84 - Semaphore 1 read lock register"]
    pub rlr1: RLR1,
    #[doc = "0x88 - Semaphore 2 read lock register"]
    pub rlr2: RLR2,
    #[doc = "0x8c - Semaphore 3 read lock register"]
    pub rlr3: RLR3,
    #[doc = "0x90 - Semaphore 4 read lock read lock register"]
    pub rlr4: RLR4,
    #[doc = "0x94 - Semaphore 5 read lock register"]
    pub rlr5: RLR5,
    #[doc = "0x98 - Semaphore 6 read lock register"]
    pub rlr6: RLR6,
    #[doc = "0x9c - Semaphore 7 read lock register"]
    pub rlr7: RLR7,
    #[doc = "0xa0 - Semaphore 8 read lock register"]
    pub rlr8: RLR8,
    #[doc = "0xa4 - Semaphore 9 read lock register"]
    pub rlr9: RLR9,
    #[doc = "0xa8 - Semaphore 10 read lock register"]
    pub rlr10: RLR10,
    #[doc = "0xac - Semaphore 11 read lock register"]
    pub rlr11: RLR11,
    #[doc = "0xb0 - Semaphore 12 read lock register"]
    pub rlr12: RLR12,
    #[doc = "0xb4 - Semaphore 13 read lock register"]
    pub rlr13: RLR13,
    #[doc = "0xb8 - Semaphore 14 read lock register"]
    pub rlr14: RLR14,
    #[doc = "0xbc - Semaphore 15 read lock register"]
    pub rlr15: RLR15,
    #[doc = "0xc0 - Semaphore 16 read lock register"]
    pub rlr16: RLR16,
    #[doc = "0xc4 - Semaphore 17 read lock register"]
    pub rlr17: RLR17,
    #[doc = "0xc8 - Semaphore 18 read lock register"]
    pub rlr18: RLR18,
    #[doc = "0xcc - Semaphore 19 read lock register"]
    pub rlr19: RLR19,
    #[doc = "0xd0 - Semaphore 20 read lock register"]
    pub rlr20: RLR20,
    #[doc = "0xd4 - Semaphore 21 read lock register"]
    pub rlr21: RLR21,
    #[doc = "0xd8 - Semaphore 22 read lock register"]
    pub rlr22: RLR22,
    #[doc = "0xdc - Semaphore 23 read lock register"]
    pub rlr23: RLR23,
    #[doc = "0xe0 - Semaphore 24 read lock register"]
    pub rlr24: RLR24,
    #[doc = "0xe4 - Semaphore 25 read lock register"]
    pub rlr25: RLR25,
    #[doc = "0xe8 - Semaphore 26 read lock register"]
    pub rlr26: RLR26,
    #[doc = "0xec - Semaphore 27 read lock register"]
    pub rlr27: RLR27,
    #[doc = "0xf0 - Semaphore 28 read lock register"]
    pub rlr28: RLR28,
    #[doc = "0xf4 - Semaphore 29 read lock register"]
    pub rlr29: RLR29,
    #[doc = "0xf8 - Semaphore 30 read lock register"]
    pub rlr30: RLR30,
    #[doc = "0xfc - Semaphore 31 read lock register"]
    pub rlr31: RLR31,
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub c1ier0: C1IER0,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub c1icr: C1ICR,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub c1isr: C1ISR,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub c1misr: C1MISR,
    #[doc = "0x110 - HSEM Interrupt enable register"]
    pub c2ier0: C2IER0,
    #[doc = "0x114 - HSEM Interrupt clear register"]
    pub c2icr: C2ICR,
    #[doc = "0x118 - HSEM Interrupt status register"]
    pub c2isr: C2ISR,
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    pub c2misr: C2MISR,
    _reserved72: [u8; 32usize],
    #[doc = "0x140 - Semaphore Clear register"]
    pub cr: CR,
    #[doc = "0x144 - Interrupt clear register"]
    pub keyr: KEYR,
    _reserved74: [u8; 676usize],
    #[doc = "0x3ec - Semaphore hardware configuration register 2"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Semaphore hardware configuration register 1"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - HSEM version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - HSEM indentification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - HSEM size indentification register"]
    pub sidr: SIDR,
}
#[doc = "Semaphore 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r0](r0) module"]
pub type R0 = crate::Reg<u32, _R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R0;
#[doc = "`read()` method returns [r0::R](r0::R) reader structure"]
impl crate::Readable for R0 {}
#[doc = "`write(|w| ..)` method takes [r0::W](r0::W) writer structure"]
impl crate::Writable for R0 {}
#[doc = "Semaphore 0 register"]
pub mod r0;
#[doc = "Semaphore 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1](r1) module"]
pub type R1 = crate::Reg<u32, _R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R1;
#[doc = "`read()` method returns [r1::R](r1::R) reader structure"]
impl crate::Readable for R1 {}
#[doc = "`write(|w| ..)` method takes [r1::W](r1::W) writer structure"]
impl crate::Writable for R1 {}
#[doc = "Semaphore 1 register"]
pub mod r1;
#[doc = "Semaphore 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2](r2) module"]
pub type R2 = crate::Reg<u32, _R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R2;
#[doc = "`read()` method returns [r2::R](r2::R) reader structure"]
impl crate::Readable for R2 {}
#[doc = "`write(|w| ..)` method takes [r2::W](r2::W) writer structure"]
impl crate::Writable for R2 {}
#[doc = "Semaphore 2 register"]
pub mod r2;
#[doc = "Semaphore 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3](r3) module"]
pub type R3 = crate::Reg<u32, _R3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R3;
#[doc = "`read()` method returns [r3::R](r3::R) reader structure"]
impl crate::Readable for R3 {}
#[doc = "`write(|w| ..)` method takes [r3::W](r3::W) writer structure"]
impl crate::Writable for R3 {}
#[doc = "Semaphore 3 register"]
pub mod r3;
#[doc = "Semaphore 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r4](r4) module"]
pub type R4 = crate::Reg<u32, _R4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R4;
#[doc = "`read()` method returns [r4::R](r4::R) reader structure"]
impl crate::Readable for R4 {}
#[doc = "`write(|w| ..)` method takes [r4::W](r4::W) writer structure"]
impl crate::Writable for R4 {}
#[doc = "Semaphore 4 register"]
pub mod r4;
#[doc = "Semaphore 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5](r5) module"]
pub type R5 = crate::Reg<u32, _R5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R5;
#[doc = "`read()` method returns [r5::R](r5::R) reader structure"]
impl crate::Readable for R5 {}
#[doc = "`write(|w| ..)` method takes [r5::W](r5::W) writer structure"]
impl crate::Writable for R5 {}
#[doc = "Semaphore 5 register"]
pub mod r5;
#[doc = "Semaphore 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r6](r6) module"]
pub type R6 = crate::Reg<u32, _R6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R6;
#[doc = "`read()` method returns [r6::R](r6::R) reader structure"]
impl crate::Readable for R6 {}
#[doc = "`write(|w| ..)` method takes [r6::W](r6::W) writer structure"]
impl crate::Writable for R6 {}
#[doc = "Semaphore 6 register"]
pub mod r6;
#[doc = "Semaphore 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r7](r7) module"]
pub type R7 = crate::Reg<u32, _R7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R7;
#[doc = "`read()` method returns [r7::R](r7::R) reader structure"]
impl crate::Readable for R7 {}
#[doc = "`write(|w| ..)` method takes [r7::W](r7::W) writer structure"]
impl crate::Writable for R7 {}
#[doc = "Semaphore 7 register"]
pub mod r7;
#[doc = "Semaphore 8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8](r8) module"]
pub type R8 = crate::Reg<u32, _R8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R8;
#[doc = "`read()` method returns [r8::R](r8::R) reader structure"]
impl crate::Readable for R8 {}
#[doc = "`write(|w| ..)` method takes [r8::W](r8::W) writer structure"]
impl crate::Writable for R8 {}
#[doc = "Semaphore 8 register"]
pub mod r8;
#[doc = "Semaphore 9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r9](r9) module"]
pub type R9 = crate::Reg<u32, _R9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R9;
#[doc = "`read()` method returns [r9::R](r9::R) reader structure"]
impl crate::Readable for R9 {}
#[doc = "`write(|w| ..)` method takes [r9::W](r9::W) writer structure"]
impl crate::Writable for R9 {}
#[doc = "Semaphore 9 register"]
pub mod r9;
#[doc = "Semaphore 10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r10](r10) module"]
pub type R10 = crate::Reg<u32, _R10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R10;
#[doc = "`read()` method returns [r10::R](r10::R) reader structure"]
impl crate::Readable for R10 {}
#[doc = "`write(|w| ..)` method takes [r10::W](r10::W) writer structure"]
impl crate::Writable for R10 {}
#[doc = "Semaphore 10 register"]
pub mod r10;
#[doc = "Semaphore 11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r11](r11) module"]
pub type R11 = crate::Reg<u32, _R11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R11;
#[doc = "`read()` method returns [r11::R](r11::R) reader structure"]
impl crate::Readable for R11 {}
#[doc = "`write(|w| ..)` method takes [r11::W](r11::W) writer structure"]
impl crate::Writable for R11 {}
#[doc = "Semaphore 11 register"]
pub mod r11;
#[doc = "Semaphore 12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r12](r12) module"]
pub type R12 = crate::Reg<u32, _R12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R12;
#[doc = "`read()` method returns [r12::R](r12::R) reader structure"]
impl crate::Readable for R12 {}
#[doc = "`write(|w| ..)` method takes [r12::W](r12::W) writer structure"]
impl crate::Writable for R12 {}
#[doc = "Semaphore 12 register"]
pub mod r12;
#[doc = "Semaphore 13 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r13](r13) module"]
pub type R13 = crate::Reg<u32, _R13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R13;
#[doc = "`read()` method returns [r13::R](r13::R) reader structure"]
impl crate::Readable for R13 {}
#[doc = "`write(|w| ..)` method takes [r13::W](r13::W) writer structure"]
impl crate::Writable for R13 {}
#[doc = "Semaphore 13 register"]
pub mod r13;
#[doc = "Semaphore 14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r14](r14) module"]
pub type R14 = crate::Reg<u32, _R14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R14;
#[doc = "`read()` method returns [r14::R](r14::R) reader structure"]
impl crate::Readable for R14 {}
#[doc = "`write(|w| ..)` method takes [r14::W](r14::W) writer structure"]
impl crate::Writable for R14 {}
#[doc = "Semaphore 14 register"]
pub mod r14;
#[doc = "Semaphore 15 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r15](r15) module"]
pub type R15 = crate::Reg<u32, _R15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R15;
#[doc = "`read()` method returns [r15::R](r15::R) reader structure"]
impl crate::Readable for R15 {}
#[doc = "`write(|w| ..)` method takes [r15::W](r15::W) writer structure"]
impl crate::Writable for R15 {}
#[doc = "Semaphore 15 register"]
pub mod r15;
#[doc = "Semaphore 16 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r16](r16) module"]
pub type R16 = crate::Reg<u32, _R16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R16;
#[doc = "`read()` method returns [r16::R](r16::R) reader structure"]
impl crate::Readable for R16 {}
#[doc = "`write(|w| ..)` method takes [r16::W](r16::W) writer structure"]
impl crate::Writable for R16 {}
#[doc = "Semaphore 16 register"]
pub mod r16;
#[doc = "Semaphore 17 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r17](r17) module"]
pub type R17 = crate::Reg<u32, _R17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R17;
#[doc = "`read()` method returns [r17::R](r17::R) reader structure"]
impl crate::Readable for R17 {}
#[doc = "`write(|w| ..)` method takes [r17::W](r17::W) writer structure"]
impl crate::Writable for R17 {}
#[doc = "Semaphore 17 register"]
pub mod r17;
#[doc = "Semaphore 18 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r18](r18) module"]
pub type R18 = crate::Reg<u32, _R18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R18;
#[doc = "`read()` method returns [r18::R](r18::R) reader structure"]
impl crate::Readable for R18 {}
#[doc = "`write(|w| ..)` method takes [r18::W](r18::W) writer structure"]
impl crate::Writable for R18 {}
#[doc = "Semaphore 18 register"]
pub mod r18;
#[doc = "Semaphore 19 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r19](r19) module"]
pub type R19 = crate::Reg<u32, _R19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R19;
#[doc = "`read()` method returns [r19::R](r19::R) reader structure"]
impl crate::Readable for R19 {}
#[doc = "`write(|w| ..)` method takes [r19::W](r19::W) writer structure"]
impl crate::Writable for R19 {}
#[doc = "Semaphore 19 register"]
pub mod r19;
#[doc = "Semaphore 20 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r20](r20) module"]
pub type R20 = crate::Reg<u32, _R20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R20;
#[doc = "`read()` method returns [r20::R](r20::R) reader structure"]
impl crate::Readable for R20 {}
#[doc = "`write(|w| ..)` method takes [r20::W](r20::W) writer structure"]
impl crate::Writable for R20 {}
#[doc = "Semaphore 20 register"]
pub mod r20;
#[doc = "Semaphore 21 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r21](r21) module"]
pub type R21 = crate::Reg<u32, _R21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R21;
#[doc = "`read()` method returns [r21::R](r21::R) reader structure"]
impl crate::Readable for R21 {}
#[doc = "`write(|w| ..)` method takes [r21::W](r21::W) writer structure"]
impl crate::Writable for R21 {}
#[doc = "Semaphore 21 register"]
pub mod r21;
#[doc = "Semaphore 22 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r22](r22) module"]
pub type R22 = crate::Reg<u32, _R22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R22;
#[doc = "`read()` method returns [r22::R](r22::R) reader structure"]
impl crate::Readable for R22 {}
#[doc = "`write(|w| ..)` method takes [r22::W](r22::W) writer structure"]
impl crate::Writable for R22 {}
#[doc = "Semaphore 22 register"]
pub mod r22;
#[doc = "Semaphore 23 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r23](r23) module"]
pub type R23 = crate::Reg<u32, _R23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R23;
#[doc = "`read()` method returns [r23::R](r23::R) reader structure"]
impl crate::Readable for R23 {}
#[doc = "`write(|w| ..)` method takes [r23::W](r23::W) writer structure"]
impl crate::Writable for R23 {}
#[doc = "Semaphore 23 register"]
pub mod r23;
#[doc = "Semaphore 24 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r24](r24) module"]
pub type R24 = crate::Reg<u32, _R24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R24;
#[doc = "`read()` method returns [r24::R](r24::R) reader structure"]
impl crate::Readable for R24 {}
#[doc = "`write(|w| ..)` method takes [r24::W](r24::W) writer structure"]
impl crate::Writable for R24 {}
#[doc = "Semaphore 24 register"]
pub mod r24;
#[doc = "Semaphore 25 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r25](r25) module"]
pub type R25 = crate::Reg<u32, _R25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R25;
#[doc = "`read()` method returns [r25::R](r25::R) reader structure"]
impl crate::Readable for R25 {}
#[doc = "`write(|w| ..)` method takes [r25::W](r25::W) writer structure"]
impl crate::Writable for R25 {}
#[doc = "Semaphore 25 register"]
pub mod r25;
#[doc = "Semaphore 26 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r26](r26) module"]
pub type R26 = crate::Reg<u32, _R26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R26;
#[doc = "`read()` method returns [r26::R](r26::R) reader structure"]
impl crate::Readable for R26 {}
#[doc = "`write(|w| ..)` method takes [r26::W](r26::W) writer structure"]
impl crate::Writable for R26 {}
#[doc = "Semaphore 26 register"]
pub mod r26;
#[doc = "Semaphore 27 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r27](r27) module"]
pub type R27 = crate::Reg<u32, _R27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R27;
#[doc = "`read()` method returns [r27::R](r27::R) reader structure"]
impl crate::Readable for R27 {}
#[doc = "`write(|w| ..)` method takes [r27::W](r27::W) writer structure"]
impl crate::Writable for R27 {}
#[doc = "Semaphore 27 register"]
pub mod r27;
#[doc = "Semaphore 28 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r28](r28) module"]
pub type R28 = crate::Reg<u32, _R28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R28;
#[doc = "`read()` method returns [r28::R](r28::R) reader structure"]
impl crate::Readable for R28 {}
#[doc = "`write(|w| ..)` method takes [r28::W](r28::W) writer structure"]
impl crate::Writable for R28 {}
#[doc = "Semaphore 28 register"]
pub mod r28;
#[doc = "Semaphore 29 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r29](r29) module"]
pub type R29 = crate::Reg<u32, _R29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R29;
#[doc = "`read()` method returns [r29::R](r29::R) reader structure"]
impl crate::Readable for R29 {}
#[doc = "`write(|w| ..)` method takes [r29::W](r29::W) writer structure"]
impl crate::Writable for R29 {}
#[doc = "Semaphore 29 register"]
pub mod r29;
#[doc = "Semaphore 30 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r30](r30) module"]
pub type R30 = crate::Reg<u32, _R30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R30;
#[doc = "`read()` method returns [r30::R](r30::R) reader structure"]
impl crate::Readable for R30 {}
#[doc = "`write(|w| ..)` method takes [r30::W](r30::W) writer structure"]
impl crate::Writable for R30 {}
#[doc = "Semaphore 30 register"]
pub mod r30;
#[doc = "Semaphore 31 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r31](r31) module"]
pub type R31 = crate::Reg<u32, _R31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R31;
#[doc = "`read()` method returns [r31::R](r31::R) reader structure"]
impl crate::Readable for R31 {}
#[doc = "`write(|w| ..)` method takes [r31::W](r31::W) writer structure"]
impl crate::Writable for R31 {}
#[doc = "Semaphore 31 register"]
pub mod r31;
#[doc = "Semaphore 0 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr0](rlr0) module"]
pub type RLR0 = crate::Reg<u32, _RLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR0;
#[doc = "`read()` method returns [rlr0::R](rlr0::R) reader structure"]
impl crate::Readable for RLR0 {}
#[doc = "Semaphore 0 read lock register"]
pub mod rlr0;
#[doc = "Semaphore 1 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr1](rlr1) module"]
pub type RLR1 = crate::Reg<u32, _RLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR1;
#[doc = "`read()` method returns [rlr1::R](rlr1::R) reader structure"]
impl crate::Readable for RLR1 {}
#[doc = "Semaphore 1 read lock register"]
pub mod rlr1;
#[doc = "Semaphore 2 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr2](rlr2) module"]
pub type RLR2 = crate::Reg<u32, _RLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR2;
#[doc = "`read()` method returns [rlr2::R](rlr2::R) reader structure"]
impl crate::Readable for RLR2 {}
#[doc = "Semaphore 2 read lock register"]
pub mod rlr2;
#[doc = "Semaphore 3 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr3](rlr3) module"]
pub type RLR3 = crate::Reg<u32, _RLR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR3;
#[doc = "`read()` method returns [rlr3::R](rlr3::R) reader structure"]
impl crate::Readable for RLR3 {}
#[doc = "Semaphore 3 read lock register"]
pub mod rlr3;
#[doc = "Semaphore 4 read lock read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr4](rlr4) module"]
pub type RLR4 = crate::Reg<u32, _RLR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR4;
#[doc = "`read()` method returns [rlr4::R](rlr4::R) reader structure"]
impl crate::Readable for RLR4 {}
#[doc = "Semaphore 4 read lock read lock register"]
pub mod rlr4;
#[doc = "Semaphore 5 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr5](rlr5) module"]
pub type RLR5 = crate::Reg<u32, _RLR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR5;
#[doc = "`read()` method returns [rlr5::R](rlr5::R) reader structure"]
impl crate::Readable for RLR5 {}
#[doc = "Semaphore 5 read lock register"]
pub mod rlr5;
#[doc = "Semaphore 6 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr6](rlr6) module"]
pub type RLR6 = crate::Reg<u32, _RLR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR6;
#[doc = "`read()` method returns [rlr6::R](rlr6::R) reader structure"]
impl crate::Readable for RLR6 {}
#[doc = "Semaphore 6 read lock register"]
pub mod rlr6;
#[doc = "Semaphore 7 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr7](rlr7) module"]
pub type RLR7 = crate::Reg<u32, _RLR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR7;
#[doc = "`read()` method returns [rlr7::R](rlr7::R) reader structure"]
impl crate::Readable for RLR7 {}
#[doc = "Semaphore 7 read lock register"]
pub mod rlr7;
#[doc = "Semaphore 8 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr8](rlr8) module"]
pub type RLR8 = crate::Reg<u32, _RLR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR8;
#[doc = "`read()` method returns [rlr8::R](rlr8::R) reader structure"]
impl crate::Readable for RLR8 {}
#[doc = "Semaphore 8 read lock register"]
pub mod rlr8;
#[doc = "Semaphore 9 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr9](rlr9) module"]
pub type RLR9 = crate::Reg<u32, _RLR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR9;
#[doc = "`read()` method returns [rlr9::R](rlr9::R) reader structure"]
impl crate::Readable for RLR9 {}
#[doc = "Semaphore 9 read lock register"]
pub mod rlr9;
#[doc = "Semaphore 10 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr10](rlr10) module"]
pub type RLR10 = crate::Reg<u32, _RLR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR10;
#[doc = "`read()` method returns [rlr10::R](rlr10::R) reader structure"]
impl crate::Readable for RLR10 {}
#[doc = "Semaphore 10 read lock register"]
pub mod rlr10;
#[doc = "Semaphore 11 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr11](rlr11) module"]
pub type RLR11 = crate::Reg<u32, _RLR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR11;
#[doc = "`read()` method returns [rlr11::R](rlr11::R) reader structure"]
impl crate::Readable for RLR11 {}
#[doc = "Semaphore 11 read lock register"]
pub mod rlr11;
#[doc = "Semaphore 12 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr12](rlr12) module"]
pub type RLR12 = crate::Reg<u32, _RLR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR12;
#[doc = "`read()` method returns [rlr12::R](rlr12::R) reader structure"]
impl crate::Readable for RLR12 {}
#[doc = "Semaphore 12 read lock register"]
pub mod rlr12;
#[doc = "Semaphore 13 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr13](rlr13) module"]
pub type RLR13 = crate::Reg<u32, _RLR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR13;
#[doc = "`read()` method returns [rlr13::R](rlr13::R) reader structure"]
impl crate::Readable for RLR13 {}
#[doc = "Semaphore 13 read lock register"]
pub mod rlr13;
#[doc = "Semaphore 14 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr14](rlr14) module"]
pub type RLR14 = crate::Reg<u32, _RLR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR14;
#[doc = "`read()` method returns [rlr14::R](rlr14::R) reader structure"]
impl crate::Readable for RLR14 {}
#[doc = "Semaphore 14 read lock register"]
pub mod rlr14;
#[doc = "Semaphore 15 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr15](rlr15) module"]
pub type RLR15 = crate::Reg<u32, _RLR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR15;
#[doc = "`read()` method returns [rlr15::R](rlr15::R) reader structure"]
impl crate::Readable for RLR15 {}
#[doc = "Semaphore 15 read lock register"]
pub mod rlr15;
#[doc = "Semaphore 16 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr16](rlr16) module"]
pub type RLR16 = crate::Reg<u32, _RLR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR16;
#[doc = "`read()` method returns [rlr16::R](rlr16::R) reader structure"]
impl crate::Readable for RLR16 {}
#[doc = "Semaphore 16 read lock register"]
pub mod rlr16;
#[doc = "Semaphore 17 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr17](rlr17) module"]
pub type RLR17 = crate::Reg<u32, _RLR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR17;
#[doc = "`read()` method returns [rlr17::R](rlr17::R) reader structure"]
impl crate::Readable for RLR17 {}
#[doc = "Semaphore 17 read lock register"]
pub mod rlr17;
#[doc = "Semaphore 18 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr18](rlr18) module"]
pub type RLR18 = crate::Reg<u32, _RLR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR18;
#[doc = "`read()` method returns [rlr18::R](rlr18::R) reader structure"]
impl crate::Readable for RLR18 {}
#[doc = "Semaphore 18 read lock register"]
pub mod rlr18;
#[doc = "Semaphore 19 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr19](rlr19) module"]
pub type RLR19 = crate::Reg<u32, _RLR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR19;
#[doc = "`read()` method returns [rlr19::R](rlr19::R) reader structure"]
impl crate::Readable for RLR19 {}
#[doc = "Semaphore 19 read lock register"]
pub mod rlr19;
#[doc = "Semaphore 20 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr20](rlr20) module"]
pub type RLR20 = crate::Reg<u32, _RLR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR20;
#[doc = "`read()` method returns [rlr20::R](rlr20::R) reader structure"]
impl crate::Readable for RLR20 {}
#[doc = "Semaphore 20 read lock register"]
pub mod rlr20;
#[doc = "Semaphore 21 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr21](rlr21) module"]
pub type RLR21 = crate::Reg<u32, _RLR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR21;
#[doc = "`read()` method returns [rlr21::R](rlr21::R) reader structure"]
impl crate::Readable for RLR21 {}
#[doc = "Semaphore 21 read lock register"]
pub mod rlr21;
#[doc = "Semaphore 22 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr22](rlr22) module"]
pub type RLR22 = crate::Reg<u32, _RLR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR22;
#[doc = "`read()` method returns [rlr22::R](rlr22::R) reader structure"]
impl crate::Readable for RLR22 {}
#[doc = "Semaphore 22 read lock register"]
pub mod rlr22;
#[doc = "Semaphore 23 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr23](rlr23) module"]
pub type RLR23 = crate::Reg<u32, _RLR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR23;
#[doc = "`read()` method returns [rlr23::R](rlr23::R) reader structure"]
impl crate::Readable for RLR23 {}
#[doc = "Semaphore 23 read lock register"]
pub mod rlr23;
#[doc = "Semaphore 24 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr24](rlr24) module"]
pub type RLR24 = crate::Reg<u32, _RLR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR24;
#[doc = "`read()` method returns [rlr24::R](rlr24::R) reader structure"]
impl crate::Readable for RLR24 {}
#[doc = "Semaphore 24 read lock register"]
pub mod rlr24;
#[doc = "Semaphore 25 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr25](rlr25) module"]
pub type RLR25 = crate::Reg<u32, _RLR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR25;
#[doc = "`read()` method returns [rlr25::R](rlr25::R) reader structure"]
impl crate::Readable for RLR25 {}
#[doc = "Semaphore 25 read lock register"]
pub mod rlr25;
#[doc = "Semaphore 26 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr26](rlr26) module"]
pub type RLR26 = crate::Reg<u32, _RLR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR26;
#[doc = "`read()` method returns [rlr26::R](rlr26::R) reader structure"]
impl crate::Readable for RLR26 {}
#[doc = "Semaphore 26 read lock register"]
pub mod rlr26;
#[doc = "Semaphore 27 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr27](rlr27) module"]
pub type RLR27 = crate::Reg<u32, _RLR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR27;
#[doc = "`read()` method returns [rlr27::R](rlr27::R) reader structure"]
impl crate::Readable for RLR27 {}
#[doc = "Semaphore 27 read lock register"]
pub mod rlr27;
#[doc = "Semaphore 28 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr28](rlr28) module"]
pub type RLR28 = crate::Reg<u32, _RLR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR28;
#[doc = "`read()` method returns [rlr28::R](rlr28::R) reader structure"]
impl crate::Readable for RLR28 {}
#[doc = "Semaphore 28 read lock register"]
pub mod rlr28;
#[doc = "Semaphore 29 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr29](rlr29) module"]
pub type RLR29 = crate::Reg<u32, _RLR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR29;
#[doc = "`read()` method returns [rlr29::R](rlr29::R) reader structure"]
impl crate::Readable for RLR29 {}
#[doc = "Semaphore 29 read lock register"]
pub mod rlr29;
#[doc = "Semaphore 30 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr30](rlr30) module"]
pub type RLR30 = crate::Reg<u32, _RLR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR30;
#[doc = "`read()` method returns [rlr30::R](rlr30::R) reader structure"]
impl crate::Readable for RLR30 {}
#[doc = "Semaphore 30 read lock register"]
pub mod rlr30;
#[doc = "Semaphore 31 read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr31](rlr31) module"]
pub type RLR31 = crate::Reg<u32, _RLR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLR31;
#[doc = "`read()` method returns [rlr31::R](rlr31::R) reader structure"]
impl crate::Readable for RLR31 {}
#[doc = "Semaphore 31 read lock register"]
pub mod rlr31;
#[doc = "Semaphore Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Semaphore Clear register"]
pub mod cr;
#[doc = "Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr](keyr) module"]
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
#[doc = "`read()` method returns [keyr::R](keyr::R) reader structure"]
impl crate::Readable for KEYR {}
#[doc = "`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure"]
impl crate::Writable for KEYR {}
#[doc = "Interrupt clear register"]
pub mod keyr;
#[doc = "Semaphore hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](hwcfgr2) module"]
pub type HWCFGR2 = crate::Reg<u32, _HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR2;
#[doc = "`read()` method returns [hwcfgr2::R](hwcfgr2::R) reader structure"]
impl crate::Readable for HWCFGR2 {}
#[doc = "Semaphore hardware configuration register 2"]
pub mod hwcfgr2;
#[doc = "Semaphore hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](hwcfgr1) module"]
pub type HWCFGR1 = crate::Reg<u32, _HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR1;
#[doc = "`read()` method returns [hwcfgr1::R](hwcfgr1::R) reader structure"]
impl crate::Readable for HWCFGR1 {}
#[doc = "Semaphore hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "HSEM version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "HSEM version register"]
pub mod verr;
#[doc = "HSEM indentification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "HSEM indentification register"]
pub mod ipidr;
#[doc = "HSEM size indentification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "HSEM size indentification register"]
pub mod sidr;
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1ier0](c1ier0) module"]
pub type C1IER0 = crate::Reg<u32, _C1IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1IER0;
#[doc = "`read()` method returns [c1ier0::R](c1ier0::R) reader structure"]
impl crate::Readable for C1IER0 {}
#[doc = "`write(|w| ..)` method takes [c1ier0::W](c1ier0::W) writer structure"]
impl crate::Writable for C1IER0 {}
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier0;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1icr](c1icr) module"]
pub type C1ICR = crate::Reg<u32, _C1ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1ICR;
#[doc = "`read()` method returns [c1icr::R](c1icr::R) reader structure"]
impl crate::Readable for C1ICR {}
#[doc = "`write(|w| ..)` method takes [c1icr::W](c1icr::W) writer structure"]
impl crate::Writable for C1ICR {}
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1isr](c1isr) module"]
pub type C1ISR = crate::Reg<u32, _C1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1ISR;
#[doc = "`read()` method returns [c1isr::R](c1isr::R) reader structure"]
impl crate::Readable for C1ISR {}
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1misr](c1misr) module"]
pub type C1MISR = crate::Reg<u32, _C1MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1MISR;
#[doc = "`read()` method returns [c1misr::R](c1misr::R) reader structure"]
impl crate::Readable for C1MISR {}
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ier0](c2ier0) module"]
pub type C2IER0 = crate::Reg<u32, _C2IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IER0;
#[doc = "`read()` method returns [c2ier0::R](c2ier0::R) reader structure"]
impl crate::Readable for C2IER0 {}
#[doc = "`write(|w| ..)` method takes [c2ier0::W](c2ier0::W) writer structure"]
impl crate::Writable for C2IER0 {}
#[doc = "HSEM Interrupt enable register"]
pub mod c2ier0;
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2icr](c2icr) module"]
pub type C2ICR = crate::Reg<u32, _C2ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2ICR;
#[doc = "`read()` method returns [c2icr::R](c2icr::R) reader structure"]
impl crate::Readable for C2ICR {}
#[doc = "`write(|w| ..)` method takes [c2icr::W](c2icr::W) writer structure"]
impl crate::Writable for C2ICR {}
#[doc = "HSEM Interrupt clear register"]
pub mod c2icr;
#[doc = "HSEM Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2isr](c2isr) module"]
pub type C2ISR = crate::Reg<u32, _C2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2ISR;
#[doc = "`read()` method returns [c2isr::R](c2isr::R) reader structure"]
impl crate::Readable for C2ISR {}
#[doc = "HSEM Interrupt status register"]
pub mod c2isr;
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2misr](c2misr) module"]
pub type C2MISR = crate::Reg<u32, _C2MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2MISR;
#[doc = "`read()` method returns [c2misr::R](c2misr::R) reader structure"]
impl crate::Readable for C2MISR {}
#[doc = "HSEM Masked interrupt status register"]
pub mod c2misr;
