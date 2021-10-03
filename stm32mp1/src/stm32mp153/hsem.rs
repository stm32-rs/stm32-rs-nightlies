#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r0: crate::Reg<hsem_r0::HSEM_R0_SPEC>,
    #[doc = "0x04 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r1: crate::Reg<hsem_r1::HSEM_R1_SPEC>,
    #[doc = "0x08 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r2: crate::Reg<hsem_r2::HSEM_R2_SPEC>,
    #[doc = "0x0c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r3: crate::Reg<hsem_r3::HSEM_R3_SPEC>,
    #[doc = "0x10 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r4: crate::Reg<hsem_r4::HSEM_R4_SPEC>,
    #[doc = "0x14 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r5: crate::Reg<hsem_r5::HSEM_R5_SPEC>,
    #[doc = "0x18 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r6: crate::Reg<hsem_r6::HSEM_R6_SPEC>,
    #[doc = "0x1c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r7: crate::Reg<hsem_r7::HSEM_R7_SPEC>,
    #[doc = "0x20 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r8: crate::Reg<hsem_r8::HSEM_R8_SPEC>,
    #[doc = "0x24 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r9: crate::Reg<hsem_r9::HSEM_R9_SPEC>,
    #[doc = "0x28 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r10: crate::Reg<hsem_r10::HSEM_R10_SPEC>,
    #[doc = "0x2c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r11: crate::Reg<hsem_r11::HSEM_R11_SPEC>,
    #[doc = "0x30 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r12: crate::Reg<hsem_r12::HSEM_R12_SPEC>,
    #[doc = "0x34 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r13: crate::Reg<hsem_r13::HSEM_R13_SPEC>,
    #[doc = "0x38 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r14: crate::Reg<hsem_r14::HSEM_R14_SPEC>,
    #[doc = "0x3c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r15: crate::Reg<hsem_r15::HSEM_R15_SPEC>,
    #[doc = "0x40 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r16: crate::Reg<hsem_r16::HSEM_R16_SPEC>,
    #[doc = "0x44 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r17: crate::Reg<hsem_r17::HSEM_R17_SPEC>,
    #[doc = "0x48 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r18: crate::Reg<hsem_r18::HSEM_R18_SPEC>,
    #[doc = "0x4c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r19: crate::Reg<hsem_r19::HSEM_R19_SPEC>,
    #[doc = "0x50 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r20: crate::Reg<hsem_r20::HSEM_R20_SPEC>,
    #[doc = "0x54 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r21: crate::Reg<hsem_r21::HSEM_R21_SPEC>,
    #[doc = "0x58 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r22: crate::Reg<hsem_r22::HSEM_R22_SPEC>,
    #[doc = "0x5c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r23: crate::Reg<hsem_r23::HSEM_R23_SPEC>,
    #[doc = "0x60 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r24: crate::Reg<hsem_r24::HSEM_R24_SPEC>,
    #[doc = "0x64 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r25: crate::Reg<hsem_r25::HSEM_R25_SPEC>,
    #[doc = "0x68 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r26: crate::Reg<hsem_r26::HSEM_R26_SPEC>,
    #[doc = "0x6c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r27: crate::Reg<hsem_r27::HSEM_R27_SPEC>,
    #[doc = "0x70 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r28: crate::Reg<hsem_r28::HSEM_R28_SPEC>,
    #[doc = "0x74 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r29: crate::Reg<hsem_r29::HSEM_R29_SPEC>,
    #[doc = "0x78 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r30: crate::Reg<hsem_r30::HSEM_R30_SPEC>,
    #[doc = "0x7c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_r31: crate::Reg<hsem_r31::HSEM_R31_SPEC>,
    #[doc = "0x80 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr0: crate::Reg<hsem_rlr0::HSEM_RLR0_SPEC>,
    #[doc = "0x84 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr1: crate::Reg<hsem_rlr1::HSEM_RLR1_SPEC>,
    #[doc = "0x88 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr2: crate::Reg<hsem_rlr2::HSEM_RLR2_SPEC>,
    #[doc = "0x8c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr3: crate::Reg<hsem_rlr3::HSEM_RLR3_SPEC>,
    #[doc = "0x90 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr4: crate::Reg<hsem_rlr4::HSEM_RLR4_SPEC>,
    #[doc = "0x94 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr5: crate::Reg<hsem_rlr5::HSEM_RLR5_SPEC>,
    #[doc = "0x98 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr6: crate::Reg<hsem_rlr6::HSEM_RLR6_SPEC>,
    #[doc = "0x9c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr7: crate::Reg<hsem_rlr7::HSEM_RLR7_SPEC>,
    #[doc = "0xa0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr8: crate::Reg<hsem_rlr8::HSEM_RLR8_SPEC>,
    #[doc = "0xa4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr9: crate::Reg<hsem_rlr9::HSEM_RLR9_SPEC>,
    #[doc = "0xa8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr10: crate::Reg<hsem_rlr10::HSEM_RLR10_SPEC>,
    #[doc = "0xac - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr11: crate::Reg<hsem_rlr11::HSEM_RLR11_SPEC>,
    #[doc = "0xb0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr12: crate::Reg<hsem_rlr12::HSEM_RLR12_SPEC>,
    #[doc = "0xb4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr13: crate::Reg<hsem_rlr13::HSEM_RLR13_SPEC>,
    #[doc = "0xb8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr14: crate::Reg<hsem_rlr14::HSEM_RLR14_SPEC>,
    #[doc = "0xbc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr15: crate::Reg<hsem_rlr15::HSEM_RLR15_SPEC>,
    #[doc = "0xc0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr16: crate::Reg<hsem_rlr16::HSEM_RLR16_SPEC>,
    #[doc = "0xc4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr17: crate::Reg<hsem_rlr17::HSEM_RLR17_SPEC>,
    #[doc = "0xc8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr18: crate::Reg<hsem_rlr18::HSEM_RLR18_SPEC>,
    #[doc = "0xcc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr19: crate::Reg<hsem_rlr19::HSEM_RLR19_SPEC>,
    #[doc = "0xd0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr20: crate::Reg<hsem_rlr20::HSEM_RLR20_SPEC>,
    #[doc = "0xd4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr21: crate::Reg<hsem_rlr21::HSEM_RLR21_SPEC>,
    #[doc = "0xd8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr22: crate::Reg<hsem_rlr22::HSEM_RLR22_SPEC>,
    #[doc = "0xdc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr23: crate::Reg<hsem_rlr23::HSEM_RLR23_SPEC>,
    #[doc = "0xe0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr24: crate::Reg<hsem_rlr24::HSEM_RLR24_SPEC>,
    #[doc = "0xe4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr25: crate::Reg<hsem_rlr25::HSEM_RLR25_SPEC>,
    #[doc = "0xe8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr26: crate::Reg<hsem_rlr26::HSEM_RLR26_SPEC>,
    #[doc = "0xec - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr27: crate::Reg<hsem_rlr27::HSEM_RLR27_SPEC>,
    #[doc = "0xf0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr28: crate::Reg<hsem_rlr28::HSEM_RLR28_SPEC>,
    #[doc = "0xf4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr29: crate::Reg<hsem_rlr29::HSEM_RLR29_SPEC>,
    #[doc = "0xf8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr30: crate::Reg<hsem_rlr30::HSEM_RLR30_SPEC>,
    #[doc = "0xfc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_rlr31: crate::Reg<hsem_rlr31::HSEM_RLR31_SPEC>,
    #[doc = "0x100 - HSEM i1terrupt enable register"]
    pub hsem_c1ier: crate::Reg<hsem_c1ier::HSEM_C1IER_SPEC>,
    #[doc = "0x104 - HSEM i1terrupt clear register"]
    pub hsem_c1icr: crate::Reg<hsem_c1icr::HSEM_C1ICR_SPEC>,
    #[doc = "0x108 - HSEM i1terrupt status register"]
    pub hsem_c1isr: crate::Reg<hsem_c1isr::HSEM_C1ISR_SPEC>,
    #[doc = "0x10c - HSEM i1terrupt status register"]
    pub hsem_c1misr: crate::Reg<hsem_c1misr::HSEM_C1MISR_SPEC>,
    #[doc = "0x110 - HSEM i2terrupt enable register"]
    pub hsem_c2ier: crate::Reg<hsem_c2ier::HSEM_C2IER_SPEC>,
    #[doc = "0x114 - HSEM i2terrupt clear register"]
    pub hsem_c2icr: crate::Reg<hsem_c2icr::HSEM_C2ICR_SPEC>,
    #[doc = "0x118 - HSEM i2terrupt status register"]
    pub hsem_c2isr: crate::Reg<hsem_c2isr::HSEM_C2ISR_SPEC>,
    #[doc = "0x11c - HSEM i2terrupt status register"]
    pub hsem_c2misr: crate::Reg<hsem_c2misr::HSEM_C2MISR_SPEC>,
    _reserved72: [u8; 0x20],
    #[doc = "0x140 - Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
    pub hsem_cr: crate::Reg<hsem_cr::HSEM_CR_SPEC>,
    #[doc = "0x144 - HSEM interrupt clear register"]
    pub hsem_keyr: crate::Reg<hsem_keyr::HSEM_KEYR_SPEC>,
    _reserved74: [u8; 0x02a4],
    #[doc = "0x3ec - HSEM hardware configuration register 2"]
    pub hsem_hwcfgr2: crate::Reg<hsem_hwcfgr2::HSEM_HWCFGR2_SPEC>,
    #[doc = "0x3f0 - HSEM hardware configuration register 1"]
    pub hsem_hwcfgr1: crate::Reg<hsem_hwcfgr1::HSEM_HWCFGR1_SPEC>,
    #[doc = "0x3f4 - HSEM IP version register"]
    pub hsem_verr: crate::Reg<hsem_verr::HSEM_VERR_SPEC>,
    #[doc = "0x3f8 - HSEM IP identification register"]
    pub hsem_ipidr: crate::Reg<hsem_ipidr::HSEM_IPIDR_SPEC>,
    #[doc = "0x3fc - HSEM size identification register"]
    pub hsem_sidr: crate::Reg<hsem_sidr::HSEM_SIDR_SPEC>,
}
#[doc = "HSEM_R0 register accessor: an alias for `Reg<HSEM_R0_SPEC>`"]
pub type HSEM_R0 = crate::Reg<hsem_r0::HSEM_R0_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r0;
#[doc = "HSEM_R1 register accessor: an alias for `Reg<HSEM_R1_SPEC>`"]
pub type HSEM_R1 = crate::Reg<hsem_r1::HSEM_R1_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r1;
#[doc = "HSEM_R2 register accessor: an alias for `Reg<HSEM_R2_SPEC>`"]
pub type HSEM_R2 = crate::Reg<hsem_r2::HSEM_R2_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r2;
#[doc = "HSEM_R3 register accessor: an alias for `Reg<HSEM_R3_SPEC>`"]
pub type HSEM_R3 = crate::Reg<hsem_r3::HSEM_R3_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r3;
#[doc = "HSEM_R4 register accessor: an alias for `Reg<HSEM_R4_SPEC>`"]
pub type HSEM_R4 = crate::Reg<hsem_r4::HSEM_R4_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r4;
#[doc = "HSEM_R5 register accessor: an alias for `Reg<HSEM_R5_SPEC>`"]
pub type HSEM_R5 = crate::Reg<hsem_r5::HSEM_R5_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r5;
#[doc = "HSEM_R6 register accessor: an alias for `Reg<HSEM_R6_SPEC>`"]
pub type HSEM_R6 = crate::Reg<hsem_r6::HSEM_R6_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r6;
#[doc = "HSEM_R7 register accessor: an alias for `Reg<HSEM_R7_SPEC>`"]
pub type HSEM_R7 = crate::Reg<hsem_r7::HSEM_R7_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r7;
#[doc = "HSEM_R8 register accessor: an alias for `Reg<HSEM_R8_SPEC>`"]
pub type HSEM_R8 = crate::Reg<hsem_r8::HSEM_R8_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r8;
#[doc = "HSEM_R9 register accessor: an alias for `Reg<HSEM_R9_SPEC>`"]
pub type HSEM_R9 = crate::Reg<hsem_r9::HSEM_R9_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r9;
#[doc = "HSEM_R10 register accessor: an alias for `Reg<HSEM_R10_SPEC>`"]
pub type HSEM_R10 = crate::Reg<hsem_r10::HSEM_R10_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r10;
#[doc = "HSEM_R11 register accessor: an alias for `Reg<HSEM_R11_SPEC>`"]
pub type HSEM_R11 = crate::Reg<hsem_r11::HSEM_R11_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r11;
#[doc = "HSEM_R12 register accessor: an alias for `Reg<HSEM_R12_SPEC>`"]
pub type HSEM_R12 = crate::Reg<hsem_r12::HSEM_R12_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r12;
#[doc = "HSEM_R13 register accessor: an alias for `Reg<HSEM_R13_SPEC>`"]
pub type HSEM_R13 = crate::Reg<hsem_r13::HSEM_R13_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r13;
#[doc = "HSEM_R14 register accessor: an alias for `Reg<HSEM_R14_SPEC>`"]
pub type HSEM_R14 = crate::Reg<hsem_r14::HSEM_R14_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r14;
#[doc = "HSEM_R15 register accessor: an alias for `Reg<HSEM_R15_SPEC>`"]
pub type HSEM_R15 = crate::Reg<hsem_r15::HSEM_R15_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r15;
#[doc = "HSEM_R16 register accessor: an alias for `Reg<HSEM_R16_SPEC>`"]
pub type HSEM_R16 = crate::Reg<hsem_r16::HSEM_R16_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r16;
#[doc = "HSEM_R17 register accessor: an alias for `Reg<HSEM_R17_SPEC>`"]
pub type HSEM_R17 = crate::Reg<hsem_r17::HSEM_R17_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r17;
#[doc = "HSEM_R18 register accessor: an alias for `Reg<HSEM_R18_SPEC>`"]
pub type HSEM_R18 = crate::Reg<hsem_r18::HSEM_R18_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r18;
#[doc = "HSEM_R19 register accessor: an alias for `Reg<HSEM_R19_SPEC>`"]
pub type HSEM_R19 = crate::Reg<hsem_r19::HSEM_R19_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r19;
#[doc = "HSEM_R20 register accessor: an alias for `Reg<HSEM_R20_SPEC>`"]
pub type HSEM_R20 = crate::Reg<hsem_r20::HSEM_R20_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r20;
#[doc = "HSEM_R21 register accessor: an alias for `Reg<HSEM_R21_SPEC>`"]
pub type HSEM_R21 = crate::Reg<hsem_r21::HSEM_R21_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r21;
#[doc = "HSEM_R22 register accessor: an alias for `Reg<HSEM_R22_SPEC>`"]
pub type HSEM_R22 = crate::Reg<hsem_r22::HSEM_R22_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r22;
#[doc = "HSEM_R23 register accessor: an alias for `Reg<HSEM_R23_SPEC>`"]
pub type HSEM_R23 = crate::Reg<hsem_r23::HSEM_R23_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r23;
#[doc = "HSEM_R24 register accessor: an alias for `Reg<HSEM_R24_SPEC>`"]
pub type HSEM_R24 = crate::Reg<hsem_r24::HSEM_R24_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r24;
#[doc = "HSEM_R25 register accessor: an alias for `Reg<HSEM_R25_SPEC>`"]
pub type HSEM_R25 = crate::Reg<hsem_r25::HSEM_R25_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r25;
#[doc = "HSEM_R26 register accessor: an alias for `Reg<HSEM_R26_SPEC>`"]
pub type HSEM_R26 = crate::Reg<hsem_r26::HSEM_R26_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r26;
#[doc = "HSEM_R27 register accessor: an alias for `Reg<HSEM_R27_SPEC>`"]
pub type HSEM_R27 = crate::Reg<hsem_r27::HSEM_R27_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r27;
#[doc = "HSEM_R28 register accessor: an alias for `Reg<HSEM_R28_SPEC>`"]
pub type HSEM_R28 = crate::Reg<hsem_r28::HSEM_R28_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r28;
#[doc = "HSEM_R29 register accessor: an alias for `Reg<HSEM_R29_SPEC>`"]
pub type HSEM_R29 = crate::Reg<hsem_r29::HSEM_R29_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r29;
#[doc = "HSEM_R30 register accessor: an alias for `Reg<HSEM_R30_SPEC>`"]
pub type HSEM_R30 = crate::Reg<hsem_r30::HSEM_R30_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r30;
#[doc = "HSEM_R31 register accessor: an alias for `Reg<HSEM_R31_SPEC>`"]
pub type HSEM_R31 = crate::Reg<hsem_r31::HSEM_R31_SPEC>;
#[doc = "The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_r31;
#[doc = "HSEM_RLR0 register accessor: an alias for `Reg<HSEM_RLR0_SPEC>`"]
pub type HSEM_RLR0 = crate::Reg<hsem_rlr0::HSEM_RLR0_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr0;
#[doc = "HSEM_RLR1 register accessor: an alias for `Reg<HSEM_RLR1_SPEC>`"]
pub type HSEM_RLR1 = crate::Reg<hsem_rlr1::HSEM_RLR1_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr1;
#[doc = "HSEM_RLR2 register accessor: an alias for `Reg<HSEM_RLR2_SPEC>`"]
pub type HSEM_RLR2 = crate::Reg<hsem_rlr2::HSEM_RLR2_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr2;
#[doc = "HSEM_RLR3 register accessor: an alias for `Reg<HSEM_RLR3_SPEC>`"]
pub type HSEM_RLR3 = crate::Reg<hsem_rlr3::HSEM_RLR3_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr3;
#[doc = "HSEM_RLR4 register accessor: an alias for `Reg<HSEM_RLR4_SPEC>`"]
pub type HSEM_RLR4 = crate::Reg<hsem_rlr4::HSEM_RLR4_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr4;
#[doc = "HSEM_RLR5 register accessor: an alias for `Reg<HSEM_RLR5_SPEC>`"]
pub type HSEM_RLR5 = crate::Reg<hsem_rlr5::HSEM_RLR5_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr5;
#[doc = "HSEM_RLR6 register accessor: an alias for `Reg<HSEM_RLR6_SPEC>`"]
pub type HSEM_RLR6 = crate::Reg<hsem_rlr6::HSEM_RLR6_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr6;
#[doc = "HSEM_RLR7 register accessor: an alias for `Reg<HSEM_RLR7_SPEC>`"]
pub type HSEM_RLR7 = crate::Reg<hsem_rlr7::HSEM_RLR7_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr7;
#[doc = "HSEM_RLR8 register accessor: an alias for `Reg<HSEM_RLR8_SPEC>`"]
pub type HSEM_RLR8 = crate::Reg<hsem_rlr8::HSEM_RLR8_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr8;
#[doc = "HSEM_RLR9 register accessor: an alias for `Reg<HSEM_RLR9_SPEC>`"]
pub type HSEM_RLR9 = crate::Reg<hsem_rlr9::HSEM_RLR9_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr9;
#[doc = "HSEM_RLR10 register accessor: an alias for `Reg<HSEM_RLR10_SPEC>`"]
pub type HSEM_RLR10 = crate::Reg<hsem_rlr10::HSEM_RLR10_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr10;
#[doc = "HSEM_RLR11 register accessor: an alias for `Reg<HSEM_RLR11_SPEC>`"]
pub type HSEM_RLR11 = crate::Reg<hsem_rlr11::HSEM_RLR11_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr11;
#[doc = "HSEM_RLR12 register accessor: an alias for `Reg<HSEM_RLR12_SPEC>`"]
pub type HSEM_RLR12 = crate::Reg<hsem_rlr12::HSEM_RLR12_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr12;
#[doc = "HSEM_RLR13 register accessor: an alias for `Reg<HSEM_RLR13_SPEC>`"]
pub type HSEM_RLR13 = crate::Reg<hsem_rlr13::HSEM_RLR13_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr13;
#[doc = "HSEM_RLR14 register accessor: an alias for `Reg<HSEM_RLR14_SPEC>`"]
pub type HSEM_RLR14 = crate::Reg<hsem_rlr14::HSEM_RLR14_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr14;
#[doc = "HSEM_RLR15 register accessor: an alias for `Reg<HSEM_RLR15_SPEC>`"]
pub type HSEM_RLR15 = crate::Reg<hsem_rlr15::HSEM_RLR15_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr15;
#[doc = "HSEM_RLR16 register accessor: an alias for `Reg<HSEM_RLR16_SPEC>`"]
pub type HSEM_RLR16 = crate::Reg<hsem_rlr16::HSEM_RLR16_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr16;
#[doc = "HSEM_RLR17 register accessor: an alias for `Reg<HSEM_RLR17_SPEC>`"]
pub type HSEM_RLR17 = crate::Reg<hsem_rlr17::HSEM_RLR17_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr17;
#[doc = "HSEM_RLR18 register accessor: an alias for `Reg<HSEM_RLR18_SPEC>`"]
pub type HSEM_RLR18 = crate::Reg<hsem_rlr18::HSEM_RLR18_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr18;
#[doc = "HSEM_RLR19 register accessor: an alias for `Reg<HSEM_RLR19_SPEC>`"]
pub type HSEM_RLR19 = crate::Reg<hsem_rlr19::HSEM_RLR19_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr19;
#[doc = "HSEM_RLR20 register accessor: an alias for `Reg<HSEM_RLR20_SPEC>`"]
pub type HSEM_RLR20 = crate::Reg<hsem_rlr20::HSEM_RLR20_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr20;
#[doc = "HSEM_RLR21 register accessor: an alias for `Reg<HSEM_RLR21_SPEC>`"]
pub type HSEM_RLR21 = crate::Reg<hsem_rlr21::HSEM_RLR21_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr21;
#[doc = "HSEM_RLR22 register accessor: an alias for `Reg<HSEM_RLR22_SPEC>`"]
pub type HSEM_RLR22 = crate::Reg<hsem_rlr22::HSEM_RLR22_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr22;
#[doc = "HSEM_RLR23 register accessor: an alias for `Reg<HSEM_RLR23_SPEC>`"]
pub type HSEM_RLR23 = crate::Reg<hsem_rlr23::HSEM_RLR23_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr23;
#[doc = "HSEM_RLR24 register accessor: an alias for `Reg<HSEM_RLR24_SPEC>`"]
pub type HSEM_RLR24 = crate::Reg<hsem_rlr24::HSEM_RLR24_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr24;
#[doc = "HSEM_RLR25 register accessor: an alias for `Reg<HSEM_RLR25_SPEC>`"]
pub type HSEM_RLR25 = crate::Reg<hsem_rlr25::HSEM_RLR25_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr25;
#[doc = "HSEM_RLR26 register accessor: an alias for `Reg<HSEM_RLR26_SPEC>`"]
pub type HSEM_RLR26 = crate::Reg<hsem_rlr26::HSEM_RLR26_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr26;
#[doc = "HSEM_RLR27 register accessor: an alias for `Reg<HSEM_RLR27_SPEC>`"]
pub type HSEM_RLR27 = crate::Reg<hsem_rlr27::HSEM_RLR27_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr27;
#[doc = "HSEM_RLR28 register accessor: an alias for `Reg<HSEM_RLR28_SPEC>`"]
pub type HSEM_RLR28 = crate::Reg<hsem_rlr28::HSEM_RLR28_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr28;
#[doc = "HSEM_RLR29 register accessor: an alias for `Reg<HSEM_RLR29_SPEC>`"]
pub type HSEM_RLR29 = crate::Reg<hsem_rlr29::HSEM_RLR29_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr29;
#[doc = "HSEM_RLR30 register accessor: an alias for `Reg<HSEM_RLR30_SPEC>`"]
pub type HSEM_RLR30 = crate::Reg<hsem_rlr30::HSEM_RLR30_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr30;
#[doc = "HSEM_RLR31 register accessor: an alias for `Reg<HSEM_RLR31_SPEC>`"]
pub type HSEM_RLR31 = crate::Reg<hsem_rlr31::HSEM_RLR31_SPEC>;
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_rlr31;
#[doc = "HSEM_C1IER register accessor: an alias for `Reg<HSEM_C1IER_SPEC>`"]
pub type HSEM_C1IER = crate::Reg<hsem_c1ier::HSEM_C1IER_SPEC>;
#[doc = "HSEM i1terrupt enable register"]
pub mod hsem_c1ier;
#[doc = "HSEM_C1ICR register accessor: an alias for `Reg<HSEM_C1ICR_SPEC>`"]
pub type HSEM_C1ICR = crate::Reg<hsem_c1icr::HSEM_C1ICR_SPEC>;
#[doc = "HSEM i1terrupt clear register"]
pub mod hsem_c1icr;
#[doc = "HSEM_C1ISR register accessor: an alias for `Reg<HSEM_C1ISR_SPEC>`"]
pub type HSEM_C1ISR = crate::Reg<hsem_c1isr::HSEM_C1ISR_SPEC>;
#[doc = "HSEM i1terrupt status register"]
pub mod hsem_c1isr;
#[doc = "HSEM_C1MISR register accessor: an alias for `Reg<HSEM_C1MISR_SPEC>`"]
pub type HSEM_C1MISR = crate::Reg<hsem_c1misr::HSEM_C1MISR_SPEC>;
#[doc = "HSEM i1terrupt status register"]
pub mod hsem_c1misr;
#[doc = "HSEM_C2IER register accessor: an alias for `Reg<HSEM_C2IER_SPEC>`"]
pub type HSEM_C2IER = crate::Reg<hsem_c2ier::HSEM_C2IER_SPEC>;
#[doc = "HSEM i2terrupt enable register"]
pub mod hsem_c2ier;
#[doc = "HSEM_C2ICR register accessor: an alias for `Reg<HSEM_C2ICR_SPEC>`"]
pub type HSEM_C2ICR = crate::Reg<hsem_c2icr::HSEM_C2ICR_SPEC>;
#[doc = "HSEM i2terrupt clear register"]
pub mod hsem_c2icr;
#[doc = "HSEM_C2ISR register accessor: an alias for `Reg<HSEM_C2ISR_SPEC>`"]
pub type HSEM_C2ISR = crate::Reg<hsem_c2isr::HSEM_C2ISR_SPEC>;
#[doc = "HSEM i2terrupt status register"]
pub mod hsem_c2isr;
#[doc = "HSEM_C2MISR register accessor: an alias for `Reg<HSEM_C2MISR_SPEC>`"]
pub type HSEM_C2MISR = crate::Reg<hsem_c2misr::HSEM_C2MISR_SPEC>;
#[doc = "HSEM i2terrupt status register"]
pub mod hsem_c2misr;
#[doc = "HSEM_CR register accessor: an alias for `Reg<HSEM_CR_SPEC>`"]
pub type HSEM_CR = crate::Reg<hsem_cr::HSEM_CR_SPEC>;
#[doc = "Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded."]
pub mod hsem_cr;
#[doc = "HSEM_KEYR register accessor: an alias for `Reg<HSEM_KEYR_SPEC>`"]
pub type HSEM_KEYR = crate::Reg<hsem_keyr::HSEM_KEYR_SPEC>;
#[doc = "HSEM interrupt clear register"]
pub mod hsem_keyr;
#[doc = "HSEM_HWCFGR2 register accessor: an alias for `Reg<HSEM_HWCFGR2_SPEC>`"]
pub type HSEM_HWCFGR2 = crate::Reg<hsem_hwcfgr2::HSEM_HWCFGR2_SPEC>;
#[doc = "HSEM hardware configuration register 2"]
pub mod hsem_hwcfgr2;
#[doc = "HSEM_HWCFGR1 register accessor: an alias for `Reg<HSEM_HWCFGR1_SPEC>`"]
pub type HSEM_HWCFGR1 = crate::Reg<hsem_hwcfgr1::HSEM_HWCFGR1_SPEC>;
#[doc = "HSEM hardware configuration register 1"]
pub mod hsem_hwcfgr1;
#[doc = "HSEM_VERR register accessor: an alias for `Reg<HSEM_VERR_SPEC>`"]
pub type HSEM_VERR = crate::Reg<hsem_verr::HSEM_VERR_SPEC>;
#[doc = "HSEM IP version register"]
pub mod hsem_verr;
#[doc = "HSEM_IPIDR register accessor: an alias for `Reg<HSEM_IPIDR_SPEC>`"]
pub type HSEM_IPIDR = crate::Reg<hsem_ipidr::HSEM_IPIDR_SPEC>;
#[doc = "HSEM IP identification register"]
pub mod hsem_ipidr;
#[doc = "HSEM_SIDR register accessor: an alias for `Reg<HSEM_SIDR_SPEC>`"]
pub type HSEM_SIDR = crate::Reg<hsem_sidr::HSEM_SIDR_SPEC>;
#[doc = "HSEM size identification register"]
pub mod hsem_sidr;
