#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec control register"]
    pub confr0: crate::Reg<confr0::CONFR0_SPEC>,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub confr1: crate::Reg<confr1::CONFR1_SPEC>,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub confr2: crate::Reg<confr2::CONFR2_SPEC>,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub confr3: crate::Reg<confr3::CONFR3_SPEC>,
    #[doc = "0x10 - JPEG codec configuration register 4-7"]
    pub confrn1: crate::Reg<confrn1::CONFRN1_SPEC>,
    #[doc = "0x14 - JPEG codec configuration register 4-7"]
    pub confrn2: crate::Reg<confrn2::CONFRN2_SPEC>,
    #[doc = "0x18 - JPEG codec configuration register 4-7"]
    pub confrn3: crate::Reg<confrn3::CONFRN3_SPEC>,
    #[doc = "0x1c - JPEG codec configuration register 4-7"]
    pub confrn4: crate::Reg<confrn4::CONFRN4_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x30 - JPEG control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x34 - JPEG status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x38 - JPEG clear flag register"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x40 - JPEG data input register"]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    #[doc = "0x44 - JPEG data output register"]
    pub dor: crate::Reg<dor::DOR_SPEC>,
}
#[doc = "CONFR0 register accessor: an alias for `Reg<CONFR0_SPEC>`"]
pub type CONFR0 = crate::Reg<confr0::CONFR0_SPEC>;
#[doc = "JPEG codec control register"]
pub mod confr0;
#[doc = "CONFR1 register accessor: an alias for `Reg<CONFR1_SPEC>`"]
pub type CONFR1 = crate::Reg<confr1::CONFR1_SPEC>;
#[doc = "JPEG codec configuration register 1"]
pub mod confr1;
#[doc = "CONFR2 register accessor: an alias for `Reg<CONFR2_SPEC>`"]
pub type CONFR2 = crate::Reg<confr2::CONFR2_SPEC>;
#[doc = "JPEG codec configuration register 2"]
pub mod confr2;
#[doc = "CONFR3 register accessor: an alias for `Reg<CONFR3_SPEC>`"]
pub type CONFR3 = crate::Reg<confr3::CONFR3_SPEC>;
#[doc = "JPEG codec configuration register 3"]
pub mod confr3;
#[doc = "CONFRN1 register accessor: an alias for `Reg<CONFRN1_SPEC>`"]
pub type CONFRN1 = crate::Reg<confrn1::CONFRN1_SPEC>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn1;
#[doc = "CONFRN2 register accessor: an alias for `Reg<CONFRN2_SPEC>`"]
pub type CONFRN2 = crate::Reg<confrn2::CONFRN2_SPEC>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn2;
#[doc = "CONFRN3 register accessor: an alias for `Reg<CONFRN3_SPEC>`"]
pub type CONFRN3 = crate::Reg<confrn3::CONFRN3_SPEC>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn3;
#[doc = "CONFRN4 register accessor: an alias for `Reg<CONFRN4_SPEC>`"]
pub type CONFRN4 = crate::Reg<confrn4::CONFRN4_SPEC>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn4;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "JPEG control register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "JPEG status register"]
pub mod sr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "JPEG clear flag register"]
pub mod cfr;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "JPEG data input register"]
pub mod dir;
#[doc = "DOR register accessor: an alias for `Reg<DOR_SPEC>`"]
pub type DOR = crate::Reg<dor::DOR_SPEC>;
#[doc = "JPEG data output register"]
pub mod dor;
