#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub mdios_cr: crate::Reg<mdios_cr::MDIOS_CR_SPEC>,
    #[doc = "0x04 - MDIOS write flag register"]
    pub mdios_wrfr: crate::Reg<mdios_wrfr::MDIOS_WRFR_SPEC>,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub mdios_cwrfr: crate::Reg<mdios_cwrfr::MDIOS_CWRFR_SPEC>,
    #[doc = "0x0c - MDIOS read flag register"]
    pub mdios_rdfr: crate::Reg<mdios_rdfr::MDIOS_RDFR_SPEC>,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub mdios_crdfr: crate::Reg<mdios_crdfr::MDIOS_CRDFR_SPEC>,
    #[doc = "0x14 - MDIOS status register"]
    pub mdios_sr: crate::Reg<mdios_sr::MDIOS_SR_SPEC>,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub mdios_clrfr: crate::Reg<mdios_clrfr::MDIOS_CLRFR_SPEC>,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - MDIOS input data register"]
    pub mdios_dinr0: crate::Reg<mdios_dinr0::MDIOS_DINR0_SPEC>,
    #[doc = "0x104 - MDIOS input data register"]
    pub mdios_dinr1: crate::Reg<mdios_dinr1::MDIOS_DINR1_SPEC>,
    #[doc = "0x108 - MDIOS input data register"]
    pub mdios_dinr2: crate::Reg<mdios_dinr2::MDIOS_DINR2_SPEC>,
    #[doc = "0x10c - MDIOS input data register"]
    pub mdios_dinr3: crate::Reg<mdios_dinr3::MDIOS_DINR3_SPEC>,
    #[doc = "0x110 - MDIOS input data register"]
    pub mdios_dinr4: crate::Reg<mdios_dinr4::MDIOS_DINR4_SPEC>,
    #[doc = "0x114 - MDIOS input data register"]
    pub mdios_dinr5: crate::Reg<mdios_dinr5::MDIOS_DINR5_SPEC>,
    #[doc = "0x118 - MDIOS input data register"]
    pub mdios_dinr6: crate::Reg<mdios_dinr6::MDIOS_DINR6_SPEC>,
    #[doc = "0x11c - MDIOS input data register"]
    pub mdios_dinr7: crate::Reg<mdios_dinr7::MDIOS_DINR7_SPEC>,
    #[doc = "0x120 - MDIOS input data register"]
    pub mdios_dinr8: crate::Reg<mdios_dinr8::MDIOS_DINR8_SPEC>,
    #[doc = "0x124 - MDIOS input data register"]
    pub mdios_dinr9: crate::Reg<mdios_dinr9::MDIOS_DINR9_SPEC>,
    #[doc = "0x128 - MDIOS input data register"]
    pub mdios_dinr10: crate::Reg<mdios_dinr10::MDIOS_DINR10_SPEC>,
    #[doc = "0x12c - MDIOS input data register"]
    pub mdios_dinr11: crate::Reg<mdios_dinr11::MDIOS_DINR11_SPEC>,
    #[doc = "0x130 - MDIOS input data register"]
    pub mdios_dinr12: crate::Reg<mdios_dinr12::MDIOS_DINR12_SPEC>,
    #[doc = "0x134 - MDIOS input data register"]
    pub mdios_dinr13: crate::Reg<mdios_dinr13::MDIOS_DINR13_SPEC>,
    #[doc = "0x138 - MDIOS input data register"]
    pub mdios_dinr14: crate::Reg<mdios_dinr14::MDIOS_DINR14_SPEC>,
    #[doc = "0x13c - MDIOS input data register"]
    pub mdios_dinr15: crate::Reg<mdios_dinr15::MDIOS_DINR15_SPEC>,
    #[doc = "0x140 - MDIOS input data register"]
    pub mdios_dinr16: crate::Reg<mdios_dinr16::MDIOS_DINR16_SPEC>,
    #[doc = "0x144 - MDIOS input data register"]
    pub mdios_dinr17: crate::Reg<mdios_dinr17::MDIOS_DINR17_SPEC>,
    #[doc = "0x148 - MDIOS input data register"]
    pub mdios_dinr18: crate::Reg<mdios_dinr18::MDIOS_DINR18_SPEC>,
    #[doc = "0x14c - MDIOS input data register"]
    pub mdios_dinr19: crate::Reg<mdios_dinr19::MDIOS_DINR19_SPEC>,
    #[doc = "0x150 - MDIOS input data register"]
    pub mdios_dinr20: crate::Reg<mdios_dinr20::MDIOS_DINR20_SPEC>,
    #[doc = "0x154 - MDIOS input data register"]
    pub mdios_dinr21: crate::Reg<mdios_dinr21::MDIOS_DINR21_SPEC>,
    #[doc = "0x158 - MDIOS input data register"]
    pub mdios_dinr22: crate::Reg<mdios_dinr22::MDIOS_DINR22_SPEC>,
    #[doc = "0x15c - MDIOS input data register"]
    pub mdios_dinr23: crate::Reg<mdios_dinr23::MDIOS_DINR23_SPEC>,
    #[doc = "0x160 - MDIOS input data register"]
    pub mdios_dinr24: crate::Reg<mdios_dinr24::MDIOS_DINR24_SPEC>,
    #[doc = "0x164 - MDIOS input data register"]
    pub mdios_dinr25: crate::Reg<mdios_dinr25::MDIOS_DINR25_SPEC>,
    #[doc = "0x168 - MDIOS input data register"]
    pub mdios_dinr26: crate::Reg<mdios_dinr26::MDIOS_DINR26_SPEC>,
    #[doc = "0x16c - MDIOS input data register"]
    pub mdios_dinr27: crate::Reg<mdios_dinr27::MDIOS_DINR27_SPEC>,
    #[doc = "0x170 - MDIOS input data register"]
    pub mdios_dinr28: crate::Reg<mdios_dinr28::MDIOS_DINR28_SPEC>,
    #[doc = "0x174 - MDIOS input data register"]
    pub mdios_dinr29: crate::Reg<mdios_dinr29::MDIOS_DINR29_SPEC>,
    #[doc = "0x178 - MDIOS input data register"]
    pub mdios_dinr30: crate::Reg<mdios_dinr30::MDIOS_DINR30_SPEC>,
    #[doc = "0x17c - MDIOS input data register"]
    pub mdios_dinr31: crate::Reg<mdios_dinr31::MDIOS_DINR31_SPEC>,
    #[doc = "0x180 - MDIOS input data register"]
    pub mdios_doutr0: crate::Reg<mdios_doutr0::MDIOS_DOUTR0_SPEC>,
    #[doc = "0x184 - MDIOS input data register"]
    pub mdios_doutr1: crate::Reg<mdios_doutr1::MDIOS_DOUTR1_SPEC>,
    #[doc = "0x188 - MDIOS output data register"]
    pub mdios_doutr2: crate::Reg<mdios_doutr2::MDIOS_DOUTR2_SPEC>,
    #[doc = "0x18c - MDIOS output data register"]
    pub mdios_doutr3: crate::Reg<mdios_doutr3::MDIOS_DOUTR3_SPEC>,
    #[doc = "0x190 - MDIOS output data register"]
    pub mdios_doutr4: crate::Reg<mdios_doutr4::MDIOS_DOUTR4_SPEC>,
    #[doc = "0x194 - MDIOS output data register"]
    pub mdios_doutr5: crate::Reg<mdios_doutr5::MDIOS_DOUTR5_SPEC>,
    #[doc = "0x198 - MDIOS output data register"]
    pub mdios_doutr6: crate::Reg<mdios_doutr6::MDIOS_DOUTR6_SPEC>,
    #[doc = "0x19c - MDIOS output data register"]
    pub mdios_doutr7: crate::Reg<mdios_doutr7::MDIOS_DOUTR7_SPEC>,
    #[doc = "0x1a0 - MDIOS output data register"]
    pub mdios_doutr8: crate::Reg<mdios_doutr8::MDIOS_DOUTR8_SPEC>,
    #[doc = "0x1a4 - MDIOS output data register"]
    pub mdios_doutr9: crate::Reg<mdios_doutr9::MDIOS_DOUTR9_SPEC>,
    #[doc = "0x1a8 - MDIOS output data register"]
    pub mdios_doutr10: crate::Reg<mdios_doutr10::MDIOS_DOUTR10_SPEC>,
    #[doc = "0x1ac - MDIOS output data register"]
    pub mdios_doutr11: crate::Reg<mdios_doutr11::MDIOS_DOUTR11_SPEC>,
    #[doc = "0x1b0 - MDIOS output data register"]
    pub mdios_doutr12: crate::Reg<mdios_doutr12::MDIOS_DOUTR12_SPEC>,
    #[doc = "0x1b4 - MDIOS output data register"]
    pub mdios_doutr13: crate::Reg<mdios_doutr13::MDIOS_DOUTR13_SPEC>,
    #[doc = "0x1b8 - MDIOS output data register"]
    pub mdios_doutr14: crate::Reg<mdios_doutr14::MDIOS_DOUTR14_SPEC>,
    #[doc = "0x1bc - MDIOS output data register"]
    pub mdios_doutr15: crate::Reg<mdios_doutr15::MDIOS_DOUTR15_SPEC>,
    #[doc = "0x1c0 - MDIOS output data register"]
    pub mdios_doutr16: crate::Reg<mdios_doutr16::MDIOS_DOUTR16_SPEC>,
    #[doc = "0x1c4 - MDIOS output data register"]
    pub mdios_doutr17: crate::Reg<mdios_doutr17::MDIOS_DOUTR17_SPEC>,
    #[doc = "0x1c8 - MDIOS output data register"]
    pub mdios_doutr18: crate::Reg<mdios_doutr18::MDIOS_DOUTR18_SPEC>,
    #[doc = "0x1cc - MDIOS output data register"]
    pub mdios_doutr19: crate::Reg<mdios_doutr19::MDIOS_DOUTR19_SPEC>,
    #[doc = "0x1d0 - MDIOS output data register"]
    pub mdios_doutr20: crate::Reg<mdios_doutr20::MDIOS_DOUTR20_SPEC>,
    #[doc = "0x1d4 - MDIOS output data register"]
    pub mdios_doutr21: crate::Reg<mdios_doutr21::MDIOS_DOUTR21_SPEC>,
    #[doc = "0x1d8 - MDIOS output data register"]
    pub mdios_doutr22: crate::Reg<mdios_doutr22::MDIOS_DOUTR22_SPEC>,
    #[doc = "0x1dc - MDIOS output data register"]
    pub mdios_doutr23: crate::Reg<mdios_doutr23::MDIOS_DOUTR23_SPEC>,
    #[doc = "0x1e0 - MDIOS output data register"]
    pub mdios_doutr24: crate::Reg<mdios_doutr24::MDIOS_DOUTR24_SPEC>,
    #[doc = "0x1e4 - MDIOS output data register"]
    pub mdios_doutr25: crate::Reg<mdios_doutr25::MDIOS_DOUTR25_SPEC>,
    #[doc = "0x1e8 - MDIOS output data register"]
    pub mdios_doutr26: crate::Reg<mdios_doutr26::MDIOS_DOUTR26_SPEC>,
    #[doc = "0x1ec - MDIOS output data register"]
    pub mdios_doutr27: crate::Reg<mdios_doutr27::MDIOS_DOUTR27_SPEC>,
    #[doc = "0x1f0 - MDIOS output data register"]
    pub mdios_doutr28: crate::Reg<mdios_doutr28::MDIOS_DOUTR28_SPEC>,
    #[doc = "0x1f4 - MDIOS output data register"]
    pub mdios_doutr29: crate::Reg<mdios_doutr29::MDIOS_DOUTR29_SPEC>,
    #[doc = "0x1f8 - MDIOS output data register"]
    pub mdios_doutr30: crate::Reg<mdios_doutr30::MDIOS_DOUTR30_SPEC>,
    #[doc = "0x1fc - MDIOS output data register"]
    pub mdios_doutr31: crate::Reg<mdios_doutr31::MDIOS_DOUTR31_SPEC>,
    _reserved71: [u8; 0x01f0],
    #[doc = "0x3f0 - MDIOS HW configuration register"]
    pub mdios_hwcfgr: crate::Reg<mdios_hwcfgr::MDIOS_HWCFGR_SPEC>,
    #[doc = "0x3f4 - MDIOS version register"]
    pub mdios_verr: crate::Reg<mdios_verr::MDIOS_VERR_SPEC>,
    #[doc = "0x3f8 - MDIOS identification register"]
    pub mdios_ipidr: crate::Reg<mdios_ipidr::MDIOS_IPIDR_SPEC>,
    #[doc = "0x3fc - MDIOS size identification register"]
    pub mdios_sidr: crate::Reg<mdios_sidr::MDIOS_SIDR_SPEC>,
}
#[doc = "MDIOS_CR register accessor: an alias for `Reg<MDIOS_CR_SPEC>`"]
pub type MDIOS_CR = crate::Reg<mdios_cr::MDIOS_CR_SPEC>;
#[doc = "MDIOS configuration register"]
pub mod mdios_cr;
#[doc = "MDIOS_WRFR register accessor: an alias for `Reg<MDIOS_WRFR_SPEC>`"]
pub type MDIOS_WRFR = crate::Reg<mdios_wrfr::MDIOS_WRFR_SPEC>;
#[doc = "MDIOS write flag register"]
pub mod mdios_wrfr;
#[doc = "MDIOS_CWRFR register accessor: an alias for `Reg<MDIOS_CWRFR_SPEC>`"]
pub type MDIOS_CWRFR = crate::Reg<mdios_cwrfr::MDIOS_CWRFR_SPEC>;
#[doc = "MDIOS clear write flag register"]
pub mod mdios_cwrfr;
#[doc = "MDIOS_RDFR register accessor: an alias for `Reg<MDIOS_RDFR_SPEC>`"]
pub type MDIOS_RDFR = crate::Reg<mdios_rdfr::MDIOS_RDFR_SPEC>;
#[doc = "MDIOS read flag register"]
pub mod mdios_rdfr;
#[doc = "MDIOS_CRDFR register accessor: an alias for `Reg<MDIOS_CRDFR_SPEC>`"]
pub type MDIOS_CRDFR = crate::Reg<mdios_crdfr::MDIOS_CRDFR_SPEC>;
#[doc = "MDIOS clear read flag register"]
pub mod mdios_crdfr;
#[doc = "MDIOS_SR register accessor: an alias for `Reg<MDIOS_SR_SPEC>`"]
pub type MDIOS_SR = crate::Reg<mdios_sr::MDIOS_SR_SPEC>;
#[doc = "MDIOS status register"]
pub mod mdios_sr;
#[doc = "MDIOS_CLRFR register accessor: an alias for `Reg<MDIOS_CLRFR_SPEC>`"]
pub type MDIOS_CLRFR = crate::Reg<mdios_clrfr::MDIOS_CLRFR_SPEC>;
#[doc = "MDIOS clear flag register"]
pub mod mdios_clrfr;
#[doc = "MDIOS_DINR0 register accessor: an alias for `Reg<MDIOS_DINR0_SPEC>`"]
pub type MDIOS_DINR0 = crate::Reg<mdios_dinr0::MDIOS_DINR0_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr0;
#[doc = "MDIOS_DINR1 register accessor: an alias for `Reg<MDIOS_DINR1_SPEC>`"]
pub type MDIOS_DINR1 = crate::Reg<mdios_dinr1::MDIOS_DINR1_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr1;
#[doc = "MDIOS_DINR2 register accessor: an alias for `Reg<MDIOS_DINR2_SPEC>`"]
pub type MDIOS_DINR2 = crate::Reg<mdios_dinr2::MDIOS_DINR2_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr2;
#[doc = "MDIOS_DINR3 register accessor: an alias for `Reg<MDIOS_DINR3_SPEC>`"]
pub type MDIOS_DINR3 = crate::Reg<mdios_dinr3::MDIOS_DINR3_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr3;
#[doc = "MDIOS_DINR4 register accessor: an alias for `Reg<MDIOS_DINR4_SPEC>`"]
pub type MDIOS_DINR4 = crate::Reg<mdios_dinr4::MDIOS_DINR4_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr4;
#[doc = "MDIOS_DINR5 register accessor: an alias for `Reg<MDIOS_DINR5_SPEC>`"]
pub type MDIOS_DINR5 = crate::Reg<mdios_dinr5::MDIOS_DINR5_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr5;
#[doc = "MDIOS_DINR6 register accessor: an alias for `Reg<MDIOS_DINR6_SPEC>`"]
pub type MDIOS_DINR6 = crate::Reg<mdios_dinr6::MDIOS_DINR6_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr6;
#[doc = "MDIOS_DINR7 register accessor: an alias for `Reg<MDIOS_DINR7_SPEC>`"]
pub type MDIOS_DINR7 = crate::Reg<mdios_dinr7::MDIOS_DINR7_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr7;
#[doc = "MDIOS_DINR8 register accessor: an alias for `Reg<MDIOS_DINR8_SPEC>`"]
pub type MDIOS_DINR8 = crate::Reg<mdios_dinr8::MDIOS_DINR8_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr8;
#[doc = "MDIOS_DINR9 register accessor: an alias for `Reg<MDIOS_DINR9_SPEC>`"]
pub type MDIOS_DINR9 = crate::Reg<mdios_dinr9::MDIOS_DINR9_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr9;
#[doc = "MDIOS_DINR10 register accessor: an alias for `Reg<MDIOS_DINR10_SPEC>`"]
pub type MDIOS_DINR10 = crate::Reg<mdios_dinr10::MDIOS_DINR10_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr10;
#[doc = "MDIOS_DINR11 register accessor: an alias for `Reg<MDIOS_DINR11_SPEC>`"]
pub type MDIOS_DINR11 = crate::Reg<mdios_dinr11::MDIOS_DINR11_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr11;
#[doc = "MDIOS_DINR12 register accessor: an alias for `Reg<MDIOS_DINR12_SPEC>`"]
pub type MDIOS_DINR12 = crate::Reg<mdios_dinr12::MDIOS_DINR12_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr12;
#[doc = "MDIOS_DINR13 register accessor: an alias for `Reg<MDIOS_DINR13_SPEC>`"]
pub type MDIOS_DINR13 = crate::Reg<mdios_dinr13::MDIOS_DINR13_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr13;
#[doc = "MDIOS_DINR14 register accessor: an alias for `Reg<MDIOS_DINR14_SPEC>`"]
pub type MDIOS_DINR14 = crate::Reg<mdios_dinr14::MDIOS_DINR14_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr14;
#[doc = "MDIOS_DINR15 register accessor: an alias for `Reg<MDIOS_DINR15_SPEC>`"]
pub type MDIOS_DINR15 = crate::Reg<mdios_dinr15::MDIOS_DINR15_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr15;
#[doc = "MDIOS_DINR16 register accessor: an alias for `Reg<MDIOS_DINR16_SPEC>`"]
pub type MDIOS_DINR16 = crate::Reg<mdios_dinr16::MDIOS_DINR16_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr16;
#[doc = "MDIOS_DINR17 register accessor: an alias for `Reg<MDIOS_DINR17_SPEC>`"]
pub type MDIOS_DINR17 = crate::Reg<mdios_dinr17::MDIOS_DINR17_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr17;
#[doc = "MDIOS_DINR18 register accessor: an alias for `Reg<MDIOS_DINR18_SPEC>`"]
pub type MDIOS_DINR18 = crate::Reg<mdios_dinr18::MDIOS_DINR18_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr18;
#[doc = "MDIOS_DINR19 register accessor: an alias for `Reg<MDIOS_DINR19_SPEC>`"]
pub type MDIOS_DINR19 = crate::Reg<mdios_dinr19::MDIOS_DINR19_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr19;
#[doc = "MDIOS_DINR20 register accessor: an alias for `Reg<MDIOS_DINR20_SPEC>`"]
pub type MDIOS_DINR20 = crate::Reg<mdios_dinr20::MDIOS_DINR20_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr20;
#[doc = "MDIOS_DINR21 register accessor: an alias for `Reg<MDIOS_DINR21_SPEC>`"]
pub type MDIOS_DINR21 = crate::Reg<mdios_dinr21::MDIOS_DINR21_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr21;
#[doc = "MDIOS_DINR22 register accessor: an alias for `Reg<MDIOS_DINR22_SPEC>`"]
pub type MDIOS_DINR22 = crate::Reg<mdios_dinr22::MDIOS_DINR22_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr22;
#[doc = "MDIOS_DINR23 register accessor: an alias for `Reg<MDIOS_DINR23_SPEC>`"]
pub type MDIOS_DINR23 = crate::Reg<mdios_dinr23::MDIOS_DINR23_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr23;
#[doc = "MDIOS_DINR24 register accessor: an alias for `Reg<MDIOS_DINR24_SPEC>`"]
pub type MDIOS_DINR24 = crate::Reg<mdios_dinr24::MDIOS_DINR24_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr24;
#[doc = "MDIOS_DINR25 register accessor: an alias for `Reg<MDIOS_DINR25_SPEC>`"]
pub type MDIOS_DINR25 = crate::Reg<mdios_dinr25::MDIOS_DINR25_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr25;
#[doc = "MDIOS_DINR26 register accessor: an alias for `Reg<MDIOS_DINR26_SPEC>`"]
pub type MDIOS_DINR26 = crate::Reg<mdios_dinr26::MDIOS_DINR26_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr26;
#[doc = "MDIOS_DINR27 register accessor: an alias for `Reg<MDIOS_DINR27_SPEC>`"]
pub type MDIOS_DINR27 = crate::Reg<mdios_dinr27::MDIOS_DINR27_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr27;
#[doc = "MDIOS_DINR28 register accessor: an alias for `Reg<MDIOS_DINR28_SPEC>`"]
pub type MDIOS_DINR28 = crate::Reg<mdios_dinr28::MDIOS_DINR28_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr28;
#[doc = "MDIOS_DINR29 register accessor: an alias for `Reg<MDIOS_DINR29_SPEC>`"]
pub type MDIOS_DINR29 = crate::Reg<mdios_dinr29::MDIOS_DINR29_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr29;
#[doc = "MDIOS_DINR30 register accessor: an alias for `Reg<MDIOS_DINR30_SPEC>`"]
pub type MDIOS_DINR30 = crate::Reg<mdios_dinr30::MDIOS_DINR30_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr30;
#[doc = "MDIOS_DINR31 register accessor: an alias for `Reg<MDIOS_DINR31_SPEC>`"]
pub type MDIOS_DINR31 = crate::Reg<mdios_dinr31::MDIOS_DINR31_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_dinr31;
#[doc = "MDIOS_DOUTR0 register accessor: an alias for `Reg<MDIOS_DOUTR0_SPEC>`"]
pub type MDIOS_DOUTR0 = crate::Reg<mdios_doutr0::MDIOS_DOUTR0_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_doutr0;
#[doc = "MDIOS_DOUTR1 register accessor: an alias for `Reg<MDIOS_DOUTR1_SPEC>`"]
pub type MDIOS_DOUTR1 = crate::Reg<mdios_doutr1::MDIOS_DOUTR1_SPEC>;
#[doc = "MDIOS input data register"]
pub mod mdios_doutr1;
#[doc = "MDIOS_DOUTR2 register accessor: an alias for `Reg<MDIOS_DOUTR2_SPEC>`"]
pub type MDIOS_DOUTR2 = crate::Reg<mdios_doutr2::MDIOS_DOUTR2_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr2;
#[doc = "MDIOS_DOUTR3 register accessor: an alias for `Reg<MDIOS_DOUTR3_SPEC>`"]
pub type MDIOS_DOUTR3 = crate::Reg<mdios_doutr3::MDIOS_DOUTR3_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr3;
#[doc = "MDIOS_DOUTR4 register accessor: an alias for `Reg<MDIOS_DOUTR4_SPEC>`"]
pub type MDIOS_DOUTR4 = crate::Reg<mdios_doutr4::MDIOS_DOUTR4_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr4;
#[doc = "MDIOS_DOUTR5 register accessor: an alias for `Reg<MDIOS_DOUTR5_SPEC>`"]
pub type MDIOS_DOUTR5 = crate::Reg<mdios_doutr5::MDIOS_DOUTR5_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr5;
#[doc = "MDIOS_DOUTR6 register accessor: an alias for `Reg<MDIOS_DOUTR6_SPEC>`"]
pub type MDIOS_DOUTR6 = crate::Reg<mdios_doutr6::MDIOS_DOUTR6_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr6;
#[doc = "MDIOS_DOUTR7 register accessor: an alias for `Reg<MDIOS_DOUTR7_SPEC>`"]
pub type MDIOS_DOUTR7 = crate::Reg<mdios_doutr7::MDIOS_DOUTR7_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr7;
#[doc = "MDIOS_DOUTR8 register accessor: an alias for `Reg<MDIOS_DOUTR8_SPEC>`"]
pub type MDIOS_DOUTR8 = crate::Reg<mdios_doutr8::MDIOS_DOUTR8_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr8;
#[doc = "MDIOS_DOUTR9 register accessor: an alias for `Reg<MDIOS_DOUTR9_SPEC>`"]
pub type MDIOS_DOUTR9 = crate::Reg<mdios_doutr9::MDIOS_DOUTR9_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr9;
#[doc = "MDIOS_DOUTR10 register accessor: an alias for `Reg<MDIOS_DOUTR10_SPEC>`"]
pub type MDIOS_DOUTR10 = crate::Reg<mdios_doutr10::MDIOS_DOUTR10_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr10;
#[doc = "MDIOS_DOUTR11 register accessor: an alias for `Reg<MDIOS_DOUTR11_SPEC>`"]
pub type MDIOS_DOUTR11 = crate::Reg<mdios_doutr11::MDIOS_DOUTR11_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr11;
#[doc = "MDIOS_DOUTR12 register accessor: an alias for `Reg<MDIOS_DOUTR12_SPEC>`"]
pub type MDIOS_DOUTR12 = crate::Reg<mdios_doutr12::MDIOS_DOUTR12_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr12;
#[doc = "MDIOS_DOUTR13 register accessor: an alias for `Reg<MDIOS_DOUTR13_SPEC>`"]
pub type MDIOS_DOUTR13 = crate::Reg<mdios_doutr13::MDIOS_DOUTR13_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr13;
#[doc = "MDIOS_DOUTR14 register accessor: an alias for `Reg<MDIOS_DOUTR14_SPEC>`"]
pub type MDIOS_DOUTR14 = crate::Reg<mdios_doutr14::MDIOS_DOUTR14_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr14;
#[doc = "MDIOS_DOUTR15 register accessor: an alias for `Reg<MDIOS_DOUTR15_SPEC>`"]
pub type MDIOS_DOUTR15 = crate::Reg<mdios_doutr15::MDIOS_DOUTR15_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr15;
#[doc = "MDIOS_DOUTR16 register accessor: an alias for `Reg<MDIOS_DOUTR16_SPEC>`"]
pub type MDIOS_DOUTR16 = crate::Reg<mdios_doutr16::MDIOS_DOUTR16_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr16;
#[doc = "MDIOS_DOUTR17 register accessor: an alias for `Reg<MDIOS_DOUTR17_SPEC>`"]
pub type MDIOS_DOUTR17 = crate::Reg<mdios_doutr17::MDIOS_DOUTR17_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr17;
#[doc = "MDIOS_DOUTR18 register accessor: an alias for `Reg<MDIOS_DOUTR18_SPEC>`"]
pub type MDIOS_DOUTR18 = crate::Reg<mdios_doutr18::MDIOS_DOUTR18_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr18;
#[doc = "MDIOS_DOUTR19 register accessor: an alias for `Reg<MDIOS_DOUTR19_SPEC>`"]
pub type MDIOS_DOUTR19 = crate::Reg<mdios_doutr19::MDIOS_DOUTR19_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr19;
#[doc = "MDIOS_DOUTR20 register accessor: an alias for `Reg<MDIOS_DOUTR20_SPEC>`"]
pub type MDIOS_DOUTR20 = crate::Reg<mdios_doutr20::MDIOS_DOUTR20_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr20;
#[doc = "MDIOS_DOUTR21 register accessor: an alias for `Reg<MDIOS_DOUTR21_SPEC>`"]
pub type MDIOS_DOUTR21 = crate::Reg<mdios_doutr21::MDIOS_DOUTR21_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr21;
#[doc = "MDIOS_DOUTR22 register accessor: an alias for `Reg<MDIOS_DOUTR22_SPEC>`"]
pub type MDIOS_DOUTR22 = crate::Reg<mdios_doutr22::MDIOS_DOUTR22_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr22;
#[doc = "MDIOS_DOUTR23 register accessor: an alias for `Reg<MDIOS_DOUTR23_SPEC>`"]
pub type MDIOS_DOUTR23 = crate::Reg<mdios_doutr23::MDIOS_DOUTR23_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr23;
#[doc = "MDIOS_DOUTR24 register accessor: an alias for `Reg<MDIOS_DOUTR24_SPEC>`"]
pub type MDIOS_DOUTR24 = crate::Reg<mdios_doutr24::MDIOS_DOUTR24_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr24;
#[doc = "MDIOS_DOUTR25 register accessor: an alias for `Reg<MDIOS_DOUTR25_SPEC>`"]
pub type MDIOS_DOUTR25 = crate::Reg<mdios_doutr25::MDIOS_DOUTR25_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr25;
#[doc = "MDIOS_DOUTR26 register accessor: an alias for `Reg<MDIOS_DOUTR26_SPEC>`"]
pub type MDIOS_DOUTR26 = crate::Reg<mdios_doutr26::MDIOS_DOUTR26_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr26;
#[doc = "MDIOS_DOUTR27 register accessor: an alias for `Reg<MDIOS_DOUTR27_SPEC>`"]
pub type MDIOS_DOUTR27 = crate::Reg<mdios_doutr27::MDIOS_DOUTR27_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr27;
#[doc = "MDIOS_DOUTR28 register accessor: an alias for `Reg<MDIOS_DOUTR28_SPEC>`"]
pub type MDIOS_DOUTR28 = crate::Reg<mdios_doutr28::MDIOS_DOUTR28_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr28;
#[doc = "MDIOS_DOUTR29 register accessor: an alias for `Reg<MDIOS_DOUTR29_SPEC>`"]
pub type MDIOS_DOUTR29 = crate::Reg<mdios_doutr29::MDIOS_DOUTR29_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr29;
#[doc = "MDIOS_DOUTR30 register accessor: an alias for `Reg<MDIOS_DOUTR30_SPEC>`"]
pub type MDIOS_DOUTR30 = crate::Reg<mdios_doutr30::MDIOS_DOUTR30_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr30;
#[doc = "MDIOS_DOUTR31 register accessor: an alias for `Reg<MDIOS_DOUTR31_SPEC>`"]
pub type MDIOS_DOUTR31 = crate::Reg<mdios_doutr31::MDIOS_DOUTR31_SPEC>;
#[doc = "MDIOS output data register"]
pub mod mdios_doutr31;
#[doc = "MDIOS_HWCFGR register accessor: an alias for `Reg<MDIOS_HWCFGR_SPEC>`"]
pub type MDIOS_HWCFGR = crate::Reg<mdios_hwcfgr::MDIOS_HWCFGR_SPEC>;
#[doc = "MDIOS HW configuration register"]
pub mod mdios_hwcfgr;
#[doc = "MDIOS_VERR register accessor: an alias for `Reg<MDIOS_VERR_SPEC>`"]
pub type MDIOS_VERR = crate::Reg<mdios_verr::MDIOS_VERR_SPEC>;
#[doc = "MDIOS version register"]
pub mod mdios_verr;
#[doc = "MDIOS_IPIDR register accessor: an alias for `Reg<MDIOS_IPIDR_SPEC>`"]
pub type MDIOS_IPIDR = crate::Reg<mdios_ipidr::MDIOS_IPIDR_SPEC>;
#[doc = "MDIOS identification register"]
pub mod mdios_ipidr;
#[doc = "MDIOS_SIDR register accessor: an alias for `Reg<MDIOS_SIDR_SPEC>`"]
pub type MDIOS_SIDR = crate::Reg<mdios_sidr::MDIOS_SIDR_SPEC>;
#[doc = "MDIOS size identification register"]
pub mod mdios_sidr;
