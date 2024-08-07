#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    eth_dmamr: ETH_DMAMR,
    eth_dmasbmr: ETH_DMASBMR,
    eth_dmaisr: ETH_DMAISR,
    eth_dmadsr: ETH_DMADSR,
    _reserved4: [u8; 0x10],
    eth_dmaa4tx_acr: ETH_DMAA4TX_ACR,
    eth_dmaa4rx_acr: ETH_DMAA4RX_ACR,
    eth_dmaa4dacr: ETH_DMAA4DACR,
    _reserved7: [u8; 0xd4],
    eth_dmac0cr: ETH_DMAC0CR,
    eth_dmac0tx_cr: ETH_DMAC0TX_CR,
    eth_dmac0rx_cr: ETH_DMAC0RX_CR,
    _reserved10: [u8; 0x08],
    eth_dmac0tx_dlar: ETH_DMAC0TX_DLAR,
    _reserved11: [u8; 0x04],
    eth_dmac0rx_dlar: ETH_DMAC0RX_DLAR,
    eth_dmac0tx_dtpr: ETH_DMAC0TX_DTPR,
    _reserved13: [u8; 0x04],
    eth_dmac0rx_dtpr: ETH_DMAC0RX_DTPR,
    eth_dmac0tx_rlr: ETH_DMAC0TX_RLR,
    eth_dmac0rx_rlr: ETH_DMAC0RX_RLR,
    eth_dmac0ier: ETH_DMAC0IER,
    eth_dmac0rx_iwtr: ETH_DMAC0RX_IWTR,
    eth_dmac0sfcsr: ETH_DMAC0SFCSR,
    _reserved19: [u8; 0x04],
    eth_dmac0catx_dr: ETH_DMAC0CATX_DR,
    _reserved20: [u8; 0x04],
    eth_dmac0carx_dr: ETH_DMAC0CARX_DR,
    _reserved21: [u8; 0x04],
    eth_dmac0catx_br: ETH_DMAC0CATX_BR,
    _reserved22: [u8; 0x04],
    eth_dmac0carx_br: ETH_DMAC0CARX_BR,
    eth_dmac0sr: ETH_DMAC0SR,
    _reserved24: [u8; 0x08],
    eth_dmac0mfcr: ETH_DMAC0MFCR,
    _reserved25: [u8; 0x10],
    eth_dmac1cr: ETH_DMAC1CR,
    eth_dmac1tx_cr: ETH_DMAC1TX_CR,
    _reserved27: [u8; 0x0c],
    eth_dmac1tx_dlar: ETH_DMAC1TX_DLAR,
    _reserved28: [u8; 0x08],
    eth_dmac1tx_dtpr: ETH_DMAC1TX_DTPR,
    _reserved29: [u8; 0x08],
    eth_dmac1tx_rlr: ETH_DMAC1TX_RLR,
    _reserved30: [u8; 0x04],
    eth_dmac1ier: ETH_DMAC1IER,
    _reserved31: [u8; 0x04],
    eth_dmac1sfcsr: ETH_DMAC1SFCSR,
    _reserved32: [u8; 0x04],
    eth_dmac1catx_dr: ETH_DMAC1CATX_DR,
    _reserved33: [u8; 0x0c],
    eth_dmac1catx_br: ETH_DMAC1CATX_BR,
    _reserved34: [u8; 0x08],
    eth_dmac1sr: ETH_DMAC1SR,
    _reserved35: [u8; 0x08],
    eth_dmac1mfcr: ETH_DMAC1MFCR,
}
impl RegisterBlock {
    ///0x00 - DMA mode register
    #[inline(always)]
    pub const fn eth_dmamr(&self) -> &ETH_DMAMR {
        &self.eth_dmamr
    }
    ///0x04 - System bus mode register
    #[inline(always)]
    pub const fn eth_dmasbmr(&self) -> &ETH_DMASBMR {
        &self.eth_dmasbmr
    }
    ///0x08 - Interrupt status register
    #[inline(always)]
    pub const fn eth_dmaisr(&self) -> &ETH_DMAISR {
        &self.eth_dmaisr
    }
    ///0x0c - Debug status register
    #[inline(always)]
    pub const fn eth_dmadsr(&self) -> &ETH_DMADSR {
        &self.eth_dmadsr
    }
    ///0x20 - AXI4 transmit channel ACE control register
    #[inline(always)]
    pub const fn eth_dmaa4tx_acr(&self) -> &ETH_DMAA4TX_ACR {
        &self.eth_dmaa4tx_acr
    }
    ///0x24 - AXI4 receive channel ACE control register
    #[inline(always)]
    pub const fn eth_dmaa4rx_acr(&self) -> &ETH_DMAA4RX_ACR {
        &self.eth_dmaa4rx_acr
    }
    ///0x28 - AXI4 descriptor ACE control register
    #[inline(always)]
    pub const fn eth_dmaa4dacr(&self) -> &ETH_DMAA4DACR {
        &self.eth_dmaa4dacr
    }
    ///0x100 - Channel 0 control register
    #[inline(always)]
    pub const fn eth_dmac0cr(&self) -> &ETH_DMAC0CR {
        &self.eth_dmac0cr
    }
    ///0x104 - Channel 0 transmit control register
    #[inline(always)]
    pub const fn eth_dmac0tx_cr(&self) -> &ETH_DMAC0TX_CR {
        &self.eth_dmac0tx_cr
    }
    ///0x108 - Channel receive control register
    #[inline(always)]
    pub const fn eth_dmac0rx_cr(&self) -> &ETH_DMAC0RX_CR {
        &self.eth_dmac0rx_cr
    }
    ///0x114 - Channel i Tx descriptor list address register
    #[inline(always)]
    pub const fn eth_dmac0tx_dlar(&self) -> &ETH_DMAC0TX_DLAR {
        &self.eth_dmac0tx_dlar
    }
    ///0x11c - Channel Rx descriptor list address register
    #[inline(always)]
    pub const fn eth_dmac0rx_dlar(&self) -> &ETH_DMAC0RX_DLAR {
        &self.eth_dmac0rx_dlar
    }
    ///0x120 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn eth_dmac0tx_dtpr(&self) -> &ETH_DMAC0TX_DTPR {
        &self.eth_dmac0tx_dtpr
    }
    ///0x128 - Channel Rx descriptor tail pointer register
    #[inline(always)]
    pub const fn eth_dmac0rx_dtpr(&self) -> &ETH_DMAC0RX_DTPR {
        &self.eth_dmac0rx_dtpr
    }
    ///0x12c - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn eth_dmac0tx_rlr(&self) -> &ETH_DMAC0TX_RLR {
        &self.eth_dmac0tx_rlr
    }
    ///0x130 - Channel Rx descriptor ring length register
    #[inline(always)]
    pub const fn eth_dmac0rx_rlr(&self) -> &ETH_DMAC0RX_RLR {
        &self.eth_dmac0rx_rlr
    }
    ///0x134 - Channel interrupt enable register
    #[inline(always)]
    pub const fn eth_dmac0ier(&self) -> &ETH_DMAC0IER {
        &self.eth_dmac0ier
    }
    ///0x138 - Channel Rx interrupt watchdog timer register
    #[inline(always)]
    pub const fn eth_dmac0rx_iwtr(&self) -> &ETH_DMAC0RX_IWTR {
        &self.eth_dmac0rx_iwtr
    }
    ///0x13c - Channel i slot function control status register
    #[inline(always)]
    pub const fn eth_dmac0sfcsr(&self) -> &ETH_DMAC0SFCSR {
        &self.eth_dmac0sfcsr
    }
    ///0x144 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn eth_dmac0catx_dr(&self) -> &ETH_DMAC0CATX_DR {
        &self.eth_dmac0catx_dr
    }
    ///0x14c - Channel 0 current application receive descriptor register
    #[inline(always)]
    pub const fn eth_dmac0carx_dr(&self) -> &ETH_DMAC0CARX_DR {
        &self.eth_dmac0carx_dr
    }
    ///0x154 - Channel 0 current application transmit buffer register
    #[inline(always)]
    pub const fn eth_dmac0catx_br(&self) -> &ETH_DMAC0CATX_BR {
        &self.eth_dmac0catx_br
    }
    ///0x15c - Channel current application receive buffer register
    #[inline(always)]
    pub const fn eth_dmac0carx_br(&self) -> &ETH_DMAC0CARX_BR {
        &self.eth_dmac0carx_br
    }
    ///0x160 - Channel status register
    #[inline(always)]
    pub const fn eth_dmac0sr(&self) -> &ETH_DMAC0SR {
        &self.eth_dmac0sr
    }
    ///0x16c - Channel missed frame count register
    #[inline(always)]
    pub const fn eth_dmac0mfcr(&self) -> &ETH_DMAC0MFCR {
        &self.eth_dmac0mfcr
    }
    ///0x180 - Channel 1 control register
    #[inline(always)]
    pub const fn eth_dmac1cr(&self) -> &ETH_DMAC1CR {
        &self.eth_dmac1cr
    }
    ///0x184 - Channel 1 transmit control register
    #[inline(always)]
    pub const fn eth_dmac1tx_cr(&self) -> &ETH_DMAC1TX_CR {
        &self.eth_dmac1tx_cr
    }
    ///0x194 - Channel i Tx descriptor list address register
    #[inline(always)]
    pub const fn eth_dmac1tx_dlar(&self) -> &ETH_DMAC1TX_DLAR {
        &self.eth_dmac1tx_dlar
    }
    ///0x1a0 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn eth_dmac1tx_dtpr(&self) -> &ETH_DMAC1TX_DTPR {
        &self.eth_dmac1tx_dtpr
    }
    ///0x1ac - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn eth_dmac1tx_rlr(&self) -> &ETH_DMAC1TX_RLR {
        &self.eth_dmac1tx_rlr
    }
    ///0x1b4 - Channel interrupt enable register
    #[inline(always)]
    pub const fn eth_dmac1ier(&self) -> &ETH_DMAC1IER {
        &self.eth_dmac1ier
    }
    ///0x1bc - Channel i slot function control status register
    #[inline(always)]
    pub const fn eth_dmac1sfcsr(&self) -> &ETH_DMAC1SFCSR {
        &self.eth_dmac1sfcsr
    }
    ///0x1c4 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn eth_dmac1catx_dr(&self) -> &ETH_DMAC1CATX_DR {
        &self.eth_dmac1catx_dr
    }
    ///0x1d4 - Channel 0 current application transmit buffer register
    #[inline(always)]
    pub const fn eth_dmac1catx_br(&self) -> &ETH_DMAC1CATX_BR {
        &self.eth_dmac1catx_br
    }
    ///0x1e0 - Channel status register
    #[inline(always)]
    pub const fn eth_dmac1sr(&self) -> &ETH_DMAC1SR {
        &self.eth_dmac1sr
    }
    ///0x1ec - Channel missed frame count register
    #[inline(always)]
    pub const fn eth_dmac1mfcr(&self) -> &ETH_DMAC1MFCR {
        &self.eth_dmac1mfcr
    }
}
/**ETH_DMAMR (rw) register accessor: DMA mode register

You can [`read`](crate::Reg::read) this register and get [`eth_dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAMR)

For information about available fields see [`mod@eth_dmamr`]
module*/
pub type ETH_DMAMR = crate::Reg<eth_dmamr::ETH_DMAMRrs>;
///DMA mode register
pub mod eth_dmamr;
/**ETH_DMASBMR (rw) register accessor: System bus mode register

You can [`read`](crate::Reg::read) this register and get [`eth_dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMASBMR)

For information about available fields see [`mod@eth_dmasbmr`]
module*/
pub type ETH_DMASBMR = crate::Reg<eth_dmasbmr::ETH_DMASBMRrs>;
///System bus mode register
pub mod eth_dmasbmr;
/**ETH_DMAISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAISR)

For information about available fields see [`mod@eth_dmaisr`]
module*/
pub type ETH_DMAISR = crate::Reg<eth_dmaisr::ETH_DMAISRrs>;
///Interrupt status register
pub mod eth_dmaisr;
/**ETH_DMADSR (r) register accessor: Debug status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMADSR)

For information about available fields see [`mod@eth_dmadsr`]
module*/
pub type ETH_DMADSR = crate::Reg<eth_dmadsr::ETH_DMADSRrs>;
///Debug status register
pub mod eth_dmadsr;
/**ETH_DMAA4TxACR (rw) register accessor: AXI4 transmit channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmaa4tx_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmaa4tx_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAA4TxACR)

For information about available fields see [`mod@eth_dmaa4tx_acr`]
module*/
#[doc(alias = "ETH_DMAA4TxACR")]
pub type ETH_DMAA4TX_ACR = crate::Reg<eth_dmaa4tx_acr::ETH_DMAA4TX_ACRrs>;
///AXI4 transmit channel ACE control register
pub mod eth_dmaa4tx_acr;
/**ETH_DMAA4RxACR (rw) register accessor: AXI4 receive channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmaa4rx_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmaa4rx_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAA4RxACR)

For information about available fields see [`mod@eth_dmaa4rx_acr`]
module*/
#[doc(alias = "ETH_DMAA4RxACR")]
pub type ETH_DMAA4RX_ACR = crate::Reg<eth_dmaa4rx_acr::ETH_DMAA4RX_ACRrs>;
///AXI4 receive channel ACE control register
pub mod eth_dmaa4rx_acr;
/**ETH_DMAA4DACR (rw) register accessor: AXI4 descriptor ACE control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmaa4dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmaa4dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAA4DACR)

For information about available fields see [`mod@eth_dmaa4dacr`]
module*/
pub type ETH_DMAA4DACR = crate::Reg<eth_dmaa4dacr::ETH_DMAA4DACRrs>;
///AXI4 descriptor ACE control register
pub mod eth_dmaa4dacr;
/**ETH_DMAC0CR (rw) register accessor: Channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CR)

For information about available fields see [`mod@eth_dmac0cr`]
module*/
pub type ETH_DMAC0CR = crate::Reg<eth_dmac0cr::ETH_DMAC0CRrs>;
///Channel 0 control register
pub mod eth_dmac0cr;
/**ETH_DMAC1CR (rw) register accessor: Channel 1 control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1CR)

For information about available fields see [`mod@eth_dmac1cr`]
module*/
pub type ETH_DMAC1CR = crate::Reg<eth_dmac1cr::ETH_DMAC1CRrs>;
///Channel 1 control register
pub mod eth_dmac1cr;
/**ETH_DMAC0TxCR (rw) register accessor: Channel 0 transmit control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxCR)

For information about available fields see [`mod@eth_dmac0tx_cr`]
module*/
#[doc(alias = "ETH_DMAC0TxCR")]
pub type ETH_DMAC0TX_CR = crate::Reg<eth_dmac0tx_cr::ETH_DMAC0TX_CRrs>;
///Channel 0 transmit control register
pub mod eth_dmac0tx_cr;
/**ETH_DMAC1TxCR (rw) register accessor: Channel 1 transmit control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1tx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1tx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1TxCR)

For information about available fields see [`mod@eth_dmac1tx_cr`]
module*/
#[doc(alias = "ETH_DMAC1TxCR")]
pub type ETH_DMAC1TX_CR = crate::Reg<eth_dmac1tx_cr::ETH_DMAC1TX_CRrs>;
///Channel 1 transmit control register
pub mod eth_dmac1tx_cr;
/**ETH_DMAC0RxCR (rw) register accessor: Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxCR)

For information about available fields see [`mod@eth_dmac0rx_cr`]
module*/
#[doc(alias = "ETH_DMAC0RxCR")]
pub type ETH_DMAC0RX_CR = crate::Reg<eth_dmac0rx_cr::ETH_DMAC0RX_CRrs>;
///Channel receive control register
pub mod eth_dmac0rx_cr;
/**ETH_DMAC0TxDLAR (rw) register accessor: Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxDLAR)

For information about available fields see [`mod@eth_dmac0tx_dlar`]
module*/
#[doc(alias = "ETH_DMAC0TxDLAR")]
pub type ETH_DMAC0TX_DLAR = crate::Reg<eth_dmac0tx_dlar::ETH_DMAC0TX_DLARrs>;
///Channel i Tx descriptor list address register
pub mod eth_dmac0tx_dlar;
/**ETH_DMAC1TxDLAR (rw) register accessor: Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1tx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1tx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1TxDLAR)

For information about available fields see [`mod@eth_dmac1tx_dlar`]
module*/
#[doc(alias = "ETH_DMAC1TxDLAR")]
pub type ETH_DMAC1TX_DLAR = crate::Reg<eth_dmac1tx_dlar::ETH_DMAC1TX_DLARrs>;
///Channel i Tx descriptor list address register
pub mod eth_dmac1tx_dlar;
/**ETH_DMAC0RxDLAR (rw) register accessor: Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxDLAR)

For information about available fields see [`mod@eth_dmac0rx_dlar`]
module*/
#[doc(alias = "ETH_DMAC0RxDLAR")]
pub type ETH_DMAC0RX_DLAR = crate::Reg<eth_dmac0rx_dlar::ETH_DMAC0RX_DLARrs>;
///Channel Rx descriptor list address register
pub mod eth_dmac0rx_dlar;
/**ETH_DMAC0TxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxDTPR)

For information about available fields see [`mod@eth_dmac0tx_dtpr`]
module*/
#[doc(alias = "ETH_DMAC0TxDTPR")]
pub type ETH_DMAC0TX_DTPR = crate::Reg<eth_dmac0tx_dtpr::ETH_DMAC0TX_DTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod eth_dmac0tx_dtpr;
/**ETH_DMAC1TxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1tx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1tx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1TxDTPR)

For information about available fields see [`mod@eth_dmac1tx_dtpr`]
module*/
#[doc(alias = "ETH_DMAC1TxDTPR")]
pub type ETH_DMAC1TX_DTPR = crate::Reg<eth_dmac1tx_dtpr::ETH_DMAC1TX_DTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod eth_dmac1tx_dtpr;
/**ETH_DMAC0RxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxDTPR)

For information about available fields see [`mod@eth_dmac0rx_dtpr`]
module*/
#[doc(alias = "ETH_DMAC0RxDTPR")]
pub type ETH_DMAC0RX_DTPR = crate::Reg<eth_dmac0rx_dtpr::ETH_DMAC0RX_DTPRrs>;
///Channel Rx descriptor tail pointer register
pub mod eth_dmac0rx_dtpr;
/**ETH_DMAC0TxRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0tx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0tx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0TxRLR)

For information about available fields see [`mod@eth_dmac0tx_rlr`]
module*/
#[doc(alias = "ETH_DMAC0TxRLR")]
pub type ETH_DMAC0TX_RLR = crate::Reg<eth_dmac0tx_rlr::ETH_DMAC0TX_RLRrs>;
///Channel Tx descriptor ring length register
pub mod eth_dmac0tx_rlr;
/**ETH_DMAC1TxRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1tx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1tx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1TxRLR)

For information about available fields see [`mod@eth_dmac1tx_rlr`]
module*/
#[doc(alias = "ETH_DMAC1TxRLR")]
pub type ETH_DMAC1TX_RLR = crate::Reg<eth_dmac1tx_rlr::ETH_DMAC1TX_RLRrs>;
///Channel Tx descriptor ring length register
pub mod eth_dmac1tx_rlr;
/**ETH_DMAC0RxRLR (rw) register accessor: Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxRLR)

For information about available fields see [`mod@eth_dmac0rx_rlr`]
module*/
#[doc(alias = "ETH_DMAC0RxRLR")]
pub type ETH_DMAC0RX_RLR = crate::Reg<eth_dmac0rx_rlr::ETH_DMAC0RX_RLRrs>;
///Channel Rx descriptor ring length register
pub mod eth_dmac0rx_rlr;
/**ETH_DMAC0IER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0IER)

For information about available fields see [`mod@eth_dmac0ier`]
module*/
pub type ETH_DMAC0IER = crate::Reg<eth_dmac0ier::ETH_DMAC0IERrs>;
///Channel interrupt enable register
pub mod eth_dmac0ier;
/**ETH_DMAC1IER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1IER)

For information about available fields see [`mod@eth_dmac1ier`]
module*/
pub type ETH_DMAC1IER = crate::Reg<eth_dmac1ier::ETH_DMAC1IERrs>;
///Channel interrupt enable register
pub mod eth_dmac1ier;
/**ETH_DMAC0RxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0rx_iwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0rx_iwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0RxIWTR)

For information about available fields see [`mod@eth_dmac0rx_iwtr`]
module*/
#[doc(alias = "ETH_DMAC0RxIWTR")]
pub type ETH_DMAC0RX_IWTR = crate::Reg<eth_dmac0rx_iwtr::ETH_DMAC0RX_IWTRrs>;
///Channel Rx interrupt watchdog timer register
pub mod eth_dmac0rx_iwtr;
/**ETH_DMAC0SFCSR (rw) register accessor: Channel i slot function control status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0SFCSR)

For information about available fields see [`mod@eth_dmac0sfcsr`]
module*/
pub type ETH_DMAC0SFCSR = crate::Reg<eth_dmac0sfcsr::ETH_DMAC0SFCSRrs>;
///Channel i slot function control status register
pub mod eth_dmac0sfcsr;
/**ETH_DMAC1SFCSR (rw) register accessor: Channel i slot function control status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1SFCSR)

For information about available fields see [`mod@eth_dmac1sfcsr`]
module*/
pub type ETH_DMAC1SFCSR = crate::Reg<eth_dmac1sfcsr::ETH_DMAC1SFCSRrs>;
///Channel i slot function control status register
pub mod eth_dmac1sfcsr;
/**ETH_DMAC0CATxDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0catx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CATxDR)

For information about available fields see [`mod@eth_dmac0catx_dr`]
module*/
#[doc(alias = "ETH_DMAC0CATxDR")]
pub type ETH_DMAC0CATX_DR = crate::Reg<eth_dmac0catx_dr::ETH_DMAC0CATX_DRrs>;
///Channel current application transmit descriptor register
pub mod eth_dmac0catx_dr;
/**ETH_DMAC1CATxDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1catx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1CATxDR)

For information about available fields see [`mod@eth_dmac1catx_dr`]
module*/
#[doc(alias = "ETH_DMAC1CATxDR")]
pub type ETH_DMAC1CATX_DR = crate::Reg<eth_dmac1catx_dr::ETH_DMAC1CATX_DRrs>;
///Channel current application transmit descriptor register
pub mod eth_dmac1catx_dr;
/**ETH_DMAC0CARxDR (r) register accessor: Channel 0 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0carx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CARxDR)

For information about available fields see [`mod@eth_dmac0carx_dr`]
module*/
#[doc(alias = "ETH_DMAC0CARxDR")]
pub type ETH_DMAC0CARX_DR = crate::Reg<eth_dmac0carx_dr::ETH_DMAC0CARX_DRrs>;
///Channel 0 current application receive descriptor register
pub mod eth_dmac0carx_dr;
/**ETH_DMAC0CATxBR (r) register accessor: Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0catx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CATxBR)

For information about available fields see [`mod@eth_dmac0catx_br`]
module*/
#[doc(alias = "ETH_DMAC0CATxBR")]
pub type ETH_DMAC0CATX_BR = crate::Reg<eth_dmac0catx_br::ETH_DMAC0CATX_BRrs>;
///Channel 0 current application transmit buffer register
pub mod eth_dmac0catx_br;
/**ETH_DMAC1CATxBR (r) register accessor: Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1catx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1CATxBR)

For information about available fields see [`mod@eth_dmac1catx_br`]
module*/
#[doc(alias = "ETH_DMAC1CATxBR")]
pub type ETH_DMAC1CATX_BR = crate::Reg<eth_dmac1catx_br::ETH_DMAC1CATX_BRrs>;
///Channel 0 current application transmit buffer register
pub mod eth_dmac1catx_br;
/**ETH_DMAC0CARxBR (r) register accessor: Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0carx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CARxBR)

For information about available fields see [`mod@eth_dmac0carx_br`]
module*/
#[doc(alias = "ETH_DMAC0CARxBR")]
pub type ETH_DMAC0CARX_BR = crate::Reg<eth_dmac0carx_br::ETH_DMAC0CARX_BRrs>;
///Channel current application receive buffer register
pub mod eth_dmac0carx_br;
/**ETH_DMAC0SR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac0sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0SR)

For information about available fields see [`mod@eth_dmac0sr`]
module*/
pub type ETH_DMAC0SR = crate::Reg<eth_dmac0sr::ETH_DMAC0SRrs>;
///Channel status register
pub mod eth_dmac0sr;
/**ETH_DMAC1SR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_dmac1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1SR)

For information about available fields see [`mod@eth_dmac1sr`]
module*/
pub type ETH_DMAC1SR = crate::Reg<eth_dmac1sr::ETH_DMAC1SRrs>;
///Channel status register
pub mod eth_dmac1sr;
/**ETH_DMAC0MFCR (r) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0mfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0MFCR)

For information about available fields see [`mod@eth_dmac0mfcr`]
module*/
pub type ETH_DMAC0MFCR = crate::Reg<eth_dmac0mfcr::ETH_DMAC0MFCRrs>;
///Channel missed frame count register
pub mod eth_dmac0mfcr;
/**ETH_DMAC1MFCR (r) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac1mfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC1MFCR)

For information about available fields see [`mod@eth_dmac1mfcr`]
module*/
pub type ETH_DMAC1MFCR = crate::Reg<eth_dmac1mfcr::ETH_DMAC1MFCRrs>;
///Channel missed frame count register
pub mod eth_dmac1mfcr;
