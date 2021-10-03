#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb_fz1: crate::Reg<apb_fz1::APB_FZ1_SPEC>,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb_fz2: crate::Reg<apb_fz2::APB_FZ2_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "APB_FZ1 register accessor: an alias for `Reg<APB_FZ1_SPEC>`"]
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1_SPEC>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb_fz1;
#[doc = "APB_FZ2 register accessor: an alias for `Reg<APB_FZ2_SPEC>`"]
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2_SPEC>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb_fz2;
