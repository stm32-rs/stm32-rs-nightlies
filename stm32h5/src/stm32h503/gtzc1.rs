#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    gtzc1_tzsc_privcfgr1: GTZC1_TZSC_PRIVCFGR1,
    gtzc1_tzsc_privcfgr2: GTZC1_TZSC_PRIVCFGR2,
    gtzc1_tzsc_privcfgr3: GTZC1_TZSC_PRIVCFGR3,
    _reserved3: [u8; 0x44],
    gtzc1_tzsc_mpcwm4acfgr: GTZC1_TZSC_MPCWM4ACFGR,
    gtzc1_tzsc_mpcwm4ar: GTZC1_TZSC_MPCWM4AR,
    gtzc1_tzsc_mpcwm4bcfgr: GTZC1_TZSC_MPCWM4BCFGR,
    gtzc1_tzsc_mpcwm4br: GTZC1_TZSC_MPCWM4BR,
    _reserved7: [u8; 0x0180],
    gtzc1_mpcbb1_privcfgr0: GTZC1_MPCBB1_PRIVCFGR0,
    gtzc1_mpcbb1_privcfgr1: GTZC1_MPCBB1_PRIVCFGR1,
    gtzc1_mpcbb1_privcfgr2: GTZC1_MPCBB1_PRIVCFGR2,
    gtzc1_mpcbb1_privcfgr3: GTZC1_MPCBB1_PRIVCFGR3,
    gtzc1_mpcbb1_privcfgr4: GTZC1_MPCBB1_PRIVCFGR4,
    gtzc1_mpcbb1_privcfgr5: GTZC1_MPCBB1_PRIVCFGR5,
    gtzc1_mpcbb1_privcfgr6: GTZC1_MPCBB1_PRIVCFGR6,
    gtzc1_mpcbb1_privcfgr7: GTZC1_MPCBB1_PRIVCFGR7,
    gtzc1_mpcbb1_privcfgr8: GTZC1_MPCBB1_PRIVCFGR8,
    gtzc1_mpcbb1_privcfgr9: GTZC1_MPCBB1_PRIVCFGR9,
    gtzc1_mpcbb1_privcfgr10: GTZC1_MPCBB1_PRIVCFGR10,
    gtzc1_mpcbb1_privcfgr11: GTZC1_MPCBB1_PRIVCFGR11,
    gtzc1_mpcbb1_privcfgr12: GTZC1_MPCBB1_PRIVCFGR12,
    gtzc1_mpcbb1_privcfgr13: GTZC1_MPCBB1_PRIVCFGR13,
    gtzc1_mpcbb1_privcfgr14: GTZC1_MPCBB1_PRIVCFGR14,
    gtzc1_mpcbb1_privcfgr15: GTZC1_MPCBB1_PRIVCFGR15,
    gtzc1_mpcbb1_privcfgr16: GTZC1_MPCBB1_PRIVCFGR16,
    gtzc1_mpcbb1_privcfgr17: GTZC1_MPCBB1_PRIVCFGR17,
    gtzc1_mpcbb1_privcfgr18: GTZC1_MPCBB1_PRIVCFGR18,
    gtzc1_mpcbb1_privcfgr19: GTZC1_MPCBB1_PRIVCFGR19,
    gtzc1_mpcbb1_privcfgr20: GTZC1_MPCBB1_PRIVCFGR20,
    gtzc1_mpcbb1_privcfgr21: GTZC1_MPCBB1_PRIVCFGR21,
    gtzc1_mpcbb1_privcfgr22: GTZC1_MPCBB1_PRIVCFGR22,
    gtzc1_mpcbb1_privcfgr23: GTZC1_MPCBB1_PRIVCFGR23,
    gtzc1_mpcbb1_privcfgr24: GTZC1_MPCBB1_PRIVCFGR24,
    gtzc1_mpcbb1_privcfgr25: GTZC1_MPCBB1_PRIVCFGR25,
    gtzc1_mpcbb1_privcfgr26: GTZC1_MPCBB1_PRIVCFGR26,
    gtzc1_mpcbb1_privcfgr27: GTZC1_MPCBB1_PRIVCFGR27,
    gtzc1_mpcbb1_privcfgr28: GTZC1_MPCBB1_PRIVCFGR28,
    gtzc1_mpcbb1_privcfgr29: GTZC1_MPCBB1_PRIVCFGR29,
    gtzc1_mpcbb1_privcfgr30: GTZC1_MPCBB1_PRIVCFGR30,
    gtzc1_mpcbb1_privcfgr31: GTZC1_MPCBB1_PRIVCFGR31,
    _reserved39: [u8; 0x0380],
    gtzc1_mpcbb2_privcfgr0: GTZC1_MPCBB2_PRIVCFGR0,
    gtzc1_mpcbb2_privcfgr1: GTZC1_MPCBB2_PRIVCFGR1,
    gtzc1_mpcbb2_privcfgr2: GTZC1_MPCBB2_PRIVCFGR2,
    gtzc1_mpcbb2_privcfgr3: GTZC1_MPCBB2_PRIVCFGR3,
    gtzc1_mpcbb2_privcfgr4: GTZC1_MPCBB2_PRIVCFGR4,
    gtzc1_mpcbb2_privcfgr5: GTZC1_MPCBB2_PRIVCFGR5,
    gtzc1_mpcbb2_privcfgr6: GTZC1_MPCBB2_PRIVCFGR6,
    gtzc1_mpcbb2_privcfgr7: GTZC1_MPCBB2_PRIVCFGR7,
    gtzc1_mpcbb2_privcfgr8: GTZC1_MPCBB2_PRIVCFGR8,
    gtzc1_mpcbb2_privcfgr9: GTZC1_MPCBB2_PRIVCFGR9,
    gtzc1_mpcbb2_privcfgr10: GTZC1_MPCBB2_PRIVCFGR10,
    gtzc1_mpcbb2_privcfgr11: GTZC1_MPCBB2_PRIVCFGR11,
    gtzc1_mpcbb2_privcfgr12: GTZC1_MPCBB2_PRIVCFGR12,
    gtzc1_mpcbb2_privcfgr13: GTZC1_MPCBB2_PRIVCFGR13,
    gtzc1_mpcbb2_privcfgr14: GTZC1_MPCBB2_PRIVCFGR14,
    gtzc1_mpcbb2_privcfgr15: GTZC1_MPCBB2_PRIVCFGR15,
    gtzc1_mpcbb2_privcfgr16: GTZC1_MPCBB2_PRIVCFGR16,
    gtzc1_mpcbb2_privcfgr17: GTZC1_MPCBB2_PRIVCFGR17,
    gtzc1_mpcbb2_privcfgr18: GTZC1_MPCBB2_PRIVCFGR18,
    gtzc1_mpcbb2_privcfgr19: GTZC1_MPCBB2_PRIVCFGR19,
    gtzc1_mpcbb2_privcfgr20: GTZC1_MPCBB2_PRIVCFGR20,
    gtzc1_mpcbb2_privcfgr21: GTZC1_MPCBB2_PRIVCFGR21,
    gtzc1_mpcbb2_privcfgr22: GTZC1_MPCBB2_PRIVCFGR22,
    gtzc1_mpcbb2_privcfgr23: GTZC1_MPCBB2_PRIVCFGR23,
    gtzc1_mpcbb2_privcfgr24: GTZC1_MPCBB2_PRIVCFGR24,
    gtzc1_mpcbb2_privcfgr25: GTZC1_MPCBB2_PRIVCFGR25,
    gtzc1_mpcbb2_privcfgr26: GTZC1_MPCBB2_PRIVCFGR26,
    gtzc1_mpcbb2_privcfgr27: GTZC1_MPCBB2_PRIVCFGR27,
    gtzc1_mpcbb2_privcfgr28: GTZC1_MPCBB2_PRIVCFGR28,
    gtzc1_mpcbb2_privcfgr29: GTZC1_MPCBB2_PRIVCFGR29,
    gtzc1_mpcbb2_privcfgr30: GTZC1_MPCBB2_PRIVCFGR30,
    gtzc1_mpcbb2_privcfgr31: GTZC1_MPCBB2_PRIVCFGR31,
}
impl RegisterBlock {
    #[doc = "0x20 - GTZC1 TZSC privilege configuration register 1"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_privcfgr1(&self) -> &GTZC1_TZSC_PRIVCFGR1 {
        &self.gtzc1_tzsc_privcfgr1
    }
    #[doc = "0x24 - GTZC1 TZSC privilege configuration register 2"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_privcfgr2(&self) -> &GTZC1_TZSC_PRIVCFGR2 {
        &self.gtzc1_tzsc_privcfgr2
    }
    #[doc = "0x28 - GTZC1 TZSC privilege configuration register 3"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_privcfgr3(&self) -> &GTZC1_TZSC_PRIVCFGR3 {
        &self.gtzc1_tzsc_privcfgr3
    }
    #[doc = "0x70 - GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_mpcwm4acfgr(&self) -> &GTZC1_TZSC_MPCWM4ACFGR {
        &self.gtzc1_tzsc_mpcwm4acfgr
    }
    #[doc = "0x74 - GTZC1 TZSC BKPSRAM sub-region A watermark register"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_mpcwm4ar(&self) -> &GTZC1_TZSC_MPCWM4AR {
        &self.gtzc1_tzsc_mpcwm4ar
    }
    #[doc = "0x78 - GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_mpcwm4bcfgr(&self) -> &GTZC1_TZSC_MPCWM4BCFGR {
        &self.gtzc1_tzsc_mpcwm4bcfgr
    }
    #[doc = "0x7c - GTZC1 TZSC BKPSRAM sub-region B watermark register"]
    #[inline(always)]
    pub const fn gtzc1_tzsc_mpcwm4br(&self) -> &GTZC1_TZSC_MPCWM4BR {
        &self.gtzc1_tzsc_mpcwm4br
    }
    #[doc = "0x200 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr0(&self) -> &GTZC1_MPCBB1_PRIVCFGR0 {
        &self.gtzc1_mpcbb1_privcfgr0
    }
    #[doc = "0x204 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr1(&self) -> &GTZC1_MPCBB1_PRIVCFGR1 {
        &self.gtzc1_mpcbb1_privcfgr1
    }
    #[doc = "0x208 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr2(&self) -> &GTZC1_MPCBB1_PRIVCFGR2 {
        &self.gtzc1_mpcbb1_privcfgr2
    }
    #[doc = "0x20c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr3(&self) -> &GTZC1_MPCBB1_PRIVCFGR3 {
        &self.gtzc1_mpcbb1_privcfgr3
    }
    #[doc = "0x210 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr4(&self) -> &GTZC1_MPCBB1_PRIVCFGR4 {
        &self.gtzc1_mpcbb1_privcfgr4
    }
    #[doc = "0x214 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr5(&self) -> &GTZC1_MPCBB1_PRIVCFGR5 {
        &self.gtzc1_mpcbb1_privcfgr5
    }
    #[doc = "0x218 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr6(&self) -> &GTZC1_MPCBB1_PRIVCFGR6 {
        &self.gtzc1_mpcbb1_privcfgr6
    }
    #[doc = "0x21c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr7(&self) -> &GTZC1_MPCBB1_PRIVCFGR7 {
        &self.gtzc1_mpcbb1_privcfgr7
    }
    #[doc = "0x220 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr8(&self) -> &GTZC1_MPCBB1_PRIVCFGR8 {
        &self.gtzc1_mpcbb1_privcfgr8
    }
    #[doc = "0x224 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr9(&self) -> &GTZC1_MPCBB1_PRIVCFGR9 {
        &self.gtzc1_mpcbb1_privcfgr9
    }
    #[doc = "0x228 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr10(&self) -> &GTZC1_MPCBB1_PRIVCFGR10 {
        &self.gtzc1_mpcbb1_privcfgr10
    }
    #[doc = "0x22c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr11(&self) -> &GTZC1_MPCBB1_PRIVCFGR11 {
        &self.gtzc1_mpcbb1_privcfgr11
    }
    #[doc = "0x230 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr12(&self) -> &GTZC1_MPCBB1_PRIVCFGR12 {
        &self.gtzc1_mpcbb1_privcfgr12
    }
    #[doc = "0x234 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr13(&self) -> &GTZC1_MPCBB1_PRIVCFGR13 {
        &self.gtzc1_mpcbb1_privcfgr13
    }
    #[doc = "0x238 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr14(&self) -> &GTZC1_MPCBB1_PRIVCFGR14 {
        &self.gtzc1_mpcbb1_privcfgr14
    }
    #[doc = "0x23c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr15(&self) -> &GTZC1_MPCBB1_PRIVCFGR15 {
        &self.gtzc1_mpcbb1_privcfgr15
    }
    #[doc = "0x240 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr16(&self) -> &GTZC1_MPCBB1_PRIVCFGR16 {
        &self.gtzc1_mpcbb1_privcfgr16
    }
    #[doc = "0x244 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr17(&self) -> &GTZC1_MPCBB1_PRIVCFGR17 {
        &self.gtzc1_mpcbb1_privcfgr17
    }
    #[doc = "0x248 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr18(&self) -> &GTZC1_MPCBB1_PRIVCFGR18 {
        &self.gtzc1_mpcbb1_privcfgr18
    }
    #[doc = "0x24c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr19(&self) -> &GTZC1_MPCBB1_PRIVCFGR19 {
        &self.gtzc1_mpcbb1_privcfgr19
    }
    #[doc = "0x250 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr20(&self) -> &GTZC1_MPCBB1_PRIVCFGR20 {
        &self.gtzc1_mpcbb1_privcfgr20
    }
    #[doc = "0x254 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr21(&self) -> &GTZC1_MPCBB1_PRIVCFGR21 {
        &self.gtzc1_mpcbb1_privcfgr21
    }
    #[doc = "0x258 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr22(&self) -> &GTZC1_MPCBB1_PRIVCFGR22 {
        &self.gtzc1_mpcbb1_privcfgr22
    }
    #[doc = "0x25c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr23(&self) -> &GTZC1_MPCBB1_PRIVCFGR23 {
        &self.gtzc1_mpcbb1_privcfgr23
    }
    #[doc = "0x260 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr24(&self) -> &GTZC1_MPCBB1_PRIVCFGR24 {
        &self.gtzc1_mpcbb1_privcfgr24
    }
    #[doc = "0x264 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr25(&self) -> &GTZC1_MPCBB1_PRIVCFGR25 {
        &self.gtzc1_mpcbb1_privcfgr25
    }
    #[doc = "0x268 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr26(&self) -> &GTZC1_MPCBB1_PRIVCFGR26 {
        &self.gtzc1_mpcbb1_privcfgr26
    }
    #[doc = "0x26c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr27(&self) -> &GTZC1_MPCBB1_PRIVCFGR27 {
        &self.gtzc1_mpcbb1_privcfgr27
    }
    #[doc = "0x270 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr28(&self) -> &GTZC1_MPCBB1_PRIVCFGR28 {
        &self.gtzc1_mpcbb1_privcfgr28
    }
    #[doc = "0x274 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr29(&self) -> &GTZC1_MPCBB1_PRIVCFGR29 {
        &self.gtzc1_mpcbb1_privcfgr29
    }
    #[doc = "0x278 - GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr30(&self) -> &GTZC1_MPCBB1_PRIVCFGR30 {
        &self.gtzc1_mpcbb1_privcfgr30
    }
    #[doc = "0x27c - GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb1_privcfgr31(&self) -> &GTZC1_MPCBB1_PRIVCFGR31 {
        &self.gtzc1_mpcbb1_privcfgr31
    }
    #[doc = "0x600 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr0(&self) -> &GTZC1_MPCBB2_PRIVCFGR0 {
        &self.gtzc1_mpcbb2_privcfgr0
    }
    #[doc = "0x604 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr1(&self) -> &GTZC1_MPCBB2_PRIVCFGR1 {
        &self.gtzc1_mpcbb2_privcfgr1
    }
    #[doc = "0x608 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr2(&self) -> &GTZC1_MPCBB2_PRIVCFGR2 {
        &self.gtzc1_mpcbb2_privcfgr2
    }
    #[doc = "0x60c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr3(&self) -> &GTZC1_MPCBB2_PRIVCFGR3 {
        &self.gtzc1_mpcbb2_privcfgr3
    }
    #[doc = "0x610 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr4(&self) -> &GTZC1_MPCBB2_PRIVCFGR4 {
        &self.gtzc1_mpcbb2_privcfgr4
    }
    #[doc = "0x614 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr5(&self) -> &GTZC1_MPCBB2_PRIVCFGR5 {
        &self.gtzc1_mpcbb2_privcfgr5
    }
    #[doc = "0x618 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr6(&self) -> &GTZC1_MPCBB2_PRIVCFGR6 {
        &self.gtzc1_mpcbb2_privcfgr6
    }
    #[doc = "0x61c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr7(&self) -> &GTZC1_MPCBB2_PRIVCFGR7 {
        &self.gtzc1_mpcbb2_privcfgr7
    }
    #[doc = "0x620 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr8(&self) -> &GTZC1_MPCBB2_PRIVCFGR8 {
        &self.gtzc1_mpcbb2_privcfgr8
    }
    #[doc = "0x624 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr9(&self) -> &GTZC1_MPCBB2_PRIVCFGR9 {
        &self.gtzc1_mpcbb2_privcfgr9
    }
    #[doc = "0x628 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr10(&self) -> &GTZC1_MPCBB2_PRIVCFGR10 {
        &self.gtzc1_mpcbb2_privcfgr10
    }
    #[doc = "0x62c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr11(&self) -> &GTZC1_MPCBB2_PRIVCFGR11 {
        &self.gtzc1_mpcbb2_privcfgr11
    }
    #[doc = "0x630 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr12(&self) -> &GTZC1_MPCBB2_PRIVCFGR12 {
        &self.gtzc1_mpcbb2_privcfgr12
    }
    #[doc = "0x634 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr13(&self) -> &GTZC1_MPCBB2_PRIVCFGR13 {
        &self.gtzc1_mpcbb2_privcfgr13
    }
    #[doc = "0x638 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr14(&self) -> &GTZC1_MPCBB2_PRIVCFGR14 {
        &self.gtzc1_mpcbb2_privcfgr14
    }
    #[doc = "0x63c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr15(&self) -> &GTZC1_MPCBB2_PRIVCFGR15 {
        &self.gtzc1_mpcbb2_privcfgr15
    }
    #[doc = "0x640 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr16(&self) -> &GTZC1_MPCBB2_PRIVCFGR16 {
        &self.gtzc1_mpcbb2_privcfgr16
    }
    #[doc = "0x644 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr17(&self) -> &GTZC1_MPCBB2_PRIVCFGR17 {
        &self.gtzc1_mpcbb2_privcfgr17
    }
    #[doc = "0x648 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr18(&self) -> &GTZC1_MPCBB2_PRIVCFGR18 {
        &self.gtzc1_mpcbb2_privcfgr18
    }
    #[doc = "0x64c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr19(&self) -> &GTZC1_MPCBB2_PRIVCFGR19 {
        &self.gtzc1_mpcbb2_privcfgr19
    }
    #[doc = "0x650 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr20(&self) -> &GTZC1_MPCBB2_PRIVCFGR20 {
        &self.gtzc1_mpcbb2_privcfgr20
    }
    #[doc = "0x654 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr21(&self) -> &GTZC1_MPCBB2_PRIVCFGR21 {
        &self.gtzc1_mpcbb2_privcfgr21
    }
    #[doc = "0x658 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr22(&self) -> &GTZC1_MPCBB2_PRIVCFGR22 {
        &self.gtzc1_mpcbb2_privcfgr22
    }
    #[doc = "0x65c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr23(&self) -> &GTZC1_MPCBB2_PRIVCFGR23 {
        &self.gtzc1_mpcbb2_privcfgr23
    }
    #[doc = "0x660 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr24(&self) -> &GTZC1_MPCBB2_PRIVCFGR24 {
        &self.gtzc1_mpcbb2_privcfgr24
    }
    #[doc = "0x664 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr25(&self) -> &GTZC1_MPCBB2_PRIVCFGR25 {
        &self.gtzc1_mpcbb2_privcfgr25
    }
    #[doc = "0x668 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr26(&self) -> &GTZC1_MPCBB2_PRIVCFGR26 {
        &self.gtzc1_mpcbb2_privcfgr26
    }
    #[doc = "0x66c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr27(&self) -> &GTZC1_MPCBB2_PRIVCFGR27 {
        &self.gtzc1_mpcbb2_privcfgr27
    }
    #[doc = "0x670 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr28(&self) -> &GTZC1_MPCBB2_PRIVCFGR28 {
        &self.gtzc1_mpcbb2_privcfgr28
    }
    #[doc = "0x674 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr29(&self) -> &GTZC1_MPCBB2_PRIVCFGR29 {
        &self.gtzc1_mpcbb2_privcfgr29
    }
    #[doc = "0x678 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr30(&self) -> &GTZC1_MPCBB2_PRIVCFGR30 {
        &self.gtzc1_mpcbb2_privcfgr30
    }
    #[doc = "0x67c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
    #[inline(always)]
    pub const fn gtzc1_mpcbb2_privcfgr31(&self) -> &GTZC1_MPCBB2_PRIVCFGR31 {
        &self.gtzc1_mpcbb2_privcfgr31
    }
}
#[doc = "GTZC1_TZSC_PRIVCFGR1 (rw) register accessor: GTZC1 TZSC privilege configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_privcfgr1`]
module"]
pub type GTZC1_TZSC_PRIVCFGR1 = crate::Reg<gtzc1_tzsc_privcfgr1::GTZC1_TZSC_PRIVCFGR1rs>;
#[doc = "GTZC1 TZSC privilege configuration register 1"]
pub mod gtzc1_tzsc_privcfgr1;
#[doc = "GTZC1_TZSC_PRIVCFGR2 (rw) register accessor: GTZC1 TZSC privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_privcfgr2`]
module"]
pub type GTZC1_TZSC_PRIVCFGR2 = crate::Reg<gtzc1_tzsc_privcfgr2::GTZC1_TZSC_PRIVCFGR2rs>;
#[doc = "GTZC1 TZSC privilege configuration register 2"]
pub mod gtzc1_tzsc_privcfgr2;
#[doc = "GTZC1_TZSC_PRIVCFGR3 (rw) register accessor: GTZC1 TZSC privilege configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_privcfgr3`]
module"]
pub type GTZC1_TZSC_PRIVCFGR3 = crate::Reg<gtzc1_tzsc_privcfgr3::GTZC1_TZSC_PRIVCFGR3rs>;
#[doc = "GTZC1 TZSC privilege configuration register 3"]
pub mod gtzc1_tzsc_privcfgr3;
#[doc = "GTZC1_TZSC_MPCWM4ACFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_mpcwm4acfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_mpcwm4acfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_mpcwm4acfgr`]
module"]
pub type GTZC1_TZSC_MPCWM4ACFGR = crate::Reg<gtzc1_tzsc_mpcwm4acfgr::GTZC1_TZSC_MPCWM4ACFGRrs>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4acfgr;
#[doc = "GTZC1_TZSC_MPCWM4AR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region A watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_mpcwm4ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_mpcwm4ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_mpcwm4ar`]
module"]
pub type GTZC1_TZSC_MPCWM4AR = crate::Reg<gtzc1_tzsc_mpcwm4ar::GTZC1_TZSC_MPCWM4ARrs>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region A watermark register"]
pub mod gtzc1_tzsc_mpcwm4ar;
#[doc = "GTZC1_TZSC_MPCWM4BCFGR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_mpcwm4bcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_mpcwm4bcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_mpcwm4bcfgr`]
module"]
pub type GTZC1_TZSC_MPCWM4BCFGR = crate::Reg<gtzc1_tzsc_mpcwm4bcfgr::GTZC1_TZSC_MPCWM4BCFGRrs>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark configuration register"]
pub mod gtzc1_tzsc_mpcwm4bcfgr;
#[doc = "GTZC1_TZSC_MPCWM4BR (rw) register accessor: GTZC1 TZSC BKPSRAM sub-region B watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_tzsc_mpcwm4br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_tzsc_mpcwm4br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_tzsc_mpcwm4br`]
module"]
pub type GTZC1_TZSC_MPCWM4BR = crate::Reg<gtzc1_tzsc_mpcwm4br::GTZC1_TZSC_MPCWM4BRrs>;
#[doc = "GTZC1 TZSC BKPSRAM sub-region B watermark register"]
pub mod gtzc1_tzsc_mpcwm4br;
#[doc = "GTZC1_MPCBB1_PRIVCFGR0 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr0`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb1_privcfgr0::GTZC1_MPCBB1_PRIVCFGR0rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 0 register"]
pub mod gtzc1_mpcbb1_privcfgr0;
#[doc = "GTZC1_MPCBB1_PRIVCFGR1 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr1`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb1_privcfgr1::GTZC1_MPCBB1_PRIVCFGR1rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 1 register"]
pub mod gtzc1_mpcbb1_privcfgr1;
#[doc = "GTZC1_MPCBB1_PRIVCFGR2 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr2`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb1_privcfgr2::GTZC1_MPCBB1_PRIVCFGR2rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 2 register"]
pub mod gtzc1_mpcbb1_privcfgr2;
#[doc = "GTZC1_MPCBB1_PRIVCFGR3 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr3`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb1_privcfgr3::GTZC1_MPCBB1_PRIVCFGR3rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 3 register"]
pub mod gtzc1_mpcbb1_privcfgr3;
#[doc = "GTZC1_MPCBB1_PRIVCFGR4 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr4`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb1_privcfgr4::GTZC1_MPCBB1_PRIVCFGR4rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 4 register"]
pub mod gtzc1_mpcbb1_privcfgr4;
#[doc = "GTZC1_MPCBB1_PRIVCFGR5 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr5`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb1_privcfgr5::GTZC1_MPCBB1_PRIVCFGR5rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 5 register"]
pub mod gtzc1_mpcbb1_privcfgr5;
#[doc = "GTZC1_MPCBB1_PRIVCFGR6 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr6`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb1_privcfgr6::GTZC1_MPCBB1_PRIVCFGR6rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 6 register"]
pub mod gtzc1_mpcbb1_privcfgr6;
#[doc = "GTZC1_MPCBB1_PRIVCFGR7 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr7`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb1_privcfgr7::GTZC1_MPCBB1_PRIVCFGR7rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 7 register"]
pub mod gtzc1_mpcbb1_privcfgr7;
#[doc = "GTZC1_MPCBB1_PRIVCFGR8 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr8`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb1_privcfgr8::GTZC1_MPCBB1_PRIVCFGR8rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 8 register"]
pub mod gtzc1_mpcbb1_privcfgr8;
#[doc = "GTZC1_MPCBB1_PRIVCFGR9 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr9`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb1_privcfgr9::GTZC1_MPCBB1_PRIVCFGR9rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 9 register"]
pub mod gtzc1_mpcbb1_privcfgr9;
#[doc = "GTZC1_MPCBB1_PRIVCFGR10 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr10`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR10 = crate::Reg<gtzc1_mpcbb1_privcfgr10::GTZC1_MPCBB1_PRIVCFGR10rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 10 register"]
pub mod gtzc1_mpcbb1_privcfgr10;
#[doc = "GTZC1_MPCBB1_PRIVCFGR11 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr11`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR11 = crate::Reg<gtzc1_mpcbb1_privcfgr11::GTZC1_MPCBB1_PRIVCFGR11rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 11 register"]
pub mod gtzc1_mpcbb1_privcfgr11;
#[doc = "GTZC1_MPCBB1_PRIVCFGR12 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr12`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR12 = crate::Reg<gtzc1_mpcbb1_privcfgr12::GTZC1_MPCBB1_PRIVCFGR12rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 12 register"]
pub mod gtzc1_mpcbb1_privcfgr12;
#[doc = "GTZC1_MPCBB1_PRIVCFGR13 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr13`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR13 = crate::Reg<gtzc1_mpcbb1_privcfgr13::GTZC1_MPCBB1_PRIVCFGR13rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 13 register"]
pub mod gtzc1_mpcbb1_privcfgr13;
#[doc = "GTZC1_MPCBB1_PRIVCFGR14 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr14`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR14 = crate::Reg<gtzc1_mpcbb1_privcfgr14::GTZC1_MPCBB1_PRIVCFGR14rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 14 register"]
pub mod gtzc1_mpcbb1_privcfgr14;
#[doc = "GTZC1_MPCBB1_PRIVCFGR15 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr15`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR15 = crate::Reg<gtzc1_mpcbb1_privcfgr15::GTZC1_MPCBB1_PRIVCFGR15rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 15 register"]
pub mod gtzc1_mpcbb1_privcfgr15;
#[doc = "GTZC1_MPCBB1_PRIVCFGR16 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr16`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR16 = crate::Reg<gtzc1_mpcbb1_privcfgr16::GTZC1_MPCBB1_PRIVCFGR16rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 16 register"]
pub mod gtzc1_mpcbb1_privcfgr16;
#[doc = "GTZC1_MPCBB1_PRIVCFGR17 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr17`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR17 = crate::Reg<gtzc1_mpcbb1_privcfgr17::GTZC1_MPCBB1_PRIVCFGR17rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 17 register"]
pub mod gtzc1_mpcbb1_privcfgr17;
#[doc = "GTZC1_MPCBB1_PRIVCFGR18 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr18`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR18 = crate::Reg<gtzc1_mpcbb1_privcfgr18::GTZC1_MPCBB1_PRIVCFGR18rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 18 register"]
pub mod gtzc1_mpcbb1_privcfgr18;
#[doc = "GTZC1_MPCBB1_PRIVCFGR19 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr19`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR19 = crate::Reg<gtzc1_mpcbb1_privcfgr19::GTZC1_MPCBB1_PRIVCFGR19rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 19 register"]
pub mod gtzc1_mpcbb1_privcfgr19;
#[doc = "GTZC1_MPCBB1_PRIVCFGR20 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr20`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR20 = crate::Reg<gtzc1_mpcbb1_privcfgr20::GTZC1_MPCBB1_PRIVCFGR20rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 20 register"]
pub mod gtzc1_mpcbb1_privcfgr20;
#[doc = "GTZC1_MPCBB1_PRIVCFGR21 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr21`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR21 = crate::Reg<gtzc1_mpcbb1_privcfgr21::GTZC1_MPCBB1_PRIVCFGR21rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 21 register"]
pub mod gtzc1_mpcbb1_privcfgr21;
#[doc = "GTZC1_MPCBB1_PRIVCFGR22 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr22`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR22 = crate::Reg<gtzc1_mpcbb1_privcfgr22::GTZC1_MPCBB1_PRIVCFGR22rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register"]
pub mod gtzc1_mpcbb1_privcfgr22;
#[doc = "GTZC1_MPCBB1_PRIVCFGR23 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr23`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR23 = crate::Reg<gtzc1_mpcbb1_privcfgr23::GTZC1_MPCBB1_PRIVCFGR23rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 23 register"]
pub mod gtzc1_mpcbb1_privcfgr23;
#[doc = "GTZC1_MPCBB1_PRIVCFGR24 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr24`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR24 = crate::Reg<gtzc1_mpcbb1_privcfgr24::GTZC1_MPCBB1_PRIVCFGR24rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 24 register"]
pub mod gtzc1_mpcbb1_privcfgr24;
#[doc = "GTZC1_MPCBB1_PRIVCFGR25 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr25`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR25 = crate::Reg<gtzc1_mpcbb1_privcfgr25::GTZC1_MPCBB1_PRIVCFGR25rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 25 register"]
pub mod gtzc1_mpcbb1_privcfgr25;
#[doc = "GTZC1_MPCBB1_PRIVCFGR26 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr26`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR26 = crate::Reg<gtzc1_mpcbb1_privcfgr26::GTZC1_MPCBB1_PRIVCFGR26rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 26 register"]
pub mod gtzc1_mpcbb1_privcfgr26;
#[doc = "GTZC1_MPCBB1_PRIVCFGR27 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr27`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR27 = crate::Reg<gtzc1_mpcbb1_privcfgr27::GTZC1_MPCBB1_PRIVCFGR27rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 27 register"]
pub mod gtzc1_mpcbb1_privcfgr27;
#[doc = "GTZC1_MPCBB1_PRIVCFGR28 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr28`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR28 = crate::Reg<gtzc1_mpcbb1_privcfgr28::GTZC1_MPCBB1_PRIVCFGR28rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 28 register"]
pub mod gtzc1_mpcbb1_privcfgr28;
#[doc = "GTZC1_MPCBB1_PRIVCFGR29 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr29`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR29 = crate::Reg<gtzc1_mpcbb1_privcfgr29::GTZC1_MPCBB1_PRIVCFGR29rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 29 register"]
pub mod gtzc1_mpcbb1_privcfgr29;
#[doc = "GTZC1_MPCBB1_PRIVCFGR30 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr30`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR30 = crate::Reg<gtzc1_mpcbb1_privcfgr30::GTZC1_MPCBB1_PRIVCFGR30rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 30 register"]
pub mod gtzc1_mpcbb1_privcfgr30;
#[doc = "GTZC1_MPCBB1_PRIVCFGR31 (rw) register accessor: GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb1_privcfgr31`]
module"]
pub type GTZC1_MPCBB1_PRIVCFGR31 = crate::Reg<gtzc1_mpcbb1_privcfgr31::GTZC1_MPCBB1_PRIVCFGR31rs>;
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 31 register"]
pub mod gtzc1_mpcbb1_privcfgr31;
#[doc = "GTZC1_MPCBB2_PRIVCFGR0 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr0`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb2_privcfgr0::GTZC1_MPCBB2_PRIVCFGR0rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register"]
pub mod gtzc1_mpcbb2_privcfgr0;
#[doc = "GTZC1_MPCBB2_PRIVCFGR1 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr1`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb2_privcfgr1::GTZC1_MPCBB2_PRIVCFGR1rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register"]
pub mod gtzc1_mpcbb2_privcfgr1;
#[doc = "GTZC1_MPCBB2_PRIVCFGR2 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr2`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb2_privcfgr2::GTZC1_MPCBB2_PRIVCFGR2rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register"]
pub mod gtzc1_mpcbb2_privcfgr2;
#[doc = "GTZC1_MPCBB2_PRIVCFGR3 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr3`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb2_privcfgr3::GTZC1_MPCBB2_PRIVCFGR3rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register"]
pub mod gtzc1_mpcbb2_privcfgr3;
#[doc = "GTZC1_MPCBB2_PRIVCFGR4 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr4`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb2_privcfgr4::GTZC1_MPCBB2_PRIVCFGR4rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register"]
pub mod gtzc1_mpcbb2_privcfgr4;
#[doc = "GTZC1_MPCBB2_PRIVCFGR5 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr5`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb2_privcfgr5::GTZC1_MPCBB2_PRIVCFGR5rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register"]
pub mod gtzc1_mpcbb2_privcfgr5;
#[doc = "GTZC1_MPCBB2_PRIVCFGR6 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr6`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb2_privcfgr6::GTZC1_MPCBB2_PRIVCFGR6rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register"]
pub mod gtzc1_mpcbb2_privcfgr6;
#[doc = "GTZC1_MPCBB2_PRIVCFGR7 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr7`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb2_privcfgr7::GTZC1_MPCBB2_PRIVCFGR7rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register"]
pub mod gtzc1_mpcbb2_privcfgr7;
#[doc = "GTZC1_MPCBB2_PRIVCFGR8 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr8`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb2_privcfgr8::GTZC1_MPCBB2_PRIVCFGR8rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register"]
pub mod gtzc1_mpcbb2_privcfgr8;
#[doc = "GTZC1_MPCBB2_PRIVCFGR9 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr9`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb2_privcfgr9::GTZC1_MPCBB2_PRIVCFGR9rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register"]
pub mod gtzc1_mpcbb2_privcfgr9;
#[doc = "GTZC1_MPCBB2_PRIVCFGR10 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr10`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR10 = crate::Reg<gtzc1_mpcbb2_privcfgr10::GTZC1_MPCBB2_PRIVCFGR10rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register"]
pub mod gtzc1_mpcbb2_privcfgr10;
#[doc = "GTZC1_MPCBB2_PRIVCFGR11 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr11`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR11 = crate::Reg<gtzc1_mpcbb2_privcfgr11::GTZC1_MPCBB2_PRIVCFGR11rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register"]
pub mod gtzc1_mpcbb2_privcfgr11;
#[doc = "GTZC1_MPCBB2_PRIVCFGR12 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr12`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR12 = crate::Reg<gtzc1_mpcbb2_privcfgr12::GTZC1_MPCBB2_PRIVCFGR12rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register"]
pub mod gtzc1_mpcbb2_privcfgr12;
#[doc = "GTZC1_MPCBB2_PRIVCFGR13 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr13`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR13 = crate::Reg<gtzc1_mpcbb2_privcfgr13::GTZC1_MPCBB2_PRIVCFGR13rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register"]
pub mod gtzc1_mpcbb2_privcfgr13;
#[doc = "GTZC1_MPCBB2_PRIVCFGR14 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr14`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR14 = crate::Reg<gtzc1_mpcbb2_privcfgr14::GTZC1_MPCBB2_PRIVCFGR14rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register"]
pub mod gtzc1_mpcbb2_privcfgr14;
#[doc = "GTZC1_MPCBB2_PRIVCFGR15 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr15`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR15 = crate::Reg<gtzc1_mpcbb2_privcfgr15::GTZC1_MPCBB2_PRIVCFGR15rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register"]
pub mod gtzc1_mpcbb2_privcfgr15;
#[doc = "GTZC1_MPCBB2_PRIVCFGR16 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr16`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR16 = crate::Reg<gtzc1_mpcbb2_privcfgr16::GTZC1_MPCBB2_PRIVCFGR16rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register"]
pub mod gtzc1_mpcbb2_privcfgr16;
#[doc = "GTZC1_MPCBB2_PRIVCFGR17 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr17`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR17 = crate::Reg<gtzc1_mpcbb2_privcfgr17::GTZC1_MPCBB2_PRIVCFGR17rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register"]
pub mod gtzc1_mpcbb2_privcfgr17;
#[doc = "GTZC1_MPCBB2_PRIVCFGR18 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr18`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR18 = crate::Reg<gtzc1_mpcbb2_privcfgr18::GTZC1_MPCBB2_PRIVCFGR18rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register"]
pub mod gtzc1_mpcbb2_privcfgr18;
#[doc = "GTZC1_MPCBB2_PRIVCFGR19 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr19`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR19 = crate::Reg<gtzc1_mpcbb2_privcfgr19::GTZC1_MPCBB2_PRIVCFGR19rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register"]
pub mod gtzc1_mpcbb2_privcfgr19;
#[doc = "GTZC1_MPCBB2_PRIVCFGR20 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr20`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR20 = crate::Reg<gtzc1_mpcbb2_privcfgr20::GTZC1_MPCBB2_PRIVCFGR20rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register"]
pub mod gtzc1_mpcbb2_privcfgr20;
#[doc = "GTZC1_MPCBB2_PRIVCFGR21 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr21`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR21 = crate::Reg<gtzc1_mpcbb2_privcfgr21::GTZC1_MPCBB2_PRIVCFGR21rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register"]
pub mod gtzc1_mpcbb2_privcfgr21;
#[doc = "GTZC1_MPCBB2_PRIVCFGR22 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr22`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR22 = crate::Reg<gtzc1_mpcbb2_privcfgr22::GTZC1_MPCBB2_PRIVCFGR22rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register"]
pub mod gtzc1_mpcbb2_privcfgr22;
#[doc = "GTZC1_MPCBB2_PRIVCFGR23 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr23`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR23 = crate::Reg<gtzc1_mpcbb2_privcfgr23::GTZC1_MPCBB2_PRIVCFGR23rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register"]
pub mod gtzc1_mpcbb2_privcfgr23;
#[doc = "GTZC1_MPCBB2_PRIVCFGR24 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr24`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR24 = crate::Reg<gtzc1_mpcbb2_privcfgr24::GTZC1_MPCBB2_PRIVCFGR24rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register"]
pub mod gtzc1_mpcbb2_privcfgr24;
#[doc = "GTZC1_MPCBB2_PRIVCFGR25 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr25`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR25 = crate::Reg<gtzc1_mpcbb2_privcfgr25::GTZC1_MPCBB2_PRIVCFGR25rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register"]
pub mod gtzc1_mpcbb2_privcfgr25;
#[doc = "GTZC1_MPCBB2_PRIVCFGR26 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr26`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR26 = crate::Reg<gtzc1_mpcbb2_privcfgr26::GTZC1_MPCBB2_PRIVCFGR26rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register"]
pub mod gtzc1_mpcbb2_privcfgr26;
#[doc = "GTZC1_MPCBB2_PRIVCFGR27 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr27`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR27 = crate::Reg<gtzc1_mpcbb2_privcfgr27::GTZC1_MPCBB2_PRIVCFGR27rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register"]
pub mod gtzc1_mpcbb2_privcfgr27;
#[doc = "GTZC1_MPCBB2_PRIVCFGR28 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr28`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR28 = crate::Reg<gtzc1_mpcbb2_privcfgr28::GTZC1_MPCBB2_PRIVCFGR28rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register"]
pub mod gtzc1_mpcbb2_privcfgr28;
#[doc = "GTZC1_MPCBB2_PRIVCFGR29 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr29`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR29 = crate::Reg<gtzc1_mpcbb2_privcfgr29::GTZC1_MPCBB2_PRIVCFGR29rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register"]
pub mod gtzc1_mpcbb2_privcfgr29;
#[doc = "GTZC1_MPCBB2_PRIVCFGR30 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr30`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR30 = crate::Reg<gtzc1_mpcbb2_privcfgr30::GTZC1_MPCBB2_PRIVCFGR30rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register"]
pub mod gtzc1_mpcbb2_privcfgr30;
#[doc = "GTZC1_MPCBB2_PRIVCFGR31 (rw) register accessor: GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb2_privcfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb2_privcfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtzc1_mpcbb2_privcfgr31`]
module"]
pub type GTZC1_MPCBB2_PRIVCFGR31 = crate::Reg<gtzc1_mpcbb2_privcfgr31::GTZC1_MPCBB2_PRIVCFGR31rs>;
#[doc = "GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register"]
pub mod gtzc1_mpcbb2_privcfgr31;
