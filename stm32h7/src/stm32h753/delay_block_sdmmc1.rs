#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DLYB control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - DLYB configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DLYB control register"]
pub mod cr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "DLYB configuration register"]
pub mod cfgr;
