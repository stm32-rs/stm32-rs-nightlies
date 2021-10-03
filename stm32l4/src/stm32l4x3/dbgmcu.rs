#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb1fzr1: crate::Reg<apb1fzr1::APB1FZR1_SPEC>,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb1fzr2: crate::Reg<apb1fzr2::APB1FZR2_SPEC>,
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
#[doc = "APB1FZR1 register accessor: an alias for `Reg<APB1FZR1_SPEC>`"]
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1_SPEC>;
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1fzr1;
#[doc = "APB1FZR2 register accessor: an alias for `Reg<APB1FZR2_SPEC>`"]
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2_SPEC>;
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb1fzr2;
#[doc = "APB2FZR register accessor: an alias for `Reg<APB2FZR_SPEC>`"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2fzr;
