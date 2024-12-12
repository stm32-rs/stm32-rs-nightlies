#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mpcbb5_cr: MPCBB5_CR,
    _reserved1: [u8; 0x0c],
    mpcbb5_cfglock1: MPCBB5_CFGLOCK1,
    mpcbb5_cfglock2: MPCBB5_CFGLOCK2,
    _reserved3: [u8; 0xe8],
    mpcbb5_seccfgr0: MPCBB5_SECCFGR0,
    mpcbb5_seccfgr1: MPCBB5_SECCFGR1,
    mpcbb5_seccfgr2: MPCBB5_SECCFGR2,
    mpcbb5_seccfgr3: MPCBB5_SECCFGR3,
    mpcbb5_seccfgr4: MPCBB5_SECCFGR4,
    mpcbb5_seccfgr5: MPCBB5_SECCFGR5,
    mpcbb5_seccfgr6: MPCBB5_SECCFGR6,
    mpcbb5_seccfgr7: MPCBB5_SECCFGR7,
    mpcbb5_seccfgr8: MPCBB5_SECCFGR8,
    mpcbb5_seccfgr9: MPCBB5_SECCFGR9,
    mpcbb5_seccfgr10: MPCBB5_SECCFGR10,
    mpcbb5_seccfgr11: MPCBB5_SECCFGR11,
    mpcbb5_seccfgr12: MPCBB5_SECCFGR12,
    mpcbb5_seccfgr13: MPCBB5_SECCFGR13,
    mpcbb5_seccfgr14: MPCBB5_SECCFGR14,
    mpcbb5_seccfgr15: MPCBB5_SECCFGR15,
    mpcbb5_seccfgr16: MPCBB5_SECCFGR16,
    mpcbb5_seccfgr17: MPCBB5_SECCFGR17,
    mpcbb5_seccfgr18: MPCBB5_SECCFGR18,
    mpcbb5_seccfgr19: MPCBB5_SECCFGR19,
    mpcbb5_seccfgr20: MPCBB5_SECCFGR20,
    mpcbb5_seccfgr21: MPCBB5_SECCFGR21,
    mpcbb5_seccfgr22: MPCBB5_SECCFGR22,
    mpcbb5_seccfgr23: MPCBB5_SECCFGR23,
    mpcbb5_seccfgr24: MPCBB5_SECCFGR24,
    mpcbb5_seccfgr25: MPCBB5_SECCFGR25,
    mpcbb5_seccfgr26: MPCBB5_SECCFGR26,
    mpcbb5_seccfgr27: MPCBB5_SECCFGR27,
    mpcbb5_seccfgr28: MPCBB5_SECCFGR28,
    mpcbb5_seccfgr29: MPCBB5_SECCFGR29,
    mpcbb5_seccfgr30: MPCBB5_SECCFGR30,
    mpcbb5_seccfgr31: MPCBB5_SECCFGR31,
    mpcbb5_seccfgr32: MPCBB5_SECCFGR32,
    mpcbb5_seccfgr33: MPCBB5_SECCFGR33,
    mpcbb5_seccfgr34: MPCBB5_SECCFGR34,
    mpcbb5_seccfgr35: MPCBB5_SECCFGR35,
    mpcbb5_seccfgr36: MPCBB5_SECCFGR36,
    mpcbb5_seccfgr37: MPCBB5_SECCFGR37,
    mpcbb5_seccfgr38: MPCBB5_SECCFGR38,
    mpcbb5_seccfgr39: MPCBB5_SECCFGR39,
    mpcbb5_seccfgr40: MPCBB5_SECCFGR40,
    mpcbb5_seccfgr41: MPCBB5_SECCFGR41,
    mpcbb5_seccfgr42: MPCBB5_SECCFGR42,
    mpcbb5_seccfgr43: MPCBB5_SECCFGR43,
    mpcbb5_seccfgr44: MPCBB5_SECCFGR44,
    mpcbb5_seccfgr45: MPCBB5_SECCFGR45,
    mpcbb5_seccfgr46: MPCBB5_SECCFGR46,
    mpcbb5_seccfgr47: MPCBB5_SECCFGR47,
    mpcbb5_seccfgr48: MPCBB5_SECCFGR48,
    mpcbb5_seccfgr49: MPCBB5_SECCFGR49,
    mpcbb5_seccfgr50: MPCBB5_SECCFGR50,
    mpcbb5_seccfgr51: MPCBB5_SECCFGR51,
    _reserved55: [u8; 0x30],
    mpcbb5_privcfgr0: MPCBB5_PRIVCFGR0,
    mpcbb5_privcfgr1: MPCBB5_PRIVCFGR1,
    mpcbb5_privcfgr2: MPCBB5_PRIVCFGR2,
    mpcbb5_privcfgr3: MPCBB5_PRIVCFGR3,
    mpcbb5_privcfgr4: MPCBB5_PRIVCFGR4,
    mpcbb5_privcfgr5: MPCBB5_PRIVCFGR5,
    mpcbb5_privcfgr6: MPCBB5_PRIVCFGR6,
    mpcbb5_privcfgr7: MPCBB5_PRIVCFGR7,
    mpcbb5_privcfgr8: MPCBB5_PRIVCFGR8,
    mpcbb5_privcfgr9: MPCBB5_PRIVCFGR9,
    mpcbb5_privcfgr10: MPCBB5_PRIVCFGR10,
    mpcbb5_privcfgr11: MPCBB5_PRIVCFGR11,
    mpcbb5_privcfgr12: MPCBB5_PRIVCFGR12,
    mpcbb5_privcfgr13: MPCBB5_PRIVCFGR13,
    mpcbb5_privcfgr14: MPCBB5_PRIVCFGR14,
    mpcbb5_privcfgr15: MPCBB5_PRIVCFGR15,
    mpcbb5_privcfgr16: MPCBB5_PRIVCFGR16,
    mpcbb5_privcfgr17: MPCBB5_PRIVCFGR17,
    mpcbb5_privcfgr18: MPCBB5_PRIVCFGR18,
    mpcbb5_privcfgr19: MPCBB5_PRIVCFGR19,
    mpcbb5_privcfgr20: MPCBB5_PRIVCFGR20,
    mpcbb5_privcfgr21: MPCBB5_PRIVCFGR21,
    mpcbb5_privcfgr22: MPCBB5_PRIVCFGR22,
    mpcbb5_privcfgr23: MPCBB5_PRIVCFGR23,
    mpcbb5_privcfgr24: MPCBB5_PRIVCFGR24,
    mpcbb5_privcfgr25: MPCBB5_PRIVCFGR25,
    mpcbb5_privcfgr26: MPCBB5_PRIVCFGR26,
    mpcbb5_privcfgr27: MPCBB5_PRIVCFGR27,
    mpcbb5_privcfgr28: MPCBB5_PRIVCFGR28,
    mpcbb5_privcfgr29: MPCBB5_PRIVCFGR29,
    mpcbb5_privcfgr30: MPCBB5_PRIVCFGR30,
    mpcbb5_privcfgr31: MPCBB5_PRIVCFGR31,
    mpcbb5_privcfgr32: MPCBB5_PRIVCFGR32,
    mpcbb5_privcfgr33: MPCBB5_PRIVCFGR33,
    mpcbb5_privcfgr34: MPCBB5_PRIVCFGR34,
    mpcbb5_privcfgr35: MPCBB5_PRIVCFGR35,
    mpcbb5_privcfgr36: MPCBB5_PRIVCFGR36,
    mpcbb5_privcfgr37: MPCBB5_PRIVCFGR37,
    mpcbb5_privcfgr38: MPCBB5_PRIVCFGR38,
    mpcbb5_privcfgr39: MPCBB5_PRIVCFGR39,
    mpcbb5_privcfgr40: MPCBB5_PRIVCFGR40,
    mpcbb5_privcfgr41: MPCBB5_PRIVCFGR41,
    mpcbb5_privcfgr42: MPCBB5_PRIVCFGR42,
    mpcbb5_privcfgr43: MPCBB5_PRIVCFGR43,
    mpcbb5_privcfgr44: MPCBB5_PRIVCFGR44,
    mpcbb5_privcfgr45: MPCBB5_PRIVCFGR45,
    mpcbb5_privcfgr46: MPCBB5_PRIVCFGR46,
    mpcbb5_privcfgr47: MPCBB5_PRIVCFGR47,
    mpcbb5_privcfgr48: MPCBB5_PRIVCFGR48,
    mpcbb5_privcfgr49: MPCBB5_PRIVCFGR49,
    mpcbb5_privcfgr50: MPCBB5_PRIVCFGR50,
    mpcbb5_privcfgr51: MPCBB5_PRIVCFGR51,
}
impl RegisterBlock {
    ///0x00 - MPCBB control register
    #[inline(always)]
    pub const fn mpcbb5_cr(&self) -> &MPCBB5_CR {
        &self.mpcbb5_cr
    }
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    #[inline(always)]
    pub const fn mpcbb5_cfglock1(&self) -> &MPCBB5_CFGLOCK1 {
        &self.mpcbb5_cfglock1
    }
    ///0x14 - GTZC1 SRAMz MPCBB configuration lock register 2
    #[inline(always)]
    pub const fn mpcbb5_cfglock2(&self) -> &MPCBB5_CFGLOCK2 {
        &self.mpcbb5_cfglock2
    }
    ///0x100 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr0(&self) -> &MPCBB5_SECCFGR0 {
        &self.mpcbb5_seccfgr0
    }
    ///0x104 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr1(&self) -> &MPCBB5_SECCFGR1 {
        &self.mpcbb5_seccfgr1
    }
    ///0x108 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr2(&self) -> &MPCBB5_SECCFGR2 {
        &self.mpcbb5_seccfgr2
    }
    ///0x10c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr3(&self) -> &MPCBB5_SECCFGR3 {
        &self.mpcbb5_seccfgr3
    }
    ///0x110 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr4(&self) -> &MPCBB5_SECCFGR4 {
        &self.mpcbb5_seccfgr4
    }
    ///0x114 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr5(&self) -> &MPCBB5_SECCFGR5 {
        &self.mpcbb5_seccfgr5
    }
    ///0x118 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr6(&self) -> &MPCBB5_SECCFGR6 {
        &self.mpcbb5_seccfgr6
    }
    ///0x11c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr7(&self) -> &MPCBB5_SECCFGR7 {
        &self.mpcbb5_seccfgr7
    }
    ///0x120 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr8(&self) -> &MPCBB5_SECCFGR8 {
        &self.mpcbb5_seccfgr8
    }
    ///0x124 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr9(&self) -> &MPCBB5_SECCFGR9 {
        &self.mpcbb5_seccfgr9
    }
    ///0x128 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr10(&self) -> &MPCBB5_SECCFGR10 {
        &self.mpcbb5_seccfgr10
    }
    ///0x12c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr11(&self) -> &MPCBB5_SECCFGR11 {
        &self.mpcbb5_seccfgr11
    }
    ///0x130 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr12(&self) -> &MPCBB5_SECCFGR12 {
        &self.mpcbb5_seccfgr12
    }
    ///0x134 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr13(&self) -> &MPCBB5_SECCFGR13 {
        &self.mpcbb5_seccfgr13
    }
    ///0x138 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr14(&self) -> &MPCBB5_SECCFGR14 {
        &self.mpcbb5_seccfgr14
    }
    ///0x13c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr15(&self) -> &MPCBB5_SECCFGR15 {
        &self.mpcbb5_seccfgr15
    }
    ///0x140 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr16(&self) -> &MPCBB5_SECCFGR16 {
        &self.mpcbb5_seccfgr16
    }
    ///0x144 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr17(&self) -> &MPCBB5_SECCFGR17 {
        &self.mpcbb5_seccfgr17
    }
    ///0x148 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr18(&self) -> &MPCBB5_SECCFGR18 {
        &self.mpcbb5_seccfgr18
    }
    ///0x14c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr19(&self) -> &MPCBB5_SECCFGR19 {
        &self.mpcbb5_seccfgr19
    }
    ///0x150 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr20(&self) -> &MPCBB5_SECCFGR20 {
        &self.mpcbb5_seccfgr20
    }
    ///0x154 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr21(&self) -> &MPCBB5_SECCFGR21 {
        &self.mpcbb5_seccfgr21
    }
    ///0x158 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr22(&self) -> &MPCBB5_SECCFGR22 {
        &self.mpcbb5_seccfgr22
    }
    ///0x15c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr23(&self) -> &MPCBB5_SECCFGR23 {
        &self.mpcbb5_seccfgr23
    }
    ///0x160 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr24(&self) -> &MPCBB5_SECCFGR24 {
        &self.mpcbb5_seccfgr24
    }
    ///0x164 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr25(&self) -> &MPCBB5_SECCFGR25 {
        &self.mpcbb5_seccfgr25
    }
    ///0x168 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr26(&self) -> &MPCBB5_SECCFGR26 {
        &self.mpcbb5_seccfgr26
    }
    ///0x16c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr27(&self) -> &MPCBB5_SECCFGR27 {
        &self.mpcbb5_seccfgr27
    }
    ///0x170 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr28(&self) -> &MPCBB5_SECCFGR28 {
        &self.mpcbb5_seccfgr28
    }
    ///0x174 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr29(&self) -> &MPCBB5_SECCFGR29 {
        &self.mpcbb5_seccfgr29
    }
    ///0x178 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr30(&self) -> &MPCBB5_SECCFGR30 {
        &self.mpcbb5_seccfgr30
    }
    ///0x17c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr31(&self) -> &MPCBB5_SECCFGR31 {
        &self.mpcbb5_seccfgr31
    }
    ///0x180 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr32(&self) -> &MPCBB5_SECCFGR32 {
        &self.mpcbb5_seccfgr32
    }
    ///0x184 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr33(&self) -> &MPCBB5_SECCFGR33 {
        &self.mpcbb5_seccfgr33
    }
    ///0x188 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr34(&self) -> &MPCBB5_SECCFGR34 {
        &self.mpcbb5_seccfgr34
    }
    ///0x18c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr35(&self) -> &MPCBB5_SECCFGR35 {
        &self.mpcbb5_seccfgr35
    }
    ///0x190 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr36(&self) -> &MPCBB5_SECCFGR36 {
        &self.mpcbb5_seccfgr36
    }
    ///0x194 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr37(&self) -> &MPCBB5_SECCFGR37 {
        &self.mpcbb5_seccfgr37
    }
    ///0x198 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr38(&self) -> &MPCBB5_SECCFGR38 {
        &self.mpcbb5_seccfgr38
    }
    ///0x19c - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr39(&self) -> &MPCBB5_SECCFGR39 {
        &self.mpcbb5_seccfgr39
    }
    ///0x1a0 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr40(&self) -> &MPCBB5_SECCFGR40 {
        &self.mpcbb5_seccfgr40
    }
    ///0x1a4 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr41(&self) -> &MPCBB5_SECCFGR41 {
        &self.mpcbb5_seccfgr41
    }
    ///0x1a8 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr42(&self) -> &MPCBB5_SECCFGR42 {
        &self.mpcbb5_seccfgr42
    }
    ///0x1ac - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr43(&self) -> &MPCBB5_SECCFGR43 {
        &self.mpcbb5_seccfgr43
    }
    ///0x1b0 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr44(&self) -> &MPCBB5_SECCFGR44 {
        &self.mpcbb5_seccfgr44
    }
    ///0x1b4 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr45(&self) -> &MPCBB5_SECCFGR45 {
        &self.mpcbb5_seccfgr45
    }
    ///0x1b8 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr46(&self) -> &MPCBB5_SECCFGR46 {
        &self.mpcbb5_seccfgr46
    }
    ///0x1bc - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr47(&self) -> &MPCBB5_SECCFGR47 {
        &self.mpcbb5_seccfgr47
    }
    ///0x1c0 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr48(&self) -> &MPCBB5_SECCFGR48 {
        &self.mpcbb5_seccfgr48
    }
    ///0x1c4 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr49(&self) -> &MPCBB5_SECCFGR49 {
        &self.mpcbb5_seccfgr49
    }
    ///0x1c8 - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr50(&self) -> &MPCBB5_SECCFGR50 {
        &self.mpcbb5_seccfgr50
    }
    ///0x1cc - MPCBBx security configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_seccfgr51(&self) -> &MPCBB5_SECCFGR51 {
        &self.mpcbb5_seccfgr51
    }
    ///0x200 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr0(&self) -> &MPCBB5_PRIVCFGR0 {
        &self.mpcbb5_privcfgr0
    }
    ///0x204 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr1(&self) -> &MPCBB5_PRIVCFGR1 {
        &self.mpcbb5_privcfgr1
    }
    ///0x208 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr2(&self) -> &MPCBB5_PRIVCFGR2 {
        &self.mpcbb5_privcfgr2
    }
    ///0x20c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr3(&self) -> &MPCBB5_PRIVCFGR3 {
        &self.mpcbb5_privcfgr3
    }
    ///0x210 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr4(&self) -> &MPCBB5_PRIVCFGR4 {
        &self.mpcbb5_privcfgr4
    }
    ///0x214 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr5(&self) -> &MPCBB5_PRIVCFGR5 {
        &self.mpcbb5_privcfgr5
    }
    ///0x218 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr6(&self) -> &MPCBB5_PRIVCFGR6 {
        &self.mpcbb5_privcfgr6
    }
    ///0x21c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr7(&self) -> &MPCBB5_PRIVCFGR7 {
        &self.mpcbb5_privcfgr7
    }
    ///0x220 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr8(&self) -> &MPCBB5_PRIVCFGR8 {
        &self.mpcbb5_privcfgr8
    }
    ///0x224 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr9(&self) -> &MPCBB5_PRIVCFGR9 {
        &self.mpcbb5_privcfgr9
    }
    ///0x228 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr10(&self) -> &MPCBB5_PRIVCFGR10 {
        &self.mpcbb5_privcfgr10
    }
    ///0x22c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr11(&self) -> &MPCBB5_PRIVCFGR11 {
        &self.mpcbb5_privcfgr11
    }
    ///0x230 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr12(&self) -> &MPCBB5_PRIVCFGR12 {
        &self.mpcbb5_privcfgr12
    }
    ///0x234 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr13(&self) -> &MPCBB5_PRIVCFGR13 {
        &self.mpcbb5_privcfgr13
    }
    ///0x238 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr14(&self) -> &MPCBB5_PRIVCFGR14 {
        &self.mpcbb5_privcfgr14
    }
    ///0x23c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr15(&self) -> &MPCBB5_PRIVCFGR15 {
        &self.mpcbb5_privcfgr15
    }
    ///0x240 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr16(&self) -> &MPCBB5_PRIVCFGR16 {
        &self.mpcbb5_privcfgr16
    }
    ///0x244 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr17(&self) -> &MPCBB5_PRIVCFGR17 {
        &self.mpcbb5_privcfgr17
    }
    ///0x248 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr18(&self) -> &MPCBB5_PRIVCFGR18 {
        &self.mpcbb5_privcfgr18
    }
    ///0x24c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr19(&self) -> &MPCBB5_PRIVCFGR19 {
        &self.mpcbb5_privcfgr19
    }
    ///0x250 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr20(&self) -> &MPCBB5_PRIVCFGR20 {
        &self.mpcbb5_privcfgr20
    }
    ///0x254 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr21(&self) -> &MPCBB5_PRIVCFGR21 {
        &self.mpcbb5_privcfgr21
    }
    ///0x258 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr22(&self) -> &MPCBB5_PRIVCFGR22 {
        &self.mpcbb5_privcfgr22
    }
    ///0x25c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr23(&self) -> &MPCBB5_PRIVCFGR23 {
        &self.mpcbb5_privcfgr23
    }
    ///0x260 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr24(&self) -> &MPCBB5_PRIVCFGR24 {
        &self.mpcbb5_privcfgr24
    }
    ///0x264 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr25(&self) -> &MPCBB5_PRIVCFGR25 {
        &self.mpcbb5_privcfgr25
    }
    ///0x268 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr26(&self) -> &MPCBB5_PRIVCFGR26 {
        &self.mpcbb5_privcfgr26
    }
    ///0x26c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr27(&self) -> &MPCBB5_PRIVCFGR27 {
        &self.mpcbb5_privcfgr27
    }
    ///0x270 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr28(&self) -> &MPCBB5_PRIVCFGR28 {
        &self.mpcbb5_privcfgr28
    }
    ///0x274 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr29(&self) -> &MPCBB5_PRIVCFGR29 {
        &self.mpcbb5_privcfgr29
    }
    ///0x278 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr30(&self) -> &MPCBB5_PRIVCFGR30 {
        &self.mpcbb5_privcfgr30
    }
    ///0x27c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr31(&self) -> &MPCBB5_PRIVCFGR31 {
        &self.mpcbb5_privcfgr31
    }
    ///0x280 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr32(&self) -> &MPCBB5_PRIVCFGR32 {
        &self.mpcbb5_privcfgr32
    }
    ///0x284 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr33(&self) -> &MPCBB5_PRIVCFGR33 {
        &self.mpcbb5_privcfgr33
    }
    ///0x288 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr34(&self) -> &MPCBB5_PRIVCFGR34 {
        &self.mpcbb5_privcfgr34
    }
    ///0x28c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr35(&self) -> &MPCBB5_PRIVCFGR35 {
        &self.mpcbb5_privcfgr35
    }
    ///0x290 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr36(&self) -> &MPCBB5_PRIVCFGR36 {
        &self.mpcbb5_privcfgr36
    }
    ///0x294 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr37(&self) -> &MPCBB5_PRIVCFGR37 {
        &self.mpcbb5_privcfgr37
    }
    ///0x298 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr38(&self) -> &MPCBB5_PRIVCFGR38 {
        &self.mpcbb5_privcfgr38
    }
    ///0x29c - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr39(&self) -> &MPCBB5_PRIVCFGR39 {
        &self.mpcbb5_privcfgr39
    }
    ///0x2a0 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr40(&self) -> &MPCBB5_PRIVCFGR40 {
        &self.mpcbb5_privcfgr40
    }
    ///0x2a4 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr41(&self) -> &MPCBB5_PRIVCFGR41 {
        &self.mpcbb5_privcfgr41
    }
    ///0x2a8 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr42(&self) -> &MPCBB5_PRIVCFGR42 {
        &self.mpcbb5_privcfgr42
    }
    ///0x2ac - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr43(&self) -> &MPCBB5_PRIVCFGR43 {
        &self.mpcbb5_privcfgr43
    }
    ///0x2b0 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr44(&self) -> &MPCBB5_PRIVCFGR44 {
        &self.mpcbb5_privcfgr44
    }
    ///0x2b4 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr45(&self) -> &MPCBB5_PRIVCFGR45 {
        &self.mpcbb5_privcfgr45
    }
    ///0x2b8 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr46(&self) -> &MPCBB5_PRIVCFGR46 {
        &self.mpcbb5_privcfgr46
    }
    ///0x2bc - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr47(&self) -> &MPCBB5_PRIVCFGR47 {
        &self.mpcbb5_privcfgr47
    }
    ///0x2c0 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr48(&self) -> &MPCBB5_PRIVCFGR48 {
        &self.mpcbb5_privcfgr48
    }
    ///0x2c4 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr49(&self) -> &MPCBB5_PRIVCFGR49 {
        &self.mpcbb5_privcfgr49
    }
    ///0x2c8 - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr50(&self) -> &MPCBB5_PRIVCFGR50 {
        &self.mpcbb5_privcfgr50
    }
    ///0x2cc - MPCBB privileged configuration for super-block x register
    #[inline(always)]
    pub const fn mpcbb5_privcfgr51(&self) -> &MPCBB5_PRIVCFGR51 {
        &self.mpcbb5_privcfgr51
    }
}
/**MPCBB5_CR (rw) register accessor: MPCBB control register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_CR)

For information about available fields see [`mod@mpcbb5_cr`]
module*/
pub type MPCBB5_CR = crate::Reg<mpcbb5_cr::MPCBB5_CRrs>;
///MPCBB control register
pub mod mpcbb5_cr;
/**MPCBB5_CFGLOCK1 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_cfglock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_cfglock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_CFGLOCK1)

For information about available fields see [`mod@mpcbb5_cfglock1`]
module*/
pub type MPCBB5_CFGLOCK1 = crate::Reg<mpcbb5_cfglock1::MPCBB5_CFGLOCK1rs>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod mpcbb5_cfglock1;
/**MPCBB5_CFGLOCK2 (rw) register accessor: GTZC1 SRAMz MPCBB configuration lock register 2

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_cfglock2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_cfglock2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_CFGLOCK2)

For information about available fields see [`mod@mpcbb5_cfglock2`]
module*/
pub type MPCBB5_CFGLOCK2 = crate::Reg<mpcbb5_cfglock2::MPCBB5_CFGLOCK2rs>;
///GTZC1 SRAMz MPCBB configuration lock register 2
pub mod mpcbb5_cfglock2;
/**MPCBB5_SECCFGR0 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR0)

For information about available fields see [`mod@mpcbb5_seccfgr0`]
module*/
pub type MPCBB5_SECCFGR0 = crate::Reg<mpcbb5_seccfgr0::MPCBB5_SECCFGR0rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr0;
/**MPCBB5_SECCFGR1 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR1)

For information about available fields see [`mod@mpcbb5_seccfgr1`]
module*/
pub type MPCBB5_SECCFGR1 = crate::Reg<mpcbb5_seccfgr1::MPCBB5_SECCFGR1rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr1;
/**MPCBB5_SECCFGR2 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR2)

For information about available fields see [`mod@mpcbb5_seccfgr2`]
module*/
pub type MPCBB5_SECCFGR2 = crate::Reg<mpcbb5_seccfgr2::MPCBB5_SECCFGR2rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr2;
/**MPCBB5_SECCFGR3 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR3)

For information about available fields see [`mod@mpcbb5_seccfgr3`]
module*/
pub type MPCBB5_SECCFGR3 = crate::Reg<mpcbb5_seccfgr3::MPCBB5_SECCFGR3rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr3;
/**MPCBB5_SECCFGR4 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR4)

For information about available fields see [`mod@mpcbb5_seccfgr4`]
module*/
pub type MPCBB5_SECCFGR4 = crate::Reg<mpcbb5_seccfgr4::MPCBB5_SECCFGR4rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr4;
/**MPCBB5_SECCFGR5 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR5)

For information about available fields see [`mod@mpcbb5_seccfgr5`]
module*/
pub type MPCBB5_SECCFGR5 = crate::Reg<mpcbb5_seccfgr5::MPCBB5_SECCFGR5rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr5;
/**MPCBB5_SECCFGR6 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR6)

For information about available fields see [`mod@mpcbb5_seccfgr6`]
module*/
pub type MPCBB5_SECCFGR6 = crate::Reg<mpcbb5_seccfgr6::MPCBB5_SECCFGR6rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr6;
/**MPCBB5_SECCFGR7 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR7)

For information about available fields see [`mod@mpcbb5_seccfgr7`]
module*/
pub type MPCBB5_SECCFGR7 = crate::Reg<mpcbb5_seccfgr7::MPCBB5_SECCFGR7rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr7;
/**MPCBB5_SECCFGR8 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR8)

For information about available fields see [`mod@mpcbb5_seccfgr8`]
module*/
pub type MPCBB5_SECCFGR8 = crate::Reg<mpcbb5_seccfgr8::MPCBB5_SECCFGR8rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr8;
/**MPCBB5_SECCFGR9 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR9)

For information about available fields see [`mod@mpcbb5_seccfgr9`]
module*/
pub type MPCBB5_SECCFGR9 = crate::Reg<mpcbb5_seccfgr9::MPCBB5_SECCFGR9rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr9;
/**MPCBB5_SECCFGR10 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR10)

For information about available fields see [`mod@mpcbb5_seccfgr10`]
module*/
pub type MPCBB5_SECCFGR10 = crate::Reg<mpcbb5_seccfgr10::MPCBB5_SECCFGR10rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr10;
/**MPCBB5_SECCFGR11 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR11)

For information about available fields see [`mod@mpcbb5_seccfgr11`]
module*/
pub type MPCBB5_SECCFGR11 = crate::Reg<mpcbb5_seccfgr11::MPCBB5_SECCFGR11rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr11;
/**MPCBB5_SECCFGR12 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR12)

For information about available fields see [`mod@mpcbb5_seccfgr12`]
module*/
pub type MPCBB5_SECCFGR12 = crate::Reg<mpcbb5_seccfgr12::MPCBB5_SECCFGR12rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr12;
/**MPCBB5_SECCFGR13 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR13)

For information about available fields see [`mod@mpcbb5_seccfgr13`]
module*/
pub type MPCBB5_SECCFGR13 = crate::Reg<mpcbb5_seccfgr13::MPCBB5_SECCFGR13rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr13;
/**MPCBB5_SECCFGR14 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR14)

For information about available fields see [`mod@mpcbb5_seccfgr14`]
module*/
pub type MPCBB5_SECCFGR14 = crate::Reg<mpcbb5_seccfgr14::MPCBB5_SECCFGR14rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr14;
/**MPCBB5_SECCFGR15 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR15)

For information about available fields see [`mod@mpcbb5_seccfgr15`]
module*/
pub type MPCBB5_SECCFGR15 = crate::Reg<mpcbb5_seccfgr15::MPCBB5_SECCFGR15rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr15;
/**MPCBB5_SECCFGR16 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR16)

For information about available fields see [`mod@mpcbb5_seccfgr16`]
module*/
pub type MPCBB5_SECCFGR16 = crate::Reg<mpcbb5_seccfgr16::MPCBB5_SECCFGR16rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr16;
/**MPCBB5_SECCFGR17 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR17)

For information about available fields see [`mod@mpcbb5_seccfgr17`]
module*/
pub type MPCBB5_SECCFGR17 = crate::Reg<mpcbb5_seccfgr17::MPCBB5_SECCFGR17rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr17;
/**MPCBB5_SECCFGR18 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR18)

For information about available fields see [`mod@mpcbb5_seccfgr18`]
module*/
pub type MPCBB5_SECCFGR18 = crate::Reg<mpcbb5_seccfgr18::MPCBB5_SECCFGR18rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr18;
/**MPCBB5_SECCFGR19 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR19)

For information about available fields see [`mod@mpcbb5_seccfgr19`]
module*/
pub type MPCBB5_SECCFGR19 = crate::Reg<mpcbb5_seccfgr19::MPCBB5_SECCFGR19rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr19;
/**MPCBB5_SECCFGR20 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR20)

For information about available fields see [`mod@mpcbb5_seccfgr20`]
module*/
pub type MPCBB5_SECCFGR20 = crate::Reg<mpcbb5_seccfgr20::MPCBB5_SECCFGR20rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr20;
/**MPCBB5_SECCFGR21 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR21)

For information about available fields see [`mod@mpcbb5_seccfgr21`]
module*/
pub type MPCBB5_SECCFGR21 = crate::Reg<mpcbb5_seccfgr21::MPCBB5_SECCFGR21rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr21;
/**MPCBB5_SECCFGR22 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR22)

For information about available fields see [`mod@mpcbb5_seccfgr22`]
module*/
pub type MPCBB5_SECCFGR22 = crate::Reg<mpcbb5_seccfgr22::MPCBB5_SECCFGR22rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr22;
/**MPCBB5_SECCFGR23 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR23)

For information about available fields see [`mod@mpcbb5_seccfgr23`]
module*/
pub type MPCBB5_SECCFGR23 = crate::Reg<mpcbb5_seccfgr23::MPCBB5_SECCFGR23rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr23;
/**MPCBB5_SECCFGR24 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR24)

For information about available fields see [`mod@mpcbb5_seccfgr24`]
module*/
pub type MPCBB5_SECCFGR24 = crate::Reg<mpcbb5_seccfgr24::MPCBB5_SECCFGR24rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr24;
/**MPCBB5_SECCFGR25 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR25)

For information about available fields see [`mod@mpcbb5_seccfgr25`]
module*/
pub type MPCBB5_SECCFGR25 = crate::Reg<mpcbb5_seccfgr25::MPCBB5_SECCFGR25rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr25;
/**MPCBB5_SECCFGR26 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR26)

For information about available fields see [`mod@mpcbb5_seccfgr26`]
module*/
pub type MPCBB5_SECCFGR26 = crate::Reg<mpcbb5_seccfgr26::MPCBB5_SECCFGR26rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr26;
/**MPCBB5_SECCFGR27 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR27)

For information about available fields see [`mod@mpcbb5_seccfgr27`]
module*/
pub type MPCBB5_SECCFGR27 = crate::Reg<mpcbb5_seccfgr27::MPCBB5_SECCFGR27rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr27;
/**MPCBB5_SECCFGR28 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR28)

For information about available fields see [`mod@mpcbb5_seccfgr28`]
module*/
pub type MPCBB5_SECCFGR28 = crate::Reg<mpcbb5_seccfgr28::MPCBB5_SECCFGR28rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr28;
/**MPCBB5_SECCFGR29 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR29)

For information about available fields see [`mod@mpcbb5_seccfgr29`]
module*/
pub type MPCBB5_SECCFGR29 = crate::Reg<mpcbb5_seccfgr29::MPCBB5_SECCFGR29rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr29;
/**MPCBB5_SECCFGR30 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR30)

For information about available fields see [`mod@mpcbb5_seccfgr30`]
module*/
pub type MPCBB5_SECCFGR30 = crate::Reg<mpcbb5_seccfgr30::MPCBB5_SECCFGR30rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr30;
/**MPCBB5_SECCFGR31 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR31)

For information about available fields see [`mod@mpcbb5_seccfgr31`]
module*/
pub type MPCBB5_SECCFGR31 = crate::Reg<mpcbb5_seccfgr31::MPCBB5_SECCFGR31rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr31;
/**MPCBB5_SECCFGR32 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR32)

For information about available fields see [`mod@mpcbb5_seccfgr32`]
module*/
pub type MPCBB5_SECCFGR32 = crate::Reg<mpcbb5_seccfgr32::MPCBB5_SECCFGR32rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr32;
/**MPCBB5_SECCFGR33 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR33)

For information about available fields see [`mod@mpcbb5_seccfgr33`]
module*/
pub type MPCBB5_SECCFGR33 = crate::Reg<mpcbb5_seccfgr33::MPCBB5_SECCFGR33rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr33;
/**MPCBB5_SECCFGR34 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR34)

For information about available fields see [`mod@mpcbb5_seccfgr34`]
module*/
pub type MPCBB5_SECCFGR34 = crate::Reg<mpcbb5_seccfgr34::MPCBB5_SECCFGR34rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr34;
/**MPCBB5_SECCFGR35 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR35)

For information about available fields see [`mod@mpcbb5_seccfgr35`]
module*/
pub type MPCBB5_SECCFGR35 = crate::Reg<mpcbb5_seccfgr35::MPCBB5_SECCFGR35rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr35;
/**MPCBB5_SECCFGR36 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR36)

For information about available fields see [`mod@mpcbb5_seccfgr36`]
module*/
pub type MPCBB5_SECCFGR36 = crate::Reg<mpcbb5_seccfgr36::MPCBB5_SECCFGR36rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr36;
/**MPCBB5_SECCFGR37 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR37)

For information about available fields see [`mod@mpcbb5_seccfgr37`]
module*/
pub type MPCBB5_SECCFGR37 = crate::Reg<mpcbb5_seccfgr37::MPCBB5_SECCFGR37rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr37;
/**MPCBB5_SECCFGR38 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR38)

For information about available fields see [`mod@mpcbb5_seccfgr38`]
module*/
pub type MPCBB5_SECCFGR38 = crate::Reg<mpcbb5_seccfgr38::MPCBB5_SECCFGR38rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr38;
/**MPCBB5_SECCFGR39 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR39)

For information about available fields see [`mod@mpcbb5_seccfgr39`]
module*/
pub type MPCBB5_SECCFGR39 = crate::Reg<mpcbb5_seccfgr39::MPCBB5_SECCFGR39rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr39;
/**MPCBB5_SECCFGR40 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR40)

For information about available fields see [`mod@mpcbb5_seccfgr40`]
module*/
pub type MPCBB5_SECCFGR40 = crate::Reg<mpcbb5_seccfgr40::MPCBB5_SECCFGR40rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr40;
/**MPCBB5_SECCFGR41 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR41)

For information about available fields see [`mod@mpcbb5_seccfgr41`]
module*/
pub type MPCBB5_SECCFGR41 = crate::Reg<mpcbb5_seccfgr41::MPCBB5_SECCFGR41rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr41;
/**MPCBB5_SECCFGR42 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR42)

For information about available fields see [`mod@mpcbb5_seccfgr42`]
module*/
pub type MPCBB5_SECCFGR42 = crate::Reg<mpcbb5_seccfgr42::MPCBB5_SECCFGR42rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr42;
/**MPCBB5_SECCFGR43 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR43)

For information about available fields see [`mod@mpcbb5_seccfgr43`]
module*/
pub type MPCBB5_SECCFGR43 = crate::Reg<mpcbb5_seccfgr43::MPCBB5_SECCFGR43rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr43;
/**MPCBB5_SECCFGR44 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR44)

For information about available fields see [`mod@mpcbb5_seccfgr44`]
module*/
pub type MPCBB5_SECCFGR44 = crate::Reg<mpcbb5_seccfgr44::MPCBB5_SECCFGR44rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr44;
/**MPCBB5_SECCFGR45 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR45)

For information about available fields see [`mod@mpcbb5_seccfgr45`]
module*/
pub type MPCBB5_SECCFGR45 = crate::Reg<mpcbb5_seccfgr45::MPCBB5_SECCFGR45rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr45;
/**MPCBB5_SECCFGR46 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR46)

For information about available fields see [`mod@mpcbb5_seccfgr46`]
module*/
pub type MPCBB5_SECCFGR46 = crate::Reg<mpcbb5_seccfgr46::MPCBB5_SECCFGR46rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr46;
/**MPCBB5_SECCFGR47 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR47)

For information about available fields see [`mod@mpcbb5_seccfgr47`]
module*/
pub type MPCBB5_SECCFGR47 = crate::Reg<mpcbb5_seccfgr47::MPCBB5_SECCFGR47rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr47;
/**MPCBB5_SECCFGR48 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR48)

For information about available fields see [`mod@mpcbb5_seccfgr48`]
module*/
pub type MPCBB5_SECCFGR48 = crate::Reg<mpcbb5_seccfgr48::MPCBB5_SECCFGR48rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr48;
/**MPCBB5_SECCFGR49 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR49)

For information about available fields see [`mod@mpcbb5_seccfgr49`]
module*/
pub type MPCBB5_SECCFGR49 = crate::Reg<mpcbb5_seccfgr49::MPCBB5_SECCFGR49rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr49;
/**MPCBB5_SECCFGR50 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR50)

For information about available fields see [`mod@mpcbb5_seccfgr50`]
module*/
pub type MPCBB5_SECCFGR50 = crate::Reg<mpcbb5_seccfgr50::MPCBB5_SECCFGR50rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr50;
/**MPCBB5_SECCFGR51 (rw) register accessor: MPCBBx security configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_seccfgr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_seccfgr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_SECCFGR51)

For information about available fields see [`mod@mpcbb5_seccfgr51`]
module*/
pub type MPCBB5_SECCFGR51 = crate::Reg<mpcbb5_seccfgr51::MPCBB5_SECCFGR51rs>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb5_seccfgr51;
/**MPCBB5_PRIVCFGR0 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR0)

For information about available fields see [`mod@mpcbb5_privcfgr0`]
module*/
pub type MPCBB5_PRIVCFGR0 = crate::Reg<mpcbb5_privcfgr0::MPCBB5_PRIVCFGR0rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr0;
/**MPCBB5_PRIVCFGR1 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR1)

For information about available fields see [`mod@mpcbb5_privcfgr1`]
module*/
pub type MPCBB5_PRIVCFGR1 = crate::Reg<mpcbb5_privcfgr1::MPCBB5_PRIVCFGR1rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr1;
/**MPCBB5_PRIVCFGR2 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR2)

For information about available fields see [`mod@mpcbb5_privcfgr2`]
module*/
pub type MPCBB5_PRIVCFGR2 = crate::Reg<mpcbb5_privcfgr2::MPCBB5_PRIVCFGR2rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr2;
/**MPCBB5_PRIVCFGR3 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR3)

For information about available fields see [`mod@mpcbb5_privcfgr3`]
module*/
pub type MPCBB5_PRIVCFGR3 = crate::Reg<mpcbb5_privcfgr3::MPCBB5_PRIVCFGR3rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr3;
/**MPCBB5_PRIVCFGR4 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR4)

For information about available fields see [`mod@mpcbb5_privcfgr4`]
module*/
pub type MPCBB5_PRIVCFGR4 = crate::Reg<mpcbb5_privcfgr4::MPCBB5_PRIVCFGR4rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr4;
/**MPCBB5_PRIVCFGR5 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR5)

For information about available fields see [`mod@mpcbb5_privcfgr5`]
module*/
pub type MPCBB5_PRIVCFGR5 = crate::Reg<mpcbb5_privcfgr5::MPCBB5_PRIVCFGR5rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr5;
/**MPCBB5_PRIVCFGR6 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR6)

For information about available fields see [`mod@mpcbb5_privcfgr6`]
module*/
pub type MPCBB5_PRIVCFGR6 = crate::Reg<mpcbb5_privcfgr6::MPCBB5_PRIVCFGR6rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr6;
/**MPCBB5_PRIVCFGR7 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR7)

For information about available fields see [`mod@mpcbb5_privcfgr7`]
module*/
pub type MPCBB5_PRIVCFGR7 = crate::Reg<mpcbb5_privcfgr7::MPCBB5_PRIVCFGR7rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr7;
/**MPCBB5_PRIVCFGR8 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR8)

For information about available fields see [`mod@mpcbb5_privcfgr8`]
module*/
pub type MPCBB5_PRIVCFGR8 = crate::Reg<mpcbb5_privcfgr8::MPCBB5_PRIVCFGR8rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr8;
/**MPCBB5_PRIVCFGR9 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR9)

For information about available fields see [`mod@mpcbb5_privcfgr9`]
module*/
pub type MPCBB5_PRIVCFGR9 = crate::Reg<mpcbb5_privcfgr9::MPCBB5_PRIVCFGR9rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr9;
/**MPCBB5_PRIVCFGR10 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR10)

For information about available fields see [`mod@mpcbb5_privcfgr10`]
module*/
pub type MPCBB5_PRIVCFGR10 = crate::Reg<mpcbb5_privcfgr10::MPCBB5_PRIVCFGR10rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr10;
/**MPCBB5_PRIVCFGR11 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR11)

For information about available fields see [`mod@mpcbb5_privcfgr11`]
module*/
pub type MPCBB5_PRIVCFGR11 = crate::Reg<mpcbb5_privcfgr11::MPCBB5_PRIVCFGR11rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr11;
/**MPCBB5_PRIVCFGR12 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR12)

For information about available fields see [`mod@mpcbb5_privcfgr12`]
module*/
pub type MPCBB5_PRIVCFGR12 = crate::Reg<mpcbb5_privcfgr12::MPCBB5_PRIVCFGR12rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr12;
/**MPCBB5_PRIVCFGR13 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR13)

For information about available fields see [`mod@mpcbb5_privcfgr13`]
module*/
pub type MPCBB5_PRIVCFGR13 = crate::Reg<mpcbb5_privcfgr13::MPCBB5_PRIVCFGR13rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr13;
/**MPCBB5_PRIVCFGR14 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR14)

For information about available fields see [`mod@mpcbb5_privcfgr14`]
module*/
pub type MPCBB5_PRIVCFGR14 = crate::Reg<mpcbb5_privcfgr14::MPCBB5_PRIVCFGR14rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr14;
/**MPCBB5_PRIVCFGR15 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR15)

For information about available fields see [`mod@mpcbb5_privcfgr15`]
module*/
pub type MPCBB5_PRIVCFGR15 = crate::Reg<mpcbb5_privcfgr15::MPCBB5_PRIVCFGR15rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr15;
/**MPCBB5_PRIVCFGR16 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR16)

For information about available fields see [`mod@mpcbb5_privcfgr16`]
module*/
pub type MPCBB5_PRIVCFGR16 = crate::Reg<mpcbb5_privcfgr16::MPCBB5_PRIVCFGR16rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr16;
/**MPCBB5_PRIVCFGR17 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR17)

For information about available fields see [`mod@mpcbb5_privcfgr17`]
module*/
pub type MPCBB5_PRIVCFGR17 = crate::Reg<mpcbb5_privcfgr17::MPCBB5_PRIVCFGR17rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr17;
/**MPCBB5_PRIVCFGR18 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR18)

For information about available fields see [`mod@mpcbb5_privcfgr18`]
module*/
pub type MPCBB5_PRIVCFGR18 = crate::Reg<mpcbb5_privcfgr18::MPCBB5_PRIVCFGR18rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr18;
/**MPCBB5_PRIVCFGR19 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR19)

For information about available fields see [`mod@mpcbb5_privcfgr19`]
module*/
pub type MPCBB5_PRIVCFGR19 = crate::Reg<mpcbb5_privcfgr19::MPCBB5_PRIVCFGR19rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr19;
/**MPCBB5_PRIVCFGR20 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR20)

For information about available fields see [`mod@mpcbb5_privcfgr20`]
module*/
pub type MPCBB5_PRIVCFGR20 = crate::Reg<mpcbb5_privcfgr20::MPCBB5_PRIVCFGR20rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr20;
/**MPCBB5_PRIVCFGR21 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR21)

For information about available fields see [`mod@mpcbb5_privcfgr21`]
module*/
pub type MPCBB5_PRIVCFGR21 = crate::Reg<mpcbb5_privcfgr21::MPCBB5_PRIVCFGR21rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr21;
/**MPCBB5_PRIVCFGR22 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR22)

For information about available fields see [`mod@mpcbb5_privcfgr22`]
module*/
pub type MPCBB5_PRIVCFGR22 = crate::Reg<mpcbb5_privcfgr22::MPCBB5_PRIVCFGR22rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr22;
/**MPCBB5_PRIVCFGR23 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR23)

For information about available fields see [`mod@mpcbb5_privcfgr23`]
module*/
pub type MPCBB5_PRIVCFGR23 = crate::Reg<mpcbb5_privcfgr23::MPCBB5_PRIVCFGR23rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr23;
/**MPCBB5_PRIVCFGR24 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR24)

For information about available fields see [`mod@mpcbb5_privcfgr24`]
module*/
pub type MPCBB5_PRIVCFGR24 = crate::Reg<mpcbb5_privcfgr24::MPCBB5_PRIVCFGR24rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr24;
/**MPCBB5_PRIVCFGR25 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR25)

For information about available fields see [`mod@mpcbb5_privcfgr25`]
module*/
pub type MPCBB5_PRIVCFGR25 = crate::Reg<mpcbb5_privcfgr25::MPCBB5_PRIVCFGR25rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr25;
/**MPCBB5_PRIVCFGR26 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR26)

For information about available fields see [`mod@mpcbb5_privcfgr26`]
module*/
pub type MPCBB5_PRIVCFGR26 = crate::Reg<mpcbb5_privcfgr26::MPCBB5_PRIVCFGR26rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr26;
/**MPCBB5_PRIVCFGR27 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR27)

For information about available fields see [`mod@mpcbb5_privcfgr27`]
module*/
pub type MPCBB5_PRIVCFGR27 = crate::Reg<mpcbb5_privcfgr27::MPCBB5_PRIVCFGR27rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr27;
/**MPCBB5_PRIVCFGR28 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR28)

For information about available fields see [`mod@mpcbb5_privcfgr28`]
module*/
pub type MPCBB5_PRIVCFGR28 = crate::Reg<mpcbb5_privcfgr28::MPCBB5_PRIVCFGR28rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr28;
/**MPCBB5_PRIVCFGR29 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR29)

For information about available fields see [`mod@mpcbb5_privcfgr29`]
module*/
pub type MPCBB5_PRIVCFGR29 = crate::Reg<mpcbb5_privcfgr29::MPCBB5_PRIVCFGR29rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr29;
/**MPCBB5_PRIVCFGR30 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR30)

For information about available fields see [`mod@mpcbb5_privcfgr30`]
module*/
pub type MPCBB5_PRIVCFGR30 = crate::Reg<mpcbb5_privcfgr30::MPCBB5_PRIVCFGR30rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr30;
/**MPCBB5_PRIVCFGR31 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR31)

For information about available fields see [`mod@mpcbb5_privcfgr31`]
module*/
pub type MPCBB5_PRIVCFGR31 = crate::Reg<mpcbb5_privcfgr31::MPCBB5_PRIVCFGR31rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr31;
/**MPCBB5_PRIVCFGR32 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR32)

For information about available fields see [`mod@mpcbb5_privcfgr32`]
module*/
pub type MPCBB5_PRIVCFGR32 = crate::Reg<mpcbb5_privcfgr32::MPCBB5_PRIVCFGR32rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr32;
/**MPCBB5_PRIVCFGR33 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR33)

For information about available fields see [`mod@mpcbb5_privcfgr33`]
module*/
pub type MPCBB5_PRIVCFGR33 = crate::Reg<mpcbb5_privcfgr33::MPCBB5_PRIVCFGR33rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr33;
/**MPCBB5_PRIVCFGR34 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR34)

For information about available fields see [`mod@mpcbb5_privcfgr34`]
module*/
pub type MPCBB5_PRIVCFGR34 = crate::Reg<mpcbb5_privcfgr34::MPCBB5_PRIVCFGR34rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr34;
/**MPCBB5_PRIVCFGR35 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR35)

For information about available fields see [`mod@mpcbb5_privcfgr35`]
module*/
pub type MPCBB5_PRIVCFGR35 = crate::Reg<mpcbb5_privcfgr35::MPCBB5_PRIVCFGR35rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr35;
/**MPCBB5_PRIVCFGR36 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR36)

For information about available fields see [`mod@mpcbb5_privcfgr36`]
module*/
pub type MPCBB5_PRIVCFGR36 = crate::Reg<mpcbb5_privcfgr36::MPCBB5_PRIVCFGR36rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr36;
/**MPCBB5_PRIVCFGR37 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR37)

For information about available fields see [`mod@mpcbb5_privcfgr37`]
module*/
pub type MPCBB5_PRIVCFGR37 = crate::Reg<mpcbb5_privcfgr37::MPCBB5_PRIVCFGR37rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr37;
/**MPCBB5_PRIVCFGR38 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR38)

For information about available fields see [`mod@mpcbb5_privcfgr38`]
module*/
pub type MPCBB5_PRIVCFGR38 = crate::Reg<mpcbb5_privcfgr38::MPCBB5_PRIVCFGR38rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr38;
/**MPCBB5_PRIVCFGR39 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR39)

For information about available fields see [`mod@mpcbb5_privcfgr39`]
module*/
pub type MPCBB5_PRIVCFGR39 = crate::Reg<mpcbb5_privcfgr39::MPCBB5_PRIVCFGR39rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr39;
/**MPCBB5_PRIVCFGR40 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR40)

For information about available fields see [`mod@mpcbb5_privcfgr40`]
module*/
pub type MPCBB5_PRIVCFGR40 = crate::Reg<mpcbb5_privcfgr40::MPCBB5_PRIVCFGR40rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr40;
/**MPCBB5_PRIVCFGR41 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR41)

For information about available fields see [`mod@mpcbb5_privcfgr41`]
module*/
pub type MPCBB5_PRIVCFGR41 = crate::Reg<mpcbb5_privcfgr41::MPCBB5_PRIVCFGR41rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr41;
/**MPCBB5_PRIVCFGR42 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR42)

For information about available fields see [`mod@mpcbb5_privcfgr42`]
module*/
pub type MPCBB5_PRIVCFGR42 = crate::Reg<mpcbb5_privcfgr42::MPCBB5_PRIVCFGR42rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr42;
/**MPCBB5_PRIVCFGR43 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR43)

For information about available fields see [`mod@mpcbb5_privcfgr43`]
module*/
pub type MPCBB5_PRIVCFGR43 = crate::Reg<mpcbb5_privcfgr43::MPCBB5_PRIVCFGR43rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr43;
/**MPCBB5_PRIVCFGR44 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR44)

For information about available fields see [`mod@mpcbb5_privcfgr44`]
module*/
pub type MPCBB5_PRIVCFGR44 = crate::Reg<mpcbb5_privcfgr44::MPCBB5_PRIVCFGR44rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr44;
/**MPCBB5_PRIVCFGR45 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR45)

For information about available fields see [`mod@mpcbb5_privcfgr45`]
module*/
pub type MPCBB5_PRIVCFGR45 = crate::Reg<mpcbb5_privcfgr45::MPCBB5_PRIVCFGR45rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr45;
/**MPCBB5_PRIVCFGR46 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR46)

For information about available fields see [`mod@mpcbb5_privcfgr46`]
module*/
pub type MPCBB5_PRIVCFGR46 = crate::Reg<mpcbb5_privcfgr46::MPCBB5_PRIVCFGR46rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr46;
/**MPCBB5_PRIVCFGR47 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR47)

For information about available fields see [`mod@mpcbb5_privcfgr47`]
module*/
pub type MPCBB5_PRIVCFGR47 = crate::Reg<mpcbb5_privcfgr47::MPCBB5_PRIVCFGR47rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr47;
/**MPCBB5_PRIVCFGR48 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR48)

For information about available fields see [`mod@mpcbb5_privcfgr48`]
module*/
pub type MPCBB5_PRIVCFGR48 = crate::Reg<mpcbb5_privcfgr48::MPCBB5_PRIVCFGR48rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr48;
/**MPCBB5_PRIVCFGR49 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR49)

For information about available fields see [`mod@mpcbb5_privcfgr49`]
module*/
pub type MPCBB5_PRIVCFGR49 = crate::Reg<mpcbb5_privcfgr49::MPCBB5_PRIVCFGR49rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr49;
/**MPCBB5_PRIVCFGR50 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR50)

For information about available fields see [`mod@mpcbb5_privcfgr50`]
module*/
pub type MPCBB5_PRIVCFGR50 = crate::Reg<mpcbb5_privcfgr50::MPCBB5_PRIVCFGR50rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr50;
/**MPCBB5_PRIVCFGR51 (rw) register accessor: MPCBB privileged configuration for super-block x register

You can [`read`](crate::Reg::read) this register and get [`mpcbb5_privcfgr51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcbb5_privcfgr51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#GTZC1_MPCBB5:MPCBB5_PRIVCFGR51)

For information about available fields see [`mod@mpcbb5_privcfgr51`]
module*/
pub type MPCBB5_PRIVCFGR51 = crate::Reg<mpcbb5_privcfgr51::MPCBB5_PRIVCFGR51rs>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb5_privcfgr51;
