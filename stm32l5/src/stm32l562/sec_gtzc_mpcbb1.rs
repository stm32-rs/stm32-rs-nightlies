#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mpcbb1_cr: MPCBB1_CR,
    _reserved1: [u8; 0x0c],
    mpcbb1_lckvtr1: MPCBB1_LCKVTR1,
    mpcbb1_lckvtr2: MPCBB1_LCKVTR2,
    _reserved3: [u8; 0xe8],
    mpcbb1_vctr0: MPCBB1_VCTR0,
    mpcbb1_vctr1: MPCBB1_VCTR1,
    mpcbb1_vctr2: MPCBB1_VCTR2,
    mpcbb1_vctr3: MPCBB1_VCTR3,
    mpcbb1_vctr4: MPCBB1_VCTR4,
    mpcbb1_vctr5: MPCBB1_VCTR5,
    mpcbb1_vctr6: MPCBB1_VCTR6,
    mpcbb1_vctr7: MPCBB1_VCTR7,
    mpcbb1_vctr8: MPCBB1_VCTR8,
    mpcbb1_vctr9: MPCBB1_VCTR9,
    mpcbb1_vctr10: MPCBB1_VCTR10,
    mpcbb1_vctr11: MPCBB1_VCTR11,
    mpcbb1_vctr12: MPCBB1_VCTR12,
    mpcbb1_vctr13: MPCBB1_VCTR13,
    mpcbb1_vctr14: MPCBB1_VCTR14,
    mpcbb1_vctr15: MPCBB1_VCTR15,
    mpcbb1_vctr16: MPCBB1_VCTR16,
    mpcbb1_vctr17: MPCBB1_VCTR17,
    mpcbb1_vctr18: MPCBB1_VCTR18,
    mpcbb1_vctr19: MPCBB1_VCTR19,
    mpcbb1_vctr20: MPCBB1_VCTR20,
    mpcbb1_vctr21: MPCBB1_VCTR21,
    mpcbb1_vctr22: MPCBB1_VCTR22,
    mpcbb1_vctr23: MPCBB1_VCTR23,
    mpcbb1_vctr24: MPCBB1_VCTR24,
    mpcbb1_vctr25: MPCBB1_VCTR25,
    mpcbb1_vctr26: MPCBB1_VCTR26,
    mpcbb1_vctr27: MPCBB1_VCTR27,
    mpcbb1_vctr28: MPCBB1_VCTR28,
    mpcbb1_vctr29: MPCBB1_VCTR29,
    mpcbb1_vctr30: MPCBB1_VCTR30,
    mpcbb1_vctr31: MPCBB1_VCTR31,
    mpcbb1_vctr32: MPCBB1_VCTR32,
    mpcbb1_vctr33: MPCBB1_VCTR33,
    mpcbb1_vctr34: MPCBB1_VCTR34,
    mpcbb1_vctr35: MPCBB1_VCTR35,
    mpcbb1_vctr36: MPCBB1_VCTR36,
    mpcbb1_vctr37: MPCBB1_VCTR37,
    mpcbb1_vctr38: MPCBB1_VCTR38,
    mpcbb1_vctr39: MPCBB1_VCTR39,
    mpcbb1_vctr40: MPCBB1_VCTR40,
    mpcbb1_vctr41: MPCBB1_VCTR41,
    mpcbb1_vctr42: MPCBB1_VCTR42,
    mpcbb1_vctr43: MPCBB1_VCTR43,
    mpcbb1_vctr44: MPCBB1_VCTR44,
    mpcbb1_vctr45: MPCBB1_VCTR45,
    mpcbb1_vctr46: MPCBB1_VCTR46,
    mpcbb1_vctr47: MPCBB1_VCTR47,
    mpcbb1_vctr48: MPCBB1_VCTR48,
    mpcbb1_vctr49: MPCBB1_VCTR49,
    mpcbb1_vctr50: MPCBB1_VCTR50,
    mpcbb1_vctr51: MPCBB1_VCTR51,
    mpcbb1_vctr52: MPCBB1_VCTR52,
    mpcbb1_vctr53: MPCBB1_VCTR53,
    mpcbb1_vctr54: MPCBB1_VCTR54,
    mpcbb1_vctr55: MPCBB1_VCTR55,
    mpcbb1_vctr56: MPCBB1_VCTR56,
    mpcbb1_vctr57: MPCBB1_VCTR57,
    mpcbb1_vctr58: MPCBB1_VCTR58,
    mpcbb1_vctr59: MPCBB1_VCTR59,
    mpcbb1_vctr60: MPCBB1_VCTR60,
    mpcbb1_vctr61: MPCBB1_VCTR61,
    mpcbb1_vctr62: MPCBB1_VCTR62,
    mpcbb1_vctr63: MPCBB1_VCTR63,
}
impl RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    #[inline(always)]
    pub const fn mpcbb1_cr(&self) -> &MPCBB1_CR {
        &self.mpcbb1_cr
    }
    #[doc = "0x10 - MPCBB control register"]
    #[inline(always)]
    pub const fn mpcbb1_lckvtr1(&self) -> &MPCBB1_LCKVTR1 {
        &self.mpcbb1_lckvtr1
    }
    #[doc = "0x14 - MPCBB control register"]
    #[inline(always)]
    pub const fn mpcbb1_lckvtr2(&self) -> &MPCBB1_LCKVTR2 {
        &self.mpcbb1_lckvtr2
    }
    #[doc = "0x100 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr0(&self) -> &MPCBB1_VCTR0 {
        &self.mpcbb1_vctr0
    }
    #[doc = "0x104 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr1(&self) -> &MPCBB1_VCTR1 {
        &self.mpcbb1_vctr1
    }
    #[doc = "0x108 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr2(&self) -> &MPCBB1_VCTR2 {
        &self.mpcbb1_vctr2
    }
    #[doc = "0x10c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr3(&self) -> &MPCBB1_VCTR3 {
        &self.mpcbb1_vctr3
    }
    #[doc = "0x110 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr4(&self) -> &MPCBB1_VCTR4 {
        &self.mpcbb1_vctr4
    }
    #[doc = "0x114 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr5(&self) -> &MPCBB1_VCTR5 {
        &self.mpcbb1_vctr5
    }
    #[doc = "0x118 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr6(&self) -> &MPCBB1_VCTR6 {
        &self.mpcbb1_vctr6
    }
    #[doc = "0x11c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr7(&self) -> &MPCBB1_VCTR7 {
        &self.mpcbb1_vctr7
    }
    #[doc = "0x120 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr8(&self) -> &MPCBB1_VCTR8 {
        &self.mpcbb1_vctr8
    }
    #[doc = "0x124 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr9(&self) -> &MPCBB1_VCTR9 {
        &self.mpcbb1_vctr9
    }
    #[doc = "0x128 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr10(&self) -> &MPCBB1_VCTR10 {
        &self.mpcbb1_vctr10
    }
    #[doc = "0x12c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr11(&self) -> &MPCBB1_VCTR11 {
        &self.mpcbb1_vctr11
    }
    #[doc = "0x130 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr12(&self) -> &MPCBB1_VCTR12 {
        &self.mpcbb1_vctr12
    }
    #[doc = "0x134 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr13(&self) -> &MPCBB1_VCTR13 {
        &self.mpcbb1_vctr13
    }
    #[doc = "0x138 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr14(&self) -> &MPCBB1_VCTR14 {
        &self.mpcbb1_vctr14
    }
    #[doc = "0x13c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr15(&self) -> &MPCBB1_VCTR15 {
        &self.mpcbb1_vctr15
    }
    #[doc = "0x140 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr16(&self) -> &MPCBB1_VCTR16 {
        &self.mpcbb1_vctr16
    }
    #[doc = "0x144 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr17(&self) -> &MPCBB1_VCTR17 {
        &self.mpcbb1_vctr17
    }
    #[doc = "0x148 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr18(&self) -> &MPCBB1_VCTR18 {
        &self.mpcbb1_vctr18
    }
    #[doc = "0x14c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr19(&self) -> &MPCBB1_VCTR19 {
        &self.mpcbb1_vctr19
    }
    #[doc = "0x150 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr20(&self) -> &MPCBB1_VCTR20 {
        &self.mpcbb1_vctr20
    }
    #[doc = "0x154 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr21(&self) -> &MPCBB1_VCTR21 {
        &self.mpcbb1_vctr21
    }
    #[doc = "0x158 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr22(&self) -> &MPCBB1_VCTR22 {
        &self.mpcbb1_vctr22
    }
    #[doc = "0x15c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr23(&self) -> &MPCBB1_VCTR23 {
        &self.mpcbb1_vctr23
    }
    #[doc = "0x160 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr24(&self) -> &MPCBB1_VCTR24 {
        &self.mpcbb1_vctr24
    }
    #[doc = "0x164 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr25(&self) -> &MPCBB1_VCTR25 {
        &self.mpcbb1_vctr25
    }
    #[doc = "0x168 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr26(&self) -> &MPCBB1_VCTR26 {
        &self.mpcbb1_vctr26
    }
    #[doc = "0x16c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr27(&self) -> &MPCBB1_VCTR27 {
        &self.mpcbb1_vctr27
    }
    #[doc = "0x170 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr28(&self) -> &MPCBB1_VCTR28 {
        &self.mpcbb1_vctr28
    }
    #[doc = "0x174 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr29(&self) -> &MPCBB1_VCTR29 {
        &self.mpcbb1_vctr29
    }
    #[doc = "0x178 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr30(&self) -> &MPCBB1_VCTR30 {
        &self.mpcbb1_vctr30
    }
    #[doc = "0x17c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr31(&self) -> &MPCBB1_VCTR31 {
        &self.mpcbb1_vctr31
    }
    #[doc = "0x180 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr32(&self) -> &MPCBB1_VCTR32 {
        &self.mpcbb1_vctr32
    }
    #[doc = "0x184 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr33(&self) -> &MPCBB1_VCTR33 {
        &self.mpcbb1_vctr33
    }
    #[doc = "0x188 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr34(&self) -> &MPCBB1_VCTR34 {
        &self.mpcbb1_vctr34
    }
    #[doc = "0x18c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr35(&self) -> &MPCBB1_VCTR35 {
        &self.mpcbb1_vctr35
    }
    #[doc = "0x190 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr36(&self) -> &MPCBB1_VCTR36 {
        &self.mpcbb1_vctr36
    }
    #[doc = "0x194 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr37(&self) -> &MPCBB1_VCTR37 {
        &self.mpcbb1_vctr37
    }
    #[doc = "0x198 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr38(&self) -> &MPCBB1_VCTR38 {
        &self.mpcbb1_vctr38
    }
    #[doc = "0x19c - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr39(&self) -> &MPCBB1_VCTR39 {
        &self.mpcbb1_vctr39
    }
    #[doc = "0x1a0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr40(&self) -> &MPCBB1_VCTR40 {
        &self.mpcbb1_vctr40
    }
    #[doc = "0x1a4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr41(&self) -> &MPCBB1_VCTR41 {
        &self.mpcbb1_vctr41
    }
    #[doc = "0x1a8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr42(&self) -> &MPCBB1_VCTR42 {
        &self.mpcbb1_vctr42
    }
    #[doc = "0x1ac - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr43(&self) -> &MPCBB1_VCTR43 {
        &self.mpcbb1_vctr43
    }
    #[doc = "0x1b0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr44(&self) -> &MPCBB1_VCTR44 {
        &self.mpcbb1_vctr44
    }
    #[doc = "0x1b4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr45(&self) -> &MPCBB1_VCTR45 {
        &self.mpcbb1_vctr45
    }
    #[doc = "0x1b8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr46(&self) -> &MPCBB1_VCTR46 {
        &self.mpcbb1_vctr46
    }
    #[doc = "0x1bc - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr47(&self) -> &MPCBB1_VCTR47 {
        &self.mpcbb1_vctr47
    }
    #[doc = "0x1c0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr48(&self) -> &MPCBB1_VCTR48 {
        &self.mpcbb1_vctr48
    }
    #[doc = "0x1c4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr49(&self) -> &MPCBB1_VCTR49 {
        &self.mpcbb1_vctr49
    }
    #[doc = "0x1c8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr50(&self) -> &MPCBB1_VCTR50 {
        &self.mpcbb1_vctr50
    }
    #[doc = "0x1cc - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr51(&self) -> &MPCBB1_VCTR51 {
        &self.mpcbb1_vctr51
    }
    #[doc = "0x1d0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr52(&self) -> &MPCBB1_VCTR52 {
        &self.mpcbb1_vctr52
    }
    #[doc = "0x1d4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr53(&self) -> &MPCBB1_VCTR53 {
        &self.mpcbb1_vctr53
    }
    #[doc = "0x1d8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr54(&self) -> &MPCBB1_VCTR54 {
        &self.mpcbb1_vctr54
    }
    #[doc = "0x1dc - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr55(&self) -> &MPCBB1_VCTR55 {
        &self.mpcbb1_vctr55
    }
    #[doc = "0x1e0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr56(&self) -> &MPCBB1_VCTR56 {
        &self.mpcbb1_vctr56
    }
    #[doc = "0x1e4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr57(&self) -> &MPCBB1_VCTR57 {
        &self.mpcbb1_vctr57
    }
    #[doc = "0x1e8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr58(&self) -> &MPCBB1_VCTR58 {
        &self.mpcbb1_vctr58
    }
    #[doc = "0x1ec - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr59(&self) -> &MPCBB1_VCTR59 {
        &self.mpcbb1_vctr59
    }
    #[doc = "0x1f0 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr60(&self) -> &MPCBB1_VCTR60 {
        &self.mpcbb1_vctr60
    }
    #[doc = "0x1f4 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr61(&self) -> &MPCBB1_VCTR61 {
        &self.mpcbb1_vctr61
    }
    #[doc = "0x1f8 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr62(&self) -> &MPCBB1_VCTR62 {
        &self.mpcbb1_vctr62
    }
    #[doc = "0x1fc - MPCBBx vector register"]
    #[inline(always)]
    pub const fn mpcbb1_vctr63(&self) -> &MPCBB1_VCTR63 {
        &self.mpcbb1_vctr63
    }
}
#[doc = "MPCBB1_CR (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_cr`]
module"]
pub type MPCBB1_CR = crate::Reg<mpcbb1_cr::MPCBB1_CRrs>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_cr;
#[doc = "MPCBB1_LCKVTR1 (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_lckvtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_lckvtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_lckvtr1`]
module"]
pub type MPCBB1_LCKVTR1 = crate::Reg<mpcbb1_lckvtr1::MPCBB1_LCKVTR1rs>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr1;
#[doc = "MPCBB1_LCKVTR2 (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_lckvtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_lckvtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_lckvtr2`]
module"]
pub type MPCBB1_LCKVTR2 = crate::Reg<mpcbb1_lckvtr2::MPCBB1_LCKVTR2rs>;
#[doc = "MPCBB control register"]
pub mod mpcbb1_lckvtr2;
#[doc = "MPCBB1_VCTR0 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr0`]
module"]
pub type MPCBB1_VCTR0 = crate::Reg<mpcbb1_vctr0::MPCBB1_VCTR0rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr0;
#[doc = "MPCBB1_VCTR1 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr1`]
module"]
pub type MPCBB1_VCTR1 = crate::Reg<mpcbb1_vctr1::MPCBB1_VCTR1rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr1;
#[doc = "MPCBB1_VCTR2 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr2`]
module"]
pub type MPCBB1_VCTR2 = crate::Reg<mpcbb1_vctr2::MPCBB1_VCTR2rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr2;
#[doc = "MPCBB1_VCTR3 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr3`]
module"]
pub type MPCBB1_VCTR3 = crate::Reg<mpcbb1_vctr3::MPCBB1_VCTR3rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr3;
#[doc = "MPCBB1_VCTR4 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr4`]
module"]
pub type MPCBB1_VCTR4 = crate::Reg<mpcbb1_vctr4::MPCBB1_VCTR4rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr4;
#[doc = "MPCBB1_VCTR5 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr5`]
module"]
pub type MPCBB1_VCTR5 = crate::Reg<mpcbb1_vctr5::MPCBB1_VCTR5rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr5;
#[doc = "MPCBB1_VCTR6 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr6`]
module"]
pub type MPCBB1_VCTR6 = crate::Reg<mpcbb1_vctr6::MPCBB1_VCTR6rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr6;
#[doc = "MPCBB1_VCTR7 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr7`]
module"]
pub type MPCBB1_VCTR7 = crate::Reg<mpcbb1_vctr7::MPCBB1_VCTR7rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr7;
#[doc = "MPCBB1_VCTR8 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr8`]
module"]
pub type MPCBB1_VCTR8 = crate::Reg<mpcbb1_vctr8::MPCBB1_VCTR8rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr8;
#[doc = "MPCBB1_VCTR9 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr9`]
module"]
pub type MPCBB1_VCTR9 = crate::Reg<mpcbb1_vctr9::MPCBB1_VCTR9rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr9;
#[doc = "MPCBB1_VCTR10 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr10`]
module"]
pub type MPCBB1_VCTR10 = crate::Reg<mpcbb1_vctr10::MPCBB1_VCTR10rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr10;
#[doc = "MPCBB1_VCTR11 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr11`]
module"]
pub type MPCBB1_VCTR11 = crate::Reg<mpcbb1_vctr11::MPCBB1_VCTR11rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr11;
#[doc = "MPCBB1_VCTR12 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr12`]
module"]
pub type MPCBB1_VCTR12 = crate::Reg<mpcbb1_vctr12::MPCBB1_VCTR12rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr12;
#[doc = "MPCBB1_VCTR13 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr13`]
module"]
pub type MPCBB1_VCTR13 = crate::Reg<mpcbb1_vctr13::MPCBB1_VCTR13rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr13;
#[doc = "MPCBB1_VCTR14 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr14`]
module"]
pub type MPCBB1_VCTR14 = crate::Reg<mpcbb1_vctr14::MPCBB1_VCTR14rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr14;
#[doc = "MPCBB1_VCTR15 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr15`]
module"]
pub type MPCBB1_VCTR15 = crate::Reg<mpcbb1_vctr15::MPCBB1_VCTR15rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr15;
#[doc = "MPCBB1_VCTR16 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr16`]
module"]
pub type MPCBB1_VCTR16 = crate::Reg<mpcbb1_vctr16::MPCBB1_VCTR16rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr16;
#[doc = "MPCBB1_VCTR17 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr17`]
module"]
pub type MPCBB1_VCTR17 = crate::Reg<mpcbb1_vctr17::MPCBB1_VCTR17rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr17;
#[doc = "MPCBB1_VCTR18 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr18`]
module"]
pub type MPCBB1_VCTR18 = crate::Reg<mpcbb1_vctr18::MPCBB1_VCTR18rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr18;
#[doc = "MPCBB1_VCTR19 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr19`]
module"]
pub type MPCBB1_VCTR19 = crate::Reg<mpcbb1_vctr19::MPCBB1_VCTR19rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr19;
#[doc = "MPCBB1_VCTR20 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr20`]
module"]
pub type MPCBB1_VCTR20 = crate::Reg<mpcbb1_vctr20::MPCBB1_VCTR20rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr20;
#[doc = "MPCBB1_VCTR21 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr21`]
module"]
pub type MPCBB1_VCTR21 = crate::Reg<mpcbb1_vctr21::MPCBB1_VCTR21rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr21;
#[doc = "MPCBB1_VCTR22 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr22`]
module"]
pub type MPCBB1_VCTR22 = crate::Reg<mpcbb1_vctr22::MPCBB1_VCTR22rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr22;
#[doc = "MPCBB1_VCTR23 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr23`]
module"]
pub type MPCBB1_VCTR23 = crate::Reg<mpcbb1_vctr23::MPCBB1_VCTR23rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr23;
#[doc = "MPCBB1_VCTR24 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr24`]
module"]
pub type MPCBB1_VCTR24 = crate::Reg<mpcbb1_vctr24::MPCBB1_VCTR24rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr24;
#[doc = "MPCBB1_VCTR25 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr25`]
module"]
pub type MPCBB1_VCTR25 = crate::Reg<mpcbb1_vctr25::MPCBB1_VCTR25rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr25;
#[doc = "MPCBB1_VCTR26 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr26`]
module"]
pub type MPCBB1_VCTR26 = crate::Reg<mpcbb1_vctr26::MPCBB1_VCTR26rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr26;
#[doc = "MPCBB1_VCTR27 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr27`]
module"]
pub type MPCBB1_VCTR27 = crate::Reg<mpcbb1_vctr27::MPCBB1_VCTR27rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr27;
#[doc = "MPCBB1_VCTR28 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr28`]
module"]
pub type MPCBB1_VCTR28 = crate::Reg<mpcbb1_vctr28::MPCBB1_VCTR28rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr28;
#[doc = "MPCBB1_VCTR29 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr29`]
module"]
pub type MPCBB1_VCTR29 = crate::Reg<mpcbb1_vctr29::MPCBB1_VCTR29rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr29;
#[doc = "MPCBB1_VCTR30 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr30`]
module"]
pub type MPCBB1_VCTR30 = crate::Reg<mpcbb1_vctr30::MPCBB1_VCTR30rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr30;
#[doc = "MPCBB1_VCTR31 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr31`]
module"]
pub type MPCBB1_VCTR31 = crate::Reg<mpcbb1_vctr31::MPCBB1_VCTR31rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr31;
#[doc = "MPCBB1_VCTR32 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr32`]
module"]
pub type MPCBB1_VCTR32 = crate::Reg<mpcbb1_vctr32::MPCBB1_VCTR32rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr32;
#[doc = "MPCBB1_VCTR33 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr33`]
module"]
pub type MPCBB1_VCTR33 = crate::Reg<mpcbb1_vctr33::MPCBB1_VCTR33rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr33;
#[doc = "MPCBB1_VCTR34 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr34`]
module"]
pub type MPCBB1_VCTR34 = crate::Reg<mpcbb1_vctr34::MPCBB1_VCTR34rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr34;
#[doc = "MPCBB1_VCTR35 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr35`]
module"]
pub type MPCBB1_VCTR35 = crate::Reg<mpcbb1_vctr35::MPCBB1_VCTR35rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr35;
#[doc = "MPCBB1_VCTR36 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr36`]
module"]
pub type MPCBB1_VCTR36 = crate::Reg<mpcbb1_vctr36::MPCBB1_VCTR36rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr36;
#[doc = "MPCBB1_VCTR37 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr37`]
module"]
pub type MPCBB1_VCTR37 = crate::Reg<mpcbb1_vctr37::MPCBB1_VCTR37rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr37;
#[doc = "MPCBB1_VCTR38 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr38`]
module"]
pub type MPCBB1_VCTR38 = crate::Reg<mpcbb1_vctr38::MPCBB1_VCTR38rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr38;
#[doc = "MPCBB1_VCTR39 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr39`]
module"]
pub type MPCBB1_VCTR39 = crate::Reg<mpcbb1_vctr39::MPCBB1_VCTR39rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr39;
#[doc = "MPCBB1_VCTR40 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr40`]
module"]
pub type MPCBB1_VCTR40 = crate::Reg<mpcbb1_vctr40::MPCBB1_VCTR40rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr40;
#[doc = "MPCBB1_VCTR41 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr41`]
module"]
pub type MPCBB1_VCTR41 = crate::Reg<mpcbb1_vctr41::MPCBB1_VCTR41rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr41;
#[doc = "MPCBB1_VCTR42 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr42`]
module"]
pub type MPCBB1_VCTR42 = crate::Reg<mpcbb1_vctr42::MPCBB1_VCTR42rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr42;
#[doc = "MPCBB1_VCTR43 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr43::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr43::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr43`]
module"]
pub type MPCBB1_VCTR43 = crate::Reg<mpcbb1_vctr43::MPCBB1_VCTR43rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr43;
#[doc = "MPCBB1_VCTR44 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr44`]
module"]
pub type MPCBB1_VCTR44 = crate::Reg<mpcbb1_vctr44::MPCBB1_VCTR44rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr44;
#[doc = "MPCBB1_VCTR45 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr45::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr45::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr45`]
module"]
pub type MPCBB1_VCTR45 = crate::Reg<mpcbb1_vctr45::MPCBB1_VCTR45rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr45;
#[doc = "MPCBB1_VCTR46 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr46::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr46::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr46`]
module"]
pub type MPCBB1_VCTR46 = crate::Reg<mpcbb1_vctr46::MPCBB1_VCTR46rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr46;
#[doc = "MPCBB1_VCTR47 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr47::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr47::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr47`]
module"]
pub type MPCBB1_VCTR47 = crate::Reg<mpcbb1_vctr47::MPCBB1_VCTR47rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr47;
#[doc = "MPCBB1_VCTR48 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr48::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr48::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr48`]
module"]
pub type MPCBB1_VCTR48 = crate::Reg<mpcbb1_vctr48::MPCBB1_VCTR48rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr48;
#[doc = "MPCBB1_VCTR49 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr49::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr49::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr49`]
module"]
pub type MPCBB1_VCTR49 = crate::Reg<mpcbb1_vctr49::MPCBB1_VCTR49rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr49;
#[doc = "MPCBB1_VCTR50 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr50::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr50::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr50`]
module"]
pub type MPCBB1_VCTR50 = crate::Reg<mpcbb1_vctr50::MPCBB1_VCTR50rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr50;
#[doc = "MPCBB1_VCTR51 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr51::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr51::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr51`]
module"]
pub type MPCBB1_VCTR51 = crate::Reg<mpcbb1_vctr51::MPCBB1_VCTR51rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr51;
#[doc = "MPCBB1_VCTR52 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr52::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr52::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr52`]
module"]
pub type MPCBB1_VCTR52 = crate::Reg<mpcbb1_vctr52::MPCBB1_VCTR52rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr52;
#[doc = "MPCBB1_VCTR53 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr53::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr53::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr53`]
module"]
pub type MPCBB1_VCTR53 = crate::Reg<mpcbb1_vctr53::MPCBB1_VCTR53rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr53;
#[doc = "MPCBB1_VCTR54 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr54::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr54::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr54`]
module"]
pub type MPCBB1_VCTR54 = crate::Reg<mpcbb1_vctr54::MPCBB1_VCTR54rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr54;
#[doc = "MPCBB1_VCTR55 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr55::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr55::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr55`]
module"]
pub type MPCBB1_VCTR55 = crate::Reg<mpcbb1_vctr55::MPCBB1_VCTR55rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr55;
#[doc = "MPCBB1_VCTR56 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr56::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr56::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr56`]
module"]
pub type MPCBB1_VCTR56 = crate::Reg<mpcbb1_vctr56::MPCBB1_VCTR56rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr56;
#[doc = "MPCBB1_VCTR57 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr57::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr57::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr57`]
module"]
pub type MPCBB1_VCTR57 = crate::Reg<mpcbb1_vctr57::MPCBB1_VCTR57rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr57;
#[doc = "MPCBB1_VCTR58 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr58::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr58::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr58`]
module"]
pub type MPCBB1_VCTR58 = crate::Reg<mpcbb1_vctr58::MPCBB1_VCTR58rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr58;
#[doc = "MPCBB1_VCTR59 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr59::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr59::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr59`]
module"]
pub type MPCBB1_VCTR59 = crate::Reg<mpcbb1_vctr59::MPCBB1_VCTR59rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr59;
#[doc = "MPCBB1_VCTR60 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr60::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr60::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr60`]
module"]
pub type MPCBB1_VCTR60 = crate::Reg<mpcbb1_vctr60::MPCBB1_VCTR60rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr60;
#[doc = "MPCBB1_VCTR61 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr61::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr61::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr61`]
module"]
pub type MPCBB1_VCTR61 = crate::Reg<mpcbb1_vctr61::MPCBB1_VCTR61rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr61;
#[doc = "MPCBB1_VCTR62 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr62::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr62::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr62`]
module"]
pub type MPCBB1_VCTR62 = crate::Reg<mpcbb1_vctr62::MPCBB1_VCTR62rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr62;
#[doc = "MPCBB1_VCTR63 (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcbb1_vctr63::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcbb1_vctr63::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpcbb1_vctr63`]
module"]
pub type MPCBB1_VCTR63 = crate::Reg<mpcbb1_vctr63::MPCBB1_VCTR63rs>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb1_vctr63;
