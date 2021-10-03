#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - DBG configuration register"]
    pub dbg_cr: crate::Reg<dbg_cr::DBG_CR_SPEC>,
    #[doc = "0x08 - DBG APB freeze register 1"]
    pub dbg_apb_fz1: crate::Reg<dbg_apb_fz1::DBG_APB_FZ1_SPEC>,
    #[doc = "0x0c - DBG APB freeze register 2"]
    pub dbg_apb_fz2: crate::Reg<dbg_apb_fz2::DBG_APB_FZ2_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "DBG_CR register accessor: an alias for `Reg<DBG_CR_SPEC>`"]
pub type DBG_CR = crate::Reg<dbg_cr::DBG_CR_SPEC>;
#[doc = "DBG configuration register"]
pub mod dbg_cr;
#[doc = "DBG_APB_FZ1 register accessor: an alias for `Reg<DBG_APB_FZ1_SPEC>`"]
pub type DBG_APB_FZ1 = crate::Reg<dbg_apb_fz1::DBG_APB_FZ1_SPEC>;
#[doc = "DBG APB freeze register 1"]
pub mod dbg_apb_fz1;
#[doc = "DBG_APB_FZ2 register accessor: an alias for `Reg<DBG_APB_FZ2_SPEC>`"]
pub type DBG_APB_FZ2 = crate::Reg<dbg_apb_fz2::DBG_APB_FZ2_SPEC>;
#[doc = "DBG APB freeze register 2"]
pub mod dbg_apb_fz2;
