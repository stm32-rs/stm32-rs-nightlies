#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x08 - control register 3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    #[doc = "0x0c - TAMP filter control register"]
    pub fltcr: crate::Reg<fltcr::FLTCR_SPEC>,
    #[doc = "0x10 - TAMP active tamper control register 1"]
    pub atcr1: crate::Reg<atcr1::ATCR1_SPEC>,
    #[doc = "0x14 - TAMP active tamper seed register"]
    pub atseedr: crate::Reg<atseedr::ATSEEDR_SPEC>,
    #[doc = "0x18 - TAMP active tamper output register"]
    pub ator: crate::Reg<ator::ATOR_SPEC>,
    #[doc = "0x1c - TAMP active tamper control register 2"]
    pub atcr2: crate::Reg<atcr2::ATCR2_SPEC>,
    #[doc = "0x20 - TAMP secure mode register"]
    pub smcr: crate::Reg<smcr::SMCR_SPEC>,
    #[doc = "0x24 - TAMP privilege mode control register"]
    pub privcr: crate::Reg<privcr::PRIVCR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - TAMP interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x30 - TAMP status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x34 - TAMP masked interrupt status register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub smisr: crate::Reg<smisr::SMISR_SPEC>,
    #[doc = "0x3c - TAMP status clear register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x40 - TAMP monotonic counter register"]
    pub countr: crate::Reg<countr::COUNTR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x50 - TAMP configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    _reserved17: [u8; 0xac],
    #[doc = "0x100 - TAMP backup register"]
    pub bkp0r: crate::Reg<bkp0r::BKP0R_SPEC>,
    #[doc = "0x104 - TAMP backup register"]
    pub bkp1r: crate::Reg<bkp1r::BKP1R_SPEC>,
    #[doc = "0x108 - TAMP backup register"]
    pub bkp2r: crate::Reg<bkp2r::BKP2R_SPEC>,
    #[doc = "0x10c - TAMP backup register"]
    pub bkp3r: crate::Reg<bkp3r::BKP3R_SPEC>,
    #[doc = "0x110 - TAMP backup register"]
    pub bkp4r: crate::Reg<bkp4r::BKP4R_SPEC>,
    #[doc = "0x114 - TAMP backup register"]
    pub bkp5r: crate::Reg<bkp5r::BKP5R_SPEC>,
    #[doc = "0x118 - TAMP backup register"]
    pub bkp6r: crate::Reg<bkp6r::BKP6R_SPEC>,
    #[doc = "0x11c - TAMP backup register"]
    pub bkp7r: crate::Reg<bkp7r::BKP7R_SPEC>,
    #[doc = "0x120 - TAMP backup register"]
    pub bkp8r: crate::Reg<bkp8r::BKP8R_SPEC>,
    #[doc = "0x124 - TAMP backup register"]
    pub bkp9r: crate::Reg<bkp9r::BKP9R_SPEC>,
    #[doc = "0x128 - TAMP backup register"]
    pub bkp10r: crate::Reg<bkp10r::BKP10R_SPEC>,
    #[doc = "0x12c - TAMP backup register"]
    pub bkp11r: crate::Reg<bkp11r::BKP11R_SPEC>,
    #[doc = "0x130 - TAMP backup register"]
    pub bkp12r: crate::Reg<bkp12r::BKP12R_SPEC>,
    #[doc = "0x134 - TAMP backup register"]
    pub bkp13r: crate::Reg<bkp13r::BKP13R_SPEC>,
    #[doc = "0x138 - TAMP backup register"]
    pub bkp14r: crate::Reg<bkp14r::BKP14R_SPEC>,
    #[doc = "0x13c - TAMP backup register"]
    pub bkp15r: crate::Reg<bkp15r::BKP15R_SPEC>,
    #[doc = "0x140 - TAMP backup register"]
    pub bkp16r: crate::Reg<bkp16r::BKP16R_SPEC>,
    #[doc = "0x144 - TAMP backup register"]
    pub bkp17r: crate::Reg<bkp17r::BKP17R_SPEC>,
    #[doc = "0x148 - TAMP backup register"]
    pub bkp18r: crate::Reg<bkp18r::BKP18R_SPEC>,
    #[doc = "0x14c - TAMP backup register"]
    pub bkp19r: crate::Reg<bkp19r::BKP19R_SPEC>,
    #[doc = "0x150 - TAMP backup register"]
    pub bkp20r: crate::Reg<bkp20r::BKP20R_SPEC>,
    #[doc = "0x154 - TAMP backup register"]
    pub bkp21r: crate::Reg<bkp21r::BKP21R_SPEC>,
    #[doc = "0x158 - TAMP backup register"]
    pub bkp22r: crate::Reg<bkp22r::BKP22R_SPEC>,
    #[doc = "0x15c - TAMP backup register"]
    pub bkp23r: crate::Reg<bkp23r::BKP23R_SPEC>,
    #[doc = "0x160 - TAMP backup register"]
    pub bkp24r: crate::Reg<bkp24r::BKP24R_SPEC>,
    #[doc = "0x164 - TAMP backup register"]
    pub bkp25r: crate::Reg<bkp25r::BKP25R_SPEC>,
    #[doc = "0x168 - TAMP backup register"]
    pub bkp26r: crate::Reg<bkp26r::BKP26R_SPEC>,
    #[doc = "0x16c - TAMP backup register"]
    pub bkp27r: crate::Reg<bkp27r::BKP27R_SPEC>,
    #[doc = "0x170 - TAMP backup register"]
    pub bkp28r: crate::Reg<bkp28r::BKP28R_SPEC>,
    #[doc = "0x174 - TAMP backup register"]
    pub bkp29r: crate::Reg<bkp29r::BKP29R_SPEC>,
    #[doc = "0x178 - TAMP backup register"]
    pub bkp30r: crate::Reg<bkp30r::BKP30R_SPEC>,
    #[doc = "0x17c - TAMP backup register"]
    pub bkp31r: crate::Reg<bkp31r::BKP31R_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "control register 3"]
