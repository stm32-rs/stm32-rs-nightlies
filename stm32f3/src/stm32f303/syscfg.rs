#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    #[doc = "0x04 - CCM SRAM protection register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    #[doc = "0x18 - configuration register 2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    _reserved7: [u8; 0x2c],
    #[doc = "0x48 - configuration register 4"]
    pub cfgr4: crate::Reg<cfgr4::CFGR4_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x50 - configuration register 3"]
    pub cfgr3: crate::Reg<cfgr3::CFGR3_SPEC>,
}
#[doc = "CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "CCM SRAM protection register"]
pub mod rcr;
#[doc = "CFGR3 register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "configuration register 3"]
pub mod cfgr3;
#[doc = "CFGR4 register accessor: an alias for `Reg<CFGR4_SPEC>`"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "configuration register 4"]
pub mod cfgr4;
