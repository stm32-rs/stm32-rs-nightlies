#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA mode register"]
    pub eth_dmamr: ETH_DMAMR,
    #[doc = "0x04 - System bus mode register"]
    pub eth_dmasbmr: ETH_DMASBMR,
    #[doc = "0x08 - Interrupt status register"]
    pub eth_dmaisr: ETH_DMAISR,
    #[doc = "0x0c - Debug status register"]
    pub eth_dmadsr: ETH_DMADSR,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - AXI4 transmit channel ACE control register"]
    pub eth_dmaa4tx_acr: ETH_DMAA4TXACR,
    #[doc = "0x24 - AXI4 receive channel ACE control register"]
    pub eth_dmaa4rx_acr: ETH_DMAA4RXACR,
    #[doc = "0x28 - AXI4 descriptor ACE control register"]
    pub eth_dmaa4dacr: ETH_DMAA4DACR,
    _reserved7: [u8; 212usize],
    #[doc = "0x100 - Channel 0 control register"]
    pub eth_dmac0cr: ETH_DMAC0CR,
    #[doc = "0x104 - Channel 0 transmit control register"]
    pub eth_dmac0tx_cr: ETH_DMAC0TXCR,
    #[doc = "0x108 - Channel receive control register"]
    pub eth_dmac0rx_cr: ETH_DMAC0RXCR,
    _reserved10: [u8; 8usize],
    #[doc = "0x114 - Channel i Tx descriptor list address register"]
    pub eth_dmac0tx_dlar: ETH_DMAC0TXDLAR,
    _reserved11: [u8; 4usize],
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    pub eth_dmac0rx_dlar: ETH_DMAC0RXDLAR,
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    pub eth_dmac0tx_dtpr: ETH_DMAC0TXDTPR,
    _reserved13: [u8; 4usize],
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    pub eth_dmac0rx_dtpr: ETH_DMAC0RXDTPR,
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    pub eth_dmac0tx_rlr: ETH_DMAC0TXRLR,
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    pub eth_dmac0rx_rlr: ETH_DMAC0RXRLR,
    #[doc = "0x134 - Channel interrupt enable register"]
    pub eth_dmac0ier: ETH_DMAC0IER,
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    pub eth_dmac0rx_iwtr: ETH_DMAC0RXIWTR,
    #[doc = "0x13c - Channel i slot function control status register"]
    pub eth_dmac0sfcsr: ETH_DMAC0SFCSR,
    _reserved19: [u8; 4usize],
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    pub eth_dmac0catx_dr: ETH_DMAC0CATXDR,
    _reserved20: [u8; 4usize],
    #[doc = "0x14c - Channel 0 current application receive descriptor register"]
    pub eth_dmac0carx_dr: ETH_DMAC0CARXDR,
    _reserved21: [u8; 4usize],
    #[doc = "0x154 - Channel 0 current application transmit buffer register"]
    pub eth_dmac0catx_br: ETH_DMAC0CATXBR,
    _reserved22: [u8; 4usize],
    #[doc = "0x15c - Channel current application receive buffer register"]
    pub eth_dmac0carx_br: ETH_DMAC0CARXBR,
    #[doc = "0x160 - Channel status register"]
    pub eth_dmac0sr: ETH_DMAC0SR,
    _reserved24: [u8; 8usize],
    #[doc = "0x16c - Channel missed frame count register"]
    pub eth_dmac0mfcr: ETH_DMAC0MFCR,
    _reserved25: [u8; 16usize],
    #[doc = "0x180 - Channel 1 control register"]
    pub eth_dmac1cr: ETH_DMAC1CR,
    #[doc = "0x184 - Channel 1 transmit control register"]
    pub eth_dmac1tx_cr: ETH_DMAC1TXCR,
    _reserved27: [u8; 12usize],
    #[doc = "0x194 - Channel i Tx descriptor list address register"]
    pub eth_dmac1tx_dlar: ETH_DMAC1TXDLAR,
    _reserved28: [u8; 8usize],
    #[doc = "0x1a0 - Channel Tx descriptor tail pointer register"]
    pub eth_dmac1tx_dtpr: ETH_DMAC1TXDTPR,
    _reserved29: [u8; 8usize],
    #[doc = "0x1ac - Channel Tx descriptor ring length register"]
    pub eth_dmac1tx_rlr: ETH_DMAC1TXRLR,
    _reserved30: [u8; 4usize],
    #[doc = "0x1b4 - Channel interrupt enable register"]
    pub eth_dmac1ier: ETH_DMAC1IER,
    _reserved31: [u8; 4usize],
    #[doc = "0x1bc - Channel i slot function control status register"]
    pub eth_dmac1sfcsr: ETH_DMAC1SFCSR,
    _reserved32: [u8; 4usize],
    #[doc = "0x1c4 - Channel current application transmit descriptor register"]
    pub eth_dmac1catx_dr: ETH_DMAC1CATXDR,
    _reserved33: [u8; 12usize],
    #[doc = "0x1d4 - Channel 0 current application transmit buffer register"]
    pub eth_dmac1catx_br: ETH_DMAC1CATXBR,
    _reserved34: [u8; 8usize],
    #[doc = "0x1e0 - Channel status register"]
    pub eth_dmac1sr: ETH_DMAC1SR,
    _reserved35: [u8; 8usize],
    #[doc = "0x1ec - Channel missed frame count register"]
    pub eth_dmac1mfcr: ETH_DMAC1MFCR,
}
#[doc = "DMA mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmamr](eth_dmamr) module"]
pub type ETH_DMAMR = crate::Reg<u32, _ETH_DMAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAMR;
#[doc = "`read()` method returns [eth_dmamr::R](eth_dmamr::R) reader structure"]
impl crate::Readable for ETH_DMAMR {}
#[doc = "`write(|w| ..)` method takes [eth_dmamr::W](eth_dmamr::W) writer structure"]
impl crate::Writable for ETH_DMAMR {}
#[doc = "DMA mode register"]
pub mod eth_dmamr;
#[doc = "System bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmasbmr](eth_dmasbmr) module"]
pub type ETH_DMASBMR = crate::Reg<u32, _ETH_DMASBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMASBMR;
#[doc = "`read()` method returns [eth_dmasbmr::R](eth_dmasbmr::R) reader structure"]
impl crate::Readable for ETH_DMASBMR {}
#[doc = "`write(|w| ..)` method takes [eth_dmasbmr::W](eth_dmasbmr::W) writer structure"]
impl crate::Writable for ETH_DMASBMR {}
#[doc = "System bus mode register"]
pub mod eth_dmasbmr;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaisr](eth_dmaisr) module"]
pub type ETH_DMAISR = crate::Reg<u32, _ETH_DMAISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAISR;
#[doc = "`read()` method returns [eth_dmaisr::R](eth_dmaisr::R) reader structure"]
impl crate::Readable for ETH_DMAISR {}
#[doc = "Interrupt status register"]
pub mod eth_dmaisr;
#[doc = "Debug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmadsr](eth_dmadsr) module"]
pub type ETH_DMADSR = crate::Reg<u32, _ETH_DMADSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMADSR;
#[doc = "`read()` method returns [eth_dmadsr::R](eth_dmadsr::R) reader structure"]
impl crate::Readable for ETH_DMADSR {}
#[doc = "Debug status register"]
pub mod eth_dmadsr;
#[doc = "AXI4 transmit channel ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4tx_acr](eth_dmaa4tx_acr) module"]
pub type ETH_DMAA4TXACR = crate::Reg<u32, _ETH_DMAA4TXACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAA4TXACR;
#[doc = "`read()` method returns [eth_dmaa4tx_acr::R](eth_dmaa4tx_acr::R) reader structure"]
impl crate::Readable for ETH_DMAA4TXACR {}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4tx_acr::W](eth_dmaa4tx_acr::W) writer structure"]
impl crate::Writable for ETH_DMAA4TXACR {}
#[doc = "AXI4 transmit channel ACE control register"]
pub mod eth_dmaa4tx_acr;
#[doc = "AXI4 receive channel ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4rx_acr](eth_dmaa4rx_acr) module"]
pub type ETH_DMAA4RXACR = crate::Reg<u32, _ETH_DMAA4RXACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAA4RXACR;
#[doc = "`read()` method returns [eth_dmaa4rx_acr::R](eth_dmaa4rx_acr::R) reader structure"]
impl crate::Readable for ETH_DMAA4RXACR {}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4rx_acr::W](eth_dmaa4rx_acr::W) writer structure"]
impl crate::Writable for ETH_DMAA4RXACR {}
#[doc = "AXI4 receive channel ACE control register"]
pub mod eth_dmaa4rx_acr;
#[doc = "AXI4 descriptor ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4dacr](eth_dmaa4dacr) module"]
pub type ETH_DMAA4DACR = crate::Reg<u32, _ETH_DMAA4DACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAA4DACR;
#[doc = "`read()` method returns [eth_dmaa4dacr::R](eth_dmaa4dacr::R) reader structure"]
impl crate::Readable for ETH_DMAA4DACR {}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4dacr::W](eth_dmaa4dacr::W) writer structure"]
impl crate::Writable for ETH_DMAA4DACR {}
#[doc = "AXI4 descriptor ACE control register"]
pub mod eth_dmaa4dacr;
#[doc = "Channel 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0cr](eth_dmac0cr) module"]
pub type ETH_DMAC0CR = crate::Reg<u32, _ETH_DMAC0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0CR;
#[doc = "`read()` method returns [eth_dmac0cr::R](eth_dmac0cr::R) reader structure"]
impl crate::Readable for ETH_DMAC0CR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0cr::W](eth_dmac0cr::W) writer structure"]
impl crate::Writable for ETH_DMAC0CR {}
#[doc = "Channel 0 control register"]
pub mod eth_dmac0cr;
#[doc = "Channel 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1cr](eth_dmac1cr) module"]
pub type ETH_DMAC1CR = crate::Reg<u32, _ETH_DMAC1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1CR;
#[doc = "`read()` method returns [eth_dmac1cr::R](eth_dmac1cr::R) reader structure"]
impl crate::Readable for ETH_DMAC1CR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1cr::W](eth_dmac1cr::W) writer structure"]
impl crate::Writable for ETH_DMAC1CR {}
#[doc = "Channel 1 control register"]
pub mod eth_dmac1cr;
#[doc = "Channel 0 transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_cr](eth_dmac0tx_cr) module"]
pub type ETH_DMAC0TXCR = crate::Reg<u32, _ETH_DMAC0TXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0TXCR;
#[doc = "`read()` method returns [eth_dmac0tx_cr::R](eth_dmac0tx_cr::R) reader structure"]
impl crate::Readable for ETH_DMAC0TXCR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_cr::W](eth_dmac0tx_cr::W) writer structure"]
impl crate::Writable for ETH_DMAC0TXCR {}
#[doc = "Channel 0 transmit control register"]
pub mod eth_dmac0tx_cr;
#[doc = "Channel 1 transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1tx_cr](eth_dmac1tx_cr) module"]
pub type ETH_DMAC1TXCR = crate::Reg<u32, _ETH_DMAC1TXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1TXCR;
#[doc = "`read()` method returns [eth_dmac1tx_cr::R](eth_dmac1tx_cr::R) reader structure"]
impl crate::Readable for ETH_DMAC1TXCR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1tx_cr::W](eth_dmac1tx_cr::W) writer structure"]
impl crate::Writable for ETH_DMAC1TXCR {}
#[doc = "Channel 1 transmit control register"]
pub mod eth_dmac1tx_cr;
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_cr](eth_dmac0rx_cr) module"]
pub type ETH_DMAC0RXCR = crate::Reg<u32, _ETH_DMAC0RXCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0RXCR;
#[doc = "`read()` method returns [eth_dmac0rx_cr::R](eth_dmac0rx_cr::R) reader structure"]
impl crate::Readable for ETH_DMAC0RXCR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_cr::W](eth_dmac0rx_cr::W) writer structure"]
impl crate::Writable for ETH_DMAC0RXCR {}
#[doc = "Channel receive control register"]
pub mod eth_dmac0rx_cr;
#[doc = "Channel i Tx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_dlar](eth_dmac0tx_dlar) module"]
pub type ETH_DMAC0TXDLAR = crate::Reg<u32, _ETH_DMAC0TXDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0TXDLAR;
#[doc = "`read()` method returns [eth_dmac0tx_dlar::R](eth_dmac0tx_dlar::R) reader structure"]
impl crate::Readable for ETH_DMAC0TXDLAR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_dlar::W](eth_dmac0tx_dlar::W) writer structure"]
impl crate::Writable for ETH_DMAC0TXDLAR {}
#[doc = "Channel i Tx descriptor list address register"]
pub mod eth_dmac0tx_dlar;
#[doc = "Channel i Tx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1tx_dlar](eth_dmac1tx_dlar) module"]
pub type ETH_DMAC1TXDLAR = crate::Reg<u32, _ETH_DMAC1TXDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1TXDLAR;
#[doc = "`read()` method returns [eth_dmac1tx_dlar::R](eth_dmac1tx_dlar::R) reader structure"]
impl crate::Readable for ETH_DMAC1TXDLAR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1tx_dlar::W](eth_dmac1tx_dlar::W) writer structure"]
impl crate::Writable for ETH_DMAC1TXDLAR {}
#[doc = "Channel i Tx descriptor list address register"]
pub mod eth_dmac1tx_dlar;
#[doc = "Channel Rx descriptor list address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_dlar](eth_dmac0rx_dlar) module"]
pub type ETH_DMAC0RXDLAR = crate::Reg<u32, _ETH_DMAC0RXDLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0RXDLAR;
#[doc = "`read()` method returns [eth_dmac0rx_dlar::R](eth_dmac0rx_dlar::R) reader structure"]
impl crate::Readable for ETH_DMAC0RXDLAR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_dlar::W](eth_dmac0rx_dlar::W) writer structure"]
impl crate::Writable for ETH_DMAC0RXDLAR {}
#[doc = "Channel Rx descriptor list address register"]
pub mod eth_dmac0rx_dlar;
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_dtpr](eth_dmac0tx_dtpr) module"]
pub type ETH_DMAC0TXDTPR = crate::Reg<u32, _ETH_DMAC0TXDTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0TXDTPR;
#[doc = "`read()` method returns [eth_dmac0tx_dtpr::R](eth_dmac0tx_dtpr::R) reader structure"]
impl crate::Readable for ETH_DMAC0TXDTPR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_dtpr::W](eth_dmac0tx_dtpr::W) writer structure"]
impl crate::Writable for ETH_DMAC0TXDTPR {}
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod eth_dmac0tx_dtpr;
#[doc = "Channel Tx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1tx_dtpr](eth_dmac1tx_dtpr) module"]
pub type ETH_DMAC1TXDTPR = crate::Reg<u32, _ETH_DMAC1TXDTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1TXDTPR;
#[doc = "`read()` method returns [eth_dmac1tx_dtpr::R](eth_dmac1tx_dtpr::R) reader structure"]
impl crate::Readable for ETH_DMAC1TXDTPR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1tx_dtpr::W](eth_dmac1tx_dtpr::W) writer structure"]
impl crate::Writable for ETH_DMAC1TXDTPR {}
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod eth_dmac1tx_dtpr;
#[doc = "Channel Rx descriptor tail pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_dtpr](eth_dmac0rx_dtpr) module"]
pub type ETH_DMAC0RXDTPR = crate::Reg<u32, _ETH_DMAC0RXDTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0RXDTPR;
#[doc = "`read()` method returns [eth_dmac0rx_dtpr::R](eth_dmac0rx_dtpr::R) reader structure"]
impl crate::Readable for ETH_DMAC0RXDTPR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_dtpr::W](eth_dmac0rx_dtpr::W) writer structure"]
impl crate::Writable for ETH_DMAC0RXDTPR {}
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod eth_dmac0rx_dtpr;
#[doc = "Channel Tx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_rlr](eth_dmac0tx_rlr) module"]
pub type ETH_DMAC0TXRLR = crate::Reg<u32, _ETH_DMAC0TXRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0TXRLR;
#[doc = "`read()` method returns [eth_dmac0tx_rlr::R](eth_dmac0tx_rlr::R) reader structure"]
impl crate::Readable for ETH_DMAC0TXRLR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_rlr::W](eth_dmac0tx_rlr::W) writer structure"]
impl crate::Writable for ETH_DMAC0TXRLR {}
#[doc = "Channel Tx descriptor ring length register"]
pub mod eth_dmac0tx_rlr;
#[doc = "Channel Tx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1tx_rlr](eth_dmac1tx_rlr) module"]
pub type ETH_DMAC1TXRLR = crate::Reg<u32, _ETH_DMAC1TXRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1TXRLR;
#[doc = "`read()` method returns [eth_dmac1tx_rlr::R](eth_dmac1tx_rlr::R) reader structure"]
impl crate::Readable for ETH_DMAC1TXRLR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1tx_rlr::W](eth_dmac1tx_rlr::W) writer structure"]
impl crate::Writable for ETH_DMAC1TXRLR {}
#[doc = "Channel Tx descriptor ring length register"]
pub mod eth_dmac1tx_rlr;
#[doc = "Channel Rx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_rlr](eth_dmac0rx_rlr) module"]
pub type ETH_DMAC0RXRLR = crate::Reg<u32, _ETH_DMAC0RXRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0RXRLR;
#[doc = "`read()` method returns [eth_dmac0rx_rlr::R](eth_dmac0rx_rlr::R) reader structure"]
impl crate::Readable for ETH_DMAC0RXRLR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_rlr::W](eth_dmac0rx_rlr::W) writer structure"]
impl crate::Writable for ETH_DMAC0RXRLR {}
#[doc = "Channel Rx descriptor ring length register"]
pub mod eth_dmac0rx_rlr;
#[doc = "Channel interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0ier](eth_dmac0ier) module"]
pub type ETH_DMAC0IER = crate::Reg<u32, _ETH_DMAC0IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0IER;
#[doc = "`read()` method returns [eth_dmac0ier::R](eth_dmac0ier::R) reader structure"]
impl crate::Readable for ETH_DMAC0IER {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0ier::W](eth_dmac0ier::W) writer structure"]
impl crate::Writable for ETH_DMAC0IER {}
#[doc = "Channel interrupt enable register"]
pub mod eth_dmac0ier;
#[doc = "Channel interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1ier](eth_dmac1ier) module"]
pub type ETH_DMAC1IER = crate::Reg<u32, _ETH_DMAC1IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1IER;
#[doc = "`read()` method returns [eth_dmac1ier::R](eth_dmac1ier::R) reader structure"]
impl crate::Readable for ETH_DMAC1IER {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1ier::W](eth_dmac1ier::W) writer structure"]
impl crate::Writable for ETH_DMAC1IER {}
#[doc = "Channel interrupt enable register"]
pub mod eth_dmac1ier;
#[doc = "Channel Rx interrupt watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_iwtr](eth_dmac0rx_iwtr) module"]
pub type ETH_DMAC0RXIWTR = crate::Reg<u32, _ETH_DMAC0RXIWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0RXIWTR;
#[doc = "`read()` method returns [eth_dmac0rx_iwtr::R](eth_dmac0rx_iwtr::R) reader structure"]
impl crate::Readable for ETH_DMAC0RXIWTR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_iwtr::W](eth_dmac0rx_iwtr::W) writer structure"]
impl crate::Writable for ETH_DMAC0RXIWTR {}
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod eth_dmac0rx_iwtr;
#[doc = "Channel i slot function control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0sfcsr](eth_dmac0sfcsr) module"]
pub type ETH_DMAC0SFCSR = crate::Reg<u32, _ETH_DMAC0SFCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0SFCSR;
#[doc = "`read()` method returns [eth_dmac0sfcsr::R](eth_dmac0sfcsr::R) reader structure"]
impl crate::Readable for ETH_DMAC0SFCSR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0sfcsr::W](eth_dmac0sfcsr::W) writer structure"]
impl crate::Writable for ETH_DMAC0SFCSR {}
#[doc = "Channel i slot function control status register"]
pub mod eth_dmac0sfcsr;
#[doc = "Channel i slot function control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1sfcsr](eth_dmac1sfcsr) module"]
pub type ETH_DMAC1SFCSR = crate::Reg<u32, _ETH_DMAC1SFCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1SFCSR;
#[doc = "`read()` method returns [eth_dmac1sfcsr::R](eth_dmac1sfcsr::R) reader structure"]
impl crate::Readable for ETH_DMAC1SFCSR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1sfcsr::W](eth_dmac1sfcsr::W) writer structure"]
impl crate::Writable for ETH_DMAC1SFCSR {}
#[doc = "Channel i slot function control status register"]
pub mod eth_dmac1sfcsr;
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0catx_dr](eth_dmac0catx_dr) module"]
pub type ETH_DMAC0CATXDR = crate::Reg<u32, _ETH_DMAC0CATXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0CATXDR;
#[doc = "`read()` method returns [eth_dmac0catx_dr::R](eth_dmac0catx_dr::R) reader structure"]
impl crate::Readable for ETH_DMAC0CATXDR {}
#[doc = "Channel current application transmit descriptor register"]
pub mod eth_dmac0catx_dr;
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1catx_dr](eth_dmac1catx_dr) module"]
pub type ETH_DMAC1CATXDR = crate::Reg<u32, _ETH_DMAC1CATXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1CATXDR;
#[doc = "`read()` method returns [eth_dmac1catx_dr::R](eth_dmac1catx_dr::R) reader structure"]
impl crate::Readable for ETH_DMAC1CATXDR {}
#[doc = "Channel current application transmit descriptor register"]
pub mod eth_dmac1catx_dr;
#[doc = "Channel 0 current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0carx_dr](eth_dmac0carx_dr) module"]
pub type ETH_DMAC0CARXDR = crate::Reg<u32, _ETH_DMAC0CARXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0CARXDR;
#[doc = "`read()` method returns [eth_dmac0carx_dr::R](eth_dmac0carx_dr::R) reader structure"]
impl crate::Readable for ETH_DMAC0CARXDR {}
#[doc = "Channel 0 current application receive descriptor register"]
pub mod eth_dmac0carx_dr;
#[doc = "Channel 0 current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0catx_br](eth_dmac0catx_br) module"]
pub type ETH_DMAC0CATXBR = crate::Reg<u32, _ETH_DMAC0CATXBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0CATXBR;
#[doc = "`read()` method returns [eth_dmac0catx_br::R](eth_dmac0catx_br::R) reader structure"]
impl crate::Readable for ETH_DMAC0CATXBR {}
#[doc = "Channel 0 current application transmit buffer register"]
pub mod eth_dmac0catx_br;
#[doc = "Channel 0 current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1catx_br](eth_dmac1catx_br) module"]
pub type ETH_DMAC1CATXBR = crate::Reg<u32, _ETH_DMAC1CATXBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1CATXBR;
#[doc = "`read()` method returns [eth_dmac1catx_br::R](eth_dmac1catx_br::R) reader structure"]
impl crate::Readable for ETH_DMAC1CATXBR {}
#[doc = "Channel 0 current application transmit buffer register"]
pub mod eth_dmac1catx_br;
#[doc = "Channel current application receive buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0carx_br](eth_dmac0carx_br) module"]
pub type ETH_DMAC0CARXBR = crate::Reg<u32, _ETH_DMAC0CARXBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0CARXBR;
#[doc = "`read()` method returns [eth_dmac0carx_br::R](eth_dmac0carx_br::R) reader structure"]
impl crate::Readable for ETH_DMAC0CARXBR {}
#[doc = "Channel current application receive buffer register"]
pub mod eth_dmac0carx_br;
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0sr](eth_dmac0sr) module"]
pub type ETH_DMAC0SR = crate::Reg<u32, _ETH_DMAC0SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0SR;
#[doc = "`read()` method returns [eth_dmac0sr::R](eth_dmac0sr::R) reader structure"]
impl crate::Readable for ETH_DMAC0SR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac0sr::W](eth_dmac0sr::W) writer structure"]
impl crate::Writable for ETH_DMAC0SR {}
#[doc = "Channel status register"]
pub mod eth_dmac0sr;
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1sr](eth_dmac1sr) module"]
pub type ETH_DMAC1SR = crate::Reg<u32, _ETH_DMAC1SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1SR;
#[doc = "`read()` method returns [eth_dmac1sr::R](eth_dmac1sr::R) reader structure"]
impl crate::Readable for ETH_DMAC1SR {}
#[doc = "`write(|w| ..)` method takes [eth_dmac1sr::W](eth_dmac1sr::W) writer structure"]
impl crate::Writable for ETH_DMAC1SR {}
#[doc = "Channel status register"]
pub mod eth_dmac1sr;
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0mfcr](eth_dmac0mfcr) module"]
pub type ETH_DMAC0MFCR = crate::Reg<u32, _ETH_DMAC0MFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC0MFCR;
#[doc = "`read()` method returns [eth_dmac0mfcr::R](eth_dmac0mfcr::R) reader structure"]
impl crate::Readable for ETH_DMAC0MFCR {}
#[doc = "Channel missed frame count register"]
pub mod eth_dmac0mfcr;
#[doc = "Channel missed frame count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1mfcr](eth_dmac1mfcr) module"]
pub type ETH_DMAC1MFCR = crate::Reg<u32, _ETH_DMAC1MFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_DMAC1MFCR;
#[doc = "`read()` method returns [eth_dmac1mfcr::R](eth_dmac1mfcr::R) reader structure"]
impl crate::Readable for ETH_DMAC1MFCR {}
#[doc = "Channel missed frame count register"]
pub mod eth_dmac1mfcr;
