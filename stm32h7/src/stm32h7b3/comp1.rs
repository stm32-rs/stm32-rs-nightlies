#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    pub comp1_sr: crate::Reg<comp1_sr::COMP1_SR_SPEC>,
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    pub comp1_icfr: crate::Reg<comp1_icfr::COMP1_ICFR_SPEC>,
    #[doc = "0x08 - Comparator option register"]
    pub comp1_or: crate::Reg<comp1_or::COMP1_OR_SPEC>,
    #[doc = "0x0c - Comparator configuration register 1"]
    pub comp1_cfgr1: crate::Reg<comp1_cfgr1::COMP1_CFGR1_SPEC>,
    #[doc = "0x10 - Comparator configuration register 2"]
    pub comp1_cfgr2: crate::Reg<comp1_cfgr2::COMP1_CFGR2_SPEC>,
}
#[doc = "COMP1_SR register accessor: an alias for `Reg<COMP1_SR_SPEC>`"]
pub type COMP1_SR = crate::Reg<comp1_sr::COMP1_SR_SPEC>;
#[doc = "Comparator status register"]
pub mod comp1_sr;
#[doc = "COMP1_ICFR register accessor: an alias for `Reg<COMP1_ICFR_SPEC>`"]
pub type COMP1_ICFR = crate::Reg<comp1_icfr::COMP1_ICFR_SPEC>;
#[doc = "Comparator interrupt clear flag register"]
pub mod comp1_icfr;
#[doc = "COMP1_OR register accessor: an alias for `Reg<COMP1_OR_SPEC>`"]
pub type COMP1_OR = crate::Reg<comp1_or::COMP1_OR_SPEC>;
#[doc = "Comparator option register"]
pub mod comp1_or;
#[doc = "COMP1_CFGR1 register accessor: an alias for `Reg<COMP1_CFGR1_SPEC>`"]
pub type COMP1_CFGR1 = crate::Reg<comp1_cfgr1::COMP1_CFGR1_SPEC>;
#[doc = "Comparator configuration register 1"]
pub mod comp1_cfgr1;
#[doc = "COMP1_CFGR2 register accessor: an alias for `Reg<COMP1_CFGR2_SPEC>`"]
pub type COMP1_CFGR2 = crate::Reg<comp1_cfgr2::COMP1_CFGR2_SPEC>;
#[doc = "Comparator configuration register 2"]
pub mod comp1_cfgr2;
