#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA mode register"]
    pub eth_dmamr: crate::Reg<eth_dmamr::ETH_DMAMR_SPEC>,
    #[doc = "0x04 - System bus mode register"]
    pub eth_dmasbmr: crate::Reg<eth_dmasbmr::ETH_DMASBMR_SPEC>,
    #[doc = "0x08 - Interrupt status register"]
    pub eth_dmaisr: crate::Reg<eth_dmaisr::ETH_DMAISR_SPEC>,
    #[doc = "0x0c - Debug status register"]
    pub eth_dmadsr: crate::Reg<eth_dmadsr::ETH_DMADSR_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - AXI4 transmit channel ACE control register"]
    pub eth_dmaa4tx_acr: crate::Reg<eth_dmaa4tx_acr::ETH_DMAA4TXACR_SPEC>,
    #[doc = "0x24 - AXI4 receive channel ACE control register"]
    pub eth_dmaa4rx_acr: crate::Reg<eth_dmaa4rx_acr::ETH_DMAA4RXACR_SPEC>,
    #[doc = "0x28 - AXI4 descriptor ACE control register"]
    pub eth_dmaa4dacr: crate::Reg<eth_dmaa4dacr::ETH_DMAA4DACR_SPEC>,
    _reserved7: [u8; 0xd4],
    #[doc = "0x100 - Channel 0 control register"]
    pub eth_dmac0cr: crate::Reg<eth_dmac0cr::ETH_DMAC0CR_SPEC>,
    #[doc = "0x104 - Channel 0 transmit control register"]
    pub eth_dmac0tx_cr: crate::Reg<eth_dmac0tx_cr::ETH_DMAC0TXCR_SPEC>,
    #[doc = "0x108 - Channel receive control register"]
    pub eth_dmac0rx_cr: crate::Reg<eth_dmac0rx_cr::ETH_DMAC0RXCR_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x114 - Channel i Tx descriptor list address register"]
    pub eth_dmac0tx_dlar: crate::Reg<eth_dmac0tx_dlar::ETH_DMAC0TXDLAR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    pub eth_dmac0rx_dlar: crate::Reg<eth_dmac0rx_dlar::ETH_DMAC0RXDLAR_SPEC>,
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    pub eth_dmac0tx_dtpr: crate::Reg<eth_dmac0tx_dtpr::ETH_DMAC0TXDTPR_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    pub eth_dmac0rx_dtpr: crate::Reg<eth_dmac0rx_dtpr::ETH_DMAC0RXDTPR_SPEC>,
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    pub eth_dmac0tx_rlr: crate::Reg<eth_dmac0tx_rlr::ETH_DMAC0TXRLR_SPEC>,
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    pub eth_dmac0rx_rlr: crate::Reg<eth_dmac0rx_rlr::ETH_DMAC0RXRLR_SPEC>,
    #[doc = "0x134 - Channel interrupt enable register"]
    pub eth_dmac0ier: crate::Reg<eth_dmac0ier::ETH_DMAC0IER_SPEC>,
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    pub eth_dmac0rx_iwtr: crate::Reg<eth_dmac0rx_iwtr::ETH_DMAC0RXIWTR_SPEC>,
    #[doc = "0x13c - Channel i slot function control status register"]
    pub eth_dmac0sfcsr: crate::Reg<eth_dmac0sfcsr::ETH_DMAC0SFCSR_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    pub eth_dmac0catx_dr: crate::Reg<eth_dmac0catx_dr::ETH_DMAC0CATXDR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x14c - Channel 0 current application receive descriptor register"]
    pub eth_dmac0carx_dr: crate::Reg<eth_dmac0carx_dr::ETH_DMAC0CARXDR_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x154 - Channel 0 current application transmit buffer register"]
    pub eth_dmac0catx_br: crate::Reg<eth_dmac0catx_br::ETH_DMAC0CATXBR_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x15c - Channel current application receive buffer register"]
    pub eth_dmac0carx_br: crate::Reg<eth_dmac0carx_br::ETH_DMAC0CARXBR_SPEC>,
    #[doc = "0x160 - Channel status register"]
    pub eth_dmac0sr: crate::Reg<eth_dmac0sr::ETH_DMAC0SR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x16c - Channel missed frame count register"]
    pub eth_dmac0mfcr: crate::Reg<eth_dmac0mfcr::ETH_DMAC0MFCR_SPEC>,
    _reserved25: [u8; 0x10],
    #[doc = "0x180 - Channel 1 control register"]
    pub eth_dmac1cr: crate::Reg<eth_dmac1cr::ETH_DMAC1CR_SPEC>,
    #[doc = "0x184 - Channel 1 transmit control register"]
    pub eth_dmac1tx_cr: crate::Reg<eth_dmac1tx_cr::ETH_DMAC1TXCR_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x194 - Channel i Tx descriptor list address register"]
    pub eth_dmac1tx_dlar: crate::Reg<eth_dmac1tx_dlar::ETH_DMAC1TXDLAR_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x1a0 - Channel Tx descriptor tail pointer register"]
    pub eth_dmac1tx_dtpr: crate::Reg<eth_dmac1tx_dtpr::ETH_DMAC1TXDTPR_SPEC>,
    _reserved29: [u8; 0x08],
    #[doc = "0x1ac - Channel Tx descriptor ring length register"]
    pub eth_dmac1tx_rlr: crate::Reg<eth_dmac1tx_rlr::ETH_DMAC1TXRLR_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x1b4 - Channel interrupt enable register"]
    pub eth_dmac1ier: crate::Reg<eth_dmac1ier::ETH_DMAC1IER_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x1bc - Channel i slot function control status register"]
    pub eth_dmac1sfcsr: crate::Reg<eth_dmac1sfcsr::ETH_DMAC1SFCSR_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x1c4 - Channel current application transmit descriptor register"]
    pub eth_dmac1catx_dr: crate::Reg<eth_dmac1catx_dr::ETH_DMAC1CATXDR_SPEC>,
    _reserved33: [u8; 0x0c],
    #[doc = "0x1d4 - Channel 0 current application transmit buffer register"]
    pub eth_dmac1catx_br: crate::Reg<eth_dmac1catx_br::ETH_DMAC1CATXBR_SPEC>,
    _reserved34: [u8; 0x08],
    #[doc = "0x1e0 - Channel status register"]
    pub eth_dmac1sr: crate::Reg<eth_dmac1sr::ETH_DMAC1SR_SPEC>,
    _reserved35: [u8; 0x08],
    #[doc = "0x1ec - Channel missed frame count register"]
    pub eth_dmac1mfcr: crate::Reg<eth_dmac1mfcr::ETH_DMAC1MFCR_SPEC>,
}
#[doc = "ETH_DMAMR register accessor: an alias for `Reg<ETH_DMAMR_SPEC>`"]
pub type ETH_DMAMR = crate::Reg<eth_dmamr::ETH_DMAMR_SPEC>;
#[doc = "DMA mode register"]
pub mod eth_dmamr;
#[doc = "ETH_DMASBMR register accessor: an alias for `Reg<ETH_DMASBMR_SPEC>`"]
pub type ETH_DMASBMR = crate::Reg<eth_dmasbmr::ETH_DMASBMR_SPEC>;
#[doc = "System bus mode register"]
pub mod eth_dmasbmr;
#[doc = "ETH_DMAISR register accessor: an alias for `Reg<ETH_DMAISR_SPEC>`"]
pub type ETH_DMAISR = crate::Reg<eth_dmaisr::ETH_DMAISR_SPEC>;
#[doc = "Interrupt status register"]
pub mod eth_dmaisr;
#[doc = "ETH_DMADSR register accessor: an alias for `Reg<ETH_DMADSR_SPEC>`"]
pub type ETH_DMADSR = crate::Reg<eth_dmadsr::ETH_DMADSR_SPEC>;
#[doc = "Debug status register"]
pub mod eth_dmadsr;
#[doc = "ETH_DMAA4TxACR register accessor: an alias for `Reg<ETH_DMAA4TXACR_SPEC>`"]
pub type ETH_DMAA4TXACR = crate::Reg<eth_dmaa4tx_acr::ETH_DMAA4TXACR_SPEC>;
#[doc = "AXI4 transmit channel ACE control register"]
pub mod eth_dmaa4tx_acr;
#[doc = "ETH_DMAA4RxACR register accessor: an alias for `Reg<ETH_DMAA4RXACR_SPEC>`"]
pub type ETH_DMAA4RXACR = crate::Reg<eth_dmaa4rx_acr::ETH_DMAA4RXACR_SPEC>;
#[doc = "AXI4 receive channel ACE control register"]
pub mod eth_dmaa4rx_acr;
#[doc = "ETH_DMAA4DACR register accessor: an alias for `Reg<ETH_DMAA4DACR_SPEC>`"]
pub type ETH_DMAA4DACR = crate::Reg<eth_dmaa4dacr::ETH_DMAA4DACR_SPEC>;
#[doc = "AXI4 descriptor ACE control register"]
pub mod eth_dmaa4dacr;
#[doc = "ETH_DMAC0CR register accessor: an alias for `Reg<ETH_DMAC0CR_SPEC>`"]
pub type ETH_DMAC0CR = crate::Reg<eth_dmac0cr::ETH_DMAC0CR_SPEC>;
#[doc = "Channel 0 control register"]
pub mod eth_dmac0cr;
#[doc = "ETH_DMAC1CR register accessor: an alias for `Reg<ETH_DMAC1CR_SPEC>`"]
pub type ETH_DMAC1CR = crate::Reg<eth_dmac1cr::ETH_DMAC1CR_SPEC>;
#[doc = "Channel 1 control register"]
pub mod eth_dmac1cr;
#[doc = "ETH_DMAC0TxCR register accessor: an alias for `Reg<ETH_DMAC0TXCR_SPEC>`"]
pub type ETH_DMAC0TXCR = crate::Reg<eth_dmac0tx_cr::ETH_DMAC0TXCR_SPEC>;
#[doc = "Channel 0 transmit control register"]
pub mod eth_dmac0tx_cr;
#[doc = "ETH_DMAC1TxCR register accessor: an alias for `Reg<ETH_DMAC1TXCR_SPEC>`"]
pub type ETH_DMAC1TXCR = crate::Reg<eth_dmac1tx_cr::ETH_DMAC1TXCR_SPEC>;
#[doc = "Channel 1 transmit control register"]
pub mod eth_dmac1tx_cr;
#[doc = "ETH_DMAC0RxCR register accessor: an alias for `Reg<ETH_DMAC0RXCR_SPEC>`"]
pub type ETH_DMAC0RXCR = crate::Reg<eth_dmac0rx_cr::ETH_DMAC0RXCR_SPEC>;
#[doc = "Channel receive control register"]
pub mod eth_dmac0rx_cr;
#[doc = "ETH_DMAC0TxDLAR register accessor: an alias for `Reg<ETH_DMAC0TXDLAR_SPEC>`"]
pub type ETH_DMAC0TXDLAR = crate::Reg<eth_dmac0tx_dlar::ETH_DMAC0TXDLAR_SPEC>;
#[doc = "Channel i Tx descriptor list address register"]
pub mod eth_dmac0tx_dlar;
#[doc = "ETH_DMAC1TxDLAR register accessor: an alias for `Reg<ETH_DMAC1TXDLAR_SPEC>`"]
pub type ETH_DMAC1TXDLAR = crate::Reg<eth_dmac1tx_dlar::ETH_DMAC1TXDLAR_SPEC>;
#[doc = "Channel i Tx descriptor list address register"]
pub mod eth_dmac1tx_dlar;
#[doc = "ETH_DMAC0RxDLAR register accessor: an alias for `Reg<ETH_DMAC0RXDLAR_SPEC>`"]
pub type ETH_DMAC0RXDLAR = crate::Reg<eth_dmac0rx_dlar::ETH_DMAC0RXDLAR_SPEC>;
#[doc = "Channel Rx descriptor list address register"]
pub mod eth_dmac0rx_dlar;
#[doc = "ETH_DMAC0TxDTPR register accessor: an alias for `Reg<ETH_DMAC0TXDTPR_SPEC>`"]
pub type ETH_DMAC0TXDTPR = crate::Reg<eth_dmac0tx_dtpr::ETH_DMAC0TXDTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod eth_dmac0tx_dtpr;
#[doc = "ETH_DMAC1TxDTPR register accessor: an alias for `Reg<ETH_DMAC1TXDTPR_SPEC>`"]
pub type ETH_DMAC1TXDTPR = crate::Reg<eth_dmac1tx_dtpr::ETH_DMAC1TXDTPR_SPEC>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod eth_dmac1tx_dtpr;
#[doc = "ETH_DMAC0RxDTPR register accessor: an alias for `Reg<ETH_DMAC0RXDTPR_SPEC>`"]
pub type ETH_DMAC0RXDTPR = crate::Reg<eth_dmac0rx_dtpr::ETH_DMAC0RXDTPR_SPEC>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod eth_dmac0rx_dtpr;
#[doc = "ETH_DMAC0TxRLR register accessor: an alias for `Reg<ETH_DMAC0TXRLR_SPEC>`"]
pub type ETH_DMAC0TXRLR = crate::Reg<eth_dmac0tx_rlr::ETH_DMAC0TXRLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod eth_dmac0tx_rlr;
#[doc = "ETH_DMAC1TxRLR register accessor: an alias for `Reg<ETH_DMAC1TXRLR_SPEC>`"]
pub type ETH_DMAC1TXRLR = crate::Reg<eth_dmac1tx_rlr::ETH_DMAC1TXRLR_SPEC>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod eth_dmac1tx_rlr;
#[doc = "ETH_DMAC0RxRLR register accessor: an alias for `Reg<ETH_DMAC0RXRLR_SPEC>`"]
pub type ETH_DMAC0RXRLR = crate::Reg<eth_dmac0rx_rlr::ETH_DMAC0RXRLR_SPEC>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod eth_dmac0rx_rlr;
#[doc = "ETH_DMAC0IER register accessor: an alias for `Reg<ETH_DMAC0IER_SPEC>`"]
pub type ETH_DMAC0IER = crate::Reg<eth_dmac0ier::ETH_DMAC0IER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod eth_dmac0ier;
#[doc = "ETH_DMAC1IER register accessor: an alias for `Reg<ETH_DMAC1IER_SPEC>`"]
pub type ETH_DMAC1IER = crate::Reg<eth_dmac1ier::ETH_DMAC1IER_SPEC>;
#[doc = "Channel interrupt enable register"]
pub mod eth_dmac1ier;
#[doc = "ETH_DMAC0RxIWTR register accessor: an alias for `Reg<ETH_DMAC0RXIWTR_SPEC>`"]
pub type ETH_DMAC0RXIWTR = crate::Reg<eth_dmac0rx_iwtr::ETH_DMAC0RXIWTR_SPEC>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod eth_dmac0rx_iwtr;
#[doc = "ETH_DMAC0SFCSR register accessor: an alias for `Reg<ETH_DMAC0SFCSR_SPEC>`"]
pub type ETH_DMAC0SFCSR = crate::Reg<eth_dmac0sfcsr::ETH_DMAC0SFCSR_SPEC>;
#[doc = "Channel i slot function control status register"]
pub mod eth_dmac0sfcsr;
#[doc = "ETH_DMAC1SFCSR register accessor: an alias for `Reg<ETH_DMAC1SFCSR_SPEC>`"]
pub type ETH_DMAC1SFCSR = crate::Reg<eth_dmac1sfcsr::ETH_DMAC1SFCSR_SPEC>;
#[doc = "Channel i slot function control status register"]
pub mod eth_dmac1sfcsr;
#[doc = "ETH_DMAC0CATxDR register accessor: an alias for `Reg<ETH_DMAC0CATXDR_SPEC>`"]
pub type ETH_DMAC0CATXDR = crate::Reg<eth_dmac0catx_dr::ETH_DMAC0CATXDR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod eth_dmac0catx_dr;
#[doc = "ETH_DMAC1CATxDR register accessor: an alias for `Reg<ETH_DMAC1CATXDR_SPEC>`"]
pub type ETH_DMAC1CATXDR = crate::Reg<eth_dmac1catx_dr::ETH_DMAC1CATXDR_SPEC>;
#[doc = "Channel current application transmit descriptor register"]
pub mod eth_dmac1catx_dr;
#[doc = "ETH_DMAC0CARxDR register accessor: an alias for `Reg<ETH_DMAC0CARXDR_SPEC>`"]
pub type ETH_DMAC0CARXDR = crate::Reg<eth_dmac0carx_dr::ETH_DMAC0CARXDR_SPEC>;
#[doc = "Channel 0 current application receive descriptor register"]
pub mod eth_dmac0carx_dr;
#[doc = "ETH_DMAC0CATxBR register accessor: an alias for `Reg<ETH_DMAC0CATXBR_SPEC>`"]
pub type ETH_DMAC0CATXBR = crate::Reg<eth_dmac0catx_br::ETH_DMAC0CATXBR_SPEC>;
#[doc = "Channel 0 current application transmit buffer register"]
pub mod eth_dmac0catx_br;
#[doc = "ETH_DMAC1CATxBR register accessor: an alias for `Reg<ETH_DMAC1CATXBR_SPEC>`"]
pub type ETH_DMAC1CATXBR = crate::Reg<eth_dmac1catx_br::ETH_DMAC1CATXBR_SPEC>;
#[doc = "Channel 0 current application transmit buffer register"]
pub mod eth_dmac1catx_br;
#[doc = "ETH_DMAC0CARxBR register accessor: an alias for `Reg<ETH_DMAC0CARXBR_SPEC>`"]
pub type ETH_DMAC0CARXBR = crate::Reg<eth_dmac0carx_br::ETH_DMAC0CARXBR_SPEC>;
#[doc = "Channel current application receive buffer register"]
pub mod eth_dmac0carx_br;
#[doc = "ETH_DMAC0SR register accessor: an alias for `Reg<ETH_DMAC0SR_SPEC>`"]
pub type ETH_DMAC0SR = crate::Reg<eth_dmac0sr::ETH_DMAC0SR_SPEC>;
#[doc = "Channel status register"]
pub mod eth_dmac0sr;
#[doc = "ETH_DMAC1SR register accessor: an alias for `Reg<ETH_DMAC1SR_SPEC>`"]
pub type ETH_DMAC1SR = crate::Reg<eth_dmac1sr::ETH_DMAC1SR_SPEC>;
#[doc = "Channel status register"]
pub mod eth_dmac1sr;
#[doc = "ETH_DMAC0MFCR register accessor: an alias for `Reg<ETH_DMAC0MFCR_SPEC>`"]
pub type ETH_DMAC0MFCR = crate::Reg<eth_dmac0mfcr::ETH_DMAC0MFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod eth_dmac0mfcr;
#[doc = "ETH_DMAC1MFCR register accessor: an alias for `Reg<ETH_DMAC1MFCR_SPEC>`"]
pub type ETH_DMAC1MFCR = crate::Reg<eth_dmac1mfcr::ETH_DMAC1MFCR_SPEC>;
#[doc = "Channel missed frame count register"]
pub mod eth_dmac1mfcr;
