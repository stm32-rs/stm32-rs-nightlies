#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCFG secure configuration register"]
    pub seccfgr: crate::Reg<seccfgr::SECCFGR_SPEC>,
    #[doc = "0x04 - configuration register 1"]
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    #[doc = "0x08 - FPU interrupt mask register"]
    pub fpuimr: crate::Reg<fpuimr::FPUIMR_SPEC>,
    #[doc = "0x0c - SYSCFG CPU non-secure lock register"]
    pub cnslckr: crate::Reg<cnslckr::CNSLCKR_SPEC>,
    #[doc = "0x10 - SYSCFG CPU secure lock register"]
    pub cslockr: crate::Reg<cslockr::CSLOCKR_SPEC>,
    #[doc = "0x14 - CFGR2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x18 - SCSR"]
    pub scsr: crate::Reg<scsr::SCSR_SPEC>,
    #[doc = "0x1c - SKR"]
    pub skr: crate::Reg<skr::SKR_SPEC>,
    #[doc = "0x20 - SWPR"]
    pub swpr: crate::Reg<swpr::SWPR_SPEC>,
    #[doc = "0x24 - SWPR2"]
    pub swpr2: crate::Reg<swpr2::SWPR2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x2c - RSSCMDR"]
    pub rsscmdr: crate::Reg<rsscmdr::RSSCMDR_SPEC>,
}
#[doc = "SECCFGR register accessor: an alias for `Reg<SECCFGR_SPEC>`"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
#[doc = "SYSCFG secure configuration register"]
pub mod seccfgr;
#[doc = "CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "FPUIMR register accessor: an alias for `Reg<FPUIMR_SPEC>`"]
pub type FPUIMR = crate::Reg<fpuimr::FPUIMR_SPEC>;
#[doc = "FPU interrupt mask register"]
pub mod fpuimr;
#[doc = "CNSLCKR register accessor: an alias for `Reg<CNSLCKR_SPEC>`"]
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKR_SPEC>;
#[doc = "SYSCFG CPU non-secure lock register"]
pub mod cnslckr;
#[doc = "CSLOCKR register accessor: an alias for `Reg<CSLOCKR_SPEC>`"]
pub type CSLOCKR = crate::Reg<cslockr::CSLOCKR_SPEC>;
#[doc = "SYSCFG CPU secure lock register"]
pub mod cslockr;
#[doc = "SCSR register accessor: an alias for `Reg<SCSR_SPEC>`"]
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SWPR register accessor: an alias for `Reg<SWPR_SPEC>`"]
pub type SWPR = crate::Reg<swpr::SWPR_SPEC>;
#[doc = "SWPR"]
pub mod swpr;
#[doc = "SKR register accessor: an alias for `Reg<SKR_SPEC>`"]
pub type SKR = crate::Reg<skr::SKR_SPEC>;
#[doc = "SKR"]
pub mod skr;
#[doc = "SWPR2 register accessor: an alias for `Reg<SWPR2_SPEC>`"]
pub type SWPR2 = crate::Reg<swpr2::SWPR2_SPEC>;
#[doc = "SWPR2"]
pub mod swpr2;
#[doc = "RSSCMDR register accessor: an alias for `Reg<RSSCMDR_SPEC>`"]
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDR_SPEC>;
#[doc = "RSSCMDR"]
pub mod rsscmdr;
