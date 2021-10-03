#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    pub mpcbb2_cr: crate::Reg<mpcbb2_cr::MPCBB2_CR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - MPCBB control register"]
    pub mpcbb2_lckvtr1: crate::Reg<mpcbb2_lckvtr1::MPCBB2_LCKVTR1_SPEC>,
    #[doc = "0x14 - MPCBB control register"]
    pub mpcbb2_lckvtr2: crate::Reg<mpcbb2_lckvtr2::MPCBB2_LCKVTR2_SPEC>,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100 - MPCBBx vector register"]
    pub mpcbb2_vctr0: crate::Reg<mpcbb2_vctr0::MPCBB2_VCTR0_SPEC>,
    #[doc = "0x104 - MPCBBx vector register"]
    pub mpcbb2_vctr1: crate::Reg<mpcbb2_vctr1::MPCBB2_VCTR1_SPEC>,
    #[doc = "0x108 - MPCBBx vector register"]
    pub mpcbb2_vctr2: crate::Reg<mpcbb2_vctr2::MPCBB2_VCTR2_SPEC>,
    #[doc = "0x10c - MPCBBx vector register"]
    pub mpcbb2_vctr3: crate::Reg<mpcbb2_vctr3::MPCBB2_VCTR3_SPEC>,
    #[doc = "0x110 - MPCBBx vector register"]
    pub mpcbb2_vctr4: crate::Reg<mpcbb2_vctr4::MPCBB2_VCTR4_SPEC>,
    #[doc = "0x114 - MPCBBx vector register"]
    pub mpcbb2_vctr5: crate::Reg<mpcbb2_vctr5::MPCBB2_VCTR5_SPEC>,
    #[doc = "0x118 - MPCBBx vector register"]
    pub mpcbb2_vctr6: crate::Reg<mpcbb2_vctr6::MPCBB2_VCTR6_SPEC>,
    #[doc = "0x11c - MPCBBx vector register"]
    pub mpcbb2_vctr7: crate::Reg<mpcbb2_vctr7::MPCBB2_VCTR7_SPEC>,
    #[doc = "0x120 - MPCBBx vector register"]
    pub mpcbb2_vctr8: crate::Reg<mpcbb2_vctr8::MPCBB2_VCTR8_SPEC>,
    #[doc = "0x124 - MPCBBx vector register"]
    pub mpcbb2_vctr9: crate::Reg<mpcbb2_vctr9::MPCBB2_VCTR9_SPEC>,
    #[doc = "0x128 - MPCBBx vector register"]
    pub mpcbb2_vctr10: crate::Reg<mpcbb2_vctr10::MPCBB2_VCTR10_SPEC>,
    #[doc = "0x12c - MPCBBx vector register"]
    pub mpcbb2_vctr11: crate::Reg<mpcbb2_vctr11::MPCBB2_VCTR11_SPEC>,
    #[doc = "0x130 - MPCBBx vector register"]
    pub mpcbb2_vctr12: crate::Reg<mpcbb2_vctr12::MPCBB2_VCTR12_SPEC>,
    #[doc = "0x134 - MPCBBx vector register"]
    pub mpcbb2_vctr13: crate::Reg<mpcbb2_vctr13::MPCBB2_VCTR13_SPEC>,
    #[doc = "0x138 - MPCBBx vector register"]
    pub mpcbb2_vctr14: crate::Reg<mpcbb2_vctr14::MPCBB2_VCTR14_SPEC>,
    #[doc = "0x13c - MPCBBx vector register"]
    pub mpcbb2_vctr15: crate::Reg<mpcbb2_vctr15::MPCBB2_VCTR15_SPEC>,
    #[doc = "0x140 - MPCBBx vector register"]
    pub mpcbb2_vctr16: crate::Reg<mpcbb2_vctr16::MPCBB2_VCTR16_SPEC>,
    #[doc = "0x144 - MPCBBx vector register"]
    pub mpcbb2_vctr17: crate::Reg<mpcbb2_vctr17::MPCBB2_VCTR17_SPEC>,
    #[doc = "0x148 - MPCBBx vector register"]
    pub mpcbb2_vctr18: crate::Reg<mpcbb2_vctr18::MPCBB2_VCTR18_SPEC>,
    #[doc = "0x14c - MPCBBx vector register"]
    pub mpcbb2_vctr19: crate::Reg<mpcbb2_vctr19::MPCBB2_VCTR19_SPEC>,
    #[doc = "0x150 - MPCBBx vector register"]
    pub mpcbb2_vctr20: crate::Reg<mpcbb2_vctr20::MPCBB2_VCTR20_SPEC>,
    #[doc = "0x154 - MPCBBx vector register"]
    pub mpcbb2_vctr21: crate::Reg<mpcbb2_vctr21::MPCBB2_VCTR21_SPEC>,
    #[doc = "0x158 - MPCBBx vector register"]
    pub mpcbb2_vctr22: crate::Reg<mpcbb2_vctr22::MPCBB2_VCTR22_SPEC>,
    #[doc = "0x15c - MPCBBx vector register"]
    pub mpcbb2_vctr23: crate::Reg<mpcbb2_vctr23::MPCBB2_VCTR23_SPEC>,
    #[doc = "0x160 - MPCBBx vector register"]
    pub mpcbb2_vctr24: crate::Reg<mpcbb2_vctr24::MPCBB2_VCTR24_SPEC>,
    #[doc = "0x164 - MPCBBx vector register"]
    pub mpcbb2_vctr25: crate::Reg<mpcbb2_vctr25::MPCBB2_VCTR25_SPEC>,
    #[doc = "0x168 - MPCBBx vector register"]
    pub mpcbb2_vctr26: crate::Reg<mpcbb2_vctr26::MPCBB2_VCTR26_SPEC>,
    #[doc = "0x16c - MPCBBx vector register"]
    pub mpcbb2_vctr27: crate::Reg<mpcbb2_vctr27::MPCBB2_VCTR27_SPEC>,
    #[doc = "0x170 - MPCBBx vector register"]
    pub mpcbb2_vctr28: crate::Reg<mpcbb2_vctr28::MPCBB2_VCTR28_SPEC>,
    #[doc = "0x174 - MPCBBx vector register"]
    pub mpcbb2_vctr29: crate::Reg<mpcbb2_vctr29::MPCBB2_VCTR29_SPEC>,
    #[doc = "0x178 - MPCBBx vector register"]
    pub mpcbb2_vctr30: crate::Reg<mpcbb2_vctr30::MPCBB2_VCTR30_SPEC>,
    #[doc = "0x17c - MPCBBx vector register"]
    pub mpcbb2_vctr31: crate::Reg<mpcbb2_vctr31::MPCBB2_VCTR31_SPEC>,
    #[doc = "0x180 - MPCBBx vector register"]
    pub mpcbb2_vctr32: crate::Reg<mpcbb2_vctr32::MPCBB2_VCTR32_SPEC>,
    #[doc = "0x184 - MPCBBx vector register"]
    pub mpcbb2_vctr33: crate::Reg<mpcbb2_vctr33::MPCBB2_VCTR33_SPEC>,
    #[doc = "0x188 - MPCBBx vector register"]
    pub mpcbb2_vctr34: crate::Reg<mpcbb2_vctr34::MPCBB2_VCTR34_SPEC>,
    #[doc = "0x18c - MPCBBx vector register"]
    pub mpcbb2_vctr35: crate::Reg<mpcbb2_vctr35::MPCBB2_VCTR35_SPEC>,
    #[doc = "0x190 - MPCBBx vector register"]
    pub mpcbb2_vctr36: crate::Reg<mpcbb2_vctr36::MPCBB2_VCTR36_SPEC>,
    #[doc = "0x194 - MPCBBx vector register"]
    pub mpcbb2_vctr37: crate::Reg<mpcbb2_vctr37::MPCBB2_VCTR37_SPEC>,
    #[doc = "0x198 - MPCBBx vector register"]
    pub mpcbb2_vctr38: crate::Reg<mpcbb2_vctr38::MPCBB2_VCTR38_SPEC>,
    #[doc = "0x19c - MPCBBx vector register"]
    pub mpcbb2_vctr39: crate::Reg<mpcbb2_vctr39::MPCBB2_VCTR39_SPEC>,
    #[doc = "0x1a0 - MPCBBx vector register"]
    pub mpcbb2_vctr40: crate::Reg<mpcbb2_vctr40::MPCBB2_VCTR40_SPEC>,
    #[doc = "0x1a4 - MPCBBx vector register"]
    pub mpcbb2_vctr41: crate::Reg<mpcbb2_vctr41::MPCBB2_VCTR41_SPEC>,
    #[doc = "0x1a8 - MPCBBx vector register"]
    pub mpcbb2_vctr42: crate::Reg<mpcbb2_vctr42::MPCBB2_VCTR42_SPEC>,
    #[doc = "0x1ac - MPCBBx vector register"]
    pub mpcbb2_vctr43: crate::Reg<mpcbb2_vctr43::MPCBB2_VCTR43_SPEC>,
    #[doc = "0x1b0 - MPCBBx vector register"]
    pub mpcbb2_vctr44: crate::Reg<mpcbb2_vctr44::MPCBB2_VCTR44_SPEC>,
    #[doc = "0x1b4 - MPCBBx vector register"]
    pub mpcbb2_vctr45: crate::Reg<mpcbb2_vctr45::MPCBB2_VCTR45_SPEC>,
    #[doc = "0x1b8 - MPCBBx vector register"]
    pub mpcbb2_vctr46: crate::Reg<mpcbb2_vctr46::MPCBB2_VCTR46_SPEC>,
    #[doc = "0x1bc - MPCBBx vector register"]
    pub mpcbb2_vctr47: crate::Reg<mpcbb2_vctr47::MPCBB2_VCTR47_SPEC>,
    #[doc = "0x1c0 - MPCBBx vector register"]
    pub mpcbb2_vctr48: crate::Reg<mpcbb2_vctr48::MPCBB2_VCTR48_SPEC>,
    #[doc = "0x1c4 - MPCBBx vector register"]
    pub mpcbb2_vctr49: crate::Reg<mpcbb2_vctr49::MPCBB2_VCTR49_SPEC>,
    #[doc = "0x1c8 - MPCBBx vector register"]
    pub mpcbb2_vctr50: crate::Reg<mpcbb2_vctr50::MPCBB2_VCTR50_SPEC>,
    #[doc = "0x1cc - MPCBBx vector register"]
    pub mpcbb2_vctr51: crate::Reg<mpcbb2_vctr51::MPCBB2_VCTR51_SPEC>,
    #[doc = "0x1d0 - MPCBBx vector register"]
    pub mpcbb2_vctr52: crate::Reg<mpcbb2_vctr52::MPCBB2_VCTR52_SPEC>,
    #[doc = "0x1d4 - MPCBBx vector register"]
    pub mpcbb2_vctr53: crate::Reg<mpcbb2_vctr53::MPCBB2_VCTR53_SPEC>,
    #[doc = "0x1d8 - MPCBBx vector register"]
    pub mpcbb2_vctr54: crate::Reg<mpcbb2_vctr54::MPCBB2_VCTR54_SPEC>,
    #[doc = "0x1dc - MPCBBx vector register"]
    pub mpcbb2_vctr55: crate::Reg<mpcbb2_vctr55::MPCBB2_VCTR55_SPEC>,
    #[doc = "0x1e0 - MPCBBx vector register"]
    pub mpcbb2_vctr56: crate::Reg<mpcbb2_vctr56::MPCBB2_VCTR56_SPEC>,
    #[doc = "0x1e4 - MPCBBx vector register"]
    pub mpcbb2_vctr57: crate::Reg<mpcbb2_vctr57::MPCBB2_VCTR57_SPEC>,
    #[doc = "0x1e8 - MPCBBx vector register"]
    pub mpcbb2_vctr58: crate::Reg<mpcbb2_vctr58::MPCBB2_VCTR58_SPEC>,
    #[doc = "0x1ec - MPCBBx vector register"]
    pub mpcbb2_vctr59: crate::Reg<mpcbb2_vctr59::MPCBB2_VCTR59_SPEC>,
    #[doc = "0x1f0 - MPCBBx vector register"]
    pub mpcbb2_vctr60: crate::Reg<mpcbb2_vctr60::MPCBB2_VCTR60_SPEC>,
    #[doc = "0x1f4 - MPCBBx vector register"]
    pub mpcbb2_vctr61: crate::Reg<mpcbb2_vctr61::MPCBB2_VCTR61_SPEC>,
    #[doc = "0x1f8 - MPCBBx vector register"]
    pub mpcbb2_vctr62: crate::Reg<mpcbb2_vctr62::MPCBB2_VCTR62_SPEC>,
    #[doc = "0x1fc - MPCBBx vector register"]
    pub mpcbb2_vctr63: crate::Reg<mpcbb2_vctr63::MPCBB2_VCTR63_SPEC>,
}
#[doc = "MPCBB2_CR register accessor: an alias for `Reg<MPCBB2_CR_SPEC>`"]
pub type MPCBB2_CR = crate::Reg<mpcbb2_cr::MPCBB2_CR_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_cr;
#[doc = "MPCBB2_LCKVTR1 register accessor: an alias for `Reg<MPCBB2_LCKVTR1_SPEC>`"]
pub type MPCBB2_LCKVTR1 = crate::Reg<mpcbb2_lckvtr1::MPCBB2_LCKVTR1_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr1;
#[doc = "MPCBB2_LCKVTR2 register accessor: an alias for `Reg<MPCBB2_LCKVTR2_SPEC>`"]
pub type MPCBB2_LCKVTR2 = crate::Reg<mpcbb2_lckvtr2::MPCBB2_LCKVTR2_SPEC>;
#[doc = "MPCBB control register"]
pub mod mpcbb2_lckvtr2;
#[doc = "MPCBB2_VCTR0 register accessor: an alias for `Reg<MPCBB2_VCTR0_SPEC>`"]
pub type MPCBB2_VCTR0 = crate::Reg<mpcbb2_vctr0::MPCBB2_VCTR0_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr0;
#[doc = "MPCBB2_VCTR1 register accessor: an alias for `Reg<MPCBB2_VCTR1_SPEC>`"]
pub type MPCBB2_VCTR1 = crate::Reg<mpcbb2_vctr1::MPCBB2_VCTR1_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr1;
#[doc = "MPCBB2_VCTR2 register accessor: an alias for `Reg<MPCBB2_VCTR2_SPEC>`"]
pub type MPCBB2_VCTR2 = crate::Reg<mpcbb2_vctr2::MPCBB2_VCTR2_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr2;
#[doc = "MPCBB2_VCTR3 register accessor: an alias for `Reg<MPCBB2_VCTR3_SPEC>`"]
pub type MPCBB2_VCTR3 = crate::Reg<mpcbb2_vctr3::MPCBB2_VCTR3_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr3;
#[doc = "MPCBB2_VCTR4 register accessor: an alias for `Reg<MPCBB2_VCTR4_SPEC>`"]
pub type MPCBB2_VCTR4 = crate::Reg<mpcbb2_vctr4::MPCBB2_VCTR4_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr4;
#[doc = "MPCBB2_VCTR5 register accessor: an alias for `Reg<MPCBB2_VCTR5_SPEC>`"]
pub type MPCBB2_VCTR5 = crate::Reg<mpcbb2_vctr5::MPCBB2_VCTR5_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr5;
#[doc = "MPCBB2_VCTR6 register accessor: an alias for `Reg<MPCBB2_VCTR6_SPEC>`"]
pub type MPCBB2_VCTR6 = crate::Reg<mpcbb2_vctr6::MPCBB2_VCTR6_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr6;
#[doc = "MPCBB2_VCTR7 register accessor: an alias for `Reg<MPCBB2_VCTR7_SPEC>`"]
pub type MPCBB2_VCTR7 = crate::Reg<mpcbb2_vctr7::MPCBB2_VCTR7_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr7;
#[doc = "MPCBB2_VCTR8 register accessor: an alias for `Reg<MPCBB2_VCTR8_SPEC>`"]
pub type MPCBB2_VCTR8 = crate::Reg<mpcbb2_vctr8::MPCBB2_VCTR8_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr8;
#[doc = "MPCBB2_VCTR9 register accessor: an alias for `Reg<MPCBB2_VCTR9_SPEC>`"]
pub type MPCBB2_VCTR9 = crate::Reg<mpcbb2_vctr9::MPCBB2_VCTR9_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr9;
#[doc = "MPCBB2_VCTR10 register accessor: an alias for `Reg<MPCBB2_VCTR10_SPEC>`"]
pub type MPCBB2_VCTR10 = crate::Reg<mpcbb2_vctr10::MPCBB2_VCTR10_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr10;
#[doc = "MPCBB2_VCTR11 register accessor: an alias for `Reg<MPCBB2_VCTR11_SPEC>`"]
pub type MPCBB2_VCTR11 = crate::Reg<mpcbb2_vctr11::MPCBB2_VCTR11_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr11;
#[doc = "MPCBB2_VCTR12 register accessor: an alias for `Reg<MPCBB2_VCTR12_SPEC>`"]
pub type MPCBB2_VCTR12 = crate::Reg<mpcbb2_vctr12::MPCBB2_VCTR12_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr12;
#[doc = "MPCBB2_VCTR13 register accessor: an alias for `Reg<MPCBB2_VCTR13_SPEC>`"]
pub type MPCBB2_VCTR13 = crate::Reg<mpcbb2_vctr13::MPCBB2_VCTR13_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr13;
#[doc = "MPCBB2_VCTR14 register accessor: an alias for `Reg<MPCBB2_VCTR14_SPEC>`"]
pub type MPCBB2_VCTR14 = crate::Reg<mpcbb2_vctr14::MPCBB2_VCTR14_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr14;
#[doc = "MPCBB2_VCTR15 register accessor: an alias for `Reg<MPCBB2_VCTR15_SPEC>`"]
pub type MPCBB2_VCTR15 = crate::Reg<mpcbb2_vctr15::MPCBB2_VCTR15_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr15;
#[doc = "MPCBB2_VCTR16 register accessor: an alias for `Reg<MPCBB2_VCTR16_SPEC>`"]
pub type MPCBB2_VCTR16 = crate::Reg<mpcbb2_vctr16::MPCBB2_VCTR16_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr16;
#[doc = "MPCBB2_VCTR17 register accessor: an alias for `Reg<MPCBB2_VCTR17_SPEC>`"]
pub type MPCBB2_VCTR17 = crate::Reg<mpcbb2_vctr17::MPCBB2_VCTR17_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr17;
#[doc = "MPCBB2_VCTR18 register accessor: an alias for `Reg<MPCBB2_VCTR18_SPEC>`"]
pub type MPCBB2_VCTR18 = crate::Reg<mpcbb2_vctr18::MPCBB2_VCTR18_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr18;
#[doc = "MPCBB2_VCTR19 register accessor: an alias for `Reg<MPCBB2_VCTR19_SPEC>`"]
pub type MPCBB2_VCTR19 = crate::Reg<mpcbb2_vctr19::MPCBB2_VCTR19_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr19;
#[doc = "MPCBB2_VCTR20 register accessor: an alias for `Reg<MPCBB2_VCTR20_SPEC>`"]
pub type MPCBB2_VCTR20 = crate::Reg<mpcbb2_vctr20::MPCBB2_VCTR20_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr20;
#[doc = "MPCBB2_VCTR21 register accessor: an alias for `Reg<MPCBB2_VCTR21_SPEC>`"]
pub type MPCBB2_VCTR21 = crate::Reg<mpcbb2_vctr21::MPCBB2_VCTR21_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr21;
#[doc = "MPCBB2_VCTR22 register accessor: an alias for `Reg<MPCBB2_VCTR22_SPEC>`"]
pub type MPCBB2_VCTR22 = crate::Reg<mpcbb2_vctr22::MPCBB2_VCTR22_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr22;
#[doc = "MPCBB2_VCTR23 register accessor: an alias for `Reg<MPCBB2_VCTR23_SPEC>`"]
pub type MPCBB2_VCTR23 = crate::Reg<mpcbb2_vctr23::MPCBB2_VCTR23_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr23;
#[doc = "MPCBB2_VCTR24 register accessor: an alias for `Reg<MPCBB2_VCTR24_SPEC>`"]
pub type MPCBB2_VCTR24 = crate::Reg<mpcbb2_vctr24::MPCBB2_VCTR24_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr24;
#[doc = "MPCBB2_VCTR25 register accessor: an alias for `Reg<MPCBB2_VCTR25_SPEC>`"]
pub type MPCBB2_VCTR25 = crate::Reg<mpcbb2_vctr25::MPCBB2_VCTR25_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr25;
#[doc = "MPCBB2_VCTR26 register accessor: an alias for `Reg<MPCBB2_VCTR26_SPEC>`"]
pub type MPCBB2_VCTR26 = crate::Reg<mpcbb2_vctr26::MPCBB2_VCTR26_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr26;
#[doc = "MPCBB2_VCTR27 register accessor: an alias for `Reg<MPCBB2_VCTR27_SPEC>`"]
pub type MPCBB2_VCTR27 = crate::Reg<mpcbb2_vctr27::MPCBB2_VCTR27_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr27;
#[doc = "MPCBB2_VCTR28 register accessor: an alias for `Reg<MPCBB2_VCTR28_SPEC>`"]
pub type MPCBB2_VCTR28 = crate::Reg<mpcbb2_vctr28::MPCBB2_VCTR28_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr28;
#[doc = "MPCBB2_VCTR29 register accessor: an alias for `Reg<MPCBB2_VCTR29_SPEC>`"]
pub type MPCBB2_VCTR29 = crate::Reg<mpcbb2_vctr29::MPCBB2_VCTR29_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr29;
#[doc = "MPCBB2_VCTR30 register accessor: an alias for `Reg<MPCBB2_VCTR30_SPEC>`"]
pub type MPCBB2_VCTR30 = crate::Reg<mpcbb2_vctr30::MPCBB2_VCTR30_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr30;
#[doc = "MPCBB2_VCTR31 register accessor: an alias for `Reg<MPCBB2_VCTR31_SPEC>`"]
pub type MPCBB2_VCTR31 = crate::Reg<mpcbb2_vctr31::MPCBB2_VCTR31_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr31;
#[doc = "MPCBB2_VCTR32 register accessor: an alias for `Reg<MPCBB2_VCTR32_SPEC>`"]
pub type MPCBB2_VCTR32 = crate::Reg<mpcbb2_vctr32::MPCBB2_VCTR32_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr32;
#[doc = "MPCBB2_VCTR33 register accessor: an alias for `Reg<MPCBB2_VCTR33_SPEC>`"]
pub type MPCBB2_VCTR33 = crate::Reg<mpcbb2_vctr33::MPCBB2_VCTR33_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr33;
#[doc = "MPCBB2_VCTR34 register accessor: an alias for `Reg<MPCBB2_VCTR34_SPEC>`"]
pub type MPCBB2_VCTR34 = crate::Reg<mpcbb2_vctr34::MPCBB2_VCTR34_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr34;
#[doc = "MPCBB2_VCTR35 register accessor: an alias for `Reg<MPCBB2_VCTR35_SPEC>`"]
pub type MPCBB2_VCTR35 = crate::Reg<mpcbb2_vctr35::MPCBB2_VCTR35_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr35;
#[doc = "MPCBB2_VCTR36 register accessor: an alias for `Reg<MPCBB2_VCTR36_SPEC>`"]
pub type MPCBB2_VCTR36 = crate::Reg<mpcbb2_vctr36::MPCBB2_VCTR36_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr36;
#[doc = "MPCBB2_VCTR37 register accessor: an alias for `Reg<MPCBB2_VCTR37_SPEC>`"]
pub type MPCBB2_VCTR37 = crate::Reg<mpcbb2_vctr37::MPCBB2_VCTR37_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr37;
#[doc = "MPCBB2_VCTR38 register accessor: an alias for `Reg<MPCBB2_VCTR38_SPEC>`"]
pub type MPCBB2_VCTR38 = crate::Reg<mpcbb2_vctr38::MPCBB2_VCTR38_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr38;
#[doc = "MPCBB2_VCTR39 register accessor: an alias for `Reg<MPCBB2_VCTR39_SPEC>`"]
pub type MPCBB2_VCTR39 = crate::Reg<mpcbb2_vctr39::MPCBB2_VCTR39_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr39;
#[doc = "MPCBB2_VCTR40 register accessor: an alias for `Reg<MPCBB2_VCTR40_SPEC>`"]
pub type MPCBB2_VCTR40 = crate::Reg<mpcbb2_vctr40::MPCBB2_VCTR40_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr40;
#[doc = "MPCBB2_VCTR41 register accessor: an alias for `Reg<MPCBB2_VCTR41_SPEC>`"]
pub type MPCBB2_VCTR41 = crate::Reg<mpcbb2_vctr41::MPCBB2_VCTR41_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr41;
#[doc = "MPCBB2_VCTR42 register accessor: an alias for `Reg<MPCBB2_VCTR42_SPEC>`"]
pub type MPCBB2_VCTR42 = crate::Reg<mpcbb2_vctr42::MPCBB2_VCTR42_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr42;
#[doc = "MPCBB2_VCTR43 register accessor: an alias for `Reg<MPCBB2_VCTR43_SPEC>`"]
pub type MPCBB2_VCTR43 = crate::Reg<mpcbb2_vctr43::MPCBB2_VCTR43_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr43;
#[doc = "MPCBB2_VCTR44 register accessor: an alias for `Reg<MPCBB2_VCTR44_SPEC>`"]
pub type MPCBB2_VCTR44 = crate::Reg<mpcbb2_vctr44::MPCBB2_VCTR44_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr44;
#[doc = "MPCBB2_VCTR45 register accessor: an alias for `Reg<MPCBB2_VCTR45_SPEC>`"]
pub type MPCBB2_VCTR45 = crate::Reg<mpcbb2_vctr45::MPCBB2_VCTR45_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr45;
#[doc = "MPCBB2_VCTR46 register accessor: an alias for `Reg<MPCBB2_VCTR46_SPEC>`"]
pub type MPCBB2_VCTR46 = crate::Reg<mpcbb2_vctr46::MPCBB2_VCTR46_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr46;
#[doc = "MPCBB2_VCTR47 register accessor: an alias for `Reg<MPCBB2_VCTR47_SPEC>`"]
pub type MPCBB2_VCTR47 = crate::Reg<mpcbb2_vctr47::MPCBB2_VCTR47_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr47;
#[doc = "MPCBB2_VCTR48 register accessor: an alias for `Reg<MPCBB2_VCTR48_SPEC>`"]
pub type MPCBB2_VCTR48 = crate::Reg<mpcbb2_vctr48::MPCBB2_VCTR48_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr48;
#[doc = "MPCBB2_VCTR49 register accessor: an alias for `Reg<MPCBB2_VCTR49_SPEC>`"]
pub type MPCBB2_VCTR49 = crate::Reg<mpcbb2_vctr49::MPCBB2_VCTR49_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr49;
#[doc = "MPCBB2_VCTR50 register accessor: an alias for `Reg<MPCBB2_VCTR50_SPEC>`"]
pub type MPCBB2_VCTR50 = crate::Reg<mpcbb2_vctr50::MPCBB2_VCTR50_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr50;
#[doc = "MPCBB2_VCTR51 register accessor: an alias for `Reg<MPCBB2_VCTR51_SPEC>`"]
pub type MPCBB2_VCTR51 = crate::Reg<mpcbb2_vctr51::MPCBB2_VCTR51_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr51;
#[doc = "MPCBB2_VCTR52 register accessor: an alias for `Reg<MPCBB2_VCTR52_SPEC>`"]
pub type MPCBB2_VCTR52 = crate::Reg<mpcbb2_vctr52::MPCBB2_VCTR52_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr52;
#[doc = "MPCBB2_VCTR53 register accessor: an alias for `Reg<MPCBB2_VCTR53_SPEC>`"]
pub type MPCBB2_VCTR53 = crate::Reg<mpcbb2_vctr53::MPCBB2_VCTR53_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr53;
#[doc = "MPCBB2_VCTR54 register accessor: an alias for `Reg<MPCBB2_VCTR54_SPEC>`"]
pub type MPCBB2_VCTR54 = crate::Reg<mpcbb2_vctr54::MPCBB2_VCTR54_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr54;
#[doc = "MPCBB2_VCTR55 register accessor: an alias for `Reg<MPCBB2_VCTR55_SPEC>`"]
pub type MPCBB2_VCTR55 = crate::Reg<mpcbb2_vctr55::MPCBB2_VCTR55_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr55;
#[doc = "MPCBB2_VCTR56 register accessor: an alias for `Reg<MPCBB2_VCTR56_SPEC>`"]
pub type MPCBB2_VCTR56 = crate::Reg<mpcbb2_vctr56::MPCBB2_VCTR56_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr56;
#[doc = "MPCBB2_VCTR57 register accessor: an alias for `Reg<MPCBB2_VCTR57_SPEC>`"]
pub type MPCBB2_VCTR57 = crate::Reg<mpcbb2_vctr57::MPCBB2_VCTR57_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr57;
#[doc = "MPCBB2_VCTR58 register accessor: an alias for `Reg<MPCBB2_VCTR58_SPEC>`"]
pub type MPCBB2_VCTR58 = crate::Reg<mpcbb2_vctr58::MPCBB2_VCTR58_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr58;
#[doc = "MPCBB2_VCTR59 register accessor: an alias for `Reg<MPCBB2_VCTR59_SPEC>`"]
pub type MPCBB2_VCTR59 = crate::Reg<mpcbb2_vctr59::MPCBB2_VCTR59_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr59;
#[doc = "MPCBB2_VCTR60 register accessor: an alias for `Reg<MPCBB2_VCTR60_SPEC>`"]
pub type MPCBB2_VCTR60 = crate::Reg<mpcbb2_vctr60::MPCBB2_VCTR60_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr60;
#[doc = "MPCBB2_VCTR61 register accessor: an alias for `Reg<MPCBB2_VCTR61_SPEC>`"]
pub type MPCBB2_VCTR61 = crate::Reg<mpcbb2_vctr61::MPCBB2_VCTR61_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr61;
#[doc = "MPCBB2_VCTR62 register accessor: an alias for `Reg<MPCBB2_VCTR62_SPEC>`"]
pub type MPCBB2_VCTR62 = crate::Reg<mpcbb2_vctr62::MPCBB2_VCTR62_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr62;
#[doc = "MPCBB2_VCTR63 register accessor: an alias for `Reg<MPCBB2_VCTR63_SPEC>`"]
pub type MPCBB2_VCTR63 = crate::Reg<mpcbb2_vctr63::MPCBB2_VCTR63_SPEC>;
#[doc = "MPCBBx vector register"]
pub mod mpcbb2_vctr63;
