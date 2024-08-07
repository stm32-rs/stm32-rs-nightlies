#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    hsem_r0: HSEM_R0,
    hsem_r1: HSEM_R1,
    hsem_r2: HSEM_R2,
    hsem_r3: HSEM_R3,
    hsem_r4: HSEM_R4,
    hsem_r5: HSEM_R5,
    hsem_r6: HSEM_R6,
    hsem_r7: HSEM_R7,
    hsem_r8: HSEM_R8,
    hsem_r9: HSEM_R9,
    hsem_r10: HSEM_R10,
    hsem_r11: HSEM_R11,
    hsem_r12: HSEM_R12,
    hsem_r13: HSEM_R13,
    hsem_r14: HSEM_R14,
    hsem_r15: HSEM_R15,
    hsem_r16: HSEM_R16,
    hsem_r17: HSEM_R17,
    hsem_r18: HSEM_R18,
    hsem_r19: HSEM_R19,
    hsem_r20: HSEM_R20,
    hsem_r21: HSEM_R21,
    hsem_r22: HSEM_R22,
    hsem_r23: HSEM_R23,
    hsem_r24: HSEM_R24,
    hsem_r25: HSEM_R25,
    hsem_r26: HSEM_R26,
    hsem_r27: HSEM_R27,
    hsem_r28: HSEM_R28,
    hsem_r29: HSEM_R29,
    hsem_r30: HSEM_R30,
    hsem_r31: HSEM_R31,
    hsem_rlr0: HSEM_RLR0,
    hsem_rlr1: HSEM_RLR1,
    hsem_rlr2: HSEM_RLR2,
    hsem_rlr3: HSEM_RLR3,
    hsem_rlr4: HSEM_RLR4,
    hsem_rlr5: HSEM_RLR5,
    hsem_rlr6: HSEM_RLR6,
    hsem_rlr7: HSEM_RLR7,
    hsem_rlr8: HSEM_RLR8,
    hsem_rlr9: HSEM_RLR9,
    hsem_rlr10: HSEM_RLR10,
    hsem_rlr11: HSEM_RLR11,
    hsem_rlr12: HSEM_RLR12,
    hsem_rlr13: HSEM_RLR13,
    hsem_rlr14: HSEM_RLR14,
    hsem_rlr15: HSEM_RLR15,
    hsem_rlr16: HSEM_RLR16,
    hsem_rlr17: HSEM_RLR17,
    hsem_rlr18: HSEM_RLR18,
    hsem_rlr19: HSEM_RLR19,
    hsem_rlr20: HSEM_RLR20,
    hsem_rlr21: HSEM_RLR21,
    hsem_rlr22: HSEM_RLR22,
    hsem_rlr23: HSEM_RLR23,
    hsem_rlr24: HSEM_RLR24,
    hsem_rlr25: HSEM_RLR25,
    hsem_rlr26: HSEM_RLR26,
    hsem_rlr27: HSEM_RLR27,
    hsem_rlr28: HSEM_RLR28,
    hsem_rlr29: HSEM_RLR29,
    hsem_rlr30: HSEM_RLR30,
    hsem_rlr31: HSEM_RLR31,
    hsem_c1ier: HSEM_C1IER,
    hsem_c1icr: HSEM_C1ICR,
    hsem_c1isr: HSEM_C1ISR,
    hsem_c1misr: HSEM_C1MISR,
    hsem_c2ier: HSEM_C2IER,
    hsem_c2icr: HSEM_C2ICR,
    hsem_c2isr: HSEM_C2ISR,
    hsem_c2misr: HSEM_C2MISR,
    _reserved72: [u8; 0x20],
    hsem_cr: HSEM_CR,
    hsem_keyr: HSEM_KEYR,
    _reserved74: [u8; 0x02a4],
    hsem_hwcfgr2: HSEM_HWCFGR2,
    hsem_hwcfgr1: HSEM_HWCFGR1,
    hsem_verr: HSEM_VERR,
    hsem_ipidr: HSEM_IPIDR,
    hsem_sidr: HSEM_SIDR,
}
impl RegisterBlock {
    ///0x00 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r0(&self) -> &HSEM_R0 {
        &self.hsem_r0
    }
    ///0x04 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r1(&self) -> &HSEM_R1 {
        &self.hsem_r1
    }
    ///0x08 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r2(&self) -> &HSEM_R2 {
        &self.hsem_r2
    }
    ///0x0c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r3(&self) -> &HSEM_R3 {
        &self.hsem_r3
    }
    ///0x10 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r4(&self) -> &HSEM_R4 {
        &self.hsem_r4
    }
    ///0x14 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r5(&self) -> &HSEM_R5 {
        &self.hsem_r5
    }
    ///0x18 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r6(&self) -> &HSEM_R6 {
        &self.hsem_r6
    }
    ///0x1c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r7(&self) -> &HSEM_R7 {
        &self.hsem_r7
    }
    ///0x20 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r8(&self) -> &HSEM_R8 {
        &self.hsem_r8
    }
    ///0x24 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r9(&self) -> &HSEM_R9 {
        &self.hsem_r9
    }
    ///0x28 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r10(&self) -> &HSEM_R10 {
        &self.hsem_r10
    }
    ///0x2c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r11(&self) -> &HSEM_R11 {
        &self.hsem_r11
    }
    ///0x30 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r12(&self) -> &HSEM_R12 {
        &self.hsem_r12
    }
    ///0x34 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r13(&self) -> &HSEM_R13 {
        &self.hsem_r13
    }
    ///0x38 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r14(&self) -> &HSEM_R14 {
        &self.hsem_r14
    }
    ///0x3c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r15(&self) -> &HSEM_R15 {
        &self.hsem_r15
    }
    ///0x40 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r16(&self) -> &HSEM_R16 {
        &self.hsem_r16
    }
    ///0x44 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r17(&self) -> &HSEM_R17 {
        &self.hsem_r17
    }
    ///0x48 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r18(&self) -> &HSEM_R18 {
        &self.hsem_r18
    }
    ///0x4c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r19(&self) -> &HSEM_R19 {
        &self.hsem_r19
    }
    ///0x50 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r20(&self) -> &HSEM_R20 {
        &self.hsem_r20
    }
    ///0x54 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r21(&self) -> &HSEM_R21 {
        &self.hsem_r21
    }
    ///0x58 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r22(&self) -> &HSEM_R22 {
        &self.hsem_r22
    }
    ///0x5c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r23(&self) -> &HSEM_R23 {
        &self.hsem_r23
    }
    ///0x60 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r24(&self) -> &HSEM_R24 {
        &self.hsem_r24
    }
    ///0x64 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r25(&self) -> &HSEM_R25 {
        &self.hsem_r25
    }
    ///0x68 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r26(&self) -> &HSEM_R26 {
        &self.hsem_r26
    }
    ///0x6c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r27(&self) -> &HSEM_R27 {
        &self.hsem_r27
    }
    ///0x70 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r28(&self) -> &HSEM_R28 {
        &self.hsem_r28
    }
    ///0x74 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r29(&self) -> &HSEM_R29 {
        &self.hsem_r29
    }
    ///0x78 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r30(&self) -> &HSEM_R30 {
        &self.hsem_r30
    }
    ///0x7c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_r31(&self) -> &HSEM_R31 {
        &self.hsem_r31
    }
    ///0x80 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr0(&self) -> &HSEM_RLR0 {
        &self.hsem_rlr0
    }
    ///0x84 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr1(&self) -> &HSEM_RLR1 {
        &self.hsem_rlr1
    }
    ///0x88 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr2(&self) -> &HSEM_RLR2 {
        &self.hsem_rlr2
    }
    ///0x8c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr3(&self) -> &HSEM_RLR3 {
        &self.hsem_rlr3
    }
    ///0x90 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr4(&self) -> &HSEM_RLR4 {
        &self.hsem_rlr4
    }
    ///0x94 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr5(&self) -> &HSEM_RLR5 {
        &self.hsem_rlr5
    }
    ///0x98 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr6(&self) -> &HSEM_RLR6 {
        &self.hsem_rlr6
    }
    ///0x9c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr7(&self) -> &HSEM_RLR7 {
        &self.hsem_rlr7
    }
    ///0xa0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr8(&self) -> &HSEM_RLR8 {
        &self.hsem_rlr8
    }
    ///0xa4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr9(&self) -> &HSEM_RLR9 {
        &self.hsem_rlr9
    }
    ///0xa8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr10(&self) -> &HSEM_RLR10 {
        &self.hsem_rlr10
    }
    ///0xac - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr11(&self) -> &HSEM_RLR11 {
        &self.hsem_rlr11
    }
    ///0xb0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr12(&self) -> &HSEM_RLR12 {
        &self.hsem_rlr12
    }
    ///0xb4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr13(&self) -> &HSEM_RLR13 {
        &self.hsem_rlr13
    }
    ///0xb8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr14(&self) -> &HSEM_RLR14 {
        &self.hsem_rlr14
    }
    ///0xbc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr15(&self) -> &HSEM_RLR15 {
        &self.hsem_rlr15
    }
    ///0xc0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr16(&self) -> &HSEM_RLR16 {
        &self.hsem_rlr16
    }
    ///0xc4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr17(&self) -> &HSEM_RLR17 {
        &self.hsem_rlr17
    }
    ///0xc8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr18(&self) -> &HSEM_RLR18 {
        &self.hsem_rlr18
    }
    ///0xcc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr19(&self) -> &HSEM_RLR19 {
        &self.hsem_rlr19
    }
    ///0xd0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr20(&self) -> &HSEM_RLR20 {
        &self.hsem_rlr20
    }
    ///0xd4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr21(&self) -> &HSEM_RLR21 {
        &self.hsem_rlr21
    }
    ///0xd8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr22(&self) -> &HSEM_RLR22 {
        &self.hsem_rlr22
    }
    ///0xdc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr23(&self) -> &HSEM_RLR23 {
        &self.hsem_rlr23
    }
    ///0xe0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr24(&self) -> &HSEM_RLR24 {
        &self.hsem_rlr24
    }
    ///0xe4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr25(&self) -> &HSEM_RLR25 {
        &self.hsem_rlr25
    }
    ///0xe8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr26(&self) -> &HSEM_RLR26 {
        &self.hsem_rlr26
    }
    ///0xec - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr27(&self) -> &HSEM_RLR27 {
        &self.hsem_rlr27
    }
    ///0xf0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr28(&self) -> &HSEM_RLR28 {
        &self.hsem_rlr28
    }
    ///0xf4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr29(&self) -> &HSEM_RLR29 {
        &self.hsem_rlr29
    }
    ///0xf8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr30(&self) -> &HSEM_RLR30 {
        &self.hsem_rlr30
    }
    ///0xfc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_rlr31(&self) -> &HSEM_RLR31 {
        &self.hsem_rlr31
    }
    ///0x100 - HSEM i1terrupt enable register
    #[inline(always)]
    pub const fn hsem_c1ier(&self) -> &HSEM_C1IER {
        &self.hsem_c1ier
    }
    ///0x104 - HSEM i1terrupt clear register
    #[inline(always)]
    pub const fn hsem_c1icr(&self) -> &HSEM_C1ICR {
        &self.hsem_c1icr
    }
    ///0x108 - HSEM i1terrupt status register
    #[inline(always)]
    pub const fn hsem_c1isr(&self) -> &HSEM_C1ISR {
        &self.hsem_c1isr
    }
    ///0x10c - HSEM i1terrupt status register
    #[inline(always)]
    pub const fn hsem_c1misr(&self) -> &HSEM_C1MISR {
        &self.hsem_c1misr
    }
    ///0x110 - HSEM i2terrupt enable register
    #[inline(always)]
    pub const fn hsem_c2ier(&self) -> &HSEM_C2IER {
        &self.hsem_c2ier
    }
    ///0x114 - HSEM i2terrupt clear register
    #[inline(always)]
    pub const fn hsem_c2icr(&self) -> &HSEM_C2ICR {
        &self.hsem_c2icr
    }
    ///0x118 - HSEM i2terrupt status register
    #[inline(always)]
    pub const fn hsem_c2isr(&self) -> &HSEM_C2ISR {
        &self.hsem_c2isr
    }
    ///0x11c - HSEM i2terrupt status register
    #[inline(always)]
    pub const fn hsem_c2misr(&self) -> &HSEM_C2MISR {
        &self.hsem_c2misr
    }
    ///0x140 - Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn hsem_cr(&self) -> &HSEM_CR {
        &self.hsem_cr
    }
    ///0x144 - HSEM interrupt clear register
    #[inline(always)]
    pub const fn hsem_keyr(&self) -> &HSEM_KEYR {
        &self.hsem_keyr
    }
    ///0x3ec - HSEM hardware configuration register 2
    #[inline(always)]
    pub const fn hsem_hwcfgr2(&self) -> &HSEM_HWCFGR2 {
        &self.hsem_hwcfgr2
    }
    ///0x3f0 - HSEM hardware configuration register 1
    #[inline(always)]
    pub const fn hsem_hwcfgr1(&self) -> &HSEM_HWCFGR1 {
        &self.hsem_hwcfgr1
    }
    ///0x3f4 - HSEM IP version register
    #[inline(always)]
    pub const fn hsem_verr(&self) -> &HSEM_VERR {
        &self.hsem_verr
    }
    ///0x3f8 - HSEM IP identification register
    #[inline(always)]
    pub const fn hsem_ipidr(&self) -> &HSEM_IPIDR {
        &self.hsem_ipidr
    }
    ///0x3fc - HSEM size identification register
    #[inline(always)]
    pub const fn hsem_sidr(&self) -> &HSEM_SIDR {
        &self.hsem_sidr
    }
}
/**HSEM_R0 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R0)

For information about available fields see [`mod@hsem_r0`]
module*/
pub type HSEM_R0 = crate::Reg<hsem_r0::HSEM_R0rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r0;
/**HSEM_R1 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R1)

For information about available fields see [`mod@hsem_r1`]
module*/
pub type HSEM_R1 = crate::Reg<hsem_r1::HSEM_R1rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r1;
/**HSEM_R2 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R2)

For information about available fields see [`mod@hsem_r2`]
module*/
pub type HSEM_R2 = crate::Reg<hsem_r2::HSEM_R2rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r2;
/**HSEM_R3 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R3)

For information about available fields see [`mod@hsem_r3`]
module*/
pub type HSEM_R3 = crate::Reg<hsem_r3::HSEM_R3rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r3;
/**HSEM_R4 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R4)

For information about available fields see [`mod@hsem_r4`]
module*/
pub type HSEM_R4 = crate::Reg<hsem_r4::HSEM_R4rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r4;
/**HSEM_R5 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R5)

For information about available fields see [`mod@hsem_r5`]
module*/
pub type HSEM_R5 = crate::Reg<hsem_r5::HSEM_R5rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r5;
/**HSEM_R6 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R6)

For information about available fields see [`mod@hsem_r6`]
module*/
pub type HSEM_R6 = crate::Reg<hsem_r6::HSEM_R6rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r6;
/**HSEM_R7 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R7)

For information about available fields see [`mod@hsem_r7`]
module*/
pub type HSEM_R7 = crate::Reg<hsem_r7::HSEM_R7rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r7;
/**HSEM_R8 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R8)

For information about available fields see [`mod@hsem_r8`]
module*/
pub type HSEM_R8 = crate::Reg<hsem_r8::HSEM_R8rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r8;
/**HSEM_R9 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R9)

For information about available fields see [`mod@hsem_r9`]
module*/
pub type HSEM_R9 = crate::Reg<hsem_r9::HSEM_R9rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r9;
/**HSEM_R10 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R10)

For information about available fields see [`mod@hsem_r10`]
module*/
pub type HSEM_R10 = crate::Reg<hsem_r10::HSEM_R10rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r10;
/**HSEM_R11 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R11)

For information about available fields see [`mod@hsem_r11`]
module*/
pub type HSEM_R11 = crate::Reg<hsem_r11::HSEM_R11rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r11;
/**HSEM_R12 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R12)

For information about available fields see [`mod@hsem_r12`]
module*/
pub type HSEM_R12 = crate::Reg<hsem_r12::HSEM_R12rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r12;
/**HSEM_R13 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R13)

For information about available fields see [`mod@hsem_r13`]
module*/
pub type HSEM_R13 = crate::Reg<hsem_r13::HSEM_R13rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r13;
/**HSEM_R14 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R14)

For information about available fields see [`mod@hsem_r14`]
module*/
pub type HSEM_R14 = crate::Reg<hsem_r14::HSEM_R14rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r14;
/**HSEM_R15 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R15)

For information about available fields see [`mod@hsem_r15`]
module*/
pub type HSEM_R15 = crate::Reg<hsem_r15::HSEM_R15rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r15;
/**HSEM_R16 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R16)

For information about available fields see [`mod@hsem_r16`]
module*/
pub type HSEM_R16 = crate::Reg<hsem_r16::HSEM_R16rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r16;
/**HSEM_R17 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R17)

For information about available fields see [`mod@hsem_r17`]
module*/
pub type HSEM_R17 = crate::Reg<hsem_r17::HSEM_R17rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r17;
/**HSEM_R18 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R18)

For information about available fields see [`mod@hsem_r18`]
module*/
pub type HSEM_R18 = crate::Reg<hsem_r18::HSEM_R18rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r18;
/**HSEM_R19 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R19)

For information about available fields see [`mod@hsem_r19`]
module*/
pub type HSEM_R19 = crate::Reg<hsem_r19::HSEM_R19rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r19;
/**HSEM_R20 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R20)

For information about available fields see [`mod@hsem_r20`]
module*/
pub type HSEM_R20 = crate::Reg<hsem_r20::HSEM_R20rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r20;
/**HSEM_R21 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R21)

For information about available fields see [`mod@hsem_r21`]
module*/
pub type HSEM_R21 = crate::Reg<hsem_r21::HSEM_R21rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r21;
/**HSEM_R22 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R22)

For information about available fields see [`mod@hsem_r22`]
module*/
pub type HSEM_R22 = crate::Reg<hsem_r22::HSEM_R22rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r22;
/**HSEM_R23 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R23)

For information about available fields see [`mod@hsem_r23`]
module*/
pub type HSEM_R23 = crate::Reg<hsem_r23::HSEM_R23rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r23;
/**HSEM_R24 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R24)

For information about available fields see [`mod@hsem_r24`]
module*/
pub type HSEM_R24 = crate::Reg<hsem_r24::HSEM_R24rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r24;
/**HSEM_R25 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R25)

For information about available fields see [`mod@hsem_r25`]
module*/
pub type HSEM_R25 = crate::Reg<hsem_r25::HSEM_R25rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r25;
/**HSEM_R26 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R26)

For information about available fields see [`mod@hsem_r26`]
module*/
pub type HSEM_R26 = crate::Reg<hsem_r26::HSEM_R26rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r26;
/**HSEM_R27 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R27)

For information about available fields see [`mod@hsem_r27`]
module*/
pub type HSEM_R27 = crate::Reg<hsem_r27::HSEM_R27rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r27;
/**HSEM_R28 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R28)

For information about available fields see [`mod@hsem_r28`]
module*/
pub type HSEM_R28 = crate::Reg<hsem_r28::HSEM_R28rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r28;
/**HSEM_R29 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R29)

For information about available fields see [`mod@hsem_r29`]
module*/
pub type HSEM_R29 = crate::Reg<hsem_r29::HSEM_R29rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r29;
/**HSEM_R30 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R30)

For information about available fields see [`mod@hsem_r30`]
module*/
pub type HSEM_R30 = crate::Reg<hsem_r30::HSEM_R30rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r30;
/**HSEM_R31 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_r31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_R31)

For information about available fields see [`mod@hsem_r31`]
module*/
pub type HSEM_R31 = crate::Reg<hsem_r31::HSEM_R31rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r31;
/**HSEM_RLR0 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR0)

For information about available fields see [`mod@hsem_rlr0`]
module*/
pub type HSEM_RLR0 = crate::Reg<hsem_rlr0::HSEM_RLR0rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr0;
/**HSEM_RLR1 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR1)

For information about available fields see [`mod@hsem_rlr1`]
module*/
pub type HSEM_RLR1 = crate::Reg<hsem_rlr1::HSEM_RLR1rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr1;
/**HSEM_RLR2 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR2)

For information about available fields see [`mod@hsem_rlr2`]
module*/
pub type HSEM_RLR2 = crate::Reg<hsem_rlr2::HSEM_RLR2rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr2;
/**HSEM_RLR3 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR3)

For information about available fields see [`mod@hsem_rlr3`]
module*/
pub type HSEM_RLR3 = crate::Reg<hsem_rlr3::HSEM_RLR3rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr3;
/**HSEM_RLR4 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR4)

For information about available fields see [`mod@hsem_rlr4`]
module*/
pub type HSEM_RLR4 = crate::Reg<hsem_rlr4::HSEM_RLR4rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr4;
/**HSEM_RLR5 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR5)

For information about available fields see [`mod@hsem_rlr5`]
module*/
pub type HSEM_RLR5 = crate::Reg<hsem_rlr5::HSEM_RLR5rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr5;
/**HSEM_RLR6 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR6)

For information about available fields see [`mod@hsem_rlr6`]
module*/
pub type HSEM_RLR6 = crate::Reg<hsem_rlr6::HSEM_RLR6rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr6;
/**HSEM_RLR7 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR7)

For information about available fields see [`mod@hsem_rlr7`]
module*/
pub type HSEM_RLR7 = crate::Reg<hsem_rlr7::HSEM_RLR7rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr7;
/**HSEM_RLR8 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR8)

For information about available fields see [`mod@hsem_rlr8`]
module*/
pub type HSEM_RLR8 = crate::Reg<hsem_rlr8::HSEM_RLR8rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr8;
/**HSEM_RLR9 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR9)

For information about available fields see [`mod@hsem_rlr9`]
module*/
pub type HSEM_RLR9 = crate::Reg<hsem_rlr9::HSEM_RLR9rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr9;
/**HSEM_RLR10 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR10)

For information about available fields see [`mod@hsem_rlr10`]
module*/
pub type HSEM_RLR10 = crate::Reg<hsem_rlr10::HSEM_RLR10rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr10;
/**HSEM_RLR11 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR11)

For information about available fields see [`mod@hsem_rlr11`]
module*/
pub type HSEM_RLR11 = crate::Reg<hsem_rlr11::HSEM_RLR11rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr11;
/**HSEM_RLR12 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR12)

For information about available fields see [`mod@hsem_rlr12`]
module*/
pub type HSEM_RLR12 = crate::Reg<hsem_rlr12::HSEM_RLR12rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr12;
/**HSEM_RLR13 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR13)

For information about available fields see [`mod@hsem_rlr13`]
module*/
pub type HSEM_RLR13 = crate::Reg<hsem_rlr13::HSEM_RLR13rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr13;
/**HSEM_RLR14 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR14)

For information about available fields see [`mod@hsem_rlr14`]
module*/
pub type HSEM_RLR14 = crate::Reg<hsem_rlr14::HSEM_RLR14rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr14;
/**HSEM_RLR15 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR15)

For information about available fields see [`mod@hsem_rlr15`]
module*/
pub type HSEM_RLR15 = crate::Reg<hsem_rlr15::HSEM_RLR15rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr15;
/**HSEM_RLR16 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR16)

For information about available fields see [`mod@hsem_rlr16`]
module*/
pub type HSEM_RLR16 = crate::Reg<hsem_rlr16::HSEM_RLR16rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr16;
/**HSEM_RLR17 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR17)

For information about available fields see [`mod@hsem_rlr17`]
module*/
pub type HSEM_RLR17 = crate::Reg<hsem_rlr17::HSEM_RLR17rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr17;
/**HSEM_RLR18 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR18)

For information about available fields see [`mod@hsem_rlr18`]
module*/
pub type HSEM_RLR18 = crate::Reg<hsem_rlr18::HSEM_RLR18rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr18;
/**HSEM_RLR19 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR19)

For information about available fields see [`mod@hsem_rlr19`]
module*/
pub type HSEM_RLR19 = crate::Reg<hsem_rlr19::HSEM_RLR19rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr19;
/**HSEM_RLR20 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR20)

For information about available fields see [`mod@hsem_rlr20`]
module*/
pub type HSEM_RLR20 = crate::Reg<hsem_rlr20::HSEM_RLR20rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr20;
/**HSEM_RLR21 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR21)

For information about available fields see [`mod@hsem_rlr21`]
module*/
pub type HSEM_RLR21 = crate::Reg<hsem_rlr21::HSEM_RLR21rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr21;
/**HSEM_RLR22 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR22)

For information about available fields see [`mod@hsem_rlr22`]
module*/
pub type HSEM_RLR22 = crate::Reg<hsem_rlr22::HSEM_RLR22rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr22;
/**HSEM_RLR23 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR23)

For information about available fields see [`mod@hsem_rlr23`]
module*/
pub type HSEM_RLR23 = crate::Reg<hsem_rlr23::HSEM_RLR23rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr23;
/**HSEM_RLR24 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR24)

For information about available fields see [`mod@hsem_rlr24`]
module*/
pub type HSEM_RLR24 = crate::Reg<hsem_rlr24::HSEM_RLR24rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr24;
/**HSEM_RLR25 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR25)

For information about available fields see [`mod@hsem_rlr25`]
module*/
pub type HSEM_RLR25 = crate::Reg<hsem_rlr25::HSEM_RLR25rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr25;
/**HSEM_RLR26 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR26)

For information about available fields see [`mod@hsem_rlr26`]
module*/
pub type HSEM_RLR26 = crate::Reg<hsem_rlr26::HSEM_RLR26rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr26;
/**HSEM_RLR27 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR27)

For information about available fields see [`mod@hsem_rlr27`]
module*/
pub type HSEM_RLR27 = crate::Reg<hsem_rlr27::HSEM_RLR27rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr27;
/**HSEM_RLR28 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR28)

For information about available fields see [`mod@hsem_rlr28`]
module*/
pub type HSEM_RLR28 = crate::Reg<hsem_rlr28::HSEM_RLR28rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr28;
/**HSEM_RLR29 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR29)

For information about available fields see [`mod@hsem_rlr29`]
module*/
pub type HSEM_RLR29 = crate::Reg<hsem_rlr29::HSEM_RLR29rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr29;
/**HSEM_RLR30 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR30)

For information about available fields see [`mod@hsem_rlr30`]
module*/
pub type HSEM_RLR30 = crate::Reg<hsem_rlr30::HSEM_RLR30rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr30;
/**HSEM_RLR31 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`hsem_rlr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_RLR31)

For information about available fields see [`mod@hsem_rlr31`]
module*/
pub type HSEM_RLR31 = crate::Reg<hsem_rlr31::HSEM_RLR31rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr31;
/**HSEM_C1IER (rw) register accessor: HSEM i1terrupt enable register

You can [`read`](crate::Reg::read) this register and get [`hsem_c1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C1IER)

For information about available fields see [`mod@hsem_c1ier`]
module*/
pub type HSEM_C1IER = crate::Reg<hsem_c1ier::HSEM_C1IERrs>;
///HSEM i1terrupt enable register
pub mod hsem_c1ier;
/**HSEM_C1ICR (rw) register accessor: HSEM i1terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`hsem_c1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C1ICR)

For information about available fields see [`mod@hsem_c1icr`]
module*/
pub type HSEM_C1ICR = crate::Reg<hsem_c1icr::HSEM_C1ICRrs>;
///HSEM i1terrupt clear register
pub mod hsem_c1icr;
/**HSEM_C1ISR (r) register accessor: HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C1ISR)

For information about available fields see [`mod@hsem_c1isr`]
module*/
pub type HSEM_C1ISR = crate::Reg<hsem_c1isr::HSEM_C1ISRrs>;
///HSEM i1terrupt status register
pub mod hsem_c1isr;
/**HSEM_C1MISR (r) register accessor: HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c1misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C1MISR)

For information about available fields see [`mod@hsem_c1misr`]
module*/
pub type HSEM_C1MISR = crate::Reg<hsem_c1misr::HSEM_C1MISRrs>;
///HSEM i1terrupt status register
pub mod hsem_c1misr;
/**HSEM_C2IER (rw) register accessor: HSEM i2terrupt enable register

You can [`read`](crate::Reg::read) this register and get [`hsem_c2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C2IER)

For information about available fields see [`mod@hsem_c2ier`]
module*/
pub type HSEM_C2IER = crate::Reg<hsem_c2ier::HSEM_C2IERrs>;
///HSEM i2terrupt enable register
pub mod hsem_c2ier;
/**HSEM_C2ICR (rw) register accessor: HSEM i2terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`hsem_c2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C2ICR)

For information about available fields see [`mod@hsem_c2icr`]
module*/
pub type HSEM_C2ICR = crate::Reg<hsem_c2icr::HSEM_C2ICRrs>;
///HSEM i2terrupt clear register
pub mod hsem_c2icr;
/**HSEM_C2ISR (r) register accessor: HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C2ISR)

For information about available fields see [`mod@hsem_c2isr`]
module*/
pub type HSEM_C2ISR = crate::Reg<hsem_c2isr::HSEM_C2ISRrs>;
///HSEM i2terrupt status register
pub mod hsem_c2isr;
/**HSEM_C2MISR (r) register accessor: HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`hsem_c2misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_C2MISR)

For information about available fields see [`mod@hsem_c2misr`]
module*/
pub type HSEM_C2MISR = crate::Reg<hsem_c2misr::HSEM_C2MISRrs>;
///HSEM i2terrupt status register
pub mod hsem_c2misr;
/**HSEM_CR (w) register accessor: Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_CR)

For information about available fields see [`mod@hsem_cr`]
module*/
pub type HSEM_CR = crate::Reg<hsem_cr::HSEM_CRrs>;
///Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_cr;
/**HSEM_KEYR (rw) register accessor: HSEM interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`hsem_keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_KEYR)

For information about available fields see [`mod@hsem_keyr`]
module*/
pub type HSEM_KEYR = crate::Reg<hsem_keyr::HSEM_KEYRrs>;
///HSEM interrupt clear register
pub mod hsem_keyr;
/**HSEM_HWCFGR2 (r) register accessor: HSEM hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hsem_hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_HWCFGR2)

For information about available fields see [`mod@hsem_hwcfgr2`]
module*/
pub type HSEM_HWCFGR2 = crate::Reg<hsem_hwcfgr2::HSEM_HWCFGR2rs>;
///HSEM hardware configuration register 2
pub mod hsem_hwcfgr2;
/**HSEM_HWCFGR1 (r) register accessor: HSEM hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hsem_hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_HWCFGR1)

For information about available fields see [`mod@hsem_hwcfgr1`]
module*/
pub type HSEM_HWCFGR1 = crate::Reg<hsem_hwcfgr1::HSEM_HWCFGR1rs>;
///HSEM hardware configuration register 1
pub mod hsem_hwcfgr1;
/**HSEM_VERR (r) register accessor: HSEM IP version register

You can [`read`](crate::Reg::read) this register and get [`hsem_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_VERR)

For information about available fields see [`mod@hsem_verr`]
module*/
pub type HSEM_VERR = crate::Reg<hsem_verr::HSEM_VERRrs>;
///HSEM IP version register
pub mod hsem_verr;
/**HSEM_IPIDR (r) register accessor: HSEM IP identification register

You can [`read`](crate::Reg::read) this register and get [`hsem_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_IPIDR)

For information about available fields see [`mod@hsem_ipidr`]
module*/
pub type HSEM_IPIDR = crate::Reg<hsem_ipidr::HSEM_IPIDRrs>;
///HSEM IP identification register
pub mod hsem_ipidr;
/**HSEM_SIDR (r) register accessor: HSEM size identification register

You can [`read`](crate::Reg::read) this register and get [`hsem_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:HSEM_SIDR)

For information about available fields see [`mod@hsem_sidr`]
module*/
pub type HSEM_SIDR = crate::Reg<hsem_sidr::HSEM_SIDRrs>;
///HSEM size identification register
pub mod hsem_sidr;