pub mod cr3;
#[doc = "FLTCR register accessor: an alias for `Reg<FLTCR_SPEC>`"]
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
#[doc = "TAMP filter control register"]
pub mod fltcr;
#[doc = "ATCR1 register accessor: an alias for `Reg<ATCR1_SPEC>`"]
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
#[doc = "TAMP active tamper control register 1"]
pub mod atcr1;
#[doc = "ATSEEDR register accessor: an alias for `Reg<ATSEEDR_SPEC>`"]
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
#[doc = "TAMP active tamper seed register"]
pub mod atseedr;
#[doc = "ATOR register accessor: an alias for `Reg<ATOR_SPEC>`"]
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
#[doc = "TAMP active tamper output register"]
pub mod ator;
#[doc = "ATCR2 register accessor: an alias for `Reg<ATCR2_SPEC>`"]
pub type ATCR2 = crate::Reg<atcr2::ATCR2_SPEC>;
#[doc = "TAMP active tamper control register 2"]
pub mod atcr2;
#[doc = "SMCR register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "TAMP secure mode register"]
pub mod smcr;
#[doc = "PRIVCR register accessor: an alias for `Reg<PRIVCR_SPEC>`"]
pub type PRIVCR = crate::Reg<privcr::PRIVCR_SPEC>;
#[doc = "TAMP privilege mode control register"]
pub mod privcr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TAMP interrupt enable register"]
pub mod ier;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TAMP status register"]
pub mod sr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "TAMP masked interrupt status register"]
pub mod misr;
#[doc = "SMISR register accessor: an alias for `Reg<SMISR_SPEC>`"]
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
#[doc = "TAMP secure masked interrupt status register"]
pub mod smisr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod scr;
#[doc = "COUNTR register accessor: an alias for `Reg<COUNTR_SPEC>`"]
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
#[doc = "TAMP monotonic counter register"]
pub mod countr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "TAMP configuration register"]
pub mod cfgr;
#[doc = "BKP0R register accessor: an alias for `Reg<BKP0R_SPEC>`"]
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp0r;
#[doc = "BKP1R register accessor: an alias for `Reg<BKP1R_SPEC>`"]
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp1r;
#[doc = "BKP2R register accessor: an alias for `Reg<BKP2R_SPEC>`"]
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp2r;
#[doc = "BKP3R register accessor: an alias for `Reg<BKP3R_SPEC>`"]
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp3r;
#[doc = "BKP4R register accessor: an alias for `Reg<BKP4R_SPEC>`"]
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp4r;
#[doc = "BKP5R register accessor: an alias for `Reg<BKP5R_SPEC>`"]
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp5r;
#[doc = "BKP6R register accessor: an alias for `Reg<BKP6R_SPEC>`"]
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp6r;
#[doc = "BKP7R register accessor: an alias for `Reg<BKP7R_SPEC>`"]
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp7r;
#[doc = "BKP8R register accessor: an alias for `Reg<BKP8R_SPEC>`"]
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp8r;
#[doc = "BKP9R register accessor: an alias for `Reg<BKP9R_SPEC>`"]
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp9r;
#[doc = "BKP10R register accessor: an alias for `Reg<BKP10R_SPEC>`"]
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp10r;
#[doc = "BKP11R register accessor: an alias for `Reg<BKP11R_SPEC>`"]
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp11r;
#[doc = "BKP12R register accessor: an alias for `Reg<BKP12R_SPEC>`"]
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp12r;
#[doc = "BKP13R register accessor: an alias for `Reg<BKP13R_SPEC>`"]
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp13r;
#[doc = "BKP14R register accessor: an alias for `Reg<BKP14R_SPEC>`"]
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp14r;
#[doc = "BKP15R register accessor: an alias for `Reg<BKP15R_SPEC>`"]
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp15r;
#[doc = "BKP16R register accessor: an alias for `Reg<BKP16R_SPEC>`"]
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp16r;
#[doc = "BKP17R register accessor: an alias for `Reg<BKP17R_SPEC>`"]
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp17r;
#[doc = "BKP18R register accessor: an alias for `Reg<BKP18R_SPEC>`"]
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp18r;
#[doc = "BKP19R register accessor: an alias for `Reg<BKP19R_SPEC>`"]
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp19r;
#[doc = "BKP20R register accessor: an alias for `Reg<BKP20R_SPEC>`"]
pub type BKP20R = crate::Reg<bkp20r::BKP20R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp20r;
#[doc = "BKP21R register accessor: an alias for `Reg<BKP21R_SPEC>`"]
pub type BKP21R = crate::Reg<bkp21r::BKP21R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp21r;
#[doc = "BKP22R register accessor: an alias for `Reg<BKP22R_SPEC>`"]
pub type BKP22R = crate::Reg<bkp22r::BKP22R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp22r;
#[doc = "BKP23R register accessor: an alias for `Reg<BKP23R_SPEC>`"]
pub type BKP23R = crate::Reg<bkp23r::BKP23R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp23r;
#[doc = "BKP24R register accessor: an alias for `Reg<BKP24R_SPEC>`"]
pub type BKP24R = crate::Reg<bkp24r::BKP24R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp24r;
#[doc = "BKP25R register accessor: an alias for `Reg<BKP25R_SPEC>`"]
pub type BKP25R = crate::Reg<bkp25r::BKP25R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp25r;
#[doc = "BKP26R register accessor: an alias for `Reg<BKP26R_SPEC>`"]
pub type BKP26R = crate::Reg<bkp26r::BKP26R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp26r;
#[doc = "BKP27R register accessor: an alias for `Reg<BKP27R_SPEC>`"]
pub type BKP27R = crate::Reg<bkp27r::BKP27R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp27r;
#[doc = "BKP28R register accessor: an alias for `Reg<BKP28R_SPEC>`"]
pub type BKP28R = crate::Reg<bkp28r::BKP28R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp28r;
#[doc = "BKP29R register accessor: an alias for `Reg<BKP29R_SPEC>`"]
pub type BKP29R = crate::Reg<bkp29r::BKP29R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp29r;
#[doc = "BKP30R register accessor: an alias for `Reg<BKP30R_SPEC>`"]
pub type BKP30R = crate::Reg<bkp30r::BKP30R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp30r;
#[doc = "BKP31R register accessor: an alias for `Reg<BKP31R_SPEC>`"]
pub type BKP31R = crate::Reg<bkp31r::BKP31R_SPEC>;
#[doc = "TAMP backup register"]
pub mod bkp31r;
