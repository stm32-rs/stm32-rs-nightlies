#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA mode register"]
    pub dmamr: crate::Reg<dmamr::DMAMR_SPEC>,
    #[doc = "0x04 - System bus mode register"]
    pub dmasbmr: crate::Reg<dmasbmr::DMASBMR_SPEC>,
    #[doc = "0x08 - Interrupt status register"]
    pub dmaisr: crate::Reg<dmaisr::DMAISR_SPEC>,
    #[doc = "0x0c - Debug status register"]
    pub dmadsr: crate::Reg<dmadsr::DMADSR_SPEC>,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - Channel control register"]
    pub dmaccr: crate::Reg<dmaccr::DMACCR_SPEC>,
    #[doc = "0x104 - Channel transmit control register"]
    pub dmactx_cr: crate::Reg<dmactx_cr::DMACTXCR_SPEC>,
    #[doc = "0x108 - Channel receive control register"]
    pub dmacrx_cr: crate::Reg<dmacrx_cr::DMACRXCR_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x114 - Channel Tx descriptor list address register"]
    pub dmactx_dlar: crate::Reg<dmactx_dlar::DMACTXDLAR_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    pub dmacrx_dlar: crate::Reg<dmacrx_dlar::DMACRXDLAR_SPEC>,
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    pub dmactx_dtpr: crate::Reg<dmactx_dtpr::DMACTXDTPR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    pub dmacrx_dtpr: crate::Reg<dmacrx_dtpr::DMACRXDTPR_SPEC>,
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    pub dmactx_rlr: crate::Reg<dmactx_rlr::DMACTXRLR_SPEC>,
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    pub dmacrx_rlr: crate::Reg<dmacrx_rlr::DMACRXRLR_SPEC>,
    #[doc = "0x134 - Channel interrupt enable register"]
    pub dmacier: crate::Reg<dmacier::DMACIER_SPEC>,
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    pub dmacrx_iwtr: crate::Reg<dmacrx_iwtr::DMACRXIWTR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    pub dmaccatx_dr: crate::Reg<dmaccatx_dr::DMACCATXDR_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x14c - Channel current application receive descriptor register"]
    pub dmaccarx_dr: crate::Reg<dmaccarx_dr::DMACCARXDR_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x154 - Channel current application transmit buffer register"]
    pub dmaccatx_br: crate::Reg<dmaccatx_br::DMACCATXBR_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x15c - Channel current application receive buffer register"]
    pub dmaccarx_br: crate::Reg<dmaccarx_br::DMACCARXBR_SPEC>,
    #[doc = "0x160 - Channel status register"]
    pub dmacsr: crate::Reg<dmacsr::DMACSR_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0x16c - Channel missed frame count register"]
    pub dmacmfcr: crate::Reg<dmacmfcr::DMACMFCR_SPEC>,
}
#[doc = "DMAMR register accessor: an alias for `Reg<DMAMR_SPEC>`"]
pub type DMAMR = crate::Reg<dmamr::DMAMR_SPEC>;
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "DMASBMR register accessor: an alias for `Reg<DMASBMR_SPEC>`"]
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMR_SPEC>;
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "DMAISR register accessor: an alias for `Reg<DMAISR_SPEC>`"]
pub type DMAISR = crate::Reg<dmaisr::DMAISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "DMADSR register accessor: an alias for `Reg<DMADSR_SPEC>`"]
pub type DMADSR = crate::Reg<dmadsr::DMADSR_SPEC>;
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "DMACCR register accessor: an alias for `Reg<DMACCR_SPEC>`"]
pub type DMACCR = crate::Reg<dmaccr::DMACCR_SPEC>;
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "DMACTxCR register accessor: an alias for `Reg<DMACTXCR_SPEC>`"]
pub type DMACTXCR = crate::Reg<dmactx_cr::DMACTXCR_SPEC>;
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "DMACRxCR register accessor: an alias for `Reg<DMACRXCR_SPEC>`"]
pub type DMACRXCR = crate::Reg<dmacrx_cr::DMACRXCR_SPEC>;
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "DMACTxDLAR register accessor: an alias for `Reg<DMACTXDLAR_SPEC>`"]
pub type DMACTXDLAR = crate::Reg<dmactx_dlar::DMACTXDLAR_SPEC>;
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "DMACRxDLAR register accessor: an alias for `Reg<DMACRXDLAR_SPEC>`"]
pub type DMACRXDLAR = crate::Reg<dmacrx_dlar::DMACRXDLAR_SPEC>;
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "DMACTxDTPR register accessor: an alias for `Reg<DMACTXDTPR_SPEC>`"]
pub type DMACTXDTPR = crate::Reg<dmactx_dtpr::DMACTXDTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "DMACRxDTPR register accessor: an alias for `Reg<DMACRXDTPR_SPEC>`"]
pub type DMACRXDTPR = crate::Reg<dmacrx_dtpr::DMACRXDTPR_SPEC>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "DMACTxRLR register accessor: an alias for `Reg<DMACTXRLR_SPEC>`"]
pub type DMACTXRLR = crate::Reg<dmactx_rlr::DMACTXRLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "DMACRxRLR register accessor: an alias for `Reg<DMACRXRLR_SPEC>`"]
pub type DMACRXRLR = crate::Reg<dmacrx_rlr::DMACRXRLR_SPEC>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "DMACIER register accessor: an alias for `Reg<DMACIER_SPEC>`"]
pub type DMACIER = crate::Reg<dmacier::DMACIER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "DMACRxIWTR register accessor: an alias for `Reg<DMACRXIWTR_SPEC>`"]
pub type DMACRXIWTR = crate::Reg<dmacrx_iwtr::DMACRXIWTR_SPEC>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "DMACCATxDR register accessor: an alias for `Reg<DMACCATXDR_SPEC>`"]
pub type DMACCATXDR = crate::Reg<dmaccatx_dr::DMACCATXDR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "DMACCARxDR register accessor: an alias for `Reg<DMACCARXDR_SPEC>`"]
pub type DMACCARXDR = crate::Reg<dmaccarx_dr::DMACCARXDR_SPEC>;
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "DMACCATxBR register accessor: an alias for `Reg<DMACCATXBR_SPEC>`"]
pub type DMACCATXBR = crate::Reg<dmaccatx_br::DMACCATXBR_SPEC>;
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "DMACCARxBR register accessor: an alias for `Reg<DMACCARXBR_SPEC>`"]
pub type DMACCARXBR = crate::Reg<dmaccarx_br::DMACCARXBR_SPEC>;
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "DMACSR register accessor: an alias for `Reg<DMACSR_SPEC>`"]
pub type DMACSR = crate::Reg<dmacsr::DMACSR_SPEC>;
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "DMACMFCR register accessor: an alias for `Reg<DMACMFCR_SPEC>`"]
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
