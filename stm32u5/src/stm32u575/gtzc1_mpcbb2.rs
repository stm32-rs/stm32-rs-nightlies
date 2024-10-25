#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mpcbb2_cr: MPCBB2_CR,
    _reserved1: [u8; 0x0c],
    mpcbb2_cfglockr1: MPCBB2_CFGLOCKR1,
    _reserved2: [u8; 0xec],
    mpcbb2_seccfgr0: MPCBB2_SECCFGR0,
    mpcbb2_seccfgr1: MPCBB2_SECCFGR1,
    mpcbb2_seccfgr2: MPCBB2_SECCFGR2,
    mpcbb2_seccfgr3: MPCBB2_SECCFGR3,
    mpcbb2_seccfgr4: MPCBB2_SECCFGR4,
    mpcbb2_seccfgr5: MPCBB2_SECCFGR5,
    mpcbb2_seccfgr6: MPCBB2_SECCFGR6,
    mpcbb2_seccfgr7: MPCBB2_SECCFGR7,
    mpcbb2_seccfgr8: MPCBB2_SECCFGR8,
    mpcbb2_seccfgr9: MPCBB2_SECCFGR9,
    mpcbb2_seccfgr10: MPCBB2_SECCFGR10,
    mpcbb2_seccfgr11: MPCBB2_SECCFGR11,
    mpcbb2_seccfgr12: MPCBB2_SECCFGR12,
    mpcbb2_seccfgr13: MPCBB2_SECCFGR13,
    mpcbb2_seccfgr14: MPCBB2_SECCFGR14,
    mpcbb2_seccfgr15: MPCBB2_SECCFGR15,
    mpcbb2_seccfgr16: MPCBB2_SECCFGR16,
    mpcbb2_seccfgr17: MPCBB2_SECCFGR17,
    mpcbb2_seccfgr18: MPCBB2_SECCFGR18,
    mpcbb2_seccfgr19: MPCBB2_SECCFGR19,
    mpcbb2_seccfgr20: MPCBB2_SECCFGR20,
    mpcbb2_seccfgr21: MPCBB2_SECCFGR21,
    mpcbb2_seccfgr22: MPCBB2_SECCFGR22,
    mpcbb2_seccfgr23: MPCBB2_SECCFGR23,
    mpcbb2_seccfgr24: MPCBB2_SECCFGR24,
    mpcbb2_seccfgr25: MPCBB2_SECCFGR25,
    mpcbb2_seccfgr26: MPCBB2_SECCFGR26,
    mpcbb2_seccfgr27: MPCBB2_SECCFGR27,
    mpcbb2_seccfgr28: MPCBB2_SECCFGR28,
    mpcbb2_seccfgr29: MPCBB2_SECCFGR29,
    mpcbb2_seccfgr30: MPCBB2_SECCFGR30,
    mpcbb2_seccfgr31: MPCBB2_SECCFGR31,
    _reserved34: [u8; 0x80],
    mpcbb2_privcfgr0: MPCBB2_PRIVCFGR0,
    mpcbb2_privcfgr1: MPCBB2_PRIVCFGR1,
    mpcbb2_privcfgr2: MPCBB2_PRIVCFGR2,
    mpcbb2_privcfgr3: MPCBB2_PRIVCFGR3,
    mpcbb2_privcfgr4: MPCBB2_PRIVCFGR4,
    mpcbb2_privcfgr5: MPCBB2_PRIVCFGR5,
    mpcbb2_privcfgr6: MPCBB2_PRIVCFGR6,
    mpcbb2_privcfgr7: MPCBB2_PRIVCFGR7,
    mpcbb2_privcfgr8: MPCBB2_PRIVCFGR8,
    mpcbb2_privcfgr9: MPCBB2_PRIVCFGR9,
    mpcbb2_privcfgr10: MPCBB2_PRIVCFGR10,
    mpcbb2_privcfgr11: MPCBB2_PRIVCFGR11,
    mpcbb2_privcfgr12: MPCBB2_PRIVCFGR12,
    mpcbb2_privcfgr13: MPCBB2_PRIVCFGR13,
    mpcbb2_privcfgr14: MPCBB2_PRIVCFGR14,
    mpcbb2_privcfgr15: MPCBB2_PRIVCFGR15,
    mpcbb2_privcfgr16: MPCBB2_PRIVCFGR16,
    mpcbb2_privcfgr17: MPCBB2_PRIVCFGR17,
    mpcbb2_privcfgr18: MPCBB2_PRIVCFGR18,
    mpcbb2_privcfgr19: MPCBB2_PRIVCFGR19,
    mpcbb2_privcfgr20: MPCBB2_PRIVCFGR20,
    mpcbb2_privcfgr21: MPCBB2_PRIVCFGR21,
    mpcbb2_privcfgr22: MPCBB2_PRIVCFGR22,
    mpcbb2_privcfgr23: MPCBB2_PRIVCFGR23,
    mpcbb2_privcfgr24: MPCBB2_PRIVCFGR24,
    mpcbb2_privcfgr25: MPCBB2_PRIVCFGR25,
    mpcbb2_privcfgr26: MPCBB2_PRIVCFGR26,
    mpcbb2_privcfgr27: MPCBB2_PRIVCFGR27,
    mpcbb2_privcfgr28: MPCBB2_PRIVCFGR28,
    mpcbb2_privcfgr29: MPCBB2_PRIVCFGR29,
    mpcbb2_privcfgr30: MPCBB2_PRIVCFGR30,
    mpcbb2_privcfgr31: MPCBB2_PRIVCFGR31,
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn mpcbb2_cr(&self) -> &MPCBB2_CR {
        &self.mpcbb2_cr
    }
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    #[inline(always)]
    pub const fn mpcbb2_cfglockr1(&self) -> &MPCBB2_CFGLOCKR1 {
        &self.mpcbb2_cfglockr1
    }
    ///0x100 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr0(&self) -> &MPCBB2_SECCFGR0 {
        &self.mpcbb2_seccfgr0
    }
    ///0x104 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr1(&self) -> &MPCBB2_SECCFGR1 {
        &self.mpcbb2_seccfgr1
    }
    ///0x108 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr2(&self) -> &MPCBB2_SECCFGR2 {
        &self.mpcbb2_seccfgr2
    }
    ///0x10c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr3(&self) -> &MPCBB2_SECCFGR3 {
        &self.mpcbb2_seccfgr3
    }
    ///0x110 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr4(&self) -> &MPCBB2_SECCFGR4 {
        &self.mpcbb2_seccfgr4
    }
    ///0x114 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr5(&self) -> &MPCBB2_SECCFGR5 {
        &self.mpcbb2_seccfgr5
    }
    ///0x118 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr6(&self) -> &MPCBB2_SECCFGR6 {
        &self.mpcbb2_seccfgr6
    }
    ///0x11c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr7(&self) -> &MPCBB2_SECCFGR7 {
        &self.mpcbb2_seccfgr7
    }
    ///0x120 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr8(&self) -> &MPCBB2_SECCFGR8 {
        &self.mpcbb2_seccfgr8
    }
    ///0x124 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr9(&self) -> &MPCBB2_SECCFGR9 {
        &self.mpcbb2_seccfgr9
    }
    ///0x128 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr10(&self) -> &MPCBB2_SECCFGR10 {
        &self.mpcbb2_seccfgr10
    }
    ///0x12c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr11(&self) -> &MPCBB2_SECCFGR11 {
        &self.mpcbb2_seccfgr11
    }
    ///0x130 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr12(&self) -> &MPCBB2_SECCFGR12 {
        &self.mpcbb2_seccfgr12
    }
    ///0x134 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr13(&self) -> &MPCBB2_SECCFGR13 {
        &self.mpcbb2_seccfgr13
    }
    ///0x138 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr14(&self) -> &MPCBB2_SECCFGR14 {
        &self.mpcbb2_seccfgr14
    }
    ///0x13c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr15(&self) -> &MPCBB2_SECCFGR15 {
        &self.mpcbb2_seccfgr15
    }
    ///0x140 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr16(&self) -> &MPCBB2_SECCFGR16 {
        &self.mpcbb2_seccfgr16
    }
    ///0x144 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr17(&self) -> &MPCBB2_SECCFGR17 {
        &self.mpcbb2_seccfgr17
    }
    ///0x148 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr18(&self) -> &MPCBB2_SECCFGR18 {
        &self.mpcbb2_seccfgr18
    }
    ///0x14c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr19(&self) -> &MPCBB2_SECCFGR19 {
        &self.mpcbb2_seccfgr19
    }
    ///0x150 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr20(&self) -> &MPCBB2_SECCFGR20 {
        &self.mpcbb2_seccfgr20
    }
    ///0x154 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr21(&self) -> &MPCBB2_SECCFGR21 {
        &self.mpcbb2_seccfgr21
    }
    ///0x158 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr22(&self) -> &MPCBB2_SECCFGR22 {
        &self.mpcbb2_seccfgr22
    }
    ///0x15c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr23(&self) -> &MPCBB2_SECCFGR23 {
        &self.mpcbb2_seccfgr23
    }
    ///0x160 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr24(&self) -> &MPCBB2_SECCFGR24 {
        &self.mpcbb2_seccfgr24
    }
    ///0x164 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr25(&self) -> &MPCBB2_SECCFGR25 {
        &self.mpcbb2_seccfgr25
    }
    ///0x168 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr26(&self) -> &MPCBB2_SECCFGR26 {
        &self.mpcbb2_seccfgr26
    }
    ///0x16c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr27(&self) -> &MPCBB2_SECCFGR27 {
        &self.mpcbb2_seccfgr27
    }
    ///0x170 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr28(&self) -> &MPCBB2_SECCFGR28 {
        &self.mpcbb2_seccfgr28
    }
    ///0x174 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr29(&self) -> &MPCBB2_SECCFGR29 {
        &self.mpcbb2_seccfgr29
    }
    ///0x178 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr30(&self) -> &MPCBB2_SECCFGR30 {
        &self.mpcbb2_seccfgr30
    }
    ///0x17c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_seccfgr31(&self) -> &MPCBB2_SECCFGR31 {
        &self.mpcbb2_seccfgr31
    }
    ///0x200 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr0(&self) -> &MPCBB2_PRIVCFGR0 {
        &self.mpcbb2_privcfgr0
    }
    ///0x204 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr1(&self) -> &MPCBB2_PRIVCFGR1 {
        &self.mpcbb2_privcfgr1
    }
    ///0x208 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr2(&self) -> &MPCBB2_PRIVCFGR2 {
        &self.mpcbb2_privcfgr2
    }
    ///0x20c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr3(&self) -> &MPCBB2_PRIVCFGR3 {
        &self.mpcbb2_privcfgr3
    }
    ///0x210 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr4(&self) -> &MPCBB2_PRIVCFGR4 {
        &self.mpcbb2_privcfgr4
    }
    ///0x214 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr5(&self) -> &MPCBB2_PRIVCFGR5 {
        &self.mpcbb2_privcfgr5
    }
    ///0x218 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr6(&self) -> &MPCBB2_PRIVCFGR6 {
        &self.mpcbb2_privcfgr6
    }
    ///0x21c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr7(&self) -> &MPCBB2_PRIVCFGR7 {
        &self.mpcbb2_privcfgr7
    }
    ///0x220 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr8(&self) -> &MPCBB2_PRIVCFGR8 {
        &self.mpcbb2_privcfgr8
    }
    ///0x224 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr9(&self) -> &MPCBB2_PRIVCFGR9 {
        &self.mpcbb2_privcfgr9
    }
    ///0x228 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr10(&self) -> &MPCBB2_PRIVCFGR10 {
        &self.mpcbb2_privcfgr10
    }
    ///0x22c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr11(&self) -> &MPCBB2_PRIVCFGR11 {
        &self.mpcbb2_privcfgr11
    }
    ///0x230 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr12(&self) -> &MPCBB2_PRIVCFGR12 {
        &self.mpcbb2_privcfgr12
    }
    ///0x234 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr13(&self) -> &MPCBB2_PRIVCFGR13 {
        &self.mpcbb2_privcfgr13
    }
    ///0x238 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr14(&self) -> &MPCBB2_PRIVCFGR14 {
        &self.mpcbb2_privcfgr14
    }
    ///0x23c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr15(&self) -> &MPCBB2_PRIVCFGR15 {
        &self.mpcbb2_privcfgr15
    }
    ///0x240 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr16(&self) -> &MPCBB2_PRIVCFGR16 {
        &self.mpcbb2_privcfgr16
    }
    ///0x244 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr17(&self) -> &MPCBB2_PRIVCFGR17 {
        &self.mpcbb2_privcfgr17
    }
    ///0x248 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr18(&self) -> &MPCBB2_PRIVCFGR18 {
        &self.mpcbb2_privcfgr18
    }
    ///0x24c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr19(&self) -> &MPCBB2_PRIVCFGR19 {
        &self.mpcbb2_privcfgr19
    }
    ///0x250 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr20(&self) -> &MPCBB2_PRIVCFGR20 {
        &self.mpcbb2_privcfgr20
    }
    ///0x254 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr21(&self) -> &MPCBB2_PRIVCFGR21 {
        &self.mpcbb2_privcfgr21
    }
    ///0x258 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr22(&self) -> &MPCBB2_PRIVCFGR22 {
        &self.mpcbb2_privcfgr22
    }
    ///0x25c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr23(&self) -> &MPCBB2_PRIVCFGR23 {
        &self.mpcbb2_privcfgr23
    }
    ///0x260 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr24(&self) -> &MPCBB2_PRIVCFGR24 {
        &self.mpcbb2_privcfgr24
    }
    ///0x264 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr25(&self) -> &MPCBB2_PRIVCFGR25 {
        &self.mpcbb2_privcfgr25
    }
    ///0x268 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr26(&self) -> &MPCBB2_PRIVCFGR26 {
        &self.mpcbb2_privcfgr26
    }
    ///0x26c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr27(&self) -> &MPCBB2_PRIVCFGR27 {
        &self.mpcbb2_privcfgr27
    }
    ///0x270 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr28(&self) -> &MPCBB2_PRIVCFGR28 {
        &self.mpcbb2_privcfgr28
    }
    ///0x274 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr29(&self) -> &MPCBB2_PRIVCFGR29 {
        &self.mpcbb2_privcfgr29
    }
    ///0x278 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr30(&self) -> &MPCBB2_PRIVCFGR30 {
        &self.mpcbb2_privcfgr30
    }
    ///0x27c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb2_privcfgr31(&self) -> &MPCBB2_PRIVCFGR31 {
        &self.mpcbb2_privcfgr31
    }
}
/**MPCBB2_CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_CR)

For information about available fields see [`mod@mpcbb2_cr`]
module*/
pub type MPCBB2_CR = crate::Reg<mpcbb2_cr::MPCBB2_CRrs>;
///MPCBB control register
pub mod mpcbb2_cr;
/**MPCBB2_CFGLOCKR1 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_cfglockr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_cfglockr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_CFGLOCKR1)

For information about available fields see [`mod@mpcbb2_cfglockr1`]
module*/
pub type MPCBB2_CFGLOCKR1 = crate::Reg<mpcbb2_cfglockr1::MPCBB2_CFGLOCKR1rs>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod mpcbb2_cfglockr1;
/**MPCBB2_SECCFGR0 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR0)

For information about available fields see [`mod@mpcbb2_seccfgr0`]
module*/
pub type MPCBB2_SECCFGR0 = crate::Reg<mpcbb2_seccfgr0::MPCBB2_SECCFGR0rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr0;
/**MPCBB2_SECCFGR1 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR1)

For information about available fields see [`mod@mpcbb2_seccfgr1`]
module*/
pub type MPCBB2_SECCFGR1 = crate::Reg<mpcbb2_seccfgr1::MPCBB2_SECCFGR1rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr1;
/**MPCBB2_SECCFGR2 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR2)

For information about available fields see [`mod@mpcbb2_seccfgr2`]
module*/
pub type MPCBB2_SECCFGR2 = crate::Reg<mpcbb2_seccfgr2::MPCBB2_SECCFGR2rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr2;
/**MPCBB2_SECCFGR3 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR3)

For information about available fields see [`mod@mpcbb2_seccfgr3`]
module*/
pub type MPCBB2_SECCFGR3 = crate::Reg<mpcbb2_seccfgr3::MPCBB2_SECCFGR3rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr3;
/**MPCBB2_SECCFGR4 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR4)

For information about available fields see [`mod@mpcbb2_seccfgr4`]
module*/
pub type MPCBB2_SECCFGR4 = crate::Reg<mpcbb2_seccfgr4::MPCBB2_SECCFGR4rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr4;
/**MPCBB2_SECCFGR5 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR5)

For information about available fields see [`mod@mpcbb2_seccfgr5`]
module*/
pub type MPCBB2_SECCFGR5 = crate::Reg<mpcbb2_seccfgr5::MPCBB2_SECCFGR5rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr5;
/**MPCBB2_SECCFGR6 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR6)

For information about available fields see [`mod@mpcbb2_seccfgr6`]
module*/
pub type MPCBB2_SECCFGR6 = crate::Reg<mpcbb2_seccfgr6::MPCBB2_SECCFGR6rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr6;
/**MPCBB2_SECCFGR7 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR7)

For information about available fields see [`mod@mpcbb2_seccfgr7`]
module*/
pub type MPCBB2_SECCFGR7 = crate::Reg<mpcbb2_seccfgr7::MPCBB2_SECCFGR7rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr7;
/**MPCBB2_SECCFGR8 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR8)

For information about available fields see [`mod@mpcbb2_seccfgr8`]
module*/
pub type MPCBB2_SECCFGR8 = crate::Reg<mpcbb2_seccfgr8::MPCBB2_SECCFGR8rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr8;
/**MPCBB2_SECCFGR9 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR9)

For information about available fields see [`mod@mpcbb2_seccfgr9`]
module*/
pub type MPCBB2_SECCFGR9 = crate::Reg<mpcbb2_seccfgr9::MPCBB2_SECCFGR9rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr9;
/**MPCBB2_SECCFGR10 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR10)

For information about available fields see [`mod@mpcbb2_seccfgr10`]
module*/
pub type MPCBB2_SECCFGR10 = crate::Reg<mpcbb2_seccfgr10::MPCBB2_SECCFGR10rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr10;
/**MPCBB2_SECCFGR11 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR11)

For information about available fields see [`mod@mpcbb2_seccfgr11`]
module*/
pub type MPCBB2_SECCFGR11 = crate::Reg<mpcbb2_seccfgr11::MPCBB2_SECCFGR11rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr11;
/**MPCBB2_SECCFGR12 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR12)

For information about available fields see [`mod@mpcbb2_seccfgr12`]
module*/
pub type MPCBB2_SECCFGR12 = crate::Reg<mpcbb2_seccfgr12::MPCBB2_SECCFGR12rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr12;
/**MPCBB2_SECCFGR13 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR13)

For information about available fields see [`mod@mpcbb2_seccfgr13`]
module*/
pub type MPCBB2_SECCFGR13 = crate::Reg<mpcbb2_seccfgr13::MPCBB2_SECCFGR13rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr13;
/**MPCBB2_SECCFGR14 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR14)

For information about available fields see [`mod@mpcbb2_seccfgr14`]
module*/
pub type MPCBB2_SECCFGR14 = crate::Reg<mpcbb2_seccfgr14::MPCBB2_SECCFGR14rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr14;
/**MPCBB2_SECCFGR15 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR15)

For information about available fields see [`mod@mpcbb2_seccfgr15`]
module*/
pub type MPCBB2_SECCFGR15 = crate::Reg<mpcbb2_seccfgr15::MPCBB2_SECCFGR15rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr15;
/**MPCBB2_SECCFGR16 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR16)

For information about available fields see [`mod@mpcbb2_seccfgr16`]
module*/
pub type MPCBB2_SECCFGR16 = crate::Reg<mpcbb2_seccfgr16::MPCBB2_SECCFGR16rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr16;
/**MPCBB2_SECCFGR17 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR17)

For information about available fields see [`mod@mpcbb2_seccfgr17`]
module*/
pub type MPCBB2_SECCFGR17 = crate::Reg<mpcbb2_seccfgr17::MPCBB2_SECCFGR17rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr17;
/**MPCBB2_SECCFGR18 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR18)

For information about available fields see [`mod@mpcbb2_seccfgr18`]
module*/
pub type MPCBB2_SECCFGR18 = crate::Reg<mpcbb2_seccfgr18::MPCBB2_SECCFGR18rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr18;
/**MPCBB2_SECCFGR19 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR19)

For information about available fields see [`mod@mpcbb2_seccfgr19`]
module*/
pub type MPCBB2_SECCFGR19 = crate::Reg<mpcbb2_seccfgr19::MPCBB2_SECCFGR19rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr19;
/**MPCBB2_SECCFGR20 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR20)

For information about available fields see [`mod@mpcbb2_seccfgr20`]
module*/
pub type MPCBB2_SECCFGR20 = crate::Reg<mpcbb2_seccfgr20::MPCBB2_SECCFGR20rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr20;
/**MPCBB2_SECCFGR21 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR21)

For information about available fields see [`mod@mpcbb2_seccfgr21`]
module*/
pub type MPCBB2_SECCFGR21 = crate::Reg<mpcbb2_seccfgr21::MPCBB2_SECCFGR21rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr21;
/**MPCBB2_SECCFGR22 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR22)

For information about available fields see [`mod@mpcbb2_seccfgr22`]
module*/
pub type MPCBB2_SECCFGR22 = crate::Reg<mpcbb2_seccfgr22::MPCBB2_SECCFGR22rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr22;
/**MPCBB2_SECCFGR23 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR23)

For information about available fields see [`mod@mpcbb2_seccfgr23`]
module*/
pub type MPCBB2_SECCFGR23 = crate::Reg<mpcbb2_seccfgr23::MPCBB2_SECCFGR23rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr23;
/**MPCBB2_SECCFGR24 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR24)

For information about available fields see [`mod@mpcbb2_seccfgr24`]
module*/
pub type MPCBB2_SECCFGR24 = crate::Reg<mpcbb2_seccfgr24::MPCBB2_SECCFGR24rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr24;
/**MPCBB2_SECCFGR25 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR25)

For information about available fields see [`mod@mpcbb2_seccfgr25`]
module*/
pub type MPCBB2_SECCFGR25 = crate::Reg<mpcbb2_seccfgr25::MPCBB2_SECCFGR25rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr25;
/**MPCBB2_SECCFGR26 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR26)

For information about available fields see [`mod@mpcbb2_seccfgr26`]
module*/
pub type MPCBB2_SECCFGR26 = crate::Reg<mpcbb2_seccfgr26::MPCBB2_SECCFGR26rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr26;
/**MPCBB2_SECCFGR27 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR27)

For information about available fields see [`mod@mpcbb2_seccfgr27`]
module*/
pub type MPCBB2_SECCFGR27 = crate::Reg<mpcbb2_seccfgr27::MPCBB2_SECCFGR27rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr27;
/**MPCBB2_SECCFGR28 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR28)

For information about available fields see [`mod@mpcbb2_seccfgr28`]
module*/
pub type MPCBB2_SECCFGR28 = crate::Reg<mpcbb2_seccfgr28::MPCBB2_SECCFGR28rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr28;
/**MPCBB2_SECCFGR29 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR29)

For information about available fields see [`mod@mpcbb2_seccfgr29`]
module*/
pub type MPCBB2_SECCFGR29 = crate::Reg<mpcbb2_seccfgr29::MPCBB2_SECCFGR29rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr29;
/**MPCBB2_SECCFGR30 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR30)

For information about available fields see [`mod@mpcbb2_seccfgr30`]
module*/
pub type MPCBB2_SECCFGR30 = crate::Reg<mpcbb2_seccfgr30::MPCBB2_SECCFGR30rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr30;
/**MPCBB2_SECCFGR31 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_seccfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_seccfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_SECCFGR31)

For information about available fields see [`mod@mpcbb2_seccfgr31`]
module*/
pub type MPCBB2_SECCFGR31 = crate::Reg<mpcbb2_seccfgr31::MPCBB2_SECCFGR31rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb2_seccfgr31;
/**MPCBB2_PRIVCFGR0 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR0)

For information about available fields see [`mod@mpcbb2_privcfgr0`]
module*/
pub type MPCBB2_PRIVCFGR0 = crate::Reg<mpcbb2_privcfgr0::MPCBB2_PRIVCFGR0rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr0;
/**MPCBB2_PRIVCFGR1 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR1)

For information about available fields see [`mod@mpcbb2_privcfgr1`]
module*/
pub type MPCBB2_PRIVCFGR1 = crate::Reg<mpcbb2_privcfgr1::MPCBB2_PRIVCFGR1rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr1;
/**MPCBB2_PRIVCFGR2 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR2)

For information about available fields see [`mod@mpcbb2_privcfgr2`]
module*/
pub type MPCBB2_PRIVCFGR2 = crate::Reg<mpcbb2_privcfgr2::MPCBB2_PRIVCFGR2rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr2;
/**MPCBB2_PRIVCFGR3 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR3)

For information about available fields see [`mod@mpcbb2_privcfgr3`]
module*/
pub type MPCBB2_PRIVCFGR3 = crate::Reg<mpcbb2_privcfgr3::MPCBB2_PRIVCFGR3rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr3;
/**MPCBB2_PRIVCFGR4 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR4)

For information about available fields see [`mod@mpcbb2_privcfgr4`]
module*/
pub type MPCBB2_PRIVCFGR4 = crate::Reg<mpcbb2_privcfgr4::MPCBB2_PRIVCFGR4rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr4;
/**MPCBB2_PRIVCFGR5 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR5)

For information about available fields see [`mod@mpcbb2_privcfgr5`]
module*/
pub type MPCBB2_PRIVCFGR5 = crate::Reg<mpcbb2_privcfgr5::MPCBB2_PRIVCFGR5rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr5;
/**MPCBB2_PRIVCFGR6 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR6)

For information about available fields see [`mod@mpcbb2_privcfgr6`]
module*/
pub type MPCBB2_PRIVCFGR6 = crate::Reg<mpcbb2_privcfgr6::MPCBB2_PRIVCFGR6rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr6;
/**MPCBB2_PRIVCFGR7 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR7)

For information about available fields see [`mod@mpcbb2_privcfgr7`]
module*/
pub type MPCBB2_PRIVCFGR7 = crate::Reg<mpcbb2_privcfgr7::MPCBB2_PRIVCFGR7rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr7;
/**MPCBB2_PRIVCFGR8 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR8)

For information about available fields see [`mod@mpcbb2_privcfgr8`]
module*/
pub type MPCBB2_PRIVCFGR8 = crate::Reg<mpcbb2_privcfgr8::MPCBB2_PRIVCFGR8rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr8;
/**MPCBB2_PRIVCFGR9 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR9)

For information about available fields see [`mod@mpcbb2_privcfgr9`]
module*/
pub type MPCBB2_PRIVCFGR9 = crate::Reg<mpcbb2_privcfgr9::MPCBB2_PRIVCFGR9rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr9;
/**MPCBB2_PRIVCFGR10 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR10)

For information about available fields see [`mod@mpcbb2_privcfgr10`]
module*/
pub type MPCBB2_PRIVCFGR10 = crate::Reg<mpcbb2_privcfgr10::MPCBB2_PRIVCFGR10rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr10;
/**MPCBB2_PRIVCFGR11 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR11)

For information about available fields see [`mod@mpcbb2_privcfgr11`]
module*/
pub type MPCBB2_PRIVCFGR11 = crate::Reg<mpcbb2_privcfgr11::MPCBB2_PRIVCFGR11rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr11;
/**MPCBB2_PRIVCFGR12 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR12)

For information about available fields see [`mod@mpcbb2_privcfgr12`]
module*/
pub type MPCBB2_PRIVCFGR12 = crate::Reg<mpcbb2_privcfgr12::MPCBB2_PRIVCFGR12rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr12;
/**MPCBB2_PRIVCFGR13 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR13)

For information about available fields see [`mod@mpcbb2_privcfgr13`]
module*/
pub type MPCBB2_PRIVCFGR13 = crate::Reg<mpcbb2_privcfgr13::MPCBB2_PRIVCFGR13rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr13;
/**MPCBB2_PRIVCFGR14 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR14)

For information about available fields see [`mod@mpcbb2_privcfgr14`]
module*/
pub type MPCBB2_PRIVCFGR14 = crate::Reg<mpcbb2_privcfgr14::MPCBB2_PRIVCFGR14rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr14;
/**MPCBB2_PRIVCFGR15 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR15)

For information about available fields see [`mod@mpcbb2_privcfgr15`]
module*/
pub type MPCBB2_PRIVCFGR15 = crate::Reg<mpcbb2_privcfgr15::MPCBB2_PRIVCFGR15rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr15;
/**MPCBB2_PRIVCFGR16 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR16)

For information about available fields see [`mod@mpcbb2_privcfgr16`]
module*/
pub type MPCBB2_PRIVCFGR16 = crate::Reg<mpcbb2_privcfgr16::MPCBB2_PRIVCFGR16rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr16;
/**MPCBB2_PRIVCFGR17 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR17)

For information about available fields see [`mod@mpcbb2_privcfgr17`]
module*/
pub type MPCBB2_PRIVCFGR17 = crate::Reg<mpcbb2_privcfgr17::MPCBB2_PRIVCFGR17rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr17;
/**MPCBB2_PRIVCFGR18 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR18)

For information about available fields see [`mod@mpcbb2_privcfgr18`]
module*/
pub type MPCBB2_PRIVCFGR18 = crate::Reg<mpcbb2_privcfgr18::MPCBB2_PRIVCFGR18rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr18;
/**MPCBB2_PRIVCFGR19 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR19)

For information about available fields see [`mod@mpcbb2_privcfgr19`]
module*/
pub type MPCBB2_PRIVCFGR19 = crate::Reg<mpcbb2_privcfgr19::MPCBB2_PRIVCFGR19rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr19;
/**MPCBB2_PRIVCFGR20 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR20)

For information about available fields see [`mod@mpcbb2_privcfgr20`]
module*/
pub type MPCBB2_PRIVCFGR20 = crate::Reg<mpcbb2_privcfgr20::MPCBB2_PRIVCFGR20rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr20;
/**MPCBB2_PRIVCFGR21 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR21)

For information about available fields see [`mod@mpcbb2_privcfgr21`]
module*/
pub type MPCBB2_PRIVCFGR21 = crate::Reg<mpcbb2_privcfgr21::MPCBB2_PRIVCFGR21rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr21;
/**MPCBB2_PRIVCFGR22 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR22)

For information about available fields see [`mod@mpcbb2_privcfgr22`]
module*/
pub type MPCBB2_PRIVCFGR22 = crate::Reg<mpcbb2_privcfgr22::MPCBB2_PRIVCFGR22rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr22;
/**MPCBB2_PRIVCFGR23 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR23)

For information about available fields see [`mod@mpcbb2_privcfgr23`]
module*/
pub type MPCBB2_PRIVCFGR23 = crate::Reg<mpcbb2_privcfgr23::MPCBB2_PRIVCFGR23rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr23;
/**MPCBB2_PRIVCFGR24 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR24)

For information about available fields see [`mod@mpcbb2_privcfgr24`]
module*/
pub type MPCBB2_PRIVCFGR24 = crate::Reg<mpcbb2_privcfgr24::MPCBB2_PRIVCFGR24rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr24;
/**MPCBB2_PRIVCFGR25 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR25)

For information about available fields see [`mod@mpcbb2_privcfgr25`]
module*/
pub type MPCBB2_PRIVCFGR25 = crate::Reg<mpcbb2_privcfgr25::MPCBB2_PRIVCFGR25rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr25;
/**MPCBB2_PRIVCFGR26 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR26)

For information about available fields see [`mod@mpcbb2_privcfgr26`]
module*/
pub type MPCBB2_PRIVCFGR26 = crate::Reg<mpcbb2_privcfgr26::MPCBB2_PRIVCFGR26rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr26;
/**MPCBB2_PRIVCFGR27 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR27)

For information about available fields see [`mod@mpcbb2_privcfgr27`]
module*/
pub type MPCBB2_PRIVCFGR27 = crate::Reg<mpcbb2_privcfgr27::MPCBB2_PRIVCFGR27rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr27;
/**MPCBB2_PRIVCFGR28 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR28)

For information about available fields see [`mod@mpcbb2_privcfgr28`]
module*/
pub type MPCBB2_PRIVCFGR28 = crate::Reg<mpcbb2_privcfgr28::MPCBB2_PRIVCFGR28rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr28;
/**MPCBB2_PRIVCFGR29 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR29)

For information about available fields see [`mod@mpcbb2_privcfgr29`]
module*/
pub type MPCBB2_PRIVCFGR29 = crate::Reg<mpcbb2_privcfgr29::MPCBB2_PRIVCFGR29rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr29;
/**MPCBB2_PRIVCFGR30 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR30)

For information about available fields see [`mod@mpcbb2_privcfgr30`]
module*/
pub type MPCBB2_PRIVCFGR30 = crate::Reg<mpcbb2_privcfgr30::MPCBB2_PRIVCFGR30rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr30;
/**MPCBB2_PRIVCFGR31 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb2_privcfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb2_privcfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#GTZC1_MPCBB2:MPCBB2_PRIVCFGR31)

For information about available fields see [`mod@mpcbb2_privcfgr31`]
module*/
pub type MPCBB2_PRIVCFGR31 = crate::Reg<mpcbb2_privcfgr31::MPCBB2_PRIVCFGR31rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb2_privcfgr31;
