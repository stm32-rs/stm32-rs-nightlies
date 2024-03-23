#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtzc1_mpcbb3_cr: GTZC1_MPCBB3_CR,
    _reserved1: [u8; 0x0c],
    gtzc1_mpcbb3_cfglock1: GTZC1_MPCBB3_CFGLOCK1,
    _reserved2: [u8; 0xec],
    gtzc1_mpcbb3_seccfgr0: GTZC1_MPCBB3_SECCFGR0,
    gtzc1_mpcbb3_seccfgr1: GTZC1_MPCBB3_SECCFGR1,
    gtzc1_mpcbb3_seccfgr2: GTZC1_MPCBB3_SECCFGR2,
    gtzc1_mpcbb3_seccfgr3: GTZC1_MPCBB3_SECCFGR3,
    gtzc1_mpcbb3_seccfgr4: GTZC1_MPCBB3_SECCFGR4,
    gtzc1_mpcbb3_seccfgr5: GTZC1_MPCBB3_SECCFGR5,
    gtzc1_mpcbb3_seccfgr6: GTZC1_MPCBB3_SECCFGR6,
    gtzc1_mpcbb3_seccfgr7: GTZC1_MPCBB3_SECCFGR7,
    gtzc1_mpcbb3_seccfgr8: GTZC1_MPCBB3_SECCFGR8,
    gtzc1_mpcbb3_seccfgr9: GTZC1_MPCBB3_SECCFGR9,
    gtzc1_mpcbb3_seccfgr10: GTZC1_MPCBB3_SECCFGR10,
    gtzc1_mpcbb3_seccfgr11: GTZC1_MPCBB3_SECCFGR11,
    gtzc1_mpcbb3_seccfgr12: GTZC1_MPCBB3_SECCFGR12,
    gtzc1_mpcbb3_seccfgr13: GTZC1_MPCBB3_SECCFGR13,
    gtzc1_mpcbb3_seccfgr14: GTZC1_MPCBB3_SECCFGR14,
    gtzc1_mpcbb3_seccfgr15: GTZC1_MPCBB3_SECCFGR15,
    gtzc1_mpcbb3_seccfgr16: GTZC1_MPCBB3_SECCFGR16,
    gtzc1_mpcbb3_seccfgr17: GTZC1_MPCBB3_SECCFGR17,
    gtzc1_mpcbb3_seccfgr18: GTZC1_MPCBB3_SECCFGR18,
    gtzc1_mpcbb3_seccfgr19: GTZC1_MPCBB3_SECCFGR19,
    gtzc1_mpcbb3_seccfgr20: GTZC1_MPCBB3_SECCFGR20,
    gtzc1_mpcbb3_seccfgr21: GTZC1_MPCBB3_SECCFGR21,
    gtzc1_mpcbb3_seccfgr22: GTZC1_MPCBB3_SECCFGR22,
    gtzc1_mpcbb3_seccfgr23: GTZC1_MPCBB3_SECCFGR23,
    gtzc1_mpcbb3_seccfgr24: GTZC1_MPCBB3_SECCFGR24,
    gtzc1_mpcbb3_seccfgr25: GTZC1_MPCBB3_SECCFGR25,
    gtzc1_mpcbb3_seccfgr26: GTZC1_MPCBB3_SECCFGR26,
    gtzc1_mpcbb3_seccfgr27: GTZC1_MPCBB3_SECCFGR27,
    gtzc1_mpcbb3_seccfgr28: GTZC1_MPCBB3_SECCFGR28,
    gtzc1_mpcbb3_seccfgr29: GTZC1_MPCBB3_SECCFGR29,
    gtzc1_mpcbb3_seccfgr30: GTZC1_MPCBB3_SECCFGR30,
    gtzc1_mpcbb3_seccfgr31: GTZC1_MPCBB3_SECCFGR31,
    _reserved34: [u8; 0x80],
    gtzc1_mpcbb3_privcfgr0: GTZC1_MPCBB3_PRIVCFGR0,
    gtzc1_mpcbb3_privcfgr1: GTZC1_MPCBB3_PRIVCFGR1,
    gtzc1_mpcbb3_privcfgr2: GTZC1_MPCBB3_PRIVCFGR2,
    gtzc1_mpcbb3_privcfgr3: GTZC1_MPCBB3_PRIVCFGR3,
    gtzc1_mpcbb3_privcfgr4: GTZC1_MPCBB3_PRIVCFGR4,
    gtzc1_mpcbb3_privcfgr5: GTZC1_MPCBB3_PRIVCFGR5,
    gtzc1_mpcbb3_privcfgr6: GTZC1_MPCBB3_PRIVCFGR6,
    gtzc1_mpcbb3_privcfgr7: GTZC1_MPCBB3_PRIVCFGR7,
    gtzc1_mpcbb3_privcfgr8: GTZC1_MPCBB3_PRIVCFGR8,
    gtzc1_mpcbb3_privcfgr9: GTZC1_MPCBB3_PRIVCFGR9,
    gtzc1_mpcbb3_privcfgr10: GTZC1_MPCBB3_PRIVCFGR10,
    gtzc1_mpcbb3_privcfgr11: GTZC1_MPCBB3_PRIVCFGR11,
    gtzc1_mpcbb3_privcfgr12: GTZC1_MPCBB3_PRIVCFGR12,
    gtzc1_mpcbb3_privcfgr13: GTZC1_MPCBB3_PRIVCFGR13,
    gtzc1_mpcbb3_privcfgr14: GTZC1_MPCBB3_PRIVCFGR14,
    gtzc1_mpcbb3_privcfgr15: GTZC1_MPCBB3_PRIVCFGR15,
    gtzc1_mpcbb3_privcfgr16: GTZC1_MPCBB3_PRIVCFGR16,
    gtzc1_mpcbb3_privcfgr17: GTZC1_MPCBB3_PRIVCFGR17,
    gtzc1_mpcbb3_privcfgr18: GTZC1_MPCBB3_PRIVCFGR18,
    gtzc1_mpcbb3_privcfgr19: GTZC1_MPCBB3_PRIVCFGR19,
    gtzc1_mpcbb3_privcfgr20: GTZC1_MPCBB3_PRIVCFGR20,
    gtzc1_mpcbb3_privcfgr21: GTZC1_MPCBB3_PRIVCFGR21,
    gtzc1_mpcbb3_privcfgr22: GTZC1_MPCBB3_PRIVCFGR22,
    gtzc1_mpcbb3_privcfgr23: GTZC1_MPCBB3_PRIVCFGR23,
    gtzc1_mpcbb3_privcfgr24: GTZC1_MPCBB3_PRIVCFGR24,
    gtzc1_mpcbb3_privcfgr25: GTZC1_MPCBB3_PRIVCFGR25,
    gtzc1_mpcbb3_privcfgr26: GTZC1_MPCBB3_PRIVCFGR26,
    gtzc1_mpcbb3_privcfgr27: GTZC1_MPCBB3_PRIVCFGR27,
    gtzc1_mpcbb3_privcfgr28: GTZC1_MPCBB3_PRIVCFGR28,
    gtzc1_mpcbb3_privcfgr29: GTZC1_MPCBB3_PRIVCFGR29,
    gtzc1_mpcbb3_privcfgr30: GTZC1_MPCBB3_PRIVCFGR30,
    gtzc1_mpcbb3_privcfgr31: GTZC1_MPCBB3_PRIVCFGR31,
}
impl RegisterBlock {
    #[doc = "0x00 - GTZC1 SRAM3 MPCBB control register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_cr(&self) -> &GTZC1_MPCBB3_CR {
        &self.gtzc1_mpcbb3_cr
    }
    #[doc = "0x10 - GTZC1 SRAM3 MPCBB configuration lock register 1"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_cfglock1(&self) -> &GTZC1_MPCBB3_CFGLOCK1 {
        &self.gtzc1_mpcbb3_cfglock1
    }
    #[doc = "0x100 - GTZC1 SRAM3 MPCBB security configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr0(&self) -> &GTZC1_MPCBB3_SECCFGR0 {
        &self.gtzc1_mpcbb3_seccfgr0
    }
    #[doc = "0x104 - GTZC1 SRAM3 MPCBB security configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr1(&self) -> &GTZC1_MPCBB3_SECCFGR1 {
        &self.gtzc1_mpcbb3_seccfgr1
    }
    #[doc = "0x108 - GTZC1 SRAM3 MPCBB security configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr2(&self) -> &GTZC1_MPCBB3_SECCFGR2 {
        &self.gtzc1_mpcbb3_seccfgr2
    }
    #[doc = "0x10c - GTZC1 SRAM3 MPCBB security configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr3(&self) -> &GTZC1_MPCBB3_SECCFGR3 {
        &self.gtzc1_mpcbb3_seccfgr3
    }
    #[doc = "0x110 - GTZC1 SRAM3 MPCBB security configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr4(&self) -> &GTZC1_MPCBB3_SECCFGR4 {
        &self.gtzc1_mpcbb3_seccfgr4
    }
    #[doc = "0x114 - GTZC1 SRAM3 MPCBB security configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr5(&self) -> &GTZC1_MPCBB3_SECCFGR5 {
        &self.gtzc1_mpcbb3_seccfgr5
    }
    #[doc = "0x118 - GTZC1 SRAM3 MPCBB security configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr6(&self) -> &GTZC1_MPCBB3_SECCFGR6 {
        &self.gtzc1_mpcbb3_seccfgr6
    }
    #[doc = "0x11c - GTZC1 SRAM3 MPCBB security configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr7(&self) -> &GTZC1_MPCBB3_SECCFGR7 {
        &self.gtzc1_mpcbb3_seccfgr7
    }
    #[doc = "0x120 - GTZC1 SRAM3 MPCBB security configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr8(&self) -> &GTZC1_MPCBB3_SECCFGR8 {
        &self.gtzc1_mpcbb3_seccfgr8
    }
    #[doc = "0x124 - GTZC1 SRAM3 MPCBB security configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr9(&self) -> &GTZC1_MPCBB3_SECCFGR9 {
        &self.gtzc1_mpcbb3_seccfgr9
    }
    #[doc = "0x128 - GTZC1 SRAM3 MPCBB security configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr10(&self) -> &GTZC1_MPCBB3_SECCFGR10 {
        &self.gtzc1_mpcbb3_seccfgr10
    }
    #[doc = "0x12c - GTZC1 SRAM3 MPCBB security configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr11(&self) -> &GTZC1_MPCBB3_SECCFGR11 {
        &self.gtzc1_mpcbb3_seccfgr11
    }
    #[doc = "0x130 - GTZC1 SRAM3 MPCBB security configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr12(&self) -> &GTZC1_MPCBB3_SECCFGR12 {
        &self.gtzc1_mpcbb3_seccfgr12
    }
    #[doc = "0x134 - GTZC1 SRAM3 MPCBB security configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr13(&self) -> &GTZC1_MPCBB3_SECCFGR13 {
        &self.gtzc1_mpcbb3_seccfgr13
    }
    #[doc = "0x138 - GTZC1 SRAM3 MPCBB security configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr14(&self) -> &GTZC1_MPCBB3_SECCFGR14 {
        &self.gtzc1_mpcbb3_seccfgr14
    }
    #[doc = "0x13c - GTZC1 SRAM3 MPCBB security configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr15(&self) -> &GTZC1_MPCBB3_SECCFGR15 {
        &self.gtzc1_mpcbb3_seccfgr15
    }
    #[doc = "0x140 - GTZC1 SRAM3 MPCBB security configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr16(&self) -> &GTZC1_MPCBB3_SECCFGR16 {
        &self.gtzc1_mpcbb3_seccfgr16
    }
    #[doc = "0x144 - GTZC1 SRAM3 MPCBB security configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr17(&self) -> &GTZC1_MPCBB3_SECCFGR17 {
        &self.gtzc1_mpcbb3_seccfgr17
    }
    #[doc = "0x148 - GTZC1 SRAM3 MPCBB security configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr18(&self) -> &GTZC1_MPCBB3_SECCFGR18 {
        &self.gtzc1_mpcbb3_seccfgr18
    }
    #[doc = "0x14c - GTZC1 SRAM3 MPCBB security configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr19(&self) -> &GTZC1_MPCBB3_SECCFGR19 {
        &self.gtzc1_mpcbb3_seccfgr19
    }
    #[doc = "0x150 - GTZC1 SRAM3 MPCBB security configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr20(&self) -> &GTZC1_MPCBB3_SECCFGR20 {
        &self.gtzc1_mpcbb3_seccfgr20
    }
    #[doc = "0x154 - GTZC1 SRAM3 MPCBB security configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr21(&self) -> &GTZC1_MPCBB3_SECCFGR21 {
        &self.gtzc1_mpcbb3_seccfgr21
    }
    #[doc = "0x158 - GTZC1 SRAM3 MPCBB security configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr22(&self) -> &GTZC1_MPCBB3_SECCFGR22 {
        &self.gtzc1_mpcbb3_seccfgr22
    }
    #[doc = "0x15c - GTZC1 SRAM3 MPCBB security configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr23(&self) -> &GTZC1_MPCBB3_SECCFGR23 {
        &self.gtzc1_mpcbb3_seccfgr23
    }
    #[doc = "0x160 - GTZC1 SRAM3 MPCBB security configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr24(&self) -> &GTZC1_MPCBB3_SECCFGR24 {
        &self.gtzc1_mpcbb3_seccfgr24
    }
    #[doc = "0x164 - GTZC1 SRAM3 MPCBB security configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr25(&self) -> &GTZC1_MPCBB3_SECCFGR25 {
        &self.gtzc1_mpcbb3_seccfgr25
    }
    #[doc = "0x168 - GTZC1 SRAM3 MPCBB security configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr26(&self) -> &GTZC1_MPCBB3_SECCFGR26 {
        &self.gtzc1_mpcbb3_seccfgr26
    }
    #[doc = "0x16c - GTZC1 SRAM3 MPCBB security configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr27(&self) -> &GTZC1_MPCBB3_SECCFGR27 {
        &self.gtzc1_mpcbb3_seccfgr27
    }
    #[doc = "0x170 - GTZC1 SRAM3 MPCBB security configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr28(&self) -> &GTZC1_MPCBB3_SECCFGR28 {
        &self.gtzc1_mpcbb3_seccfgr28
    }
    #[doc = "0x174 - GTZC1 SRAM3 MPCBB security configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr29(&self) -> &GTZC1_MPCBB3_SECCFGR29 {
        &self.gtzc1_mpcbb3_seccfgr29
    }
    #[doc = "0x178 - GTZC1 SRAM3 MPCBB security configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr30(&self) -> &GTZC1_MPCBB3_SECCFGR30 {
        &self.gtzc1_mpcbb3_seccfgr30
    }
    #[doc = "0x17c - GTZC1 SRAM3 MPCBB security configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_seccfgr31(&self) -> &GTZC1_MPCBB3_SECCFGR31 {
        &self.gtzc1_mpcbb3_seccfgr31
    }
    #[doc = "0x200 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr0(&self) -> &GTZC1_MPCBB3_PRIVCFGR0 {
        &self.gtzc1_mpcbb3_privcfgr0
    }
    #[doc = "0x204 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr1(&self) -> &GTZC1_MPCBB3_PRIVCFGR1 {
        &self.gtzc1_mpcbb3_privcfgr1
    }
    #[doc = "0x208 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr2(&self) -> &GTZC1_MPCBB3_PRIVCFGR2 {
        &self.gtzc1_mpcbb3_privcfgr2
    }
    #[doc = "0x20c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr3(&self) -> &GTZC1_MPCBB3_PRIVCFGR3 {
        &self.gtzc1_mpcbb3_privcfgr3
    }
    #[doc = "0x210 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr4(&self) -> &GTZC1_MPCBB3_PRIVCFGR4 {
        &self.gtzc1_mpcbb3_privcfgr4
    }
    #[doc = "0x214 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr5(&self) -> &GTZC1_MPCBB3_PRIVCFGR5 {
        &self.gtzc1_mpcbb3_privcfgr5
    }
    #[doc = "0x218 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr6(&self) -> &GTZC1_MPCBB3_PRIVCFGR6 {
        &self.gtzc1_mpcbb3_privcfgr6
    }
    #[doc = "0x21c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr7(&self) -> &GTZC1_MPCBB3_PRIVCFGR7 {
        &self.gtzc1_mpcbb3_privcfgr7
    }
    #[doc = "0x220 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr8(&self) -> &GTZC1_MPCBB3_PRIVCFGR8 {
        &self.gtzc1_mpcbb3_privcfgr8
    }
    #[doc = "0x224 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr9(&self) -> &GTZC1_MPCBB3_PRIVCFGR9 {
        &self.gtzc1_mpcbb3_privcfgr9
    }
    #[doc = "0x228 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr10(&self) -> &GTZC1_MPCBB3_PRIVCFGR10 {
        &self.gtzc1_mpcbb3_privcfgr10
    }
    #[doc = "0x22c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr11(&self) -> &GTZC1_MPCBB3_PRIVCFGR11 {
        &self.gtzc1_mpcbb3_privcfgr11
    }
    #[doc = "0x230 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr12(&self) -> &GTZC1_MPCBB3_PRIVCFGR12 {
        &self.gtzc1_mpcbb3_privcfgr12
    }
    #[doc = "0x234 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr13(&self) -> &GTZC1_MPCBB3_PRIVCFGR13 {
        &self.gtzc1_mpcbb3_privcfgr13
    }
    #[doc = "0x238 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr14(&self) -> &GTZC1_MPCBB3_PRIVCFGR14 {
        &self.gtzc1_mpcbb3_privcfgr14
    }
    #[doc = "0x23c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr15(&self) -> &GTZC1_MPCBB3_PRIVCFGR15 {
        &self.gtzc1_mpcbb3_privcfgr15
    }
    #[doc = "0x240 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr16(&self) -> &GTZC1_MPCBB3_PRIVCFGR16 {
        &self.gtzc1_mpcbb3_privcfgr16
    }
    #[doc = "0x244 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr17(&self) -> &GTZC1_MPCBB3_PRIVCFGR17 {
        &self.gtzc1_mpcbb3_privcfgr17
    }
    #[doc = "0x248 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr18(&self) -> &GTZC1_MPCBB3_PRIVCFGR18 {
        &self.gtzc1_mpcbb3_privcfgr18
    }
    #[doc = "0x24c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr19(&self) -> &GTZC1_MPCBB3_PRIVCFGR19 {
        &self.gtzc1_mpcbb3_privcfgr19
    }
    #[doc = "0x250 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr20(&self) -> &GTZC1_MPCBB3_PRIVCFGR20 {
        &self.gtzc1_mpcbb3_privcfgr20
    }
    #[doc = "0x254 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr21(&self) -> &GTZC1_MPCBB3_PRIVCFGR21 {
        &self.gtzc1_mpcbb3_privcfgr21
    }
    #[doc = "0x258 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr22(&self) -> &GTZC1_MPCBB3_PRIVCFGR22 {
        &self.gtzc1_mpcbb3_privcfgr22
    }
    #[doc = "0x25c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr23(&self) -> &GTZC1_MPCBB3_PRIVCFGR23 {
        &self.gtzc1_mpcbb3_privcfgr23
    }
    #[doc = "0x260 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr24(&self) -> &GTZC1_MPCBB3_PRIVCFGR24 {
        &self.gtzc1_mpcbb3_privcfgr24
    }
    #[doc = "0x264 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr25(&self) -> &GTZC1_MPCBB3_PRIVCFGR25 {
        &self.gtzc1_mpcbb3_privcfgr25
    }
    #[doc = "0x268 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr26(&self) -> &GTZC1_MPCBB3_PRIVCFGR26 {
        &self.gtzc1_mpcbb3_privcfgr26
    }
    #[doc = "0x26c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr27(&self) -> &GTZC1_MPCBB3_PRIVCFGR27 {
        &self.gtzc1_mpcbb3_privcfgr27
    }
    #[doc = "0x270 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr28(&self) -> &GTZC1_MPCBB3_PRIVCFGR28 {
        &self.gtzc1_mpcbb3_privcfgr28
    }
    #[doc = "0x274 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr29(&self) -> &GTZC1_MPCBB3_PRIVCFGR29 {
        &self.gtzc1_mpcbb3_privcfgr29
    }
    #[doc = "0x278 - GTZC1 SRAM3 MPCBB privileged configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr30(&self) -> &GTZC1_MPCBB3_PRIVCFGR30 {
        &self.gtzc1_mpcbb3_privcfgr30
    }
    #[doc = "0x27c - GTZC1 SRAM3 MPCBB privileged configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb3_privcfgr31(&self) -> &GTZC1_MPCBB3_PRIVCFGR31 {
        &self.gtzc1_mpcbb3_privcfgr31
    }
}
#[doc = "GTZC1_MPCBB3_CR (rw) register accessor: GTZC1 SRAM3 MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_cr`]
module"]
pub type GTZC1_MPCBB3_CR = crate::Reg<gtzc1_mpcbb3_cr::GTZC1_MPCBB3_CRrs>;
#[doc = "GTZC1 SRAM3 MPCBB control register"]
pub mod gtzc1_mpcbb3_cr;
#[doc = "GTZC1_MPCBB3_CFGLOCK1 (rw) register accessor: GTZC1 SRAM3 MPCBB configuration lock register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_cfglock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_cfglock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_cfglock1`]
module"]
pub type GTZC1_MPCBB3_CFGLOCK1 = crate::Reg<gtzc1_mpcbb3_cfglock1::GTZC1_MPCBB3_CFGLOCK1rs>;
#[doc = "GTZC1 SRAM3 MPCBB configuration lock register 1"]
pub mod gtzc1_mpcbb3_cfglock1;
#[doc = "GTZC1_MPCBB3_SECCFGR0 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr0`]
module"]
pub type GTZC1_MPCBB3_SECCFGR0 = crate::Reg<gtzc1_mpcbb3_seccfgr0::GTZC1_MPCBB3_SECCFGR0rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 0 register"]
pub mod gtzc1_mpcbb3_seccfgr0;
#[doc = "GTZC1_MPCBB3_SECCFGR1 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr1`]
module"]
pub type GTZC1_MPCBB3_SECCFGR1 = crate::Reg<gtzc1_mpcbb3_seccfgr1::GTZC1_MPCBB3_SECCFGR1rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 1 register"]
pub mod gtzc1_mpcbb3_seccfgr1;
#[doc = "GTZC1_MPCBB3_SECCFGR2 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr2`]
module"]
pub type GTZC1_MPCBB3_SECCFGR2 = crate::Reg<gtzc1_mpcbb3_seccfgr2::GTZC1_MPCBB3_SECCFGR2rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 2 register"]
pub mod gtzc1_mpcbb3_seccfgr2;
#[doc = "GTZC1_MPCBB3_SECCFGR3 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr3`]
module"]
pub type GTZC1_MPCBB3_SECCFGR3 = crate::Reg<gtzc1_mpcbb3_seccfgr3::GTZC1_MPCBB3_SECCFGR3rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 3 register"]
pub mod gtzc1_mpcbb3_seccfgr3;
#[doc = "GTZC1_MPCBB3_SECCFGR4 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr4`]
module"]
pub type GTZC1_MPCBB3_SECCFGR4 = crate::Reg<gtzc1_mpcbb3_seccfgr4::GTZC1_MPCBB3_SECCFGR4rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 4 register"]
pub mod gtzc1_mpcbb3_seccfgr4;
#[doc = "GTZC1_MPCBB3_SECCFGR5 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr5`]
module"]
pub type GTZC1_MPCBB3_SECCFGR5 = crate::Reg<gtzc1_mpcbb3_seccfgr5::GTZC1_MPCBB3_SECCFGR5rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 5 register"]
pub mod gtzc1_mpcbb3_seccfgr5;
#[doc = "GTZC1_MPCBB3_SECCFGR6 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr6`]
module"]
pub type GTZC1_MPCBB3_SECCFGR6 = crate::Reg<gtzc1_mpcbb3_seccfgr6::GTZC1_MPCBB3_SECCFGR6rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 6 register"]
pub mod gtzc1_mpcbb3_seccfgr6;
#[doc = "GTZC1_MPCBB3_SECCFGR7 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr7`]
module"]
pub type GTZC1_MPCBB3_SECCFGR7 = crate::Reg<gtzc1_mpcbb3_seccfgr7::GTZC1_MPCBB3_SECCFGR7rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 7 register"]
pub mod gtzc1_mpcbb3_seccfgr7;
#[doc = "GTZC1_MPCBB3_SECCFGR8 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr8`]
module"]
pub type GTZC1_MPCBB3_SECCFGR8 = crate::Reg<gtzc1_mpcbb3_seccfgr8::GTZC1_MPCBB3_SECCFGR8rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 8 register"]
pub mod gtzc1_mpcbb3_seccfgr8;
#[doc = "GTZC1_MPCBB3_SECCFGR9 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 9 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr9`]
module"]
pub type GTZC1_MPCBB3_SECCFGR9 = crate::Reg<gtzc1_mpcbb3_seccfgr9::GTZC1_MPCBB3_SECCFGR9rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 9 register"]
pub mod gtzc1_mpcbb3_seccfgr9;
#[doc = "GTZC1_MPCBB3_SECCFGR10 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr10`]
module"]
pub type GTZC1_MPCBB3_SECCFGR10 = crate::Reg<gtzc1_mpcbb3_seccfgr10::GTZC1_MPCBB3_SECCFGR10rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 10 register"]
pub mod gtzc1_mpcbb3_seccfgr10;
#[doc = "GTZC1_MPCBB3_SECCFGR11 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 11 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr11`]
module"]
pub type GTZC1_MPCBB3_SECCFGR11 = crate::Reg<gtzc1_mpcbb3_seccfgr11::GTZC1_MPCBB3_SECCFGR11rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 11 register"]
pub mod gtzc1_mpcbb3_seccfgr11;
#[doc = "GTZC1_MPCBB3_SECCFGR12 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr12`]
module"]
pub type GTZC1_MPCBB3_SECCFGR12 = crate::Reg<gtzc1_mpcbb3_seccfgr12::GTZC1_MPCBB3_SECCFGR12rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 12 register"]
pub mod gtzc1_mpcbb3_seccfgr12;
#[doc = "GTZC1_MPCBB3_SECCFGR13 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr13`]
module"]
pub type GTZC1_MPCBB3_SECCFGR13 = crate::Reg<gtzc1_mpcbb3_seccfgr13::GTZC1_MPCBB3_SECCFGR13rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 13 register"]
pub mod gtzc1_mpcbb3_seccfgr13;
#[doc = "GTZC1_MPCBB3_SECCFGR14 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 14 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr14`]
module"]
pub type GTZC1_MPCBB3_SECCFGR14 = crate::Reg<gtzc1_mpcbb3_seccfgr14::GTZC1_MPCBB3_SECCFGR14rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 14 register"]
pub mod gtzc1_mpcbb3_seccfgr14;
#[doc = "GTZC1_MPCBB3_SECCFGR15 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 15 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr15`]
module"]
pub type GTZC1_MPCBB3_SECCFGR15 = crate::Reg<gtzc1_mpcbb3_seccfgr15::GTZC1_MPCBB3_SECCFGR15rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 15 register"]
pub mod gtzc1_mpcbb3_seccfgr15;
#[doc = "GTZC1_MPCBB3_SECCFGR16 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 16 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr16`]
module"]
pub type GTZC1_MPCBB3_SECCFGR16 = crate::Reg<gtzc1_mpcbb3_seccfgr16::GTZC1_MPCBB3_SECCFGR16rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 16 register"]
pub mod gtzc1_mpcbb3_seccfgr16;
#[doc = "GTZC1_MPCBB3_SECCFGR17 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 17 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr17`]
module"]
pub type GTZC1_MPCBB3_SECCFGR17 = crate::Reg<gtzc1_mpcbb3_seccfgr17::GTZC1_MPCBB3_SECCFGR17rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 17 register"]
pub mod gtzc1_mpcbb3_seccfgr17;
#[doc = "GTZC1_MPCBB3_SECCFGR18 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 18 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr18`]
module"]
pub type GTZC1_MPCBB3_SECCFGR18 = crate::Reg<gtzc1_mpcbb3_seccfgr18::GTZC1_MPCBB3_SECCFGR18rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 18 register"]
pub mod gtzc1_mpcbb3_seccfgr18;
#[doc = "GTZC1_MPCBB3_SECCFGR19 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 19 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr19`]
module"]
pub type GTZC1_MPCBB3_SECCFGR19 = crate::Reg<gtzc1_mpcbb3_seccfgr19::GTZC1_MPCBB3_SECCFGR19rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 19 register"]
pub mod gtzc1_mpcbb3_seccfgr19;
#[doc = "GTZC1_MPCBB3_SECCFGR20 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 20 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr20`]
module"]
pub type GTZC1_MPCBB3_SECCFGR20 = crate::Reg<gtzc1_mpcbb3_seccfgr20::GTZC1_MPCBB3_SECCFGR20rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 20 register"]
pub mod gtzc1_mpcbb3_seccfgr20;
#[doc = "GTZC1_MPCBB3_SECCFGR21 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 21 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr21`]
module"]
pub type GTZC1_MPCBB3_SECCFGR21 = crate::Reg<gtzc1_mpcbb3_seccfgr21::GTZC1_MPCBB3_SECCFGR21rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 21 register"]
pub mod gtzc1_mpcbb3_seccfgr21;
#[doc = "GTZC1_MPCBB3_SECCFGR22 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr22`]
module"]
pub type GTZC1_MPCBB3_SECCFGR22 = crate::Reg<gtzc1_mpcbb3_seccfgr22::GTZC1_MPCBB3_SECCFGR22rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 22 register"]
pub mod gtzc1_mpcbb3_seccfgr22;
#[doc = "GTZC1_MPCBB3_SECCFGR23 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 23 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr23`]
module"]
pub type GTZC1_MPCBB3_SECCFGR23 = crate::Reg<gtzc1_mpcbb3_seccfgr23::GTZC1_MPCBB3_SECCFGR23rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 23 register"]
pub mod gtzc1_mpcbb3_seccfgr23;
#[doc = "GTZC1_MPCBB3_SECCFGR24 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 24 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr24`]
module"]
pub type GTZC1_MPCBB3_SECCFGR24 = crate::Reg<gtzc1_mpcbb3_seccfgr24::GTZC1_MPCBB3_SECCFGR24rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 24 register"]
pub mod gtzc1_mpcbb3_seccfgr24;
#[doc = "GTZC1_MPCBB3_SECCFGR25 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 25 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr25`]
module"]
pub type GTZC1_MPCBB3_SECCFGR25 = crate::Reg<gtzc1_mpcbb3_seccfgr25::GTZC1_MPCBB3_SECCFGR25rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 25 register"]
pub mod gtzc1_mpcbb3_seccfgr25;
#[doc = "GTZC1_MPCBB3_SECCFGR26 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 26 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr26`]
module"]
pub type GTZC1_MPCBB3_SECCFGR26 = crate::Reg<gtzc1_mpcbb3_seccfgr26::GTZC1_MPCBB3_SECCFGR26rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 26 register"]
pub mod gtzc1_mpcbb3_seccfgr26;
#[doc = "GTZC1_MPCBB3_SECCFGR27 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 27 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr27`]
module"]
pub type GTZC1_MPCBB3_SECCFGR27 = crate::Reg<gtzc1_mpcbb3_seccfgr27::GTZC1_MPCBB3_SECCFGR27rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 27 register"]
pub mod gtzc1_mpcbb3_seccfgr27;
#[doc = "GTZC1_MPCBB3_SECCFGR28 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 28 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr28`]
module"]
pub type GTZC1_MPCBB3_SECCFGR28 = crate::Reg<gtzc1_mpcbb3_seccfgr28::GTZC1_MPCBB3_SECCFGR28rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 28 register"]
pub mod gtzc1_mpcbb3_seccfgr28;
#[doc = "GTZC1_MPCBB3_SECCFGR29 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 29 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr29`]
module"]
pub type GTZC1_MPCBB3_SECCFGR29 = crate::Reg<gtzc1_mpcbb3_seccfgr29::GTZC1_MPCBB3_SECCFGR29rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 29 register"]
pub mod gtzc1_mpcbb3_seccfgr29;
#[doc = "GTZC1_MPCBB3_SECCFGR30 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 30 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr30`]
module"]
pub type GTZC1_MPCBB3_SECCFGR30 = crate::Reg<gtzc1_mpcbb3_seccfgr30::GTZC1_MPCBB3_SECCFGR30rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 30 register"]
pub mod gtzc1_mpcbb3_seccfgr30;
#[doc = "GTZC1_MPCBB3_SECCFGR31 (rw) register accessor: GTZC1 SRAM3 MPCBB security configuration for super-block 31 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_seccfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_seccfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_seccfgr31`]
module"]
pub type GTZC1_MPCBB3_SECCFGR31 = crate::Reg<gtzc1_mpcbb3_seccfgr31::GTZC1_MPCBB3_SECCFGR31rs>;
#[doc = "GTZC1 SRAM3 MPCBB security configuration for super-block 31 register"]
pub mod gtzc1_mpcbb3_seccfgr31;
#[doc = "GTZC1_MPCBB3_PRIVCFGR0 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr0`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb3_privcfgr0::GTZC1_MPCBB3_PRIVCFGR0rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 0 register"]
pub mod gtzc1_mpcbb3_privcfgr0;
#[doc = "GTZC1_MPCBB3_PRIVCFGR1 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr1`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb3_privcfgr1::GTZC1_MPCBB3_PRIVCFGR1rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 1 register"]
pub mod gtzc1_mpcbb3_privcfgr1;
#[doc = "GTZC1_MPCBB3_PRIVCFGR2 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr2`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb3_privcfgr2::GTZC1_MPCBB3_PRIVCFGR2rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 2 register"]
pub mod gtzc1_mpcbb3_privcfgr2;
#[doc = "GTZC1_MPCBB3_PRIVCFGR3 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr3`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb3_privcfgr3::GTZC1_MPCBB3_PRIVCFGR3rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 3 register"]
pub mod gtzc1_mpcbb3_privcfgr3;
#[doc = "GTZC1_MPCBB3_PRIVCFGR4 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr4`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb3_privcfgr4::GTZC1_MPCBB3_PRIVCFGR4rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 4 register"]
pub mod gtzc1_mpcbb3_privcfgr4;
#[doc = "GTZC1_MPCBB3_PRIVCFGR5 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr5`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb3_privcfgr5::GTZC1_MPCBB3_PRIVCFGR5rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 5 register"]
pub mod gtzc1_mpcbb3_privcfgr5;
#[doc = "GTZC1_MPCBB3_PRIVCFGR6 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr6`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb3_privcfgr6::GTZC1_MPCBB3_PRIVCFGR6rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 6 register"]
pub mod gtzc1_mpcbb3_privcfgr6;
#[doc = "GTZC1_MPCBB3_PRIVCFGR7 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr7`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb3_privcfgr7::GTZC1_MPCBB3_PRIVCFGR7rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 7 register"]
pub mod gtzc1_mpcbb3_privcfgr7;
#[doc = "GTZC1_MPCBB3_PRIVCFGR8 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr8`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb3_privcfgr8::GTZC1_MPCBB3_PRIVCFGR8rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 8 register"]
pub mod gtzc1_mpcbb3_privcfgr8;
#[doc = "GTZC1_MPCBB3_PRIVCFGR9 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr9`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb3_privcfgr9::GTZC1_MPCBB3_PRIVCFGR9rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 9 register"]
pub mod gtzc1_mpcbb3_privcfgr9;
#[doc = "GTZC1_MPCBB3_PRIVCFGR10 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr10`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR10 = crate::Reg<gtzc1_mpcbb3_privcfgr10::GTZC1_MPCBB3_PRIVCFGR10rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 10 register"]
pub mod gtzc1_mpcbb3_privcfgr10;
#[doc = "GTZC1_MPCBB3_PRIVCFGR11 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 11 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr11`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR11 = crate::Reg<gtzc1_mpcbb3_privcfgr11::GTZC1_MPCBB3_PRIVCFGR11rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 11 register"]
pub mod gtzc1_mpcbb3_privcfgr11;
#[doc = "GTZC1_MPCBB3_PRIVCFGR12 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr12`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR12 = crate::Reg<gtzc1_mpcbb3_privcfgr12::GTZC1_MPCBB3_PRIVCFGR12rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 12 register"]
pub mod gtzc1_mpcbb3_privcfgr12;
#[doc = "GTZC1_MPCBB3_PRIVCFGR13 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr13`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR13 = crate::Reg<gtzc1_mpcbb3_privcfgr13::GTZC1_MPCBB3_PRIVCFGR13rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 13 register"]
pub mod gtzc1_mpcbb3_privcfgr13;
#[doc = "GTZC1_MPCBB3_PRIVCFGR14 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 14 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr14`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR14 = crate::Reg<gtzc1_mpcbb3_privcfgr14::GTZC1_MPCBB3_PRIVCFGR14rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 14 register"]
pub mod gtzc1_mpcbb3_privcfgr14;
#[doc = "GTZC1_MPCBB3_PRIVCFGR15 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 15 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr15`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR15 = crate::Reg<gtzc1_mpcbb3_privcfgr15::GTZC1_MPCBB3_PRIVCFGR15rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 15 register"]
pub mod gtzc1_mpcbb3_privcfgr15;
#[doc = "GTZC1_MPCBB3_PRIVCFGR16 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 16 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr16`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR16 = crate::Reg<gtzc1_mpcbb3_privcfgr16::GTZC1_MPCBB3_PRIVCFGR16rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 16 register"]
pub mod gtzc1_mpcbb3_privcfgr16;
#[doc = "GTZC1_MPCBB3_PRIVCFGR17 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 17 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr17`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR17 = crate::Reg<gtzc1_mpcbb3_privcfgr17::GTZC1_MPCBB3_PRIVCFGR17rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 17 register"]
pub mod gtzc1_mpcbb3_privcfgr17;
#[doc = "GTZC1_MPCBB3_PRIVCFGR18 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 18 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr18`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR18 = crate::Reg<gtzc1_mpcbb3_privcfgr18::GTZC1_MPCBB3_PRIVCFGR18rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 18 register"]
pub mod gtzc1_mpcbb3_privcfgr18;
#[doc = "GTZC1_MPCBB3_PRIVCFGR19 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 19 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr19`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR19 = crate::Reg<gtzc1_mpcbb3_privcfgr19::GTZC1_MPCBB3_PRIVCFGR19rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 19 register"]
pub mod gtzc1_mpcbb3_privcfgr19;
#[doc = "GTZC1_MPCBB3_PRIVCFGR20 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 20 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr20`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR20 = crate::Reg<gtzc1_mpcbb3_privcfgr20::GTZC1_MPCBB3_PRIVCFGR20rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 20 register"]
pub mod gtzc1_mpcbb3_privcfgr20;
#[doc = "GTZC1_MPCBB3_PRIVCFGR21 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 21 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr21`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR21 = crate::Reg<gtzc1_mpcbb3_privcfgr21::GTZC1_MPCBB3_PRIVCFGR21rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 21 register"]
pub mod gtzc1_mpcbb3_privcfgr21;
#[doc = "GTZC1_MPCBB3_PRIVCFGR22 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr22`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR22 = crate::Reg<gtzc1_mpcbb3_privcfgr22::GTZC1_MPCBB3_PRIVCFGR22rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 22 register"]
pub mod gtzc1_mpcbb3_privcfgr22;
#[doc = "GTZC1_MPCBB3_PRIVCFGR23 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 23 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr23`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR23 = crate::Reg<gtzc1_mpcbb3_privcfgr23::GTZC1_MPCBB3_PRIVCFGR23rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 23 register"]
pub mod gtzc1_mpcbb3_privcfgr23;
#[doc = "GTZC1_MPCBB3_PRIVCFGR24 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 24 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr24`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR24 = crate::Reg<gtzc1_mpcbb3_privcfgr24::GTZC1_MPCBB3_PRIVCFGR24rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 24 register"]
pub mod gtzc1_mpcbb3_privcfgr24;
#[doc = "GTZC1_MPCBB3_PRIVCFGR25 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 25 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr25`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR25 = crate::Reg<gtzc1_mpcbb3_privcfgr25::GTZC1_MPCBB3_PRIVCFGR25rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 25 register"]
pub mod gtzc1_mpcbb3_privcfgr25;
#[doc = "GTZC1_MPCBB3_PRIVCFGR26 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 26 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr26`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR26 = crate::Reg<gtzc1_mpcbb3_privcfgr26::GTZC1_MPCBB3_PRIVCFGR26rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 26 register"]
pub mod gtzc1_mpcbb3_privcfgr26;
#[doc = "GTZC1_MPCBB3_PRIVCFGR27 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 27 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr27`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR27 = crate::Reg<gtzc1_mpcbb3_privcfgr27::GTZC1_MPCBB3_PRIVCFGR27rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 27 register"]
pub mod gtzc1_mpcbb3_privcfgr27;
#[doc = "GTZC1_MPCBB3_PRIVCFGR28 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 28 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr28`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR28 = crate::Reg<gtzc1_mpcbb3_privcfgr28::GTZC1_MPCBB3_PRIVCFGR28rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 28 register"]
pub mod gtzc1_mpcbb3_privcfgr28;
#[doc = "GTZC1_MPCBB3_PRIVCFGR29 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 29 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr29`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR29 = crate::Reg<gtzc1_mpcbb3_privcfgr29::GTZC1_MPCBB3_PRIVCFGR29rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 29 register"]
pub mod gtzc1_mpcbb3_privcfgr29;
#[doc = "GTZC1_MPCBB3_PRIVCFGR30 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 30 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr30`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR30 = crate::Reg<gtzc1_mpcbb3_privcfgr30::GTZC1_MPCBB3_PRIVCFGR30rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 30 register"]
pub mod gtzc1_mpcbb3_privcfgr30;
#[doc = "GTZC1_MPCBB3_PRIVCFGR31 (rw) register accessor: GTZC1 SRAM3 MPCBB privileged configuration for super-block 31 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb3_privcfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb3_privcfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb3_privcfgr31`]
module"]
pub type GTZC1_MPCBB3_PRIVCFGR31 = crate::Reg<gtzc1_mpcbb3_privcfgr31::GTZC1_MPCBB3_PRIVCFGR31rs>;
#[doc = "GTZC1 SRAM3 MPCBB privileged configuration for super-block 31 register"]
pub mod gtzc1_mpcbb3_privcfgr31;
