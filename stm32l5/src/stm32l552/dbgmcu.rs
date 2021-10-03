#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb1lfzr: crate::Reg<apb1lfzr::APB1LFZR_SPEC>,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb1hfzr: crate::Reg<apb1hfzr::APB1HFZR_SPEC>,
    #[doc = "0x10 - Debug MCU APB2 freeze register"]
    pub apb2fzr: crate::Reg<apb2fzr::APB2FZR_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "APB1LFZR register accessor: an alias for `Reg<APB1LFZR_SPEC>`"]
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZR_SPEC>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1lfzr;
#[doc = "APB1HFZR register accessor: an alias for `Reg<APB1HFZR_SPEC>`"]
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZR_SPEC>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb1hfzr;
#[doc = "APB2FZR register accessor: an alias for `Reg<APB2FZR_SPEC>`"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2fzr;
