#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    r0: R0,
    r1: R1,
    r2: R2,
    r3: R3,
    r4: R4,
    r5: R5,
    r6: R6,
    r7: R7,
    r8: R8,
    r9: R9,
    r10: R10,
    r11: R11,
    r12: R12,
    r13: R13,
    r14: R14,
    r15: R15,
    r16: R16,
    r17: R17,
    r18: R18,
    r19: R19,
    r20: R20,
    r21: R21,
    r22: R22,
    r23: R23,
    r24: R24,
    r25: R25,
    r26: R26,
    r27: R27,
    r28: R28,
    r29: R29,
    r30: R30,
    r31: R31,
    rlr0: RLR0,
    rlr1: RLR1,
    rlr2: RLR2,
    rlr3: RLR3,
    rlr4: RLR4,
    rlr5: RLR5,
    rlr6: RLR6,
    rlr7: RLR7,
    rlr8: RLR8,
    rlr9: RLR9,
    rlr10: RLR10,
    rlr11: RLR11,
    rlr12: RLR12,
    rlr13: RLR13,
    rlr14: RLR14,
    rlr15: RLR15,
    rlr16: RLR16,
    rlr17: RLR17,
    rlr18: RLR18,
    rlr19: RLR19,
    rlr20: RLR20,
    rlr21: RLR21,
    rlr22: RLR22,
    rlr23: RLR23,
    rlr24: RLR24,
    rlr25: RLR25,
    rlr26: RLR26,
    rlr27: RLR27,
    rlr28: RLR28,
    rlr29: RLR29,
    rlr30: RLR30,
    rlr31: RLR31,
    c1ier: C1IER,
    c1icr: C1ICR,
    c1isr: C1ISR,
    c1misr: C1MISR,
    c2ier: C2IER,
    c2icr: C2ICR,
    c2isr: C2ISR,
    c2misr: C2MISR,
    _reserved72: [u8; 0x20],
    cr: CR,
    keyr: KEYR,
    _reserved74: [u8; 0x02a4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r0(&self) -> &R0 {
        &self.r0
    }
    ///0x04 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r1(&self) -> &R1 {
        &self.r1
    }
    ///0x08 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r2(&self) -> &R2 {
        &self.r2
    }
    ///0x0c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r3(&self) -> &R3 {
        &self.r3
    }
    ///0x10 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r4(&self) -> &R4 {
        &self.r4
    }
    ///0x14 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r5(&self) -> &R5 {
        &self.r5
    }
    ///0x18 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r6(&self) -> &R6 {
        &self.r6
    }
    ///0x1c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r7(&self) -> &R7 {
        &self.r7
    }
    ///0x20 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r8(&self) -> &R8 {
        &self.r8
    }
    ///0x24 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r9(&self) -> &R9 {
        &self.r9
    }
    ///0x28 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r10(&self) -> &R10 {
        &self.r10
    }
    ///0x2c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r11(&self) -> &R11 {
        &self.r11
    }
    ///0x30 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r12(&self) -> &R12 {
        &self.r12
    }
    ///0x34 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r13(&self) -> &R13 {
        &self.r13
    }
    ///0x38 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r14(&self) -> &R14 {
        &self.r14
    }
    ///0x3c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r15(&self) -> &R15 {
        &self.r15
    }
    ///0x40 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r16(&self) -> &R16 {
        &self.r16
    }
    ///0x44 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r17(&self) -> &R17 {
        &self.r17
    }
    ///0x48 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r18(&self) -> &R18 {
        &self.r18
    }
    ///0x4c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r19(&self) -> &R19 {
        &self.r19
    }
    ///0x50 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r20(&self) -> &R20 {
        &self.r20
    }
    ///0x54 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r21(&self) -> &R21 {
        &self.r21
    }
    ///0x58 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r22(&self) -> &R22 {
        &self.r22
    }
    ///0x5c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r23(&self) -> &R23 {
        &self.r23
    }
    ///0x60 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r24(&self) -> &R24 {
        &self.r24
    }
    ///0x64 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r25(&self) -> &R25 {
        &self.r25
    }
    ///0x68 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r26(&self) -> &R26 {
        &self.r26
    }
    ///0x6c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r27(&self) -> &R27 {
        &self.r27
    }
    ///0x70 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r28(&self) -> &R28 {
        &self.r28
    }
    ///0x74 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r29(&self) -> &R29 {
        &self.r29
    }
    ///0x78 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r30(&self) -> &R30 {
        &self.r30
    }
    ///0x7c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn r31(&self) -> &R31 {
        &self.r31
    }
    ///0x80 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr0(&self) -> &RLR0 {
        &self.rlr0
    }
    ///0x84 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr1(&self) -> &RLR1 {
        &self.rlr1
    }
    ///0x88 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr2(&self) -> &RLR2 {
        &self.rlr2
    }
    ///0x8c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr3(&self) -> &RLR3 {
        &self.rlr3
    }
    ///0x90 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr4(&self) -> &RLR4 {
        &self.rlr4
    }
    ///0x94 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr5(&self) -> &RLR5 {
        &self.rlr5
    }
    ///0x98 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr6(&self) -> &RLR6 {
        &self.rlr6
    }
    ///0x9c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr7(&self) -> &RLR7 {
        &self.rlr7
    }
    ///0xa0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr8(&self) -> &RLR8 {
        &self.rlr8
    }
    ///0xa4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr9(&self) -> &RLR9 {
        &self.rlr9
    }
    ///0xa8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr10(&self) -> &RLR10 {
        &self.rlr10
    }
    ///0xac - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr11(&self) -> &RLR11 {
        &self.rlr11
    }
    ///0xb0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr12(&self) -> &RLR12 {
        &self.rlr12
    }
    ///0xb4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr13(&self) -> &RLR13 {
        &self.rlr13
    }
    ///0xb8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr14(&self) -> &RLR14 {
        &self.rlr14
    }
    ///0xbc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr15(&self) -> &RLR15 {
        &self.rlr15
    }
    ///0xc0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr16(&self) -> &RLR16 {
        &self.rlr16
    }
    ///0xc4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr17(&self) -> &RLR17 {
        &self.rlr17
    }
    ///0xc8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr18(&self) -> &RLR18 {
        &self.rlr18
    }
    ///0xcc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr19(&self) -> &RLR19 {
        &self.rlr19
    }
    ///0xd0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr20(&self) -> &RLR20 {
        &self.rlr20
    }
    ///0xd4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr21(&self) -> &RLR21 {
        &self.rlr21
    }
    ///0xd8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr22(&self) -> &RLR22 {
        &self.rlr22
    }
    ///0xdc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr23(&self) -> &RLR23 {
        &self.rlr23
    }
    ///0xe0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr24(&self) -> &RLR24 {
        &self.rlr24
    }
    ///0xe4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr25(&self) -> &RLR25 {
        &self.rlr25
    }
    ///0xe8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr26(&self) -> &RLR26 {
        &self.rlr26
    }
    ///0xec - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr27(&self) -> &RLR27 {
        &self.rlr27
    }
    ///0xf0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr28(&self) -> &RLR28 {
        &self.rlr28
    }
    ///0xf4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr29(&self) -> &RLR29 {
        &self.rlr29
    }
    ///0xf8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr30(&self) -> &RLR30 {
        &self.rlr30
    }
    ///0xfc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn rlr31(&self) -> &RLR31 {
        &self.rlr31
    }
    ///0x100 - HSEM i1terrupt enable register
    #[inline(always)]
    pub const fn c1ier(&self) -> &C1IER {
        &self.c1ier
    }
    ///0x104 - HSEM i1terrupt clear register
    #[inline(always)]
    pub const fn c1icr(&self) -> &C1ICR {
        &self.c1icr
    }
    ///0x108 - HSEM i1terrupt status register
    #[inline(always)]
    pub const fn c1isr(&self) -> &C1ISR {
        &self.c1isr
    }
    ///0x10c - HSEM i1terrupt status register
    #[inline(always)]
    pub const fn c1misr(&self) -> &C1MISR {
        &self.c1misr
    }
    ///0x110 - HSEM i2terrupt enable register
    #[inline(always)]
    pub const fn c2ier(&self) -> &C2IER {
        &self.c2ier
    }
    ///0x114 - HSEM i2terrupt clear register
    #[inline(always)]
    pub const fn c2icr(&self) -> &C2ICR {
        &self.c2icr
    }
    ///0x118 - HSEM i2terrupt status register
    #[inline(always)]
    pub const fn c2isr(&self) -> &C2ISR {
        &self.c2isr
    }
    ///0x11c - HSEM i2terrupt status register
    #[inline(always)]
    pub const fn c2misr(&self) -> &C2MISR {
        &self.c2misr
    }
    ///0x140 - Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x144 - HSEM interrupt clear register
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    ///0x3ec - HSEM hardware configuration register 2
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - HSEM hardware configuration register 1
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - HSEM IP version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - HSEM IP identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - HSEM size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**R0 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R0)

