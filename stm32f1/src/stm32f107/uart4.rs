#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART4 SR"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x04 - UART4 DR"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - UART4 BRR"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    #[doc = "0x0c - UART4 CR1"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x10 - UART4 CR2"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x14 - UART4 CR3"]
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
}
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "UART4 SR"]
pub mod sr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART4 DR"]
pub mod dr;
#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "UART4 BRR"]
pub mod brr;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "UART4 CR1"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "UART4 CR2"]
pub mod cr2;
#[doc = "CR3 register accessor: an alias for `Reg<CR3_SPEC>`"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "UART4 CR3"]
pub mod cr3;
