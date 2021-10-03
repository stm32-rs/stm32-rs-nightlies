#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSEM register HSEM_R0 HSEM_R31"]
    pub r0: crate::Reg<r0::R0_SPEC>,
    #[doc = "0x04 - HSEM register HSEM_R0 HSEM_R31"]
    pub r1: crate::Reg<r1::R1_SPEC>,
    #[doc = "0x08 - HSEM register HSEM_R0 HSEM_R31"]
    pub r2: crate::Reg<r2::R2_SPEC>,
    #[doc = "0x0c - HSEM register HSEM_R0 HSEM_R31"]
    pub r3: crate::Reg<r3::R3_SPEC>,
    #[doc = "0x10 - HSEM register HSEM_R0 HSEM_R31"]
    pub r4: crate::Reg<r4::R4_SPEC>,
    #[doc = "0x14 - HSEM register HSEM_R0 HSEM_R31"]
    pub r5: crate::Reg<r5::R5_SPEC>,
    #[doc = "0x18 - HSEM register HSEM_R0 HSEM_R31"]
    pub r6: crate::Reg<r6::R6_SPEC>,
    #[doc = "0x1c - HSEM register HSEM_R0 HSEM_R31"]
    pub r7: crate::Reg<r7::R7_SPEC>,
    #[doc = "0x20 - HSEM register HSEM_R0 HSEM_R31"]
    pub r8: crate::Reg<r8::R8_SPEC>,
    #[doc = "0x24 - HSEM register HSEM_R0 HSEM_R31"]
    pub r9: crate::Reg<r9::R9_SPEC>,
    #[doc = "0x28 - HSEM register HSEM_R0 HSEM_R31"]
    pub r10: crate::Reg<r10::R10_SPEC>,
    #[doc = "0x2c - HSEM register HSEM_R0 HSEM_R31"]
    pub r11: crate::Reg<r11::R11_SPEC>,
    #[doc = "0x30 - HSEM register HSEM_R0 HSEM_R31"]
    pub r12: crate::Reg<r12::R12_SPEC>,
    #[doc = "0x34 - HSEM register HSEM_R0 HSEM_R31"]
    pub r13: crate::Reg<r13::R13_SPEC>,
    #[doc = "0x38 - HSEM register HSEM_R0 HSEM_R31"]
    pub r14: crate::Reg<r14::R14_SPEC>,
    #[doc = "0x3c - HSEM register HSEM_R0 HSEM_R31"]
    pub r15: crate::Reg<r15::R15_SPEC>,
    _reserved16: [u8; 0x40],
    #[doc = "0x80 - HSEM Read lock register"]
    pub rlr0: crate::Reg<rlr0::RLR0_SPEC>,
    #[doc = "0x84 - HSEM Read lock register"]
    pub rlr1: crate::Reg<rlr1::RLR1_SPEC>,
    #[doc = "0x88 - HSEM Read lock register"]
    pub rlr2: crate::Reg<rlr2::RLR2_SPEC>,
    #[doc = "0x8c - HSEM Read lock register"]
    pub rlr3: crate::Reg<rlr3::RLR3_SPEC>,
    #[doc = "0x90 - HSEM Read lock register"]
    pub rlr4: crate::Reg<rlr4::RLR4_SPEC>,
    #[doc = "0x94 - HSEM Read lock register"]
    pub rlr5: crate::Reg<rlr5::RLR5_SPEC>,
    #[doc = "0x98 - HSEM Read lock register"]
    pub rlr6: crate::Reg<rlr6::RLR6_SPEC>,
    #[doc = "0x9c - HSEM Read lock register"]
    pub rlr7: crate::Reg<rlr7::RLR7_SPEC>,
    #[doc = "0xa0 - HSEM Read lock register"]
    pub rlr8: crate::Reg<rlr8::RLR8_SPEC>,
    #[doc = "0xa4 - HSEM Read lock register"]
    pub rlr9: crate::Reg<rlr9::RLR9_SPEC>,
    #[doc = "0xa8 - HSEM Read lock register"]
    pub rlr10: crate::Reg<rlr10::RLR10_SPEC>,
    #[doc = "0xac - HSEM Read lock register"]
    pub rlr11: crate::Reg<rlr11::RLR11_SPEC>,
    #[doc = "0xb0 - HSEM Read lock register"]
    pub rlr12: crate::Reg<rlr12::RLR12_SPEC>,
    #[doc = "0xb4 - HSEM Read lock register"]
    pub rlr13: crate::Reg<rlr13::RLR13_SPEC>,
    #[doc = "0xb8 - HSEM Read lock register"]
    pub rlr14: crate::Reg<rlr14::RLR14_SPEC>,
    #[doc = "0xbc - HSEM Read lock register"]
    pub rlr15: crate::Reg<rlr15::RLR15_SPEC>,
    _reserved32: [u8; 0x40],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub c1ier: crate::Reg<c1ier::C1IER_SPEC>,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub c1icr: crate::Reg<c1icr::C1ICR_SPEC>,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub c1isr: crate::Reg<c1isr::C1ISR_SPEC>,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub c1misr: crate::Reg<c1misr::C1MISR_SPEC>,
    _reserved36: [u8; 0x30],
    #[doc = "0x140 - HSEM Clear register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
}
#[doc = "R0 register accessor: an alias for `Reg<R0_SPEC>`"]
pub type R0 = crate::Reg<r0::R0_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r0;
#[doc = "R1 register accessor: an alias for `Reg<R1_SPEC>`"]
pub type R1 = crate::Reg<r1::R1_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r1;
#[doc = "R2 register accessor: an alias for `Reg<R2_SPEC>`"]
pub type R2 = crate::Reg<r2::R2_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r2;
#[doc = "R3 register accessor: an alias for `Reg<R3_SPEC>`"]
pub type R3 = crate::Reg<r3::R3_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r3;
#[doc = "R4 register accessor: an alias for `Reg<R4_SPEC>`"]
pub type R4 = crate::Reg<r4::R4_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r4;
#[doc = "R5 register accessor: an alias for `Reg<R5_SPEC>`"]
pub type R5 = crate::Reg<r5::R5_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r5;
#[doc = "R6 register accessor: an alias for `Reg<R6_SPEC>`"]
pub type R6 = crate::Reg<r6::R6_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r6;
#[doc = "R7 register accessor: an alias for `Reg<R7_SPEC>`"]
pub type R7 = crate::Reg<r7::R7_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r7;
#[doc = "R8 register accessor: an alias for `Reg<R8_SPEC>`"]
pub type R8 = crate::Reg<r8::R8_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r8;
#[doc = "R9 register accessor: an alias for `Reg<R9_SPEC>`"]
pub type R9 = crate::Reg<r9::R9_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r9;
#[doc = "R10 register accessor: an alias for `Reg<R10_SPEC>`"]
pub type R10 = crate::Reg<r10::R10_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r10;
#[doc = "R11 register accessor: an alias for `Reg<R11_SPEC>`"]
pub type R11 = crate::Reg<r11::R11_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r11;
#[doc = "R12 register accessor: an alias for `Reg<R12_SPEC>`"]
pub type R12 = crate::Reg<r12::R12_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r12;
#[doc = "R13 register accessor: an alias for `Reg<R13_SPEC>`"]
pub type R13 = crate::Reg<r13::R13_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r13;
#[doc = "R14 register accessor: an alias for `Reg<R14_SPEC>`"]
pub type R14 = crate::Reg<r14::R14_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r14;
#[doc = "R15 register accessor: an alias for `Reg<R15_SPEC>`"]
pub type R15 = crate::Reg<r15::R15_SPEC>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod r15;
#[doc = "RLR0 register accessor: an alias for `Reg<RLR0_SPEC>`"]
pub type RLR0 = crate::Reg<rlr0::RLR0_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr0;
#[doc = "RLR1 register accessor: an alias for `Reg<RLR1_SPEC>`"]
pub type RLR1 = crate::Reg<rlr1::RLR1_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr1;
#[doc = "RLR2 register accessor: an alias for `Reg<RLR2_SPEC>`"]
pub type RLR2 = crate::Reg<rlr2::RLR2_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr2;
#[doc = "RLR3 register accessor: an alias for `Reg<RLR3_SPEC>`"]
pub type RLR3 = crate::Reg<rlr3::RLR3_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr3;
#[doc = "RLR4 register accessor: an alias for `Reg<RLR4_SPEC>`"]
pub type RLR4 = crate::Reg<rlr4::RLR4_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr4;
#[doc = "RLR5 register accessor: an alias for `Reg<RLR5_SPEC>`"]
pub type RLR5 = crate::Reg<rlr5::RLR5_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr5;
#[doc = "RLR6 register accessor: an alias for `Reg<RLR6_SPEC>`"]
pub type RLR6 = crate::Reg<rlr6::RLR6_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr6;
#[doc = "RLR7 register accessor: an alias for `Reg<RLR7_SPEC>`"]
pub type RLR7 = crate::Reg<rlr7::RLR7_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr7;
#[doc = "RLR8 register accessor: an alias for `Reg<RLR8_SPEC>`"]
pub type RLR8 = crate::Reg<rlr8::RLR8_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr8;
#[doc = "RLR9 register accessor: an alias for `Reg<RLR9_SPEC>`"]
pub type RLR9 = crate::Reg<rlr9::RLR9_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr9;
#[doc = "RLR10 register accessor: an alias for `Reg<RLR10_SPEC>`"]
pub type RLR10 = crate::Reg<rlr10::RLR10_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr10;
#[doc = "RLR11 register accessor: an alias for `Reg<RLR11_SPEC>`"]
pub type RLR11 = crate::Reg<rlr11::RLR11_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr11;
#[doc = "RLR12 register accessor: an alias for `Reg<RLR12_SPEC>`"]
pub type RLR12 = crate::Reg<rlr12::RLR12_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr12;
#[doc = "RLR13 register accessor: an alias for `Reg<RLR13_SPEC>`"]
pub type RLR13 = crate::Reg<rlr13::RLR13_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr13;
#[doc = "RLR14 register accessor: an alias for `Reg<RLR14_SPEC>`"]
pub type RLR14 = crate::Reg<rlr14::RLR14_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr14;
#[doc = "RLR15 register accessor: an alias for `Reg<RLR15_SPEC>`"]
pub type RLR15 = crate::Reg<rlr15::RLR15_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr15;
#[doc = "C1IER register accessor: an alias for `Reg<C1IER_SPEC>`"]
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier;
#[doc = "C1ICR register accessor: an alias for `Reg<C1ICR_SPEC>`"]
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "C1ISR register accessor: an alias for `Reg<C1ISR_SPEC>`"]
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "C1MISR register accessor: an alias for `Reg<C1MISR_SPEC>`"]
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
