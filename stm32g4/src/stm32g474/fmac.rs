#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMAC X1 Buffer Configuration register"]
    pub x1bufcfg: crate::Reg<x1bufcfg::X1BUFCFG_SPEC>,
    #[doc = "0x04 - FMAC X2 Buffer Configuration register"]
    pub x2bufcfg: crate::Reg<x2bufcfg::X2BUFCFG_SPEC>,
    #[doc = "0x08 - FMAC Y Buffer Configuration register"]
    pub ybufcfg: crate::Reg<ybufcfg::YBUFCFG_SPEC>,
    #[doc = "0x0c - FMAC Parameter register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x10 - FMAC Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x14 - FMAC Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x18 - FMAC Write Data register"]
    pub wdata: crate::Reg<wdata::WDATA_SPEC>,
    #[doc = "0x1c - FMAC Read Data register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
}
#[doc = "X1BUFCFG register accessor: an alias for `Reg<X1BUFCFG_SPEC>`"]
pub type X1BUFCFG = crate::Reg<x1bufcfg::X1BUFCFG_SPEC>;
#[doc = "FMAC X1 Buffer Configuration register"]
pub mod x1bufcfg;
#[doc = "X2BUFCFG register accessor: an alias for `Reg<X2BUFCFG_SPEC>`"]
pub type X2BUFCFG = crate::Reg<x2bufcfg::X2BUFCFG_SPEC>;
#[doc = "FMAC X2 Buffer Configuration register"]
pub mod x2bufcfg;
#[doc = "YBUFCFG register accessor: an alias for `Reg<YBUFCFG_SPEC>`"]
pub type YBUFCFG = crate::Reg<ybufcfg::YBUFCFG_SPEC>;
#[doc = "FMAC Y Buffer Configuration register"]
pub mod ybufcfg;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "FMAC Parameter register"]
pub mod param;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "FMAC Control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "FMAC Status register"]
pub mod sr;
#[doc = "WDATA register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "FMAC Write Data register"]
pub mod wdata;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "FMAC Read Data register"]
pub mod rdata;
