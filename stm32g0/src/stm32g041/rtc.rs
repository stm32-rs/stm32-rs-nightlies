#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x04 - date register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - sub second register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x0c - initialization and status register"]
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    #[doc = "0x10 - prescaler register"]
    pub prer: crate::Reg<prer::PRER_SPEC>,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    #[doc = "0x18 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x24 - write protection register"]
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    #[doc = "0x28 - calibration register"]
    pub calr: crate::Reg<calr::CALR_SPEC>,
    #[doc = "0x2c - shift control register"]
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - alarm A register"]
    pub alrmar: crate::Reg<alrmar::ALRMAR_SPEC>,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: crate::Reg<alrmassr::ALRMASSR_SPEC>,
    #[doc = "0x48 - alarm B register"]
    pub alrmbr: crate::Reg<alrmbr::ALRMBR_SPEC>,
    #[doc = "0x4c - alarm B sub second register"]
    pub alrmbssr: crate::Reg<alrmbssr::ALRMBSSR_SPEC>,
    #[doc = "0x50 - status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x54 - masked interrupt status register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x5c - status clear register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
}
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "date register"]
pub mod dr;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "ICSR register accessor: an alias for `Reg<ICSR_SPEC>`"]
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
#[doc = "initialization and status register"]
pub mod icsr;
#[doc = "PRER register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "WPR register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "CALR register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "SHIFTR register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "TSTR register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "TSDR register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "TSSSR register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "ALRMAR register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "ALRMASSR register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "ALRMBR register accessor: an alias for `Reg<ALRMBR_SPEC>`"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "ALRMBSSR register accessor: an alias for `Reg<ALRMBSSR_SPEC>`"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "status clear register"]
pub mod scr;
