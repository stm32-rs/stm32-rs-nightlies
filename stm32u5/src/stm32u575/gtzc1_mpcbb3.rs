#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mpcbb3_cr: MPCBB3_CR,
    _reserved1: [u8; 0x0c],
    mpcbb3_cfglockr1: MPCBB3_CFGLOCKR1,
    _reserved2: [u8; 0xec],
    mpcbb3_seccfgr0: MPCBB3_SECCFGR0,
    mpcbb3_seccfgr1: MPCBB3_SECCFGR1,
    mpcbb3_seccfgr2: MPCBB3_SECCFGR2,
    mpcbb3_seccfgr3: MPCBB3_SECCFGR3,
    mpcbb3_seccfgr4: MPCBB3_SECCFGR4,
    mpcbb3_seccfgr5: MPCBB3_SECCFGR5,
    mpcbb3_seccfgr6: MPCBB3_SECCFGR6,
    mpcbb3_seccfgr7: MPCBB3_SECCFGR7,
    mpcbb3_seccfgr8: MPCBB3_SECCFGR8,
    mpcbb3_seccfgr9: MPCBB3_SECCFGR9,
    mpcbb3_seccfgr10: MPCBB3_SECCFGR10,
    mpcbb3_seccfgr11: MPCBB3_SECCFGR11,
    mpcbb3_seccfgr12: MPCBB3_SECCFGR12,
    mpcbb3_seccfgr13: MPCBB3_SECCFGR13,
    mpcbb3_seccfgr14: MPCBB3_SECCFGR14,
    mpcbb3_seccfgr15: MPCBB3_SECCFGR15,
    mpcbb3_seccfgr16: MPCBB3_SECCFGR16,
    mpcbb3_seccfgr17: MPCBB3_SECCFGR17,
    mpcbb3_seccfgr18: MPCBB3_SECCFGR18,
    mpcbb3_seccfgr19: MPCBB3_SECCFGR19,
    mpcbb3_seccfgr20: MPCBB3_SECCFGR20,
    mpcbb3_seccfgr21: MPCBB3_SECCFGR21,
    mpcbb3_seccfgr22: MPCBB3_SECCFGR22,
    mpcbb3_seccfgr23: MPCBB3_SECCFGR23,
    mpcbb3_seccfgr24: MPCBB3_SECCFGR24,
    mpcbb3_seccfgr25: MPCBB3_SECCFGR25,
    mpcbb3_seccfgr26: MPCBB3_SECCFGR26,
    mpcbb3_seccfgr27: MPCBB3_SECCFGR27,
    mpcbb3_seccfgr28: MPCBB3_SECCFGR28,
    mpcbb3_seccfgr29: MPCBB3_SECCFGR29,
    mpcbb3_seccfgr30: MPCBB3_SECCFGR30,
    mpcbb3_seccfgr31: MPCBB3_SECCFGR31,
    _reserved34: [u8; 0x80],
    mpcbb3_privcfgr0: MPCBB3_PRIVCFGR0,
    mpcbb3_privcfgr1: MPCBB3_PRIVCFGR1,
    mpcbb3_privcfgr2: MPCBB3_PRIVCFGR2,
    mpcbb3_privcfgr3: MPCBB3_PRIVCFGR3,
    mpcbb3_privcfgr4: MPCBB3_PRIVCFGR4,
    mpcbb3_privcfgr5: MPCBB3_PRIVCFGR5,
    mpcbb3_privcfgr6: MPCBB3_PRIVCFGR6,
    mpcbb3_privcfgr7: MPCBB3_PRIVCFGR7,
    mpcbb3_privcfgr8: MPCBB3_PRIVCFGR8,
    mpcbb3_privcfgr9: MPCBB3_PRIVCFGR9,
    mpcbb3_privcfgr10: MPCBB3_PRIVCFGR10,
    mpcbb3_privcfgr11: MPCBB3_PRIVCFGR11,
    mpcbb3_privcfgr12: MPCBB3_PRIVCFGR12,
    mpcbb3_privcfgr13: MPCBB3_PRIVCFGR13,
    mpcbb3_privcfgr14: MPCBB3_PRIVCFGR14,
    mpcbb3_privcfgr15: MPCBB3_PRIVCFGR15,
    mpcbb3_privcfgr16: MPCBB3_PRIVCFGR16,
    mpcbb3_privcfgr17: MPCBB3_PRIVCFGR17,
    mpcbb3_privcfgr18: MPCBB3_PRIVCFGR18,
    mpcbb3_privcfgr19: MPCBB3_PRIVCFGR19,
    mpcbb3_privcfgr20: MPCBB3_PRIVCFGR20,
    mpcbb3_privcfgr21: MPCBB3_PRIVCFGR21,
    mpcbb3_privcfgr22: MPCBB3_PRIVCFGR22,
    mpcbb3_privcfgr23: MPCBB3_PRIVCFGR23,
    mpcbb3_privcfgr24: MPCBB3_PRIVCFGR24,
    mpcbb3_privcfgr25: MPCBB3_PRIVCFGR25,
    mpcbb3_privcfgr26: MPCBB3_PRIVCFGR26,
    mpcbb3_privcfgr27: MPCBB3_PRIVCFGR27,
    mpcbb3_privcfgr28: MPCBB3_PRIVCFGR28,
    mpcbb3_privcfgr29: MPCBB3_PRIVCFGR29,
    mpcbb3_privcfgr30: MPCBB3_PRIVCFGR30,
    mpcbb3_privcfgr31: MPCBB3_PRIVCFGR31,
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn mpcbb3_cr(&self) -> &MPCBB3_CR {
        &self.mpcbb3_cr
    }
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    #[inline(always)]
    pub const fn mpcbb3_cfglockr1(&self) -> &MPCBB3_CFGLOCKR1 {
        &self.mpcbb3_cfglockr1
    }
    ///0x100 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr0(&self) -> &MPCBB3_SECCFGR0 {
        &self.mpcbb3_seccfgr0
    }
    ///0x104 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr1(&self) -> &MPCBB3_SECCFGR1 {
        &self.mpcbb3_seccfgr1
    }
    ///0x108 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr2(&self) -> &MPCBB3_SECCFGR2 {
        &self.mpcbb3_seccfgr2
    }
    ///0x10c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr3(&self) -> &MPCBB3_SECCFGR3 {
        &self.mpcbb3_seccfgr3
    }
    ///0x110 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr4(&self) -> &MPCBB3_SECCFGR4 {
        &self.mpcbb3_seccfgr4
    }
    ///0x114 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr5(&self) -> &MPCBB3_SECCFGR5 {
        &self.mpcbb3_seccfgr5
    }
    ///0x118 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr6(&self) -> &MPCBB3_SECCFGR6 {
        &self.mpcbb3_seccfgr6
    }
    ///0x11c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr7(&self) -> &MPCBB3_SECCFGR7 {
        &self.mpcbb3_seccfgr7
    }
    ///0x120 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr8(&self) -> &MPCBB3_SECCFGR8 {
        &self.mpcbb3_seccfgr8
    }
    ///0x124 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr9(&self) -> &MPCBB3_SECCFGR9 {
        &self.mpcbb3_seccfgr9
    }
    ///0x128 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr10(&self) -> &MPCBB3_SECCFGR10 {
        &self.mpcbb3_seccfgr10
    }
    ///0x12c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr11(&self) -> &MPCBB3_SECCFGR11 {
        &self.mpcbb3_seccfgr11
    }
    ///0x130 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr12(&self) -> &MPCBB3_SECCFGR12 {
        &self.mpcbb3_seccfgr12
    }
    ///0x134 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr13(&self) -> &MPCBB3_SECCFGR13 {
        &self.mpcbb3_seccfgr13
    }
    ///0x138 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr14(&self) -> &MPCBB3_SECCFGR14 {
        &self.mpcbb3_seccfgr14
    }
    ///0x13c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr15(&self) -> &MPCBB3_SECCFGR15 {
        &self.mpcbb3_seccfgr15
    }
    ///0x140 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr16(&self) -> &MPCBB3_SECCFGR16 {
        &self.mpcbb3_seccfgr16
    }
    ///0x144 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr17(&self) -> &MPCBB3_SECCFGR17 {
        &self.mpcbb3_seccfgr17
    }
    ///0x148 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr18(&self) -> &MPCBB3_SECCFGR18 {
        &self.mpcbb3_seccfgr18
    }
    ///0x14c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr19(&self) -> &MPCBB3_SECCFGR19 {
        &self.mpcbb3_seccfgr19
    }
    ///0x150 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr20(&self) -> &MPCBB3_SECCFGR20 {
        &self.mpcbb3_seccfgr20
    }
    ///0x154 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr21(&self) -> &MPCBB3_SECCFGR21 {
        &self.mpcbb3_seccfgr21
    }
    ///0x158 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr22(&self) -> &MPCBB3_SECCFGR22 {
        &self.mpcbb3_seccfgr22
    }
    ///0x15c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr23(&self) -> &MPCBB3_SECCFGR23 {
        &self.mpcbb3_seccfgr23
    }
    ///0x160 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr24(&self) -> &MPCBB3_SECCFGR24 {
        &self.mpcbb3_seccfgr24
    }
    ///0x164 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr25(&self) -> &MPCBB3_SECCFGR25 {
        &self.mpcbb3_seccfgr25
    }
    ///0x168 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr26(&self) -> &MPCBB3_SECCFGR26 {
        &self.mpcbb3_seccfgr26
    }
    ///0x16c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr27(&self) -> &MPCBB3_SECCFGR27 {
        &self.mpcbb3_seccfgr27
    }
    ///0x170 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr28(&self) -> &MPCBB3_SECCFGR28 {
        &self.mpcbb3_seccfgr28
    }
    ///0x174 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr29(&self) -> &MPCBB3_SECCFGR29 {
        &self.mpcbb3_seccfgr29
    }
    ///0x178 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr30(&self) -> &MPCBB3_SECCFGR30 {
        &self.mpcbb3_seccfgr30
    }
    ///0x17c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_seccfgr31(&self) -> &MPCBB3_SECCFGR31 {
        &self.mpcbb3_seccfgr31
    }
    ///0x200 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr0(&self) -> &MPCBB3_PRIVCFGR0 {
        &self.mpcbb3_privcfgr0
    }
    ///0x204 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr1(&self) -> &MPCBB3_PRIVCFGR1 {
        &self.mpcbb3_privcfgr1
    }
    ///0x208 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr2(&self) -> &MPCBB3_PRIVCFGR2 {
        &self.mpcbb3_privcfgr2
    }
    ///0x20c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr3(&self) -> &MPCBB3_PRIVCFGR3 {
        &self.mpcbb3_privcfgr3
    }
    ///0x210 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr4(&self) -> &MPCBB3_PRIVCFGR4 {
        &self.mpcbb3_privcfgr4
    }
    ///0x214 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr5(&self) -> &MPCBB3_PRIVCFGR5 {
        &self.mpcbb3_privcfgr5
    }
    ///0x218 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr6(&self) -> &MPCBB3_PRIVCFGR6 {
        &self.mpcbb3_privcfgr6
    }
    ///0x21c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr7(&self) -> &MPCBB3_PRIVCFGR7 {
        &self.mpcbb3_privcfgr7
    }
    ///0x220 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr8(&self) -> &MPCBB3_PRIVCFGR8 {
        &self.mpcbb3_privcfgr8
    }
    ///0x224 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr9(&self) -> &MPCBB3_PRIVCFGR9 {
        &self.mpcbb3_privcfgr9
    }
    ///0x228 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr10(&self) -> &MPCBB3_PRIVCFGR10 {
        &self.mpcbb3_privcfgr10
    }
    ///0x22c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr11(&self) -> &MPCBB3_PRIVCFGR11 {
        &self.mpcbb3_privcfgr11
    }
    ///0x230 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr12(&self) -> &MPCBB3_PRIVCFGR12 {
        &self.mpcbb3_privcfgr12
    }
    ///0x234 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr13(&self) -> &MPCBB3_PRIVCFGR13 {
        &self.mpcbb3_privcfgr13
    }
    ///0x238 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr14(&self) -> &MPCBB3_PRIVCFGR14 {
        &self.mpcbb3_privcfgr14
    }
    ///0x23c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr15(&self) -> &MPCBB3_PRIVCFGR15 {
        &self.mpcbb3_privcfgr15
    }
    ///0x240 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr16(&self) -> &MPCBB3_PRIVCFGR16 {
        &self.mpcbb3_privcfgr16
    }
    ///0x244 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr17(&self) -> &MPCBB3_PRIVCFGR17 {
        &self.mpcbb3_privcfgr17
    }
    ///0x248 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr18(&self) -> &MPCBB3_PRIVCFGR18 {
        &self.mpcbb3_privcfgr18
    }
    ///0x24c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr19(&self) -> &MPCBB3_PRIVCFGR19 {
        &self.mpcbb3_privcfgr19
    }
    ///0x250 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr20(&self) -> &MPCBB3_PRIVCFGR20 {
        &self.mpcbb3_privcfgr20
    }
    ///0x254 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr21(&self) -> &MPCBB3_PRIVCFGR21 {
        &self.mpcbb3_privcfgr21
    }
    ///0x258 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr22(&self) -> &MPCBB3_PRIVCFGR22 {
        &self.mpcbb3_privcfgr22
    }
    ///0x25c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr23(&self) -> &MPCBB3_PRIVCFGR23 {
        &self.mpcbb3_privcfgr23
    }
    ///0x260 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr24(&self) -> &MPCBB3_PRIVCFGR24 {
        &self.mpcbb3_privcfgr24
    }
    ///0x264 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr25(&self) -> &MPCBB3_PRIVCFGR25 {
        &self.mpcbb3_privcfgr25
    }
    ///0x268 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr26(&self) -> &MPCBB3_PRIVCFGR26 {
        &self.mpcbb3_privcfgr26
    }
    ///0x26c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr27(&self) -> &MPCBB3_PRIVCFGR27 {
        &self.mpcbb3_privcfgr27
    }
    ///0x270 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr28(&self) -> &MPCBB3_PRIVCFGR28 {
        &self.mpcbb3_privcfgr28
    }
    ///0x274 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr29(&self) -> &MPCBB3_PRIVCFGR29 {
        &self.mpcbb3_privcfgr29
    }
    ///0x278 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr30(&self) -> &MPCBB3_PRIVCFGR30 {
        &self.mpcbb3_privcfgr30
    }
    ///0x27c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb3_privcfgr31(&self) -> &MPCBB3_PRIVCFGR31 {
        &self.mpcbb3_privcfgr31
    }
}
/**MPCBB3_CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_CR)

For information about available fields see [`mod@mpcbb3_cr`]
module*/
pub type MPCBB3_CR = crate::Reg<mpcbb3_cr::MPCBB3_CRrs>;
///MPCBB control register
pub mod mpcbb3_cr;
/**MPCBB3_CFGLOCKR1 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_cfglockr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_cfglockr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_CFGLOCKR1)

For information about available fields see [`mod@mpcbb3_cfglockr1`]
module*/
pub type MPCBB3_CFGLOCKR1 = crate::Reg<mpcbb3_cfglockr1::MPCBB3_CFGLOCKR1rs>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod mpcbb3_cfglockr1;
/**MPCBB3_SECCFGR0 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR0)

For information about available fields see [`mod@mpcbb3_seccfgr0`]
module*/
pub type MPCBB3_SECCFGR0 = crate::Reg<mpcbb3_seccfgr0::MPCBB3_SECCFGR0rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr0;
/**MPCBB3_SECCFGR1 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR1)

For information about available fields see [`mod@mpcbb3_seccfgr1`]
module*/
pub type MPCBB3_SECCFGR1 = crate::Reg<mpcbb3_seccfgr1::MPCBB3_SECCFGR1rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr1;
/**MPCBB3_SECCFGR2 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR2)

For information about available fields see [`mod@mpcbb3_seccfgr2`]
module*/
pub type MPCBB3_SECCFGR2 = crate::Reg<mpcbb3_seccfgr2::MPCBB3_SECCFGR2rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr2;
/**MPCBB3_SECCFGR3 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR3)

For information about available fields see [`mod@mpcbb3_seccfgr3`]
module*/
pub type MPCBB3_SECCFGR3 = crate::Reg<mpcbb3_seccfgr3::MPCBB3_SECCFGR3rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr3;
/**MPCBB3_SECCFGR4 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR4)

For information about available fields see [`mod@mpcbb3_seccfgr4`]
module*/
pub type MPCBB3_SECCFGR4 = crate::Reg<mpcbb3_seccfgr4::MPCBB3_SECCFGR4rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr4;
/**MPCBB3_SECCFGR5 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR5)

For information about available fields see [`mod@mpcbb3_seccfgr5`]
module*/
pub type MPCBB3_SECCFGR5 = crate::Reg<mpcbb3_seccfgr5::MPCBB3_SECCFGR5rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr5;
/**MPCBB3_SECCFGR6 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR6)

For information about available fields see [`mod@mpcbb3_seccfgr6`]
module*/
pub type MPCBB3_SECCFGR6 = crate::Reg<mpcbb3_seccfgr6::MPCBB3_SECCFGR6rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr6;
/**MPCBB3_SECCFGR7 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR7)

For information about available fields see [`mod@mpcbb3_seccfgr7`]
module*/
pub type MPCBB3_SECCFGR7 = crate::Reg<mpcbb3_seccfgr7::MPCBB3_SECCFGR7rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr7;
/**MPCBB3_SECCFGR8 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR8)

For information about available fields see [`mod@mpcbb3_seccfgr8`]
module*/
pub type MPCBB3_SECCFGR8 = crate::Reg<mpcbb3_seccfgr8::MPCBB3_SECCFGR8rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr8;
/**MPCBB3_SECCFGR9 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR9)

For information about available fields see [`mod@mpcbb3_seccfgr9`]
module*/
pub type MPCBB3_SECCFGR9 = crate::Reg<mpcbb3_seccfgr9::MPCBB3_SECCFGR9rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr9;
/**MPCBB3_SECCFGR10 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR10)

For information about available fields see [`mod@mpcbb3_seccfgr10`]
module*/
pub type MPCBB3_SECCFGR10 = crate::Reg<mpcbb3_seccfgr10::MPCBB3_SECCFGR10rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr10;
/**MPCBB3_SECCFGR11 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR11)

For information about available fields see [`mod@mpcbb3_seccfgr11`]
module*/
pub type MPCBB3_SECCFGR11 = crate::Reg<mpcbb3_seccfgr11::MPCBB3_SECCFGR11rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr11;
/**MPCBB3_SECCFGR12 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR12)

For information about available fields see [`mod@mpcbb3_seccfgr12`]
module*/
pub type MPCBB3_SECCFGR12 = crate::Reg<mpcbb3_seccfgr12::MPCBB3_SECCFGR12rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr12;
/**MPCBB3_SECCFGR13 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR13)

For information about available fields see [`mod@mpcbb3_seccfgr13`]
module*/
pub type MPCBB3_SECCFGR13 = crate::Reg<mpcbb3_seccfgr13::MPCBB3_SECCFGR13rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr13;
/**MPCBB3_SECCFGR14 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR14)

For information about available fields see [`mod@mpcbb3_seccfgr14`]
module*/
pub type MPCBB3_SECCFGR14 = crate::Reg<mpcbb3_seccfgr14::MPCBB3_SECCFGR14rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr14;
/**MPCBB3_SECCFGR15 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR15)

For information about available fields see [`mod@mpcbb3_seccfgr15`]
module*/
pub type MPCBB3_SECCFGR15 = crate::Reg<mpcbb3_seccfgr15::MPCBB3_SECCFGR15rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr15;
/**MPCBB3_SECCFGR16 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR16)

For information about available fields see [`mod@mpcbb3_seccfgr16`]
module*/
pub type MPCBB3_SECCFGR16 = crate::Reg<mpcbb3_seccfgr16::MPCBB3_SECCFGR16rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr16;
/**MPCBB3_SECCFGR17 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR17)

For information about available fields see [`mod@mpcbb3_seccfgr17`]
module*/
pub type MPCBB3_SECCFGR17 = crate::Reg<mpcbb3_seccfgr17::MPCBB3_SECCFGR17rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr17;
/**MPCBB3_SECCFGR18 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR18)

For information about available fields see [`mod@mpcbb3_seccfgr18`]
module*/
pub type MPCBB3_SECCFGR18 = crate::Reg<mpcbb3_seccfgr18::MPCBB3_SECCFGR18rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr18;
/**MPCBB3_SECCFGR19 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR19)

For information about available fields see [`mod@mpcbb3_seccfgr19`]
module*/
pub type MPCBB3_SECCFGR19 = crate::Reg<mpcbb3_seccfgr19::MPCBB3_SECCFGR19rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr19;
/**MPCBB3_SECCFGR20 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR20)

For information about available fields see [`mod@mpcbb3_seccfgr20`]
module*/
pub type MPCBB3_SECCFGR20 = crate::Reg<mpcbb3_seccfgr20::MPCBB3_SECCFGR20rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr20;
/**MPCBB3_SECCFGR21 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR21)

For information about available fields see [`mod@mpcbb3_seccfgr21`]
module*/
pub type MPCBB3_SECCFGR21 = crate::Reg<mpcbb3_seccfgr21::MPCBB3_SECCFGR21rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr21;
/**MPCBB3_SECCFGR22 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR22)

For information about available fields see [`mod@mpcbb3_seccfgr22`]
module*/
pub type MPCBB3_SECCFGR22 = crate::Reg<mpcbb3_seccfgr22::MPCBB3_SECCFGR22rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr22;
/**MPCBB3_SECCFGR23 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR23)

For information about available fields see [`mod@mpcbb3_seccfgr23`]
module*/
pub type MPCBB3_SECCFGR23 = crate::Reg<mpcbb3_seccfgr23::MPCBB3_SECCFGR23rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr23;
/**MPCBB3_SECCFGR24 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR24)

For information about available fields see [`mod@mpcbb3_seccfgr24`]
module*/
pub type MPCBB3_SECCFGR24 = crate::Reg<mpcbb3_seccfgr24::MPCBB3_SECCFGR24rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr24;
/**MPCBB3_SECCFGR25 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR25)

For information about available fields see [`mod@mpcbb3_seccfgr25`]
module*/
pub type MPCBB3_SECCFGR25 = crate::Reg<mpcbb3_seccfgr25::MPCBB3_SECCFGR25rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr25;
/**MPCBB3_SECCFGR26 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR26)

For information about available fields see [`mod@mpcbb3_seccfgr26`]
module*/
pub type MPCBB3_SECCFGR26 = crate::Reg<mpcbb3_seccfgr26::MPCBB3_SECCFGR26rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr26;
/**MPCBB3_SECCFGR27 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR27)

For information about available fields see [`mod@mpcbb3_seccfgr27`]
module*/
pub type MPCBB3_SECCFGR27 = crate::Reg<mpcbb3_seccfgr27::MPCBB3_SECCFGR27rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr27;
/**MPCBB3_SECCFGR28 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR28)

For information about available fields see [`mod@mpcbb3_seccfgr28`]
module*/
pub type MPCBB3_SECCFGR28 = crate::Reg<mpcbb3_seccfgr28::MPCBB3_SECCFGR28rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr28;
/**MPCBB3_SECCFGR29 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR29)

For information about available fields see [`mod@mpcbb3_seccfgr29`]
module*/
pub type MPCBB3_SECCFGR29 = crate::Reg<mpcbb3_seccfgr29::MPCBB3_SECCFGR29rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr29;
/**MPCBB3_SECCFGR30 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR30)

For information about available fields see [`mod@mpcbb3_seccfgr30`]
module*/
pub type MPCBB3_SECCFGR30 = crate::Reg<mpcbb3_seccfgr30::MPCBB3_SECCFGR30rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr30;
/**MPCBB3_SECCFGR31 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_seccfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_seccfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_SECCFGR31)

For information about available fields see [`mod@mpcbb3_seccfgr31`]
module*/
pub type MPCBB3_SECCFGR31 = crate::Reg<mpcbb3_seccfgr31::MPCBB3_SECCFGR31rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr31;
/**MPCBB3_PRIVCFGR0 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR0)

For information about available fields see [`mod@mpcbb3_privcfgr0`]
module*/
pub type MPCBB3_PRIVCFGR0 = crate::Reg<mpcbb3_privcfgr0::MPCBB3_PRIVCFGR0rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr0;
/**MPCBB3_PRIVCFGR1 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR1)

For information about available fields see [`mod@mpcbb3_privcfgr1`]
module*/
pub type MPCBB3_PRIVCFGR1 = crate::Reg<mpcbb3_privcfgr1::MPCBB3_PRIVCFGR1rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr1;
/**MPCBB3_PRIVCFGR2 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR2)

For information about available fields see [`mod@mpcbb3_privcfgr2`]
module*/
pub type MPCBB3_PRIVCFGR2 = crate::Reg<mpcbb3_privcfgr2::MPCBB3_PRIVCFGR2rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr2;
/**MPCBB3_PRIVCFGR3 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR3)

For information about available fields see [`mod@mpcbb3_privcfgr3`]
module*/
pub type MPCBB3_PRIVCFGR3 = crate::Reg<mpcbb3_privcfgr3::MPCBB3_PRIVCFGR3rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr3;
/**MPCBB3_PRIVCFGR4 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR4)

For information about available fields see [`mod@mpcbb3_privcfgr4`]
module*/
pub type MPCBB3_PRIVCFGR4 = crate::Reg<mpcbb3_privcfgr4::MPCBB3_PRIVCFGR4rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr4;
/**MPCBB3_PRIVCFGR5 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR5)

For information about available fields see [`mod@mpcbb3_privcfgr5`]
module*/
pub type MPCBB3_PRIVCFGR5 = crate::Reg<mpcbb3_privcfgr5::MPCBB3_PRIVCFGR5rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr5;
/**MPCBB3_PRIVCFGR6 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR6)

For information about available fields see [`mod@mpcbb3_privcfgr6`]
module*/
pub type MPCBB3_PRIVCFGR6 = crate::Reg<mpcbb3_privcfgr6::MPCBB3_PRIVCFGR6rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr6;
/**MPCBB3_PRIVCFGR7 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR7)

For information about available fields see [`mod@mpcbb3_privcfgr7`]
module*/
pub type MPCBB3_PRIVCFGR7 = crate::Reg<mpcbb3_privcfgr7::MPCBB3_PRIVCFGR7rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr7;
/**MPCBB3_PRIVCFGR8 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR8)

For information about available fields see [`mod@mpcbb3_privcfgr8`]
module*/
pub type MPCBB3_PRIVCFGR8 = crate::Reg<mpcbb3_privcfgr8::MPCBB3_PRIVCFGR8rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr8;
/**MPCBB3_PRIVCFGR9 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR9)

For information about available fields see [`mod@mpcbb3_privcfgr9`]
module*/
pub type MPCBB3_PRIVCFGR9 = crate::Reg<mpcbb3_privcfgr9::MPCBB3_PRIVCFGR9rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr9;
/**MPCBB3_PRIVCFGR10 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR10)

For information about available fields see [`mod@mpcbb3_privcfgr10`]
module*/
pub type MPCBB3_PRIVCFGR10 = crate::Reg<mpcbb3_privcfgr10::MPCBB3_PRIVCFGR10rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr10;
/**MPCBB3_PRIVCFGR11 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR11)

For information about available fields see [`mod@mpcbb3_privcfgr11`]
module*/
pub type MPCBB3_PRIVCFGR11 = crate::Reg<mpcbb3_privcfgr11::MPCBB3_PRIVCFGR11rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr11;
/**MPCBB3_PRIVCFGR12 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR12)

For information about available fields see [`mod@mpcbb3_privcfgr12`]
module*/
pub type MPCBB3_PRIVCFGR12 = crate::Reg<mpcbb3_privcfgr12::MPCBB3_PRIVCFGR12rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr12;
/**MPCBB3_PRIVCFGR13 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR13)

For information about available fields see [`mod@mpcbb3_privcfgr13`]
module*/
pub type MPCBB3_PRIVCFGR13 = crate::Reg<mpcbb3_privcfgr13::MPCBB3_PRIVCFGR13rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr13;
/**MPCBB3_PRIVCFGR14 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR14)

For information about available fields see [`mod@mpcbb3_privcfgr14`]
module*/
pub type MPCBB3_PRIVCFGR14 = crate::Reg<mpcbb3_privcfgr14::MPCBB3_PRIVCFGR14rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr14;
/**MPCBB3_PRIVCFGR15 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR15)

For information about available fields see [`mod@mpcbb3_privcfgr15`]
module*/
pub type MPCBB3_PRIVCFGR15 = crate::Reg<mpcbb3_privcfgr15::MPCBB3_PRIVCFGR15rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr15;
/**MPCBB3_PRIVCFGR16 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR16)

For information about available fields see [`mod@mpcbb3_privcfgr16`]
module*/
pub type MPCBB3_PRIVCFGR16 = crate::Reg<mpcbb3_privcfgr16::MPCBB3_PRIVCFGR16rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr16;
/**MPCBB3_PRIVCFGR17 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR17)

For information about available fields see [`mod@mpcbb3_privcfgr17`]
module*/
pub type MPCBB3_PRIVCFGR17 = crate::Reg<mpcbb3_privcfgr17::MPCBB3_PRIVCFGR17rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr17;
/**MPCBB3_PRIVCFGR18 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR18)

For information about available fields see [`mod@mpcbb3_privcfgr18`]
module*/
pub type MPCBB3_PRIVCFGR18 = crate::Reg<mpcbb3_privcfgr18::MPCBB3_PRIVCFGR18rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr18;
/**MPCBB3_PRIVCFGR19 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR19)

For information about available fields see [`mod@mpcbb3_privcfgr19`]
module*/
pub type MPCBB3_PRIVCFGR19 = crate::Reg<mpcbb3_privcfgr19::MPCBB3_PRIVCFGR19rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr19;
/**MPCBB3_PRIVCFGR20 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR20)

For information about available fields see [`mod@mpcbb3_privcfgr20`]
module*/
pub type MPCBB3_PRIVCFGR20 = crate::Reg<mpcbb3_privcfgr20::MPCBB3_PRIVCFGR20rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr20;
/**MPCBB3_PRIVCFGR21 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR21)

For information about available fields see [`mod@mpcbb3_privcfgr21`]
module*/
pub type MPCBB3_PRIVCFGR21 = crate::Reg<mpcbb3_privcfgr21::MPCBB3_PRIVCFGR21rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr21;
/**MPCBB3_PRIVCFGR22 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR22)

For information about available fields see [`mod@mpcbb3_privcfgr22`]
module*/
pub type MPCBB3_PRIVCFGR22 = crate::Reg<mpcbb3_privcfgr22::MPCBB3_PRIVCFGR22rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr22;
/**MPCBB3_PRIVCFGR23 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR23)

For information about available fields see [`mod@mpcbb3_privcfgr23`]
module*/
pub type MPCBB3_PRIVCFGR23 = crate::Reg<mpcbb3_privcfgr23::MPCBB3_PRIVCFGR23rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr23;
/**MPCBB3_PRIVCFGR24 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR24)

For information about available fields see [`mod@mpcbb3_privcfgr24`]
module*/
pub type MPCBB3_PRIVCFGR24 = crate::Reg<mpcbb3_privcfgr24::MPCBB3_PRIVCFGR24rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr24;
/**MPCBB3_PRIVCFGR25 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR25)

For information about available fields see [`mod@mpcbb3_privcfgr25`]
module*/
pub type MPCBB3_PRIVCFGR25 = crate::Reg<mpcbb3_privcfgr25::MPCBB3_PRIVCFGR25rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr25;
/**MPCBB3_PRIVCFGR26 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR26)

For information about available fields see [`mod@mpcbb3_privcfgr26`]
module*/
pub type MPCBB3_PRIVCFGR26 = crate::Reg<mpcbb3_privcfgr26::MPCBB3_PRIVCFGR26rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr26;
/**MPCBB3_PRIVCFGR27 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR27)

For information about available fields see [`mod@mpcbb3_privcfgr27`]
module*/
pub type MPCBB3_PRIVCFGR27 = crate::Reg<mpcbb3_privcfgr27::MPCBB3_PRIVCFGR27rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr27;
/**MPCBB3_PRIVCFGR28 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR28)

For information about available fields see [`mod@mpcbb3_privcfgr28`]
module*/
pub type MPCBB3_PRIVCFGR28 = crate::Reg<mpcbb3_privcfgr28::MPCBB3_PRIVCFGR28rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr28;
/**MPCBB3_PRIVCFGR29 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR29)

For information about available fields see [`mod@mpcbb3_privcfgr29`]
module*/
pub type MPCBB3_PRIVCFGR29 = crate::Reg<mpcbb3_privcfgr29::MPCBB3_PRIVCFGR29rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr29;
/**MPCBB3_PRIVCFGR30 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR30)

For information about available fields see [`mod@mpcbb3_privcfgr30`]
module*/
pub type MPCBB3_PRIVCFGR30 = crate::Reg<mpcbb3_privcfgr30::MPCBB3_PRIVCFGR30rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr30;
/**MPCBB3_PRIVCFGR31 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb3_privcfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb3_privcfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB3:MPCBB3_PRIVCFGR31)

For information about available fields see [`mod@mpcbb3_privcfgr31`]
module*/
pub type MPCBB3_PRIVCFGR31 = crate::Reg<mpcbb3_privcfgr31::MPCBB3_PRIVCFGR31rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr31;
