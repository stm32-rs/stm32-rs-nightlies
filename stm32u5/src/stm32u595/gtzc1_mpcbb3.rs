#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpcbb3_cr: MPCBB3_CR,
    _reserved1: [u8; 0x0c],
    mpcbb3_cfglock1: MPCBB3_CFGLOCK1,
    mpcbb3_cfglock2: MPCBB3_CFGLOCK2,
    _reserved3: [u8; 0xe8],
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
    mpcbb3_seccfgr32: MPCBB3_SECCFGR32,
    mpcbb3_seccfgr33: MPCBB3_SECCFGR33,
    mpcbb3_seccfgr34: MPCBB3_SECCFGR34,
    mpcbb3_seccfgr35: MPCBB3_SECCFGR35,
    mpcbb3_seccfgr36: MPCBB3_SECCFGR36,
    mpcbb3_seccfgr37: MPCBB3_SECCFGR37,
    mpcbb3_seccfgr38: MPCBB3_SECCFGR38,
    mpcbb3_seccfgr39: MPCBB3_SECCFGR39,
    mpcbb3_seccfgr40: MPCBB3_SECCFGR40,
    mpcbb3_seccfgr41: MPCBB3_SECCFGR41,
    mpcbb3_seccfgr42: MPCBB3_SECCFGR42,
    mpcbb3_seccfgr43: MPCBB3_SECCFGR43,
    mpcbb3_seccfgr44: MPCBB3_SECCFGR44,
    mpcbb3_seccfgr45: MPCBB3_SECCFGR45,
    mpcbb3_seccfgr46: MPCBB3_SECCFGR46,
    mpcbb3_seccfgr47: MPCBB3_SECCFGR47,
    mpcbb3_seccfgr48: MPCBB3_SECCFGR48,
    mpcbb3_seccfgr49: MPCBB3_SECCFGR49,
    mpcbb3_seccfgr50: MPCBB3_SECCFGR50,
    mpcbb3_seccfgr51: MPCBB3_SECCFGR51,
    _reserved55: [u8; 0x30],
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
    mpcbb3_privcfgr32: MPCBB3_PRIVCFGR32,
    mpcbb3_privcfgr33: MPCBB3_PRIVCFGR33,
    mpcbb3_privcfgr34: MPCBB3_PRIVCFGR34,
    mpcbb3_privcfgr35: MPCBB3_PRIVCFGR35,
    mpcbb3_privcfgr36: MPCBB3_PRIVCFGR36,
    mpcbb3_privcfgr37: MPCBB3_PRIVCFGR37,
    mpcbb3_privcfgr38: MPCBB3_PRIVCFGR38,
    mpcbb3_privcfgr39: MPCBB3_PRIVCFGR39,
    mpcbb3_privcfgr40: MPCBB3_PRIVCFGR40,
    mpcbb3_privcfgr41: MPCBB3_PRIVCFGR41,
    mpcbb3_privcfgr42: MPCBB3_PRIVCFGR42,
    mpcbb3_privcfgr43: MPCBB3_PRIVCFGR43,
    mpcbb3_privcfgr44: MPCBB3_PRIVCFGR44,
    mpcbb3_privcfgr45: MPCBB3_PRIVCFGR45,
    mpcbb3_privcfgr46: MPCBB3_PRIVCFGR46,
    mpcbb3_privcfgr47: MPCBB3_PRIVCFGR47,
    mpcbb3_privcfgr48: MPCBB3_PRIVCFGR48,
    mpcbb3_privcfgr49: MPCBB3_PRIVCFGR49,
    mpcbb3_privcfgr50: MPCBB3_PRIVCFGR50,
    mpcbb3_privcfgr51: MPCBB3_PRIVCFGR51,
}
impl RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    #[inline(always)]
    pub const fn mpcbb3_cr(&self) -> &MPCBB3_CR {
        &self.mpcbb3_cr
    }
    #[doc = "0x10 - GTZC1 SRAMz MPCBB configuration lock register"]
    #[inline(always)]
    pub const fn mpcbb3_cfglock1(&self) -> &MPCBB3_CFGLOCK1 {
        &self.mpcbb3_cfglock1
    }
    #[doc = "0x14 - GTZC1 SRAMz MPCBB configuration lock register 2"]
    #[inline(always)]
    pub const fn mpcbb3_cfglock2(&self) -> &MPCBB3_CFGLOCK2 {
        &self.mpcbb3_cfglock2
    }
    #[doc = "0x100 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr0(&self) -> &MPCBB3_SECCFGR0 {
        &self.mpcbb3_seccfgr0
    }
    #[doc = "0x104 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr1(&self) -> &MPCBB3_SECCFGR1 {
        &self.mpcbb3_seccfgr1
    }
    #[doc = "0x108 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr2(&self) -> &MPCBB3_SECCFGR2 {
        &self.mpcbb3_seccfgr2
    }
    #[doc = "0x10c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr3(&self) -> &MPCBB3_SECCFGR3 {
        &self.mpcbb3_seccfgr3
    }
    #[doc = "0x110 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr4(&self) -> &MPCBB3_SECCFGR4 {
        &self.mpcbb3_seccfgr4
    }
    #[doc = "0x114 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr5(&self) -> &MPCBB3_SECCFGR5 {
        &self.mpcbb3_seccfgr5
    }
    #[doc = "0x118 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr6(&self) -> &MPCBB3_SECCFGR6 {
        &self.mpcbb3_seccfgr6
    }
    #[doc = "0x11c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr7(&self) -> &MPCBB3_SECCFGR7 {
        &self.mpcbb3_seccfgr7
    }
    #[doc = "0x120 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr8(&self) -> &MPCBB3_SECCFGR8 {
        &self.mpcbb3_seccfgr8
    }
    #[doc = "0x124 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr9(&self) -> &MPCBB3_SECCFGR9 {
        &self.mpcbb3_seccfgr9
    }
    #[doc = "0x128 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr10(&self) -> &MPCBB3_SECCFGR10 {
        &self.mpcbb3_seccfgr10
    }
    #[doc = "0x12c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr11(&self) -> &MPCBB3_SECCFGR11 {
        &self.mpcbb3_seccfgr11
    }
    #[doc = "0x130 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr12(&self) -> &MPCBB3_SECCFGR12 {
        &self.mpcbb3_seccfgr12
    }
    #[doc = "0x134 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr13(&self) -> &MPCBB3_SECCFGR13 {
        &self.mpcbb3_seccfgr13
    }
    #[doc = "0x138 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr14(&self) -> &MPCBB3_SECCFGR14 {
        &self.mpcbb3_seccfgr14
    }
    #[doc = "0x13c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr15(&self) -> &MPCBB3_SECCFGR15 {
        &self.mpcbb3_seccfgr15
    }
    #[doc = "0x140 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr16(&self) -> &MPCBB3_SECCFGR16 {
        &self.mpcbb3_seccfgr16
    }
    #[doc = "0x144 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr17(&self) -> &MPCBB3_SECCFGR17 {
        &self.mpcbb3_seccfgr17
    }
    #[doc = "0x148 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr18(&self) -> &MPCBB3_SECCFGR18 {
        &self.mpcbb3_seccfgr18
    }
    #[doc = "0x14c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr19(&self) -> &MPCBB3_SECCFGR19 {
        &self.mpcbb3_seccfgr19
    }
    #[doc = "0x150 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr20(&self) -> &MPCBB3_SECCFGR20 {
        &self.mpcbb3_seccfgr20
    }
    #[doc = "0x154 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr21(&self) -> &MPCBB3_SECCFGR21 {
        &self.mpcbb3_seccfgr21
    }
    #[doc = "0x158 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr22(&self) -> &MPCBB3_SECCFGR22 {
        &self.mpcbb3_seccfgr22
    }
    #[doc = "0x15c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr23(&self) -> &MPCBB3_SECCFGR23 {
        &self.mpcbb3_seccfgr23
    }
    #[doc = "0x160 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr24(&self) -> &MPCBB3_SECCFGR24 {
        &self.mpcbb3_seccfgr24
    }
    #[doc = "0x164 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr25(&self) -> &MPCBB3_SECCFGR25 {
        &self.mpcbb3_seccfgr25
    }
    #[doc = "0x168 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr26(&self) -> &MPCBB3_SECCFGR26 {
        &self.mpcbb3_seccfgr26
    }
    #[doc = "0x16c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr27(&self) -> &MPCBB3_SECCFGR27 {
        &self.mpcbb3_seccfgr27
    }
    #[doc = "0x170 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr28(&self) -> &MPCBB3_SECCFGR28 {
        &self.mpcbb3_seccfgr28
    }
    #[doc = "0x174 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr29(&self) -> &MPCBB3_SECCFGR29 {
        &self.mpcbb3_seccfgr29
    }
    #[doc = "0x178 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr30(&self) -> &MPCBB3_SECCFGR30 {
        &self.mpcbb3_seccfgr30
    }
    #[doc = "0x17c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr31(&self) -> &MPCBB3_SECCFGR31 {
        &self.mpcbb3_seccfgr31
    }
    #[doc = "0x180 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr32(&self) -> &MPCBB3_SECCFGR32 {
        &self.mpcbb3_seccfgr32
    }
    #[doc = "0x184 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr33(&self) -> &MPCBB3_SECCFGR33 {
        &self.mpcbb3_seccfgr33
    }
    #[doc = "0x188 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr34(&self) -> &MPCBB3_SECCFGR34 {
        &self.mpcbb3_seccfgr34
    }
    #[doc = "0x18c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr35(&self) -> &MPCBB3_SECCFGR35 {
        &self.mpcbb3_seccfgr35
    }
    #[doc = "0x190 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr36(&self) -> &MPCBB3_SECCFGR36 {
        &self.mpcbb3_seccfgr36
    }
    #[doc = "0x194 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr37(&self) -> &MPCBB3_SECCFGR37 {
        &self.mpcbb3_seccfgr37
    }
    #[doc = "0x198 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr38(&self) -> &MPCBB3_SECCFGR38 {
        &self.mpcbb3_seccfgr38
    }
    #[doc = "0x19c - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr39(&self) -> &MPCBB3_SECCFGR39 {
        &self.mpcbb3_seccfgr39
    }
    #[doc = "0x1a0 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr40(&self) -> &MPCBB3_SECCFGR40 {
        &self.mpcbb3_seccfgr40
    }
    #[doc = "0x1a4 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr41(&self) -> &MPCBB3_SECCFGR41 {
        &self.mpcbb3_seccfgr41
    }
    #[doc = "0x1a8 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr42(&self) -> &MPCBB3_SECCFGR42 {
        &self.mpcbb3_seccfgr42
    }
    #[doc = "0x1ac - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr43(&self) -> &MPCBB3_SECCFGR43 {
        &self.mpcbb3_seccfgr43
    }
    #[doc = "0x1b0 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr44(&self) -> &MPCBB3_SECCFGR44 {
        &self.mpcbb3_seccfgr44
    }
    #[doc = "0x1b4 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr45(&self) -> &MPCBB3_SECCFGR45 {
        &self.mpcbb3_seccfgr45
    }
    #[doc = "0x1b8 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr46(&self) -> &MPCBB3_SECCFGR46 {
        &self.mpcbb3_seccfgr46
    }
    #[doc = "0x1bc - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr47(&self) -> &MPCBB3_SECCFGR47 {
        &self.mpcbb3_seccfgr47
    }
    #[doc = "0x1c0 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr48(&self) -> &MPCBB3_SECCFGR48 {
        &self.mpcbb3_seccfgr48
    }
    #[doc = "0x1c4 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr49(&self) -> &MPCBB3_SECCFGR49 {
        &self.mpcbb3_seccfgr49
    }
    #[doc = "0x1c8 - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr50(&self) -> &MPCBB3_SECCFGR50 {
        &self.mpcbb3_seccfgr50
    }
    #[doc = "0x1cc - MPCBBx security configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_seccfgr51(&self) -> &MPCBB3_SECCFGR51 {
        &self.mpcbb3_seccfgr51
    }
    #[doc = "0x200 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr0(&self) -> &MPCBB3_PRIVCFGR0 {
        &self.mpcbb3_privcfgr0
    }
    #[doc = "0x204 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr1(&self) -> &MPCBB3_PRIVCFGR1 {
        &self.mpcbb3_privcfgr1
    }
    #[doc = "0x208 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr2(&self) -> &MPCBB3_PRIVCFGR2 {
        &self.mpcbb3_privcfgr2
    }
    #[doc = "0x20c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr3(&self) -> &MPCBB3_PRIVCFGR3 {
        &self.mpcbb3_privcfgr3
    }
    #[doc = "0x210 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr4(&self) -> &MPCBB3_PRIVCFGR4 {
        &self.mpcbb3_privcfgr4
    }
    #[doc = "0x214 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr5(&self) -> &MPCBB3_PRIVCFGR5 {
        &self.mpcbb3_privcfgr5
    }
    #[doc = "0x218 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr6(&self) -> &MPCBB3_PRIVCFGR6 {
        &self.mpcbb3_privcfgr6
    }
    #[doc = "0x21c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr7(&self) -> &MPCBB3_PRIVCFGR7 {
        &self.mpcbb3_privcfgr7
    }
    #[doc = "0x220 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr8(&self) -> &MPCBB3_PRIVCFGR8 {
        &self.mpcbb3_privcfgr8
    }
    #[doc = "0x224 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr9(&self) -> &MPCBB3_PRIVCFGR9 {
        &self.mpcbb3_privcfgr9
    }
    #[doc = "0x228 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr10(&self) -> &MPCBB3_PRIVCFGR10 {
        &self.mpcbb3_privcfgr10
    }
    #[doc = "0x22c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr11(&self) -> &MPCBB3_PRIVCFGR11 {
        &self.mpcbb3_privcfgr11
    }
    #[doc = "0x230 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr12(&self) -> &MPCBB3_PRIVCFGR12 {
        &self.mpcbb3_privcfgr12
    }
    #[doc = "0x234 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr13(&self) -> &MPCBB3_PRIVCFGR13 {
        &self.mpcbb3_privcfgr13
    }
    #[doc = "0x238 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr14(&self) -> &MPCBB3_PRIVCFGR14 {
        &self.mpcbb3_privcfgr14
    }
    #[doc = "0x23c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr15(&self) -> &MPCBB3_PRIVCFGR15 {
        &self.mpcbb3_privcfgr15
    }
    #[doc = "0x240 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr16(&self) -> &MPCBB3_PRIVCFGR16 {
        &self.mpcbb3_privcfgr16
    }
    #[doc = "0x244 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr17(&self) -> &MPCBB3_PRIVCFGR17 {
        &self.mpcbb3_privcfgr17
    }
    #[doc = "0x248 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr18(&self) -> &MPCBB3_PRIVCFGR18 {
        &self.mpcbb3_privcfgr18
    }
    #[doc = "0x24c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr19(&self) -> &MPCBB3_PRIVCFGR19 {
        &self.mpcbb3_privcfgr19
    }
    #[doc = "0x250 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr20(&self) -> &MPCBB3_PRIVCFGR20 {
        &self.mpcbb3_privcfgr20
    }
    #[doc = "0x254 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr21(&self) -> &MPCBB3_PRIVCFGR21 {
        &self.mpcbb3_privcfgr21
    }
    #[doc = "0x258 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr22(&self) -> &MPCBB3_PRIVCFGR22 {
        &self.mpcbb3_privcfgr22
    }
    #[doc = "0x25c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr23(&self) -> &MPCBB3_PRIVCFGR23 {
        &self.mpcbb3_privcfgr23
    }
    #[doc = "0x260 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr24(&self) -> &MPCBB3_PRIVCFGR24 {
        &self.mpcbb3_privcfgr24
    }
    #[doc = "0x264 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr25(&self) -> &MPCBB3_PRIVCFGR25 {
        &self.mpcbb3_privcfgr25
    }
    #[doc = "0x268 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr26(&self) -> &MPCBB3_PRIVCFGR26 {
        &self.mpcbb3_privcfgr26
    }
    #[doc = "0x26c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr27(&self) -> &MPCBB3_PRIVCFGR27 {
        &self.mpcbb3_privcfgr27
    }
    #[doc = "0x270 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr28(&self) -> &MPCBB3_PRIVCFGR28 {
        &self.mpcbb3_privcfgr28
    }
    #[doc = "0x274 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr29(&self) -> &MPCBB3_PRIVCFGR29 {
        &self.mpcbb3_privcfgr29
    }
    #[doc = "0x278 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr30(&self) -> &MPCBB3_PRIVCFGR30 {
        &self.mpcbb3_privcfgr30
    }
    #[doc = "0x27c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr31(&self) -> &MPCBB3_PRIVCFGR31 {
        &self.mpcbb3_privcfgr31
    }
    #[doc = "0x280 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr32(&self) -> &MPCBB3_PRIVCFGR32 {
        &self.mpcbb3_privcfgr32
    }
    #[doc = "0x284 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr33(&self) -> &MPCBB3_PRIVCFGR33 {
        &self.mpcbb3_privcfgr33
    }
    #[doc = "0x288 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr34(&self) -> &MPCBB3_PRIVCFGR34 {
        &self.mpcbb3_privcfgr34
    }
    #[doc = "0x28c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr35(&self) -> &MPCBB3_PRIVCFGR35 {
        &self.mpcbb3_privcfgr35
    }
    #[doc = "0x290 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr36(&self) -> &MPCBB3_PRIVCFGR36 {
        &self.mpcbb3_privcfgr36
    }
    #[doc = "0x294 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr37(&self) -> &MPCBB3_PRIVCFGR37 {
        &self.mpcbb3_privcfgr37
    }
    #[doc = "0x298 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr38(&self) -> &MPCBB3_PRIVCFGR38 {
        &self.mpcbb3_privcfgr38
    }
    #[doc = "0x29c - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr39(&self) -> &MPCBB3_PRIVCFGR39 {
        &self.mpcbb3_privcfgr39
    }
    #[doc = "0x2a0 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr40(&self) -> &MPCBB3_PRIVCFGR40 {
        &self.mpcbb3_privcfgr40
    }
    #[doc = "0x2a4 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr41(&self) -> &MPCBB3_PRIVCFGR41 {
        &self.mpcbb3_privcfgr41
    }
    #[doc = "0x2a8 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr42(&self) -> &MPCBB3_PRIVCFGR42 {
        &self.mpcbb3_privcfgr42
    }
    #[doc = "0x2ac - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr43(&self) -> &MPCBB3_PRIVCFGR43 {
        &self.mpcbb3_privcfgr43
    }
    #[doc = "0x2b0 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr44(&self) -> &MPCBB3_PRIVCFGR44 {
        &self.mpcbb3_privcfgr44
    }
    #[doc = "0x2b4 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr45(&self) -> &MPCBB3_PRIVCFGR45 {
        &self.mpcbb3_privcfgr45
    }
    #[doc = "0x2b8 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr46(&self) -> &MPCBB3_PRIVCFGR46 {
        &self.mpcbb3_privcfgr46
    }
    #[doc = "0x2bc - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr47(&self) -> &MPCBB3_PRIVCFGR47 {
        &self.mpcbb3_privcfgr47
    }
    #[doc = "0x2c0 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr48(&self) -> &MPCBB3_PRIVCFGR48 {
        &self.mpcbb3_privcfgr48
    }
    #[doc = "0x2c4 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr49(&self) -> &MPCBB3_PRIVCFGR49 {
        &self.mpcbb3_privcfgr49
    }
    #[doc = "0x2c8 - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr50(&self) -> &MPCBB3_PRIVCFGR50 {
        &self.mpcbb3_privcfgr50
    }
    #[doc = "0x2cc - MPCBB privileged configuration for super-block x register"]
    #[inline(always)]
    pub const fn mpcbb3_privcfgr51(&self) -> &MPCBB3_PRIVCFGR51 {
        &self.mpcbb3_privcfgr51
    }
}
#[doc = "MPCBB3_CR (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_cr`]
module"]
pub type MPCBB3_CR = crate::Reg<mpcbb3_cr::MPCBB3_CRrs>;
#[doc = "MPCBB control register"]
pub mod mpcbb3_cr;
#[doc = "MPCBB3_CFGLOCK1 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_cfglock1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_cfglock1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_cfglock1`]
module"]
pub type MPCBB3_CFGLOCK1 = crate::Reg<mpcbb3_cfglock1::MPCBB3_CFGLOCK1rs>;
#[doc = "GTZC1 SRAMz MPCBB configuration lock register"]
pub mod mpcbb3_cfglock1;
#[doc = "MPCBB3_CFGLOCK2 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_cfglock2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_cfglock2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_cfglock2`]
module"]
pub type MPCBB3_CFGLOCK2 = crate::Reg<mpcbb3_cfglock2::MPCBB3_CFGLOCK2rs>;
#[doc = "GTZC1 SRAMz MPCBB configuration lock register 2"]
pub mod mpcbb3_cfglock2;
#[doc = "MPCBB3_SECCFGR0 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr0`]
module"]
pub type MPCBB3_SECCFGR0 = crate::Reg<mpcbb3_seccfgr0::MPCBB3_SECCFGR0rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr0;
#[doc = "MPCBB3_SECCFGR1 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr1`]
module"]
pub type MPCBB3_SECCFGR1 = crate::Reg<mpcbb3_seccfgr1::MPCBB3_SECCFGR1rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr1;
#[doc = "MPCBB3_SECCFGR2 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr2`]
module"]
pub type MPCBB3_SECCFGR2 = crate::Reg<mpcbb3_seccfgr2::MPCBB3_SECCFGR2rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr2;
#[doc = "MPCBB3_SECCFGR3 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr3`]
module"]
pub type MPCBB3_SECCFGR3 = crate::Reg<mpcbb3_seccfgr3::MPCBB3_SECCFGR3rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr3;
#[doc = "MPCBB3_SECCFGR4 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr4`]
module"]
pub type MPCBB3_SECCFGR4 = crate::Reg<mpcbb3_seccfgr4::MPCBB3_SECCFGR4rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr4;
#[doc = "MPCBB3_SECCFGR5 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr5`]
module"]
pub type MPCBB3_SECCFGR5 = crate::Reg<mpcbb3_seccfgr5::MPCBB3_SECCFGR5rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr5;
#[doc = "MPCBB3_SECCFGR6 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr6`]
module"]
pub type MPCBB3_SECCFGR6 = crate::Reg<mpcbb3_seccfgr6::MPCBB3_SECCFGR6rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr6;
#[doc = "MPCBB3_SECCFGR7 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr7`]
module"]
pub type MPCBB3_SECCFGR7 = crate::Reg<mpcbb3_seccfgr7::MPCBB3_SECCFGR7rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr7;
#[doc = "MPCBB3_SECCFGR8 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr8`]
module"]
pub type MPCBB3_SECCFGR8 = crate::Reg<mpcbb3_seccfgr8::MPCBB3_SECCFGR8rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr8;
#[doc = "MPCBB3_SECCFGR9 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr9`]
module"]
pub type MPCBB3_SECCFGR9 = crate::Reg<mpcbb3_seccfgr9::MPCBB3_SECCFGR9rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr9;
#[doc = "MPCBB3_SECCFGR10 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr10`]
module"]
pub type MPCBB3_SECCFGR10 = crate::Reg<mpcbb3_seccfgr10::MPCBB3_SECCFGR10rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr10;
#[doc = "MPCBB3_SECCFGR11 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr11`]
module"]
pub type MPCBB3_SECCFGR11 = crate::Reg<mpcbb3_seccfgr11::MPCBB3_SECCFGR11rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr11;
#[doc = "MPCBB3_SECCFGR12 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr12`]
module"]
pub type MPCBB3_SECCFGR12 = crate::Reg<mpcbb3_seccfgr12::MPCBB3_SECCFGR12rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr12;
#[doc = "MPCBB3_SECCFGR13 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr13`]
module"]
pub type MPCBB3_SECCFGR13 = crate::Reg<mpcbb3_seccfgr13::MPCBB3_SECCFGR13rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr13;
#[doc = "MPCBB3_SECCFGR14 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr14`]
module"]
pub type MPCBB3_SECCFGR14 = crate::Reg<mpcbb3_seccfgr14::MPCBB3_SECCFGR14rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr14;
#[doc = "MPCBB3_SECCFGR15 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr15`]
module"]
pub type MPCBB3_SECCFGR15 = crate::Reg<mpcbb3_seccfgr15::MPCBB3_SECCFGR15rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr15;
#[doc = "MPCBB3_SECCFGR16 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr16`]
module"]
pub type MPCBB3_SECCFGR16 = crate::Reg<mpcbb3_seccfgr16::MPCBB3_SECCFGR16rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr16;
#[doc = "MPCBB3_SECCFGR17 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr17`]
module"]
pub type MPCBB3_SECCFGR17 = crate::Reg<mpcbb3_seccfgr17::MPCBB3_SECCFGR17rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr17;
#[doc = "MPCBB3_SECCFGR18 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr18`]
module"]
pub type MPCBB3_SECCFGR18 = crate::Reg<mpcbb3_seccfgr18::MPCBB3_SECCFGR18rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr18;
#[doc = "MPCBB3_SECCFGR19 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr19`]
module"]
pub type MPCBB3_SECCFGR19 = crate::Reg<mpcbb3_seccfgr19::MPCBB3_SECCFGR19rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr19;
#[doc = "MPCBB3_SECCFGR20 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr20`]
module"]
pub type MPCBB3_SECCFGR20 = crate::Reg<mpcbb3_seccfgr20::MPCBB3_SECCFGR20rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr20;
#[doc = "MPCBB3_SECCFGR21 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr21`]
module"]
pub type MPCBB3_SECCFGR21 = crate::Reg<mpcbb3_seccfgr21::MPCBB3_SECCFGR21rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr21;
#[doc = "MPCBB3_SECCFGR22 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr22`]
module"]
pub type MPCBB3_SECCFGR22 = crate::Reg<mpcbb3_seccfgr22::MPCBB3_SECCFGR22rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr22;
#[doc = "MPCBB3_SECCFGR23 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr23`]
module"]
pub type MPCBB3_SECCFGR23 = crate::Reg<mpcbb3_seccfgr23::MPCBB3_SECCFGR23rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr23;
#[doc = "MPCBB3_SECCFGR24 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr24`]
module"]
pub type MPCBB3_SECCFGR24 = crate::Reg<mpcbb3_seccfgr24::MPCBB3_SECCFGR24rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr24;
#[doc = "MPCBB3_SECCFGR25 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr25`]
module"]
pub type MPCBB3_SECCFGR25 = crate::Reg<mpcbb3_seccfgr25::MPCBB3_SECCFGR25rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr25;
#[doc = "MPCBB3_SECCFGR26 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr26`]
module"]
pub type MPCBB3_SECCFGR26 = crate::Reg<mpcbb3_seccfgr26::MPCBB3_SECCFGR26rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr26;
#[doc = "MPCBB3_SECCFGR27 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr27`]
module"]
pub type MPCBB3_SECCFGR27 = crate::Reg<mpcbb3_seccfgr27::MPCBB3_SECCFGR27rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr27;
#[doc = "MPCBB3_SECCFGR28 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr28`]
module"]
pub type MPCBB3_SECCFGR28 = crate::Reg<mpcbb3_seccfgr28::MPCBB3_SECCFGR28rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr28;
#[doc = "MPCBB3_SECCFGR29 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr29`]
module"]
pub type MPCBB3_SECCFGR29 = crate::Reg<mpcbb3_seccfgr29::MPCBB3_SECCFGR29rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr29;
#[doc = "MPCBB3_SECCFGR30 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr30`]
module"]
pub type MPCBB3_SECCFGR30 = crate::Reg<mpcbb3_seccfgr30::MPCBB3_SECCFGR30rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr30;
#[doc = "MPCBB3_SECCFGR31 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr31`]
module"]
pub type MPCBB3_SECCFGR31 = crate::Reg<mpcbb3_seccfgr31::MPCBB3_SECCFGR31rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr31;
#[doc = "MPCBB3_SECCFGR32 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr32`]
module"]
pub type MPCBB3_SECCFGR32 = crate::Reg<mpcbb3_seccfgr32::MPCBB3_SECCFGR32rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr32;
#[doc = "MPCBB3_SECCFGR33 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr33`]
module"]
pub type MPCBB3_SECCFGR33 = crate::Reg<mpcbb3_seccfgr33::MPCBB3_SECCFGR33rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr33;
#[doc = "MPCBB3_SECCFGR34 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr34`]
module"]
pub type MPCBB3_SECCFGR34 = crate::Reg<mpcbb3_seccfgr34::MPCBB3_SECCFGR34rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr34;
#[doc = "MPCBB3_SECCFGR35 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr35`]
module"]
pub type MPCBB3_SECCFGR35 = crate::Reg<mpcbb3_seccfgr35::MPCBB3_SECCFGR35rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr35;
#[doc = "MPCBB3_SECCFGR36 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr36`]
module"]
pub type MPCBB3_SECCFGR36 = crate::Reg<mpcbb3_seccfgr36::MPCBB3_SECCFGR36rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr36;
#[doc = "MPCBB3_SECCFGR37 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr37`]
module"]
pub type MPCBB3_SECCFGR37 = crate::Reg<mpcbb3_seccfgr37::MPCBB3_SECCFGR37rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr37;
#[doc = "MPCBB3_SECCFGR38 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr38`]
module"]
pub type MPCBB3_SECCFGR38 = crate::Reg<mpcbb3_seccfgr38::MPCBB3_SECCFGR38rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr38;
#[doc = "MPCBB3_SECCFGR39 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr39`]
module"]
pub type MPCBB3_SECCFGR39 = crate::Reg<mpcbb3_seccfgr39::MPCBB3_SECCFGR39rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr39;
#[doc = "MPCBB3_SECCFGR40 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr40`]
module"]
pub type MPCBB3_SECCFGR40 = crate::Reg<mpcbb3_seccfgr40::MPCBB3_SECCFGR40rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr40;
#[doc = "MPCBB3_SECCFGR41 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr41`]
module"]
pub type MPCBB3_SECCFGR41 = crate::Reg<mpcbb3_seccfgr41::MPCBB3_SECCFGR41rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr41;
#[doc = "MPCBB3_SECCFGR42 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr42`]
module"]
pub type MPCBB3_SECCFGR42 = crate::Reg<mpcbb3_seccfgr42::MPCBB3_SECCFGR42rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr42;
#[doc = "MPCBB3_SECCFGR43 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr43`]
module"]
pub type MPCBB3_SECCFGR43 = crate::Reg<mpcbb3_seccfgr43::MPCBB3_SECCFGR43rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr43;
#[doc = "MPCBB3_SECCFGR44 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr44`]
module"]
pub type MPCBB3_SECCFGR44 = crate::Reg<mpcbb3_seccfgr44::MPCBB3_SECCFGR44rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr44;
#[doc = "MPCBB3_SECCFGR45 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr45`]
module"]
pub type MPCBB3_SECCFGR45 = crate::Reg<mpcbb3_seccfgr45::MPCBB3_SECCFGR45rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr45;
#[doc = "MPCBB3_SECCFGR46 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr46`]
module"]
pub type MPCBB3_SECCFGR46 = crate::Reg<mpcbb3_seccfgr46::MPCBB3_SECCFGR46rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr46;
#[doc = "MPCBB3_SECCFGR47 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr47`]
module"]
pub type MPCBB3_SECCFGR47 = crate::Reg<mpcbb3_seccfgr47::MPCBB3_SECCFGR47rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr47;
#[doc = "MPCBB3_SECCFGR48 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr48`]
module"]
pub type MPCBB3_SECCFGR48 = crate::Reg<mpcbb3_seccfgr48::MPCBB3_SECCFGR48rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr48;
#[doc = "MPCBB3_SECCFGR49 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr49`]
module"]
pub type MPCBB3_SECCFGR49 = crate::Reg<mpcbb3_seccfgr49::MPCBB3_SECCFGR49rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr49;
#[doc = "MPCBB3_SECCFGR50 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr50`]
module"]
pub type MPCBB3_SECCFGR50 = crate::Reg<mpcbb3_seccfgr50::MPCBB3_SECCFGR50rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr50;
#[doc = "MPCBB3_SECCFGR51 (rw) register accessor: MPCBBx security configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_seccfgr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_seccfgr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_seccfgr51`]
module"]
pub type MPCBB3_SECCFGR51 = crate::Reg<mpcbb3_seccfgr51::MPCBB3_SECCFGR51rs>;
#[doc = "MPCBBx security configuration for super-block x register"]
pub mod mpcbb3_seccfgr51;
#[doc = "MPCBB3_PRIVCFGR0 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr0`]
module"]
pub type MPCBB3_PRIVCFGR0 = crate::Reg<mpcbb3_privcfgr0::MPCBB3_PRIVCFGR0rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr0;
#[doc = "MPCBB3_PRIVCFGR1 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr1`]
module"]
pub type MPCBB3_PRIVCFGR1 = crate::Reg<mpcbb3_privcfgr1::MPCBB3_PRIVCFGR1rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr1;
#[doc = "MPCBB3_PRIVCFGR2 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr2`]
module"]
pub type MPCBB3_PRIVCFGR2 = crate::Reg<mpcbb3_privcfgr2::MPCBB3_PRIVCFGR2rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr2;
#[doc = "MPCBB3_PRIVCFGR3 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr3`]
module"]
pub type MPCBB3_PRIVCFGR3 = crate::Reg<mpcbb3_privcfgr3::MPCBB3_PRIVCFGR3rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr3;
#[doc = "MPCBB3_PRIVCFGR4 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr4`]
module"]
pub type MPCBB3_PRIVCFGR4 = crate::Reg<mpcbb3_privcfgr4::MPCBB3_PRIVCFGR4rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr4;
#[doc = "MPCBB3_PRIVCFGR5 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr5`]
module"]
pub type MPCBB3_PRIVCFGR5 = crate::Reg<mpcbb3_privcfgr5::MPCBB3_PRIVCFGR5rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr5;
#[doc = "MPCBB3_PRIVCFGR6 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr6`]
module"]
pub type MPCBB3_PRIVCFGR6 = crate::Reg<mpcbb3_privcfgr6::MPCBB3_PRIVCFGR6rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr6;
#[doc = "MPCBB3_PRIVCFGR7 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr7`]
module"]
pub type MPCBB3_PRIVCFGR7 = crate::Reg<mpcbb3_privcfgr7::MPCBB3_PRIVCFGR7rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr7;
#[doc = "MPCBB3_PRIVCFGR8 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr8`]
module"]
pub type MPCBB3_PRIVCFGR8 = crate::Reg<mpcbb3_privcfgr8::MPCBB3_PRIVCFGR8rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr8;
#[doc = "MPCBB3_PRIVCFGR9 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr9`]
module"]
pub type MPCBB3_PRIVCFGR9 = crate::Reg<mpcbb3_privcfgr9::MPCBB3_PRIVCFGR9rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr9;
#[doc = "MPCBB3_PRIVCFGR10 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr10`]
module"]
pub type MPCBB3_PRIVCFGR10 = crate::Reg<mpcbb3_privcfgr10::MPCBB3_PRIVCFGR10rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr10;
#[doc = "MPCBB3_PRIVCFGR11 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr11`]
module"]
pub type MPCBB3_PRIVCFGR11 = crate::Reg<mpcbb3_privcfgr11::MPCBB3_PRIVCFGR11rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr11;
#[doc = "MPCBB3_PRIVCFGR12 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr12`]
module"]
pub type MPCBB3_PRIVCFGR12 = crate::Reg<mpcbb3_privcfgr12::MPCBB3_PRIVCFGR12rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr12;
#[doc = "MPCBB3_PRIVCFGR13 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr13`]
module"]
pub type MPCBB3_PRIVCFGR13 = crate::Reg<mpcbb3_privcfgr13::MPCBB3_PRIVCFGR13rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr13;
#[doc = "MPCBB3_PRIVCFGR14 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr14`]
module"]
pub type MPCBB3_PRIVCFGR14 = crate::Reg<mpcbb3_privcfgr14::MPCBB3_PRIVCFGR14rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr14;
#[doc = "MPCBB3_PRIVCFGR15 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr15`]
module"]
pub type MPCBB3_PRIVCFGR15 = crate::Reg<mpcbb3_privcfgr15::MPCBB3_PRIVCFGR15rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr15;
#[doc = "MPCBB3_PRIVCFGR16 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr16`]
module"]
pub type MPCBB3_PRIVCFGR16 = crate::Reg<mpcbb3_privcfgr16::MPCBB3_PRIVCFGR16rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr16;
#[doc = "MPCBB3_PRIVCFGR17 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr17`]
module"]
pub type MPCBB3_PRIVCFGR17 = crate::Reg<mpcbb3_privcfgr17::MPCBB3_PRIVCFGR17rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr17;
#[doc = "MPCBB3_PRIVCFGR18 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr18`]
module"]
pub type MPCBB3_PRIVCFGR18 = crate::Reg<mpcbb3_privcfgr18::MPCBB3_PRIVCFGR18rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr18;
#[doc = "MPCBB3_PRIVCFGR19 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr19`]
module"]
pub type MPCBB3_PRIVCFGR19 = crate::Reg<mpcbb3_privcfgr19::MPCBB3_PRIVCFGR19rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr19;
#[doc = "MPCBB3_PRIVCFGR20 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr20`]
module"]
pub type MPCBB3_PRIVCFGR20 = crate::Reg<mpcbb3_privcfgr20::MPCBB3_PRIVCFGR20rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr20;
#[doc = "MPCBB3_PRIVCFGR21 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr21`]
module"]
pub type MPCBB3_PRIVCFGR21 = crate::Reg<mpcbb3_privcfgr21::MPCBB3_PRIVCFGR21rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr21;
#[doc = "MPCBB3_PRIVCFGR22 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr22`]
module"]
pub type MPCBB3_PRIVCFGR22 = crate::Reg<mpcbb3_privcfgr22::MPCBB3_PRIVCFGR22rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr22;
#[doc = "MPCBB3_PRIVCFGR23 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr23`]
module"]
pub type MPCBB3_PRIVCFGR23 = crate::Reg<mpcbb3_privcfgr23::MPCBB3_PRIVCFGR23rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr23;
#[doc = "MPCBB3_PRIVCFGR24 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr24`]
module"]
pub type MPCBB3_PRIVCFGR24 = crate::Reg<mpcbb3_privcfgr24::MPCBB3_PRIVCFGR24rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr24;
#[doc = "MPCBB3_PRIVCFGR25 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr25`]
module"]
pub type MPCBB3_PRIVCFGR25 = crate::Reg<mpcbb3_privcfgr25::MPCBB3_PRIVCFGR25rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr25;
#[doc = "MPCBB3_PRIVCFGR26 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr26`]
module"]
pub type MPCBB3_PRIVCFGR26 = crate::Reg<mpcbb3_privcfgr26::MPCBB3_PRIVCFGR26rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr26;
#[doc = "MPCBB3_PRIVCFGR27 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr27`]
module"]
pub type MPCBB3_PRIVCFGR27 = crate::Reg<mpcbb3_privcfgr27::MPCBB3_PRIVCFGR27rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr27;
#[doc = "MPCBB3_PRIVCFGR28 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr28`]
module"]
pub type MPCBB3_PRIVCFGR28 = crate::Reg<mpcbb3_privcfgr28::MPCBB3_PRIVCFGR28rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr28;
#[doc = "MPCBB3_PRIVCFGR29 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr29`]
module"]
pub type MPCBB3_PRIVCFGR29 = crate::Reg<mpcbb3_privcfgr29::MPCBB3_PRIVCFGR29rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr29;
#[doc = "MPCBB3_PRIVCFGR30 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr30`]
module"]
pub type MPCBB3_PRIVCFGR30 = crate::Reg<mpcbb3_privcfgr30::MPCBB3_PRIVCFGR30rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr30;
#[doc = "MPCBB3_PRIVCFGR31 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr31`]
module"]
pub type MPCBB3_PRIVCFGR31 = crate::Reg<mpcbb3_privcfgr31::MPCBB3_PRIVCFGR31rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr31;
#[doc = "MPCBB3_PRIVCFGR32 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr32`]
module"]
pub type MPCBB3_PRIVCFGR32 = crate::Reg<mpcbb3_privcfgr32::MPCBB3_PRIVCFGR32rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr32;
#[doc = "MPCBB3_PRIVCFGR33 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr33`]
module"]
pub type MPCBB3_PRIVCFGR33 = crate::Reg<mpcbb3_privcfgr33::MPCBB3_PRIVCFGR33rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr33;
#[doc = "MPCBB3_PRIVCFGR34 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr34`]
module"]
pub type MPCBB3_PRIVCFGR34 = crate::Reg<mpcbb3_privcfgr34::MPCBB3_PRIVCFGR34rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr34;
#[doc = "MPCBB3_PRIVCFGR35 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr35`]
module"]
pub type MPCBB3_PRIVCFGR35 = crate::Reg<mpcbb3_privcfgr35::MPCBB3_PRIVCFGR35rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr35;
#[doc = "MPCBB3_PRIVCFGR36 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr36`]
module"]
pub type MPCBB3_PRIVCFGR36 = crate::Reg<mpcbb3_privcfgr36::MPCBB3_PRIVCFGR36rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr36;
#[doc = "MPCBB3_PRIVCFGR37 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr37`]
module"]
pub type MPCBB3_PRIVCFGR37 = crate::Reg<mpcbb3_privcfgr37::MPCBB3_PRIVCFGR37rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr37;
#[doc = "MPCBB3_PRIVCFGR38 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr38`]
module"]
pub type MPCBB3_PRIVCFGR38 = crate::Reg<mpcbb3_privcfgr38::MPCBB3_PRIVCFGR38rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr38;
#[doc = "MPCBB3_PRIVCFGR39 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr39`]
module"]
pub type MPCBB3_PRIVCFGR39 = crate::Reg<mpcbb3_privcfgr39::MPCBB3_PRIVCFGR39rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr39;
#[doc = "MPCBB3_PRIVCFGR40 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr40`]
module"]
pub type MPCBB3_PRIVCFGR40 = crate::Reg<mpcbb3_privcfgr40::MPCBB3_PRIVCFGR40rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr40;
#[doc = "MPCBB3_PRIVCFGR41 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr41`]
module"]
pub type MPCBB3_PRIVCFGR41 = crate::Reg<mpcbb3_privcfgr41::MPCBB3_PRIVCFGR41rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr41;
#[doc = "MPCBB3_PRIVCFGR42 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr42`]
module"]
pub type MPCBB3_PRIVCFGR42 = crate::Reg<mpcbb3_privcfgr42::MPCBB3_PRIVCFGR42rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr42;
#[doc = "MPCBB3_PRIVCFGR43 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr43`]
module"]
pub type MPCBB3_PRIVCFGR43 = crate::Reg<mpcbb3_privcfgr43::MPCBB3_PRIVCFGR43rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr43;
#[doc = "MPCBB3_PRIVCFGR44 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr44`]
module"]
pub type MPCBB3_PRIVCFGR44 = crate::Reg<mpcbb3_privcfgr44::MPCBB3_PRIVCFGR44rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr44;
#[doc = "MPCBB3_PRIVCFGR45 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr45`]
module"]
pub type MPCBB3_PRIVCFGR45 = crate::Reg<mpcbb3_privcfgr45::MPCBB3_PRIVCFGR45rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr45;
#[doc = "MPCBB3_PRIVCFGR46 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr46`]
module"]
pub type MPCBB3_PRIVCFGR46 = crate::Reg<mpcbb3_privcfgr46::MPCBB3_PRIVCFGR46rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr46;
#[doc = "MPCBB3_PRIVCFGR47 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr47`]
module"]
pub type MPCBB3_PRIVCFGR47 = crate::Reg<mpcbb3_privcfgr47::MPCBB3_PRIVCFGR47rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr47;
#[doc = "MPCBB3_PRIVCFGR48 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr48`]
module"]
pub type MPCBB3_PRIVCFGR48 = crate::Reg<mpcbb3_privcfgr48::MPCBB3_PRIVCFGR48rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr48;
#[doc = "MPCBB3_PRIVCFGR49 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr49`]
module"]
pub type MPCBB3_PRIVCFGR49 = crate::Reg<mpcbb3_privcfgr49::MPCBB3_PRIVCFGR49rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr49;
#[doc = "MPCBB3_PRIVCFGR50 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr50`]
module"]
pub type MPCBB3_PRIVCFGR50 = crate::Reg<mpcbb3_privcfgr50::MPCBB3_PRIVCFGR50rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr50;
#[doc = "MPCBB3_PRIVCFGR51 (rw) register accessor: MPCBB privileged configuration for super-block x register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb3_privcfgr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb3_privcfgr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb3_privcfgr51`]
module"]
pub type MPCBB3_PRIVCFGR51 = crate::Reg<mpcbb3_privcfgr51::MPCBB3_PRIVCFGR51rs>;
#[doc = "MPCBB privileged configuration for super-block x register"]
pub mod mpcbb3_privcfgr51;
