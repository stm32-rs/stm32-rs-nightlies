#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    pub kr: crate::Reg<kr::KR_SPEC>,
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "KR register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
