#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x04 - date register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - initialization and status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - prescaler register"]
    pub prer: crate::Reg<prer::PRER_SPEC>,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: crate::Reg<alrmar::ALRMAR_SPEC>,
    #[doc = "0x20 - alarm B register"]
    pub alrmbr: crate::Reg<alrmbr::ALRMBR_SPEC>,
    #[doc = "0x24 - write protection register"]
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    #[doc = "0x28 - sub second register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x2c - shift control register"]
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    #[doc = "0x3c - calibration register"]
    pub calr: crate::Reg<calr::CALR_SPEC>,
    #[doc = "0x40 - tamper configuration register"]
    pub tampcr: crate::Reg<tampcr::TAMPCR_SPEC>,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: crate::Reg<alrmassr::ALRMASSR_SPEC>,
    #[doc = "0x48 - alarm B sub second register"]
    pub alrmbssr: crate::Reg<alrmbssr::ALRMBSSR_SPEC>,
    #[doc = "0x4c - option register"]
    pub or: crate::Reg<or::OR_SPEC>,
    #[doc = "0x50 - backup register"]
    pub bkp0r: crate::Reg<bkp0r::BKP0R_SPEC>,
    #[doc = "0x54 - backup register"]
    pub bkp1r: crate::Reg<bkp1r::BKP1R_SPEC>,
    #[doc = "0x58 - backup register"]
    pub bkp2r: crate::Reg<bkp2r::BKP2R_SPEC>,
    #[doc = "0x5c - backup register"]
    pub bkp3r: crate::Reg<bkp3r::BKP3R_SPEC>,
    #[doc = "0x60 - backup register"]
    pub bkp4r: crate::Reg<bkp4r::BKP4R_SPEC>,
    #[doc = "0x64 - backup register"]
    pub bkp5r: crate::Reg<bkp5r::BKP5R_SPEC>,
    #[doc = "0x68 - backup register"]
    pub bkp6r: crate::Reg<bkp6r::BKP6R_SPEC>,
    #[doc = "0x6c - backup register"]
    pub bkp7r: crate::Reg<bkp7r::BKP7R_SPEC>,
    #[doc = "0x70 - backup register"]
    pub bkp8r: crate::Reg<bkp8r::BKP8R_SPEC>,
    #[doc = "0x74 - backup register"]
    pub bkp9r: crate::Reg<bkp9r::BKP9R_SPEC>,
    #[doc = "0x78 - backup register"]
    pub bkp10r: crate::Reg<bkp10r::BKP10R_SPEC>,
    #[doc = "0x7c - backup register"]
    pub bkp11r: crate::Reg<bkp11r::BKP11R_SPEC>,
    #[doc = "0x80 - backup register"]
    pub bkp12r: crate::Reg<bkp12r::BKP12R_SPEC>,
    #[doc = "0x84 - backup register"]
    pub bkp13r: crate::Reg<bkp13r::BKP13R_SPEC>,
    #[doc = "0x88 - backup register"]
    pub bkp14r: crate::Reg<bkp14r::BKP14R_SPEC>,
    #[doc = "0x8c - backup register"]
    pub bkp15r: crate::Reg<bkp15r::BKP15R_SPEC>,
    #[doc = "0x90 - backup register"]
    pub bkp16r: crate::Reg<bkp16r::BKP16R_SPEC>,
    #[doc = "0x94 - backup register"]
    pub bkp17r: crate::Reg<bkp17r::BKP17R_SPEC>,
    #[doc = "0x98 - backup register"]
    pub bkp18r: crate::Reg<bkp18r::BKP18R_SPEC>,
    #[doc = "0x9c - backup register"]
    pub bkp19r: crate::Reg<bkp19r::BKP19R_SPEC>,
}
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "date register"]
pub mod dr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "PRER register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "ALRMAR register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "ALRMBR register accessor: an alias for `Reg<ALRMBR_SPEC>`"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "WPR register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "sub second register"]
pub mod ssr;
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
#[doc = "CALR register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "TAMPCR register accessor: an alias for `Reg<TAMPCR_SPEC>`"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCR_SPEC>;
#[doc = "tamper configuration register"]
pub mod tampcr;
#[doc = "ALRMASSR register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "ALRMBSSR register accessor: an alias for `Reg<ALRMBSSR_SPEC>`"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "OR register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "option register"]
pub mod or;
#[doc = "BKP0R register accessor: an alias for `Reg<BKP0R_SPEC>`"]
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "BKP1R register accessor: an alias for `Reg<BKP1R_SPEC>`"]
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "BKP2R register accessor: an alias for `Reg<BKP2R_SPEC>`"]
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "BKP3R register accessor: an alias for `Reg<BKP3R_SPEC>`"]
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "BKP4R register accessor: an alias for `Reg<BKP4R_SPEC>`"]
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "BKP5R register accessor: an alias for `Reg<BKP5R_SPEC>`"]
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "BKP6R register accessor: an alias for `Reg<BKP6R_SPEC>`"]
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "BKP7R register accessor: an alias for `Reg<BKP7R_SPEC>`"]
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "BKP8R register accessor: an alias for `Reg<BKP8R_SPEC>`"]
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "BKP9R register accessor: an alias for `Reg<BKP9R_SPEC>`"]
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "BKP10R register accessor: an alias for `Reg<BKP10R_SPEC>`"]
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "BKP11R register accessor: an alias for `Reg<BKP11R_SPEC>`"]
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "BKP12R register accessor: an alias for `Reg<BKP12R_SPEC>`"]
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "BKP13R register accessor: an alias for `Reg<BKP13R_SPEC>`"]
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "BKP14R register accessor: an alias for `Reg<BKP14R_SPEC>`"]
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "BKP15R register accessor: an alias for `Reg<BKP15R_SPEC>`"]
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "BKP16R register accessor: an alias for `Reg<BKP16R_SPEC>`"]
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "BKP17R register accessor: an alias for `Reg<BKP17R_SPEC>`"]
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "BKP18R register accessor: an alias for `Reg<BKP18R_SPEC>`"]
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "BKP19R register accessor: an alias for `Reg<BKP19R_SPEC>`"]
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
#[doc = "backup register"]
pub mod bkp19r;
