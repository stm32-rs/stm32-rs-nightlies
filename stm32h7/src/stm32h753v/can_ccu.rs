#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Calibration Unit Core Release Register"]
    pub crel: crate::Reg<crel::CREL_SPEC>,
    #[doc = "0x04 - Calibration Configuration Register"]
    pub ccfg: crate::Reg<ccfg::CCFG_SPEC>,
    #[doc = "0x08 - Calibration Status Register"]
    pub cstat: crate::Reg<cstat::CSTAT_SPEC>,
    #[doc = "0x0c - Calibration Watchdog Register"]
    pub cwd: crate::Reg<cwd::CWD_SPEC>,
    #[doc = "0x10 - Clock Calibration Unit Interrupt Register"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x14 - Clock Calibration Unit Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
}
#[doc = "CREL register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Clock Calibration Unit Core Release Register"]
pub mod crel;
#[doc = "CCFG register accessor: an alias for `Reg<CCFG_SPEC>`"]
pub type CCFG = crate::Reg<ccfg::CCFG_SPEC>;
#[doc = "Calibration Configuration Register"]
pub mod ccfg;
#[doc = "CSTAT register accessor: an alias for `Reg<CSTAT_SPEC>`"]
pub type CSTAT = crate::Reg<cstat::CSTAT_SPEC>;
#[doc = "Calibration Status Register"]
pub mod cstat;
#[doc = "CWD register accessor: an alias for `Reg<CWD_SPEC>`"]
pub type CWD = crate::Reg<cwd::CWD_SPEC>;
#[doc = "Calibration Watchdog Register"]
pub mod cwd;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Clock Calibration Unit Interrupt Register"]
pub mod ir;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Clock Calibration Unit Interrupt Enable Register"]
pub mod ie;
