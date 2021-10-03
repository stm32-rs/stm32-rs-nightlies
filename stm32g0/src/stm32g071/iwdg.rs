#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register"]
    pub kr: crate::Reg<kr::KR_SPEC>,
    #[doc = "0x04 - Prescaler register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x08 - Reload register"]
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    #[doc = "0x0c - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - Window register"]
    pub winr: crate::Reg<winr::WINR_SPEC>,
    _reserved5: [u8; 0x03dc],
    #[doc = "0x3f0 - hardware configuration register"]
    pub hwcfgr: crate::Reg<hwcfgr::HWCFGR_SPEC>,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: crate::Reg<verr::VERR_SPEC>,
    #[doc = "0x3f8 - EXTI Identification register"]
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    #[doc = "0x3fc - EXTI Size ID register"]
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
#[doc = "KR register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "Key register"]
pub mod kr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "Reload register"]
pub mod rlr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "WINR register accessor: an alias for `Reg<WINR_SPEC>`"]
pub type WINR = crate::Reg<winr::WINR_SPEC>;
#[doc = "Window register"]
pub mod winr;
#[doc = "HWCFGR register accessor: an alias for `Reg<HWCFGR_SPEC>`"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
#[doc = "hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR register accessor: an alias for `Reg<VERR_SPEC>`"]
pub type VERR = crate::Reg<verr::VERR_SPEC>;
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`"]
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "SIDR register accessor: an alias for `Reg<SIDR_SPEC>`"]
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
#[doc = "EXTI Size ID register"]
pub mod sidr;
