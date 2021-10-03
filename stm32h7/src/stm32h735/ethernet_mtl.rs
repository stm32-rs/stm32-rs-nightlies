#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode Register"]
    pub mtlomr: crate::Reg<mtlomr::MTLOMR_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - Interrupt status Register"]
    pub mtlisr: crate::Reg<mtlisr::MTLISR_SPEC>,
    _reserved2: [u8; 0xdc],
    #[doc = "0x100 - Tx queue operating mode Register"]
    pub mtltx_qomr: crate::Reg<mtltx_qomr::MTLTXQOMR_SPEC>,
    #[doc = "0x104 - Tx queue underflow register"]
    pub mtltx_qur: crate::Reg<mtltx_qur::MTLTXQUR_SPEC>,
    #[doc = "0x108 - Tx queue debug Register"]
    pub mtltx_qdr: crate::Reg<mtltx_qdr::MTLTXQDR_SPEC>,
    _reserved5: [u8; 0x20],
    #[doc = "0x12c - Queue interrupt control status Register"]
    pub mtlqicsr: crate::Reg<mtlqicsr::MTLQICSR_SPEC>,
    #[doc = "0x130 - Rx queue operating mode register"]
    pub mtlrx_qomr: crate::Reg<mtlrx_qomr::MTLRXQOMR_SPEC>,
    #[doc = "0x134 - Rx queue missed packet and overflow counter register"]
    pub mtlrx_qmpocr: crate::Reg<mtlrx_qmpocr::MTLRXQMPOCR_SPEC>,
    #[doc = "0x138 - Rx queue debug register"]
    pub mtlrx_qdr: crate::Reg<mtlrx_qdr::MTLRXQDR_SPEC>,
}
#[doc = "MTLOMR register accessor: an alias for `Reg<MTLOMR_SPEC>`"]
pub type MTLOMR = crate::Reg<mtlomr::MTLOMR_SPEC>;
#[doc = "Operating mode Register"]
pub mod mtlomr;
#[doc = "MTLISR register accessor: an alias for `Reg<MTLISR_SPEC>`"]
pub type MTLISR = crate::Reg<mtlisr::MTLISR_SPEC>;
#[doc = "Interrupt status Register"]
pub mod mtlisr;
#[doc = "MTLTxQOMR register accessor: an alias for `Reg<MTLTXQOMR_SPEC>`"]
pub type MTLTXQOMR = crate::Reg<mtltx_qomr::MTLTXQOMR_SPEC>;
#[doc = "Tx queue operating mode Register"]
pub mod mtltx_qomr;
#[doc = "MTLTxQUR register accessor: an alias for `Reg<MTLTXQUR_SPEC>`"]
pub type MTLTXQUR = crate::Reg<mtltx_qur::MTLTXQUR_SPEC>;
#[doc = "Tx queue underflow register"]
pub mod mtltx_qur;
#[doc = "MTLTxQDR register accessor: an alias for `Reg<MTLTXQDR_SPEC>`"]
pub type MTLTXQDR = crate::Reg<mtltx_qdr::MTLTXQDR_SPEC>;
#[doc = "Tx queue debug Register"]
pub mod mtltx_qdr;
#[doc = "MTLQICSR register accessor: an alias for `Reg<MTLQICSR_SPEC>`"]
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSR_SPEC>;
#[doc = "Queue interrupt control status Register"]
pub mod mtlqicsr;
#[doc = "MTLRxQOMR register accessor: an alias for `Reg<MTLRXQOMR_SPEC>`"]
pub type MTLRXQOMR = crate::Reg<mtlrx_qomr::MTLRXQOMR_SPEC>;
#[doc = "Rx queue operating mode register"]
pub mod mtlrx_qomr;
#[doc = "MTLRxQMPOCR register accessor: an alias for `Reg<MTLRXQMPOCR_SPEC>`"]
pub type MTLRXQMPOCR = crate::Reg<mtlrx_qmpocr::MTLRXQMPOCR_SPEC>;
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod mtlrx_qmpocr;
#[doc = "MTLRxQDR register accessor: an alias for `Reg<MTLRXQDR_SPEC>`"]
pub type MTLRXQDR = crate::Reg<mtlrx_qdr::MTLRXQDR_SPEC>;
#[doc = "Rx queue debug register"]
pub mod mtlrx_qdr;
