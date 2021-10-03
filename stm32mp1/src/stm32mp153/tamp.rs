#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_cr1: crate::Reg<tamp_cr1::TAMP_CR1_SPEC>,
    #[doc = "0x04 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_cr2: crate::Reg<tamp_cr2::TAMP_CR2_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_fltcr: crate::Reg<tamp_fltcr::TAMP_FLTCR_SPEC>,
    #[doc = "0x10 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_atcr1: crate::Reg<tamp_atcr1::TAMP_ATCR1_SPEC>,
    #[doc = "0x14 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_atseedr: crate::Reg<tamp_atseedr::TAMP_ATSEEDR_SPEC>,
    #[doc = "0x18 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_ator: crate::Reg<tamp_ator::TAMP_ATOR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    pub tamp_smcr: crate::Reg<tamp_smcr::TAMP_SMCR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x2c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_ier: crate::Reg<tamp_ier::TAMP_IER_SPEC>,
    #[doc = "0x30 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
    pub tamp_sr: crate::Reg<tamp_sr::TAMP_SR_SPEC>,
    #[doc = "0x34 - TAMP non-secure masked interrupt status register"]
    pub tamp_misr: crate::Reg<tamp_misr::TAMP_MISR_SPEC>,
    #[doc = "0x38 - TAMP secure masked interrupt status register"]
    pub tamp_smisr: crate::Reg<tamp_smisr::TAMP_SMISR_SPEC>,
    #[doc = "0x3c - TAMP status clear register"]
    pub tamp_scr: crate::Reg<tamp_scr::TAMP_SCR_SPEC>,
    #[doc = "0x40 - TAMP monotonic counter register"]
    pub tamp_countr: crate::Reg<tamp_countr::TAMP_COUNTR_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x50 - TAMP configuration register"]
    pub tamp_cfgr: crate::Reg<tamp_cfgr::TAMP_CFGR_SPEC>,
    _reserved14: [u8; 0xac],
    #[doc = "0x100 - TAMP backup 0 register"]
    pub tamp_bkp0r: crate::Reg<tamp_bkp0r::TAMP_BKP0R_SPEC>,
    #[doc = "0x104 - TAMP backup 1 register"]
    pub tamp_bkp1r: crate::Reg<tamp_bkp1r::TAMP_BKP1R_SPEC>,
    #[doc = "0x108 - TAMP backup 2 register"]
    pub tamp_bkp2r: crate::Reg<tamp_bkp2r::TAMP_BKP2R_SPEC>,
    #[doc = "0x10c - TAMP backup 3 register"]
    pub tamp_bkp3r: crate::Reg<tamp_bkp3r::TAMP_BKP3R_SPEC>,
    #[doc = "0x110 - TAMP backup 4 register"]
    pub tamp_bkp4r: crate::Reg<tamp_bkp4r::TAMP_BKP4R_SPEC>,
    #[doc = "0x114 - TAMP backup 5 register"]
    pub tamp_bkp5r: crate::Reg<tamp_bkp5r::TAMP_BKP5R_SPEC>,
    #[doc = "0x118 - TAMP backup 6 register"]
    pub tamp_bkp6r: crate::Reg<tamp_bkp6r::TAMP_BKP6R_SPEC>,
    #[doc = "0x11c - TAMP backup 7 register"]
    pub tamp_bkp7r: crate::Reg<tamp_bkp7r::TAMP_BKP7R_SPEC>,
    #[doc = "0x120 - TAMP backup 8 register"]
    pub tamp_bkp8r: crate::Reg<tamp_bkp8r::TAMP_BKP8R_SPEC>,
    #[doc = "0x124 - TAMP backup 9 register"]
    pub tamp_bkp9r: crate::Reg<tamp_bkp9r::TAMP_BKP9R_SPEC>,
    #[doc = "0x128 - TAMP backup 10 register"]
    pub tamp_bkp10r: crate::Reg<tamp_bkp10r::TAMP_BKP10R_SPEC>,
    #[doc = "0x12c - TAMP backup 11 register"]
    pub tamp_bkp11r: crate::Reg<tamp_bkp11r::TAMP_BKP11R_SPEC>,
    #[doc = "0x130 - TAMP backup 12 register"]
    pub tamp_bkp12r: crate::Reg<tamp_bkp12r::TAMP_BKP12R_SPEC>,
    #[doc = "0x134 - TAMP backup 13 register"]
    pub tamp_bkp13r: crate::Reg<tamp_bkp13r::TAMP_BKP13R_SPEC>,
    #[doc = "0x138 - TAMP backup 14 register"]
    pub tamp_bkp14r: crate::Reg<tamp_bkp14r::TAMP_BKP14R_SPEC>,
    #[doc = "0x13c - TAMP backup 15 register"]
    pub tamp_bkp15r: crate::Reg<tamp_bkp15r::TAMP_BKP15R_SPEC>,
    #[doc = "0x140 - TAMP backup 16 register"]
    pub tamp_bkp16r: crate::Reg<tamp_bkp16r::TAMP_BKP16R_SPEC>,
    #[doc = "0x144 - TAMP backup 17 register"]
    pub tamp_bkp17r: crate::Reg<tamp_bkp17r::TAMP_BKP17R_SPEC>,
    #[doc = "0x148 - TAMP backup 18 register"]
    pub tamp_bkp18r: crate::Reg<tamp_bkp18r::TAMP_BKP18R_SPEC>,
    #[doc = "0x14c - TAMP backup 19 register"]
    pub tamp_bkp19r: crate::Reg<tamp_bkp19r::TAMP_BKP19R_SPEC>,
    #[doc = "0x150 - TAMP backup 20 register"]
    pub tamp_bkp20r: crate::Reg<tamp_bkp20r::TAMP_BKP20R_SPEC>,
    #[doc = "0x154 - TAMP backup 21 register"]
    pub tamp_bkp21r: crate::Reg<tamp_bkp21r::TAMP_BKP21R_SPEC>,
    #[doc = "0x158 - TAMP backup 22 register"]
    pub tamp_bkp22r: crate::Reg<tamp_bkp22r::TAMP_BKP22R_SPEC>,
    #[doc = "0x15c - TAMP backup 23 register"]
    pub tamp_bkp23r: crate::Reg<tamp_bkp23r::TAMP_BKP23R_SPEC>,
    #[doc = "0x160 - TAMP backup 24 register"]
    pub tamp_bkp24r: crate::Reg<tamp_bkp24r::TAMP_BKP24R_SPEC>,
    #[doc = "0x164 - TAMP backup 25 register"]
    pub tamp_bkp25r: crate::Reg<tamp_bkp25r::TAMP_BKP25R_SPEC>,
    #[doc = "0x168 - TAMP backup 26 register"]
    pub tamp_bkp26r: crate::Reg<tamp_bkp26r::TAMP_BKP26R_SPEC>,
    #[doc = "0x16c - TAMP backup 27 register"]
    pub tamp_bkp27r: crate::Reg<tamp_bkp27r::TAMP_BKP27R_SPEC>,
    #[doc = "0x170 - TAMP backup 28 register"]
    pub tamp_bkp28r: crate::Reg<tamp_bkp28r::TAMP_BKP28R_SPEC>,
    #[doc = "0x174 - TAMP backup 29 register"]
    pub tamp_bkp29r: crate::Reg<tamp_bkp29r::TAMP_BKP29R_SPEC>,
    #[doc = "0x178 - TAMP backup 30 register"]
    pub tamp_bkp30r: crate::Reg<tamp_bkp30r::TAMP_BKP30R_SPEC>,
    #[doc = "0x17c - TAMP backup 31 register"]
    pub tamp_bkp31r: crate::Reg<tamp_bkp31r::TAMP_BKP31R_SPEC>,
    _reserved46: [u8; 0x026c],
    #[doc = "0x3ec - TAMP hardware configuration register 2"]
    pub tamp_hwcfgr2: crate::Reg<tamp_hwcfgr2::TAMP_HWCFGR2_SPEC>,
    #[doc = "0x3f0 - TAMP hardware configuration register 1"]
    pub tamp_hwcfgr1: crate::Reg<tamp_hwcfgr1::TAMP_HWCFGR1_SPEC>,
    #[doc = "0x3f4 - TAMP version register"]
    pub tamp_verr: crate::Reg<tamp_verr::TAMP_VERR_SPEC>,
    #[doc = "0x3f8 - TAMP identification register"]
    pub tamp_ipidr: crate::Reg<tamp_ipidr::TAMP_IPIDR_SPEC>,
    #[doc = "0x3fc - TAMP size identification register"]
    pub tamp_sidr: crate::Reg<tamp_sidr::TAMP_SIDR_SPEC>,
}
#[doc = "TAMP_CR1 register accessor: an alias for `Reg<TAMP_CR1_SPEC>`"]
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_cr1;
#[doc = "TAMP_CR2 register accessor: an alias for `Reg<TAMP_CR2_SPEC>`"]
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_cr2;
#[doc = "TAMP_FLTCR register accessor: an alias for `Reg<TAMP_FLTCR_SPEC>`"]
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_fltcr;
#[doc = "TAMP_ATCR1 register accessor: an alias for `Reg<TAMP_ATCR1_SPEC>`"]
pub type TAMP_ATCR1 = crate::Reg<tamp_atcr1::TAMP_ATCR1_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_atcr1;
#[doc = "TAMP_ATSEEDR register accessor: an alias for `Reg<TAMP_ATSEEDR_SPEC>`"]
pub type TAMP_ATSEEDR = crate::Reg<tamp_atseedr::TAMP_ATSEEDR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_atseedr;
#[doc = "TAMP_ATOR register accessor: an alias for `Reg<TAMP_ATOR_SPEC>`"]
pub type TAMP_ATOR = crate::Reg<tamp_ator::TAMP_ATOR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_ator;
#[doc = "TAMP_SMCR register accessor: an alias for `Reg<TAMP_SMCR_SPEC>`"]
pub type TAMP_SMCR = crate::Reg<tamp_smcr::TAMP_SMCR_SPEC>;
#[doc = "This register can be written only when the APB access is secure."]
pub mod tamp_smcr;
#[doc = "TAMP_IER register accessor: an alias for `Reg<TAMP_IER_SPEC>`"]
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IER_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_ier;
#[doc = "TAMP_SR register accessor: an alias for `Reg<TAMP_SR_SPEC>`"]
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SR_SPEC>;
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes."]
pub mod tamp_sr;
#[doc = "TAMP_MISR register accessor: an alias for `Reg<TAMP_MISR_SPEC>`"]
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISR_SPEC>;
#[doc = "TAMP non-secure masked interrupt status register"]
pub mod tamp_misr;
#[doc = "TAMP_SMISR register accessor: an alias for `Reg<TAMP_SMISR_SPEC>`"]
pub type TAMP_SMISR = crate::Reg<tamp_smisr::TAMP_SMISR_SPEC>;
#[doc = "TAMP secure masked interrupt status register"]
pub mod tamp_smisr;
#[doc = "TAMP_SCR register accessor: an alias for `Reg<TAMP_SCR_SPEC>`"]
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCR_SPEC>;
#[doc = "TAMP status clear register"]
pub mod tamp_scr;
#[doc = "TAMP_COUNTR register accessor: an alias for `Reg<TAMP_COUNTR_SPEC>`"]
pub type TAMP_COUNTR = crate::Reg<tamp_countr::TAMP_COUNTR_SPEC>;
#[doc = "TAMP monotonic counter register"]
pub mod tamp_countr;
#[doc = "TAMP_CFGR register accessor: an alias for `Reg<TAMP_CFGR_SPEC>`"]
pub type TAMP_CFGR = crate::Reg<tamp_cfgr::TAMP_CFGR_SPEC>;
#[doc = "TAMP configuration register"]
pub mod tamp_cfgr;
#[doc = "TAMP_BKP0R register accessor: an alias for `Reg<TAMP_BKP0R_SPEC>`"]
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0R_SPEC>;
#[doc = "TAMP backup 0 register"]
pub mod tamp_bkp0r;
#[doc = "TAMP_BKP1R register accessor: an alias for `Reg<TAMP_BKP1R_SPEC>`"]
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1R_SPEC>;
#[doc = "TAMP backup 1 register"]
pub mod tamp_bkp1r;
#[doc = "TAMP_BKP2R register accessor: an alias for `Reg<TAMP_BKP2R_SPEC>`"]
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2R_SPEC>;
#[doc = "TAMP backup 2 register"]
pub mod tamp_bkp2r;
#[doc = "TAMP_BKP3R register accessor: an alias for `Reg<TAMP_BKP3R_SPEC>`"]
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3R_SPEC>;
#[doc = "TAMP backup 3 register"]
pub mod tamp_bkp3r;
#[doc = "TAMP_BKP4R register accessor: an alias for `Reg<TAMP_BKP4R_SPEC>`"]
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4R_SPEC>;
#[doc = "TAMP backup 4 register"]
pub mod tamp_bkp4r;
#[doc = "TAMP_BKP5R register accessor: an alias for `Reg<TAMP_BKP5R_SPEC>`"]
pub type TAMP_BKP5R = crate::Reg<tamp_bkp5r::TAMP_BKP5R_SPEC>;
#[doc = "TAMP backup 5 register"]
pub mod tamp_bkp5r;
#[doc = "TAMP_BKP6R register accessor: an alias for `Reg<TAMP_BKP6R_SPEC>`"]
pub type TAMP_BKP6R = crate::Reg<tamp_bkp6r::TAMP_BKP6R_SPEC>;
#[doc = "TAMP backup 6 register"]
pub mod tamp_bkp6r;
#[doc = "TAMP_BKP7R register accessor: an alias for `Reg<TAMP_BKP7R_SPEC>`"]
pub type TAMP_BKP7R = crate::Reg<tamp_bkp7r::TAMP_BKP7R_SPEC>;
#[doc = "TAMP backup 7 register"]
pub mod tamp_bkp7r;
#[doc = "TAMP_BKP8R register accessor: an alias for `Reg<TAMP_BKP8R_SPEC>`"]
pub type TAMP_BKP8R = crate::Reg<tamp_bkp8r::TAMP_BKP8R_SPEC>;
#[doc = "TAMP backup 8 register"]
pub mod tamp_bkp8r;
#[doc = "TAMP_BKP9R register accessor: an alias for `Reg<TAMP_BKP9R_SPEC>`"]
pub type TAMP_BKP9R = crate::Reg<tamp_bkp9r::TAMP_BKP9R_SPEC>;
#[doc = "TAMP backup 9 register"]
pub mod tamp_bkp9r;
#[doc = "TAMP_BKP10R register accessor: an alias for `Reg<TAMP_BKP10R_SPEC>`"]
pub type TAMP_BKP10R = crate::Reg<tamp_bkp10r::TAMP_BKP10R_SPEC>;
#[doc = "TAMP backup 10 register"]
pub mod tamp_bkp10r;
#[doc = "TAMP_BKP11R register accessor: an alias for `Reg<TAMP_BKP11R_SPEC>`"]
pub type TAMP_BKP11R = crate::Reg<tamp_bkp11r::TAMP_BKP11R_SPEC>;
#[doc = "TAMP backup 11 register"]
pub mod tamp_bkp11r;
#[doc = "TAMP_BKP12R register accessor: an alias for `Reg<TAMP_BKP12R_SPEC>`"]
pub type TAMP_BKP12R = crate::Reg<tamp_bkp12r::TAMP_BKP12R_SPEC>;
#[doc = "TAMP backup 12 register"]
pub mod tamp_bkp12r;
#[doc = "TAMP_BKP13R register accessor: an alias for `Reg<TAMP_BKP13R_SPEC>`"]
pub type TAMP_BKP13R = crate::Reg<tamp_bkp13r::TAMP_BKP13R_SPEC>;
#[doc = "TAMP backup 13 register"]
pub mod tamp_bkp13r;
#[doc = "TAMP_BKP14R register accessor: an alias for `Reg<TAMP_BKP14R_SPEC>`"]
pub type TAMP_BKP14R = crate::Reg<tamp_bkp14r::TAMP_BKP14R_SPEC>;
#[doc = "TAMP backup 14 register"]
pub mod tamp_bkp14r;
#[doc = "TAMP_BKP15R register accessor: an alias for `Reg<TAMP_BKP15R_SPEC>`"]
pub type TAMP_BKP15R = crate::Reg<tamp_bkp15r::TAMP_BKP15R_SPEC>;
#[doc = "TAMP backup 15 register"]
pub mod tamp_bkp15r;
#[doc = "TAMP_BKP16R register accessor: an alias for `Reg<TAMP_BKP16R_SPEC>`"]
pub type TAMP_BKP16R = crate::Reg<tamp_bkp16r::TAMP_BKP16R_SPEC>;
#[doc = "TAMP backup 16 register"]
pub mod tamp_bkp16r;
#[doc = "TAMP_BKP17R register accessor: an alias for `Reg<TAMP_BKP17R_SPEC>`"]
pub type TAMP_BKP17R = crate::Reg<tamp_bkp17r::TAMP_BKP17R_SPEC>;
#[doc = "TAMP backup 17 register"]
pub mod tamp_bkp17r;
#[doc = "TAMP_BKP18R register accessor: an alias for `Reg<TAMP_BKP18R_SPEC>`"]
pub type TAMP_BKP18R = crate::Reg<tamp_bkp18r::TAMP_BKP18R_SPEC>;
#[doc = "TAMP backup 18 register"]
pub mod tamp_bkp18r;
#[doc = "TAMP_BKP19R register accessor: an alias for `Reg<TAMP_BKP19R_SPEC>`"]
pub type TAMP_BKP19R = crate::Reg<tamp_bkp19r::TAMP_BKP19R_SPEC>;
#[doc = "TAMP backup 19 register"]
pub mod tamp_bkp19r;
#[doc = "TAMP_BKP20R register accessor: an alias for `Reg<TAMP_BKP20R_SPEC>`"]
pub type TAMP_BKP20R = crate::Reg<tamp_bkp20r::TAMP_BKP20R_SPEC>;
#[doc = "TAMP backup 20 register"]
pub mod tamp_bkp20r;
#[doc = "TAMP_BKP21R register accessor: an alias for `Reg<TAMP_BKP21R_SPEC>`"]
pub type TAMP_BKP21R = crate::Reg<tamp_bkp21r::TAMP_BKP21R_SPEC>;
#[doc = "TAMP backup 21 register"]
pub mod tamp_bkp21r;
#[doc = "TAMP_BKP22R register accessor: an alias for `Reg<TAMP_BKP22R_SPEC>`"]
pub type TAMP_BKP22R = crate::Reg<tamp_bkp22r::TAMP_BKP22R_SPEC>;
#[doc = "TAMP backup 22 register"]
pub mod tamp_bkp22r;
#[doc = "TAMP_BKP23R register accessor: an alias for `Reg<TAMP_BKP23R_SPEC>`"]
pub type TAMP_BKP23R = crate::Reg<tamp_bkp23r::TAMP_BKP23R_SPEC>;
#[doc = "TAMP backup 23 register"]
pub mod tamp_bkp23r;
#[doc = "TAMP_BKP24R register accessor: an alias for `Reg<TAMP_BKP24R_SPEC>`"]
pub type TAMP_BKP24R = crate::Reg<tamp_bkp24r::TAMP_BKP24R_SPEC>;
#[doc = "TAMP backup 24 register"]
pub mod tamp_bkp24r;
#[doc = "TAMP_BKP25R register accessor: an alias for `Reg<TAMP_BKP25R_SPEC>`"]
pub type TAMP_BKP25R = crate::Reg<tamp_bkp25r::TAMP_BKP25R_SPEC>;
#[doc = "TAMP backup 25 register"]
pub mod tamp_bkp25r;
#[doc = "TAMP_BKP26R register accessor: an alias for `Reg<TAMP_BKP26R_SPEC>`"]
pub type TAMP_BKP26R = crate::Reg<tamp_bkp26r::TAMP_BKP26R_SPEC>;
#[doc = "TAMP backup 26 register"]
pub mod tamp_bkp26r;
#[doc = "TAMP_BKP27R register accessor: an alias for `Reg<TAMP_BKP27R_SPEC>`"]
pub type TAMP_BKP27R = crate::Reg<tamp_bkp27r::TAMP_BKP27R_SPEC>;
#[doc = "TAMP backup 27 register"]
pub mod tamp_bkp27r;
#[doc = "TAMP_BKP28R register accessor: an alias for `Reg<TAMP_BKP28R_SPEC>`"]
pub type TAMP_BKP28R = crate::Reg<tamp_bkp28r::TAMP_BKP28R_SPEC>;
#[doc = "TAMP backup 28 register"]
pub mod tamp_bkp28r;
#[doc = "TAMP_BKP29R register accessor: an alias for `Reg<TAMP_BKP29R_SPEC>`"]
pub type TAMP_BKP29R = crate::Reg<tamp_bkp29r::TAMP_BKP29R_SPEC>;
#[doc = "TAMP backup 29 register"]
pub mod tamp_bkp29r;
#[doc = "TAMP_BKP30R register accessor: an alias for `Reg<TAMP_BKP30R_SPEC>`"]
pub type TAMP_BKP30R = crate::Reg<tamp_bkp30r::TAMP_BKP30R_SPEC>;
#[doc = "TAMP backup 30 register"]
pub mod tamp_bkp30r;
#[doc = "TAMP_BKP31R register accessor: an alias for `Reg<TAMP_BKP31R_SPEC>`"]
pub type TAMP_BKP31R = crate::Reg<tamp_bkp31r::TAMP_BKP31R_SPEC>;
#[doc = "TAMP backup 31 register"]
pub mod tamp_bkp31r;
#[doc = "TAMP_HWCFGR2 register accessor: an alias for `Reg<TAMP_HWCFGR2_SPEC>`"]
pub type TAMP_HWCFGR2 = crate::Reg<tamp_hwcfgr2::TAMP_HWCFGR2_SPEC>;
#[doc = "TAMP hardware configuration register 2"]
pub mod tamp_hwcfgr2;
#[doc = "TAMP_HWCFGR1 register accessor: an alias for `Reg<TAMP_HWCFGR1_SPEC>`"]
pub type TAMP_HWCFGR1 = crate::Reg<tamp_hwcfgr1::TAMP_HWCFGR1_SPEC>;
#[doc = "TAMP hardware configuration register 1"]
pub mod tamp_hwcfgr1;
#[doc = "TAMP_VERR register accessor: an alias for `Reg<TAMP_VERR_SPEC>`"]
pub type TAMP_VERR = crate::Reg<tamp_verr::TAMP_VERR_SPEC>;
#[doc = "TAMP version register"]
pub mod tamp_verr;
#[doc = "TAMP_IPIDR register accessor: an alias for `Reg<TAMP_IPIDR_SPEC>`"]
pub type TAMP_IPIDR = crate::Reg<tamp_ipidr::TAMP_IPIDR_SPEC>;
#[doc = "TAMP identification register"]
pub mod tamp_ipidr;
#[doc = "TAMP_SIDR register accessor: an alias for `Reg<TAMP_SIDR_SPEC>`"]
pub type TAMP_SIDR = crate::Reg<tamp_sidr::TAMP_SIDR_SPEC>;
#[doc = "TAMP size identification register"]
pub mod tamp_sidr;