For information about available fields see [`mod@r0`] module*/
pub type R0 = crate::Reg<r0::R0rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r0;
/**R1 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R1)

For information about available fields see [`mod@r1`] module*/
pub type R1 = crate::Reg<r1::R1rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r1;
/**R2 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R2)

For information about available fields see [`mod@r2`] module*/
pub type R2 = crate::Reg<r2::R2rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r2;
/**R3 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R3)

For information about available fields see [`mod@r3`] module*/
pub type R3 = crate::Reg<r3::R3rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r3;
/**R4 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R4)

For information about available fields see [`mod@r4`] module*/
pub type R4 = crate::Reg<r4::R4rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r4;
/**R5 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R5)

For information about available fields see [`mod@r5`] module*/
pub type R5 = crate::Reg<r5::R5rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r5;
/**R6 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R6)

For information about available fields see [`mod@r6`] module*/
pub type R6 = crate::Reg<r6::R6rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r6;
/**R7 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R7)

For information about available fields see [`mod@r7`] module*/
pub type R7 = crate::Reg<r7::R7rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r7;
/**R8 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R8)

For information about available fields see [`mod@r8`] module*/
pub type R8 = crate::Reg<r8::R8rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r8;
/**R9 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R9)

For information about available fields see [`mod@r9`] module*/
pub type R9 = crate::Reg<r9::R9rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r9;
/**R10 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R10)

For information about available fields see [`mod@r10`] module*/
pub type R10 = crate::Reg<r10::R10rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r10;
/**R11 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R11)

For information about available fields see [`mod@r11`] module*/
pub type R11 = crate::Reg<r11::R11rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r11;
/**R12 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R12)

For information about available fields see [`mod@r12`] module*/
pub type R12 = crate::Reg<r12::R12rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r12;
/**R13 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R13)

For information about available fields see [`mod@r13`] module*/
pub type R13 = crate::Reg<r13::R13rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r13;
/**R14 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R14)

For information about available fields see [`mod@r14`] module*/
pub type R14 = crate::Reg<r14::R14rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r14;
/**R15 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R15)

For information about available fields see [`mod@r15`] module*/
pub type R15 = crate::Reg<r15::R15rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r15;
/**R16 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R16)

For information about available fields see [`mod@r16`] module*/
pub type R16 = crate::Reg<r16::R16rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r16;
/**R17 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R17)

For information about available fields see [`mod@r17`] module*/
pub type R17 = crate::Reg<r17::R17rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r17;
/**R18 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R18)

For information about available fields see [`mod@r18`] module*/
pub type R18 = crate::Reg<r18::R18rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r18;
/**R19 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R19)

For information about available fields see [`mod@r19`] module*/
pub type R19 = crate::Reg<r19::R19rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r19;
/**R20 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R20)

For information about available fields see [`mod@r20`] module*/
pub type R20 = crate::Reg<r20::R20rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r20;
/**R21 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R21)

For information about available fields see [`mod@r21`] module*/
pub type R21 = crate::Reg<r21::R21rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r21;
/**R22 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R22)

For information about available fields see [`mod@r22`] module*/
pub type R22 = crate::Reg<r22::R22rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r22;
/**R23 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R23)

For information about available fields see [`mod@r23`] module*/
pub type R23 = crate::Reg<r23::R23rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r23;
/**R24 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R24)

For information about available fields see [`mod@r24`] module*/
pub type R24 = crate::Reg<r24::R24rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r24;
/**R25 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R25)

For information about available fields see [`mod@r25`] module*/
pub type R25 = crate::Reg<r25::R25rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r25;
/**R26 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R26)

For information about available fields see [`mod@r26`] module*/
pub type R26 = crate::Reg<r26::R26rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r26;
/**R27 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R27)

For information about available fields see [`mod@r27`] module*/
pub type R27 = crate::Reg<r27::R27rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r27;
/**R28 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R28)

For information about available fields see [`mod@r28`] module*/
pub type R28 = crate::Reg<r28::R28rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r28;
/**R29 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R29)

For information about available fields see [`mod@r29`] module*/
pub type R29 = crate::Reg<r29::R29rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r29;
/**R30 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R30)

For information about available fields see [`mod@r30`] module*/
pub type R30 = crate::Reg<r30::R30rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r30;
/**R31 (rw) register accessor: The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`r31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:R31)

For information about available fields see [`mod@r31`] module*/
pub type R31 = crate::Reg<r31::R31rs>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod r31;
/**RLR0 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR0)

For information about available fields see [`mod@rlr0`] module*/
pub type RLR0 = crate::Reg<rlr0::RLR0rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr0;
/**RLR1 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR1)

For information about available fields see [`mod@rlr1`] module*/
pub type RLR1 = crate::Reg<rlr1::RLR1rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr1;
/**RLR2 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR2)

For information about available fields see [`mod@rlr2`] module*/
pub type RLR2 = crate::Reg<rlr2::RLR2rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr2;
/**RLR3 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR3)

For information about available fields see [`mod@rlr3`] module*/
pub type RLR3 = crate::Reg<rlr3::RLR3rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr3;
/**RLR4 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR4)

For information about available fields see [`mod@rlr4`] module*/
pub type RLR4 = crate::Reg<rlr4::RLR4rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr4;
/**RLR5 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR5)

For information about available fields see [`mod@rlr5`] module*/
pub type RLR5 = crate::Reg<rlr5::RLR5rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr5;
/**RLR6 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR6)

For information about available fields see [`mod@rlr6`] module*/
pub type RLR6 = crate::Reg<rlr6::RLR6rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr6;
/**RLR7 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR7)

For information about available fields see [`mod@rlr7`] module*/
pub type RLR7 = crate::Reg<rlr7::RLR7rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr7;
/**RLR8 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR8)

For information about available fields see [`mod@rlr8`] module*/
pub type RLR8 = crate::Reg<rlr8::RLR8rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr8;
/**RLR9 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR9)

For information about available fields see [`mod@rlr9`] module*/
pub type RLR9 = crate::Reg<rlr9::RLR9rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr9;
/**RLR10 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR10)

For information about available fields see [`mod@rlr10`] module*/
pub type RLR10 = crate::Reg<rlr10::RLR10rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr10;
/**RLR11 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR11)

For information about available fields see [`mod@rlr11`] module*/
pub type RLR11 = crate::Reg<rlr11::RLR11rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr11;
/**RLR12 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR12)

For information about available fields see [`mod@rlr12`] module*/
pub type RLR12 = crate::Reg<rlr12::RLR12rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr12;
/**RLR13 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR13)

For information about available fields see [`mod@rlr13`] module*/
pub type RLR13 = crate::Reg<rlr13::RLR13rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr13;
/**RLR14 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR14)

For information about available fields see [`mod@rlr14`] module*/
pub type RLR14 = crate::Reg<rlr14::RLR14rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr14;
/**RLR15 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR15)

For information about available fields see [`mod@rlr15`] module*/
pub type RLR15 = crate::Reg<rlr15::RLR15rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr15;
/**RLR16 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR16)

For information about available fields see [`mod@rlr16`] module*/
pub type RLR16 = crate::Reg<rlr16::RLR16rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr16;
/**RLR17 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR17)

For information about available fields see [`mod@rlr17`] module*/
pub type RLR17 = crate::Reg<rlr17::RLR17rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr17;
/**RLR18 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR18)

For information about available fields see [`mod@rlr18`] module*/
pub type RLR18 = crate::Reg<rlr18::RLR18rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr18;
/**RLR19 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR19)

For information about available fields see [`mod@rlr19`] module*/
pub type RLR19 = crate::Reg<rlr19::RLR19rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr19;
/**RLR20 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR20)

For information about available fields see [`mod@rlr20`] module*/
pub type RLR20 = crate::Reg<rlr20::RLR20rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr20;
/**RLR21 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR21)

For information about available fields see [`mod@rlr21`] module*/
pub type RLR21 = crate::Reg<rlr21::RLR21rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr21;
/**RLR22 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR22)

For information about available fields see [`mod@rlr22`] module*/
pub type RLR22 = crate::Reg<rlr22::RLR22rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr22;
/**RLR23 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR23)

For information about available fields see [`mod@rlr23`] module*/
pub type RLR23 = crate::Reg<rlr23::RLR23rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr23;
/**RLR24 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR24)

For information about available fields see [`mod@rlr24`] module*/
pub type RLR24 = crate::Reg<rlr24::RLR24rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr24;
/**RLR25 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR25)

For information about available fields see [`mod@rlr25`] module*/
pub type RLR25 = crate::Reg<rlr25::RLR25rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr25;
/**RLR26 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR26)

For information about available fields see [`mod@rlr26`] module*/
pub type RLR26 = crate::Reg<rlr26::RLR26rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr26;
/**RLR27 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR27)

For information about available fields see [`mod@rlr27`] module*/
pub type RLR27 = crate::Reg<rlr27::RLR27rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr27;
/**RLR28 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR28)

For information about available fields see [`mod@rlr28`] module*/
pub type RLR28 = crate::Reg<rlr28::RLR28rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr28;
/**RLR29 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR29)

For information about available fields see [`mod@rlr29`] module*/
pub type RLR29 = crate::Reg<rlr29::RLR29rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr29;
/**RLR30 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR30)

For information about available fields see [`mod@rlr30`] module*/
pub type RLR30 = crate::Reg<rlr30::RLR30rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr30;
/**RLR31 (r) register accessor: Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.

You can [`read`](crate::Reg::read) this register and get [`rlr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:RLR31)

For information about available fields see [`mod@rlr31`] module*/
pub type RLR31 = crate::Reg<rlr31::RLR31rs>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod rlr31;
/**C1IER (rw) register accessor: HSEM i1terrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1IER)

For information about available fields see [`mod@c1ier`] module*/
pub type C1IER = crate::Reg<c1ier::C1IERrs>;
///HSEM i1terrupt enable register
pub mod c1ier;
/**C1ICR (rw) register accessor: HSEM i1terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1ICR)

For information about available fields see [`mod@c1icr`] module*/
pub type C1ICR = crate::Reg<c1icr::C1ICRrs>;
///HSEM i1terrupt clear register
pub mod c1icr;
/**C1ISR (r) register accessor: HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1ISR)

For information about available fields see [`mod@c1isr`] module*/
pub type C1ISR = crate::Reg<c1isr::C1ISRrs>;
///HSEM i1terrupt status register
pub mod c1isr;
/**C1MISR (r) register accessor: HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1MISR)

For information about available fields see [`mod@c1misr`] module*/
pub type C1MISR = crate::Reg<c1misr::C1MISRrs>;
///HSEM i1terrupt status register
pub mod c1misr;
/**C2IER (rw) register accessor: HSEM i2terrupt enable register

You can [`read`](crate::Reg::read) this register and get [`c2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C2IER)

For information about available fields see [`mod@c2ier`] module*/
pub type C2IER = crate::Reg<c2ier::C2IERrs>;
///HSEM i2terrupt enable register
pub mod c2ier;
/**C2ICR (rw) register accessor: HSEM i2terrupt clear register

You can [`read`](crate::Reg::read) this register and get [`c2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C2ICR)

For information about available fields see [`mod@c2icr`] module*/
pub type C2ICR = crate::Reg<c2icr::C2ICRrs>;
///HSEM i2terrupt clear register
pub mod c2icr;
/**C2ISR (r) register accessor: HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C2ISR)

For information about available fields see [`mod@c2isr`] module*/
pub type C2ISR = crate::Reg<c2isr::C2ISRrs>;
///HSEM i2terrupt status register
pub mod c2isr;
/**C2MISR (r) register accessor: HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C2MISR)

For information about available fields see [`mod@c2misr`] module*/
pub type C2MISR = crate::Reg<c2misr::C2MISRrs>;
///HSEM i2terrupt status register
pub mod c2misr;
/**CR (w) register accessor: Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod cr;
/**KEYR (rw) register accessor: HSEM interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:KEYR)

For information about available fields see [`mod@keyr`] module*/
pub type KEYR = crate::Reg<keyr::KEYRrs>;
///HSEM interrupt clear register
pub mod keyr;
/**HWCFGR2 (r) register accessor: HSEM hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///HSEM hardware configuration register 2
pub mod hwcfgr2;
/**HWCFGR1 (r) register accessor: HSEM hardware configuration register 1

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///HSEM hardware configuration register 1
pub mod hwcfgr1;
/**VERR (r) register accessor: HSEM IP version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///HSEM IP version register
pub mod verr;
/**IPIDR (r) register accessor: HSEM IP identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///HSEM IP identification register
pub mod ipidr;
/**SIDR (r) register accessor: HSEM size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///HSEM size identification register
pub mod sidr;
