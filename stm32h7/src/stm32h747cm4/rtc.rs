#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - RTC control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub prer: crate::Reg<prer::PRER_SPEC>,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub alrmar: crate::Reg<alrmar::ALRMAR_SPEC>,
    #[doc = "0x20 - This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub alrmbr: crate::Reg<alrmbr::ALRMBR_SPEC>,
    #[doc = "0x24 - RTC write protection register"]
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    #[doc = "0x28 - RTC sub second register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub calr: crate::Reg<calr::CALR_SPEC>,
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    pub tampcr: crate::Reg<tampcr::TAMPCR_SPEC>,
    #[doc = "0x44 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    pub alrmassr: crate::Reg<alrmassr::ALRMASSR_SPEC>,
    #[doc = "0x48 - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
    pub alrmbssr: crate::Reg<alrmbssr::ALRMBSSR_SPEC>,
    #[doc = "0x4c - RTC option register"]
    pub or: crate::Reg<or::OR_SPEC>,
    #[doc = "0x50 - RTC backup registers"]
    pub bkp0r: crate::Reg<bkp0r::BKP0R_SPEC>,
    #[doc = "0x54 - RTC backup registers"]
    pub bkp1r: crate::Reg<bkp1r::BKP1R_SPEC>,
    #[doc = "0x58 - RTC backup registers"]
    pub bkp2r: crate::Reg<bkp2r::BKP2R_SPEC>,
    #[doc = "0x5c - RTC backup registers"]
    pub bkp3r: crate::Reg<bkp3r::BKP3R_SPEC>,
    #[doc = "0x60 - RTC backup registers"]
    pub bkp4r: crate::Reg<bkp4r::BKP4R_SPEC>,
    #[doc = "0x64 - RTC backup registers"]
    pub bkp5r: crate::Reg<bkp5r::BKP5R_SPEC>,
    #[doc = "0x68 - RTC backup registers"]
    pub bkp6r: crate::Reg<bkp6r::BKP6R_SPEC>,
    #[doc = "0x6c - RTC backup registers"]
    pub bkp7r: crate::Reg<bkp7r::BKP7R_SPEC>,
    #[doc = "0x70 - RTC backup registers"]
    pub bkp8r: crate::Reg<bkp8r::BKP8R_SPEC>,
    #[doc = "0x74 - RTC backup registers"]
    pub bkp9r: crate::Reg<bkp9r::BKP9R_SPEC>,
    #[doc = "0x78 - RTC backup registers"]
    pub bkp10r: crate::Reg<bkp10r::BKP10R_SPEC>,
    #[doc = "0x7c - RTC backup registers"]
    pub bkp11r: crate::Reg<bkp11r::BKP11R_SPEC>,
    #[doc = "0x80 - RTC backup registers"]
    pub bkp12r: crate::Reg<bkp12r::BKP12R_SPEC>,
    #[doc = "0x84 - RTC backup registers"]
    pub bkp13r: crate::Reg<bkp13r::BKP13R_SPEC>,
    #[doc = "0x88 - RTC backup registers"]
    pub bkp14r: crate::Reg<bkp14r::BKP14R_SPEC>,
    #[doc = "0x8c - RTC backup registers"]
    pub bkp15r: crate::Reg<bkp15r::BKP15R_SPEC>,
    #[doc = "0x90 - RTC backup registers"]
    pub bkp16r: crate::Reg<bkp16r::BKP16R_SPEC>,
    #[doc = "0x94 - RTC backup registers"]
    pub bkp17r: crate::Reg<bkp17r::BKP17R_SPEC>,
    #[doc = "0x98 - RTC backup registers"]
    pub bkp18r: crate::Reg<bkp18r::BKP18R_SPEC>,
    #[doc = "0x9c - RTC backup registers"]
    pub bkp19r: crate::Reg<bkp19r::BKP19R_SPEC>,
    #[doc = "0xa0 - RTC backup registers"]
    pub bkp20r: crate::Reg<bkp20r::BKP20R_SPEC>,
    #[doc = "0xa4 - RTC backup registers"]
    pub bkp21r: crate::Reg<bkp21r::BKP21R_SPEC>,
    #[doc = "0xa8 - RTC backup registers"]
    pub bkp22r: crate::Reg<bkp22r::BKP22R_SPEC>,
    #[doc = "0xac - RTC backup registers"]
    pub bkp23r: crate::Reg<bkp23r::BKP23R_SPEC>,
    #[doc = "0xb0 - RTC backup registers"]
    pub bkp24r: crate::Reg<bkp24r::BKP24R_SPEC>,
    #[doc = "0xb4 - RTC backup registers"]
    pub bkp25r: crate::Reg<bkp25r::BKP25R_SPEC>,
    #[doc = "0xb8 - RTC backup registers"]
    pub bkp26r: crate::Reg<bkp26r::BKP26R_SPEC>,
    #[doc = "0xbc - RTC backup registers"]
    pub bkp27r: crate::Reg<bkp27r::BKP27R_SPEC>,
    #[doc = "0xc0 - RTC backup registers"]
    pub bkp28r: crate::Reg<bkp28r::BKP28R_SPEC>,
    #[doc = "0xc4 - RTC backup registers"]
    pub bkp29r: crate::Reg<bkp29r::BKP29R_SPEC>,
    #[doc = "0xc8 - RTC backup registers"]
    pub bkp30r: crate::Reg<bkp30r::BKP30R_SPEC>,
    #[doc = "0xcc - RTC backup registers"]
    pub bkp31r: crate::Reg<bkp31r::BKP31R_SPEC>,
}
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod tr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod dr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
pub mod isr;
#[doc = "PRER register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod prer;
#[doc = "WUTR register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod wutr;
#[doc = "ALRMAR register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod alrmar;
#[doc = "ALRMBR register accessor: an alias for `Reg<ALRMBR_SPEC>`"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod alrmbr;
#[doc = "WPR register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "SHIFTR register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod shiftr;
#[doc = "TSTR register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tstr;
#[doc = "TSDR register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tsdr;
#[doc = "TSSSR register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
pub mod tsssr;
#[doc = "CALR register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod calr;
#[doc = "TAMPCR register accessor: an alias for `Reg<TAMPCR_SPEC>`"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCR_SPEC>;
#[doc = "RTC tamper and alternate function configuration register"]
pub mod tampcr;
#[doc = "ALRMASSR register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
pub mod alrmassr;
#[doc = "ALRMBSSR register accessor: an alias for `Reg<ALRMBSSR_SPEC>`"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
pub mod alrmbssr;
#[doc = "BKP0R register accessor: an alias for `Reg<BKP0R_SPEC>`"]
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp0r;
#[doc = "BKP1R register accessor: an alias for `Reg<BKP1R_SPEC>`"]
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp1r;
#[doc = "BKP2R register accessor: an alias for `Reg<BKP2R_SPEC>`"]
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp2r;
#[doc = "BKP3R register accessor: an alias for `Reg<BKP3R_SPEC>`"]
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp3r;
#[doc = "BKP4R register accessor: an alias for `Reg<BKP4R_SPEC>`"]
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp4r;
#[doc = "BKP5R register accessor: an alias for `Reg<BKP5R_SPEC>`"]
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp5r;
#[doc = "BKP6R register accessor: an alias for `Reg<BKP6R_SPEC>`"]
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp6r;
#[doc = "BKP7R register accessor: an alias for `Reg<BKP7R_SPEC>`"]
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp7r;
#[doc = "BKP8R register accessor: an alias for `Reg<BKP8R_SPEC>`"]
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp8r;
#[doc = "BKP9R register accessor: an alias for `Reg<BKP9R_SPEC>`"]
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp9r;
#[doc = "BKP10R register accessor: an alias for `Reg<BKP10R_SPEC>`"]
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp10r;
#[doc = "BKP11R register accessor: an alias for `Reg<BKP11R_SPEC>`"]
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp11r;
#[doc = "BKP12R register accessor: an alias for `Reg<BKP12R_SPEC>`"]
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp12r;
#[doc = "BKP13R register accessor: an alias for `Reg<BKP13R_SPEC>`"]
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp13r;
#[doc = "BKP14R register accessor: an alias for `Reg<BKP14R_SPEC>`"]
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp14r;
#[doc = "BKP15R register accessor: an alias for `Reg<BKP15R_SPEC>`"]
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp15r;
#[doc = "OR register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "RTC option register"]
pub mod or;
#[doc = "BKP16R register accessor: an alias for `Reg<BKP16R_SPEC>`"]
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp16r;
#[doc = "BKP17R register accessor: an alias for `Reg<BKP17R_SPEC>`"]
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp17r;
#[doc = "BKP18R register accessor: an alias for `Reg<BKP18R_SPEC>`"]
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp18r;
#[doc = "BKP19R register accessor: an alias for `Reg<BKP19R_SPEC>`"]
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp19r;
#[doc = "BKP20R register accessor: an alias for `Reg<BKP20R_SPEC>`"]
pub type BKP20R = crate::Reg<bkp20r::BKP20R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp20r;
#[doc = "BKP21R register accessor: an alias for `Reg<BKP21R_SPEC>`"]
pub type BKP21R = crate::Reg<bkp21r::BKP21R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp21r;
#[doc = "BKP22R register accessor: an alias for `Reg<BKP22R_SPEC>`"]
pub type BKP22R = crate::Reg<bkp22r::BKP22R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp22r;
#[doc = "BKP23R register accessor: an alias for `Reg<BKP23R_SPEC>`"]
pub type BKP23R = crate::Reg<bkp23r::BKP23R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp23r;
#[doc = "BKP24R register accessor: an alias for `Reg<BKP24R_SPEC>`"]
pub type BKP24R = crate::Reg<bkp24r::BKP24R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp24r;
#[doc = "BKP25R register accessor: an alias for `Reg<BKP25R_SPEC>`"]
pub type BKP25R = crate::Reg<bkp25r::BKP25R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp25r;
#[doc = "BKP26R register accessor: an alias for `Reg<BKP26R_SPEC>`"]
pub type BKP26R = crate::Reg<bkp26r::BKP26R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp26r;
#[doc = "BKP27R register accessor: an alias for `Reg<BKP27R_SPEC>`"]
pub type BKP27R = crate::Reg<bkp27r::BKP27R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp27r;
#[doc = "BKP28R register accessor: an alias for `Reg<BKP28R_SPEC>`"]
pub type BKP28R = crate::Reg<bkp28r::BKP28R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp28r;
#[doc = "BKP29R register accessor: an alias for `Reg<BKP29R_SPEC>`"]
pub type BKP29R = crate::Reg<bkp29r::BKP29R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp29r;
#[doc = "BKP30R register accessor: an alias for `Reg<BKP30R_SPEC>`"]
pub type BKP30R = crate::Reg<bkp30r::BKP30R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp30r;
#[doc = "BKP31R register accessor: an alias for `Reg<BKP31R_SPEC>`"]
pub type BKP31R = crate::Reg<bkp31r::BKP31R_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkp31r;
