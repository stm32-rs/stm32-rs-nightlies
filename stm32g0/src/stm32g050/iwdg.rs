#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"]
    pub iwdg_kr: crate::Reg<iwdg_kr::IWDG_KR_SPEC>,
    #[doc = "0x04 - Prescaler register"]
    pub iwdg_pr: crate::Reg<iwdg_pr::IWDG_PR_SPEC>,
    #[doc = "0x08 - Reload register"]
    pub iwdg_rlr: crate::Reg<iwdg_rlr::IWDG_RLR_SPEC>,
    #[doc = "0x0c - Status register"]
    pub iwdg_sr: crate::Reg<iwdg_sr::IWDG_SR_SPEC>,
    #[doc = "0x10 - Window register"]
    pub iwdg_winr: crate::Reg<iwdg_winr::IWDG_WINR_SPEC>,
}
#[doc = "IWDG_KR register accessor: an alias for `Reg<IWDG_KR_SPEC>`"]
pub type IWDG_KR = crate::Reg<iwdg_kr::IWDG_KR_SPEC>;
#[doc = "Key register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR register accessor: an alias for `Reg<IWDG_PR_SPEC>`"]
pub type IWDG_PR = crate::Reg<iwdg_pr::IWDG_PR_SPEC>;
#[doc = "Prescaler register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR register accessor: an alias for `Reg<IWDG_RLR_SPEC>`"]
pub type IWDG_RLR = crate::Reg<iwdg_rlr::IWDG_RLR_SPEC>;
#[doc = "Reload register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR register accessor: an alias for `Reg<IWDG_SR_SPEC>`"]
pub type IWDG_SR = crate::Reg<iwdg_sr::IWDG_SR_SPEC>;
#[doc = "Status register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR register accessor: an alias for `Reg<IWDG_WINR_SPEC>`"]
pub type IWDG_WINR = crate::Reg<iwdg_winr::IWDG_WINR_SPEC>;
#[doc = "Window register"]
pub mod iwdg_winr;
