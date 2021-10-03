#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x08 - APB Low Freeze Register 1"]
    pub apb1_fzr1: crate::Reg<apb1_fzr1::APB1_FZR1_SPEC>,
    #[doc = "0x0c - APB Low Freeze Register 2"]
    pub apb1_fzr2: crate::Reg<apb1_fzr2::APB1_FZR2_SPEC>,
    #[doc = "0x10 - APB High Freeze Register"]
    pub apb2_fzr: crate::Reg<apb2_fzr::APB2_FZR_SPEC>,
}
#[doc = "IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1_FZR1 register accessor: an alias for `Reg<APB1_FZR1_SPEC>`"]
pub type APB1_FZR1 = crate::Reg<apb1_fzr1::APB1_FZR1_SPEC>;
#[doc = "APB Low Freeze Register 1"]
pub mod apb1_fzr1;
#[doc = "APB1_FZR2 register accessor: an alias for `Reg<APB1_FZR2_SPEC>`"]
pub type APB1_FZR2 = crate::Reg<apb1_fzr2::APB1_FZR2_SPEC>;
#[doc = "APB Low Freeze Register 2"]
pub mod apb1_fzr2;
#[doc = "APB2_FZR register accessor: an alias for `Reg<APB2_FZR_SPEC>`"]
pub type APB2_FZR = crate::Reg<apb2_fzr::APB2_FZR_SPEC>;
#[doc = "APB High Freeze Register"]
pub mod apb2_fzr;
