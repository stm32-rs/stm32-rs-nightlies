#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmamr: DMAMR,
    dmasbmr: DMASBMR,
    dmaisr: DMAISR,
    dmadsr: DMADSR,
    _reserved4: [u8; 0x10],
    dmaa4tx_acr: DMAA4TX_ACR,
    dmaa4rx_acr: DMAA4RX_ACR,
    dmaa4dacr: DMAA4DACR,
    _reserved7: [u8; 0xd4],
    dmac0cr: DMAC0CR,
    dmac0tx_cr: DMAC0TX_CR,
    dmac0rx_cr: DMAC0RX_CR,
    _reserved10: [u8; 0x08],
    dmac0tx_dlar: DMAC0TX_DLAR,
    _reserved11: [u8; 0x04],
    dmac0rx_dlar: DMAC0RX_DLAR,
    dmac0tx_dtpr: DMAC0TX_DTPR,
    _reserved13: [u8; 0x04],
    dmac0rx_dtpr: DMAC0RX_DTPR,
    dmac0tx_rlr: DMAC0TX_RLR,
    dmac0rx_rlr: DMAC0RX_RLR,
    dmac0ier: DMAC0IER,
    dmac0rx_iwtr: DMAC0RX_IWTR,
    dmac0sfcsr: DMAC0SFCSR,
    _reserved19: [u8; 0x04],
    dmac0catx_dr: DMAC0CATX_DR,
    _reserved20: [u8; 0x04],
    dmac0carx_dr: DMAC0CARX_DR,
    _reserved21: [u8; 0x04],
    dmac0catx_br: DMAC0CATX_BR,
    _reserved22: [u8; 0x04],
    dmac0carx_br: DMAC0CARX_BR,
    dmac0sr: DMAC0SR,
    _reserved24: [u8; 0x08],
    dmac0mfcr: DMAC0MFCR,
    _reserved25: [u8; 0x10],
    dmac1cr: DMAC1CR,
    dmac1tx_cr: DMAC1TX_CR,
    _reserved27: [u8; 0x0c],
    dmac1tx_dlar: DMAC1TX_DLAR,
    _reserved28: [u8; 0x08],
    dmac1tx_dtpr: DMAC1TX_DTPR,
    _reserved29: [u8; 0x08],
    dmac1tx_rlr: DMAC1TX_RLR,
    _reserved30: [u8; 0x04],
    dmac1ier: DMAC1IER,
    _reserved31: [u8; 0x04],
    dmac1sfcsr: DMAC1SFCSR,
    _reserved32: [u8; 0x04],
    dmac1catx_dr: DMAC1CATX_DR,
    _reserved33: [u8; 0x0c],
    dmac1catx_br: DMAC1CATX_BR,
    _reserved34: [u8; 0x08],
    dmac1sr: DMAC1SR,
    _reserved35: [u8; 0x08],
    dmac1mfcr: DMAC1MFCR,
}
impl RegisterBlock {
    ///0x00 - DMA mode register
    #[inline(always)]
    pub const fn dmamr(&self) -> &DMAMR {
        &self.dmamr
    }
    ///0x04 - System bus mode register
    #[inline(always)]
    pub const fn dmasbmr(&self) -> &DMASBMR {
        &self.dmasbmr
    }
    ///0x08 - Interrupt status register
    #[inline(always)]
    pub const fn dmaisr(&self) -> &DMAISR {
        &self.dmaisr
    }
    ///0x0c - Debug status register
    #[inline(always)]
    pub const fn dmadsr(&self) -> &DMADSR {
        &self.dmadsr
    }
    ///0x20 - AXI4 transmit channel ACE control register
    #[inline(always)]
    pub const fn dmaa4tx_acr(&self) -> &DMAA4TX_ACR {
        &self.dmaa4tx_acr
    }
    ///0x24 - AXI4 receive channel ACE control register
    #[inline(always)]
    pub const fn dmaa4rx_acr(&self) -> &DMAA4RX_ACR {
        &self.dmaa4rx_acr
    }
    ///0x28 - AXI4 descriptor ACE control register
    #[inline(always)]
    pub const fn dmaa4dacr(&self) -> &DMAA4DACR {
        &self.dmaa4dacr
    }
    ///0x100 - Channel 0 control register
    #[inline(always)]
    pub const fn dmac0cr(&self) -> &DMAC0CR {
        &self.dmac0cr
    }
    ///0x104 - Channel 0 transmit control register
    #[inline(always)]
    pub const fn dmac0tx_cr(&self) -> &DMAC0TX_CR {
        &self.dmac0tx_cr
    }
    ///0x108 - Channel receive control register
    #[inline(always)]
    pub const fn dmac0rx_cr(&self) -> &DMAC0RX_CR {
        &self.dmac0rx_cr
    }
    ///0x114 - Channel i Tx descriptor list address register
    #[inline(always)]
    pub const fn dmac0tx_dlar(&self) -> &DMAC0TX_DLAR {
        &self.dmac0tx_dlar
    }
    ///0x11c - Channel Rx descriptor list address register
    #[inline(always)]
    pub const fn dmac0rx_dlar(&self) -> &DMAC0RX_DLAR {
        &self.dmac0rx_dlar
    }
    ///0x120 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac0tx_dtpr(&self) -> &DMAC0TX_DTPR {
        &self.dmac0tx_dtpr
    }
    ///0x128 - Channel Rx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac0rx_dtpr(&self) -> &DMAC0RX_DTPR {
        &self.dmac0rx_dtpr
    }
    ///0x12c - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn dmac0tx_rlr(&self) -> &DMAC0TX_RLR {
        &self.dmac0tx_rlr
    }
    ///0x130 - Channel Rx descriptor ring length register
    #[inline(always)]
    pub const fn dmac0rx_rlr(&self) -> &DMAC0RX_RLR {
        &self.dmac0rx_rlr
    }
    ///0x134 - Channel interrupt enable register
    #[inline(always)]
    pub const fn dmac0ier(&self) -> &DMAC0IER {
        &self.dmac0ier
    }
    ///0x138 - Channel Rx interrupt watchdog timer register
    #[inline(always)]
    pub const fn dmac0rx_iwtr(&self) -> &DMAC0RX_IWTR {
        &self.dmac0rx_iwtr
    }
    ///0x13c - Channel i slot function control status register
    #[inline(always)]
    pub const fn dmac0sfcsr(&self) -> &DMAC0SFCSR {
        &self.dmac0sfcsr
    }
    ///0x144 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn dmac0catx_dr(&self) -> &DMAC0CATX_DR {
        &self.dmac0catx_dr
    }
    ///0x14c - Channel 0 current application receive descriptor register
    #[inline(always)]
    pub const fn dmac0carx_dr(&self) -> &DMAC0CARX_DR {
        &self.dmac0carx_dr
    }
    ///0x154 - Channel 0 current application transmit buffer register
    #[inline(always)]
    pub const fn dmac0catx_br(&self) -> &DMAC0CATX_BR {
        &self.dmac0catx_br
    }
    ///0x15c - Channel current application receive buffer register
    #[inline(always)]
    pub const fn dmac0carx_br(&self) -> &DMAC0CARX_BR {
        &self.dmac0carx_br
    }
    ///0x160 - Channel status register
    #[inline(always)]
    pub const fn dmac0sr(&self) -> &DMAC0SR {
        &self.dmac0sr
    }
    ///0x16c - Channel missed frame count register
    #[inline(always)]
    pub const fn dmac0mfcr(&self) -> &DMAC0MFCR {
        &self.dmac0mfcr
    }
    ///0x180 - Channel 1 control register
    #[inline(always)]
    pub const fn dmac1cr(&self) -> &DMAC1CR {
        &self.dmac1cr
    }
    ///0x184 - Channel 1 transmit control register
    #[inline(always)]
    pub const fn dmac1tx_cr(&self) -> &DMAC1TX_CR {
        &self.dmac1tx_cr
    }
    ///0x194 - Channel i Tx descriptor list address register
    #[inline(always)]
    pub const fn dmac1tx_dlar(&self) -> &DMAC1TX_DLAR {
        &self.dmac1tx_dlar
    }
    ///0x1a0 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmac1tx_dtpr(&self) -> &DMAC1TX_DTPR {
        &self.dmac1tx_dtpr
    }
    ///0x1ac - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn dmac1tx_rlr(&self) -> &DMAC1TX_RLR {
        &self.dmac1tx_rlr
    }
    ///0x1b4 - Channel interrupt enable register
    #[inline(always)]
    pub const fn dmac1ier(&self) -> &DMAC1IER {
        &self.dmac1ier
    }
    ///0x1bc - Channel i slot function control status register
    #[inline(always)]
    pub const fn dmac1sfcsr(&self) -> &DMAC1SFCSR {
        &self.dmac1sfcsr
    }
    ///0x1c4 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn dmac1catx_dr(&self) -> &DMAC1CATX_DR {
        &self.dmac1catx_dr
    }
    ///0x1d4 - Channel 0 current application transmit buffer register
    #[inline(always)]
    pub const fn dmac1catx_br(&self) -> &DMAC1CATX_BR {
        &self.dmac1catx_br
    }
    ///0x1e0 - Channel status register
    #[inline(always)]
    pub const fn dmac1sr(&self) -> &DMAC1SR {
        &self.dmac1sr
    }
    ///0x1ec - Channel missed frame count register
    #[inline(always)]
    pub const fn dmac1mfcr(&self) -> &DMAC1MFCR {
        &self.dmac1mfcr
    }
}
/**DMAMR (rw) register accessor: DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAMR)

For information about available fields see [`mod@dmamr`] module*/
pub type DMAMR = crate::Reg<dmamr::DMAMRrs>;
///DMA mode register
pub mod dmamr;
/**DMASBMR (rw) register accessor: System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMASBMR)

For information about available fields see [`mod@dmasbmr`] module*/
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMRrs>;
///System bus mode register
pub mod dmasbmr;
/**DMAISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAISR)

For information about available fields see [`mod@dmaisr`] module*/
pub type DMAISR = crate::Reg<dmaisr::DMAISRrs>;
///Interrupt status register
pub mod dmaisr;
/**DMADSR (r) register accessor: Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMADSR)

For information about available fields see [`mod@dmadsr`] module*/
pub type DMADSR = crate::Reg<dmadsr::DMADSRrs>;
///Debug status register
pub mod dmadsr;
/**DMAA4TxACR (rw) register accessor: AXI4 transmit channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4tx_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4tx_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAA4TxACR)

For information about available fields see [`mod@dmaa4tx_acr`] module*/
#[doc(alias = "DMAA4TxACR")]
pub type DMAA4TX_ACR = crate::Reg<dmaa4tx_acr::DMAA4TX_ACRrs>;
///AXI4 transmit channel ACE control register
pub mod dmaa4tx_acr;
/**DMAA4RxACR (rw) register accessor: AXI4 receive channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4rx_acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4rx_acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAA4RxACR)

For information about available fields see [`mod@dmaa4rx_acr`] module*/
#[doc(alias = "DMAA4RxACR")]
pub type DMAA4RX_ACR = crate::Reg<dmaa4rx_acr::DMAA4RX_ACRrs>;
///AXI4 receive channel ACE control register
pub mod dmaa4rx_acr;
/**DMAA4DACR (rw) register accessor: AXI4 descriptor ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAA4DACR)

For information about available fields see [`mod@dmaa4dacr`] module*/
pub type DMAA4DACR = crate::Reg<dmaa4dacr::DMAA4DACRrs>;
///AXI4 descriptor ACE control register
pub mod dmaa4dacr;
/**DMAC0CR (rw) register accessor: Channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`dmac0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CR)

For information about available fields see [`mod@dmac0cr`] module*/
pub type DMAC0CR = crate::Reg<dmac0cr::DMAC0CRrs>;
///Channel 0 control register
pub mod dmac0cr;
/**DMAC1CR (rw) register accessor: Channel 1 control register

You can [`read`](crate::Reg::read) this register and get [`dmac1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1CR)

For information about available fields see [`mod@dmac1cr`] module*/
pub type DMAC1CR = crate::Reg<dmac1cr::DMAC1CRrs>;
///Channel 1 control register
pub mod dmac1cr;
/**DMAC0TxCR (rw) register accessor: Channel 0 transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmac0tx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0tx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0TxCR)

For information about available fields see [`mod@dmac0tx_cr`] module*/
#[doc(alias = "DMAC0TxCR")]
pub type DMAC0TX_CR = crate::Reg<dmac0tx_cr::DMAC0TX_CRrs>;
///Channel 0 transmit control register
pub mod dmac0tx_cr;
/**DMAC1TxCR (rw) register accessor: Channel 1 transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxCR)

For information about available fields see [`mod@dmac1tx_cr`] module*/
#[doc(alias = "DMAC1TxCR")]
pub type DMAC1TX_CR = crate::Reg<dmac1tx_cr::DMAC1TX_CRrs>;
///Channel 1 transmit control register
pub mod dmac1tx_cr;
/**DMAC0RxCR (rw) register accessor: Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxCR)

For information about available fields see [`mod@dmac0rx_cr`] module*/
#[doc(alias = "DMAC0RxCR")]
pub type DMAC0RX_CR = crate::Reg<dmac0rx_cr::DMAC0RX_CRrs>;
///Channel receive control register
pub mod dmac0rx_cr;
/**DMAC0TxDLAR (rw) register accessor: Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0tx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0tx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0TxDLAR)

For information about available fields see [`mod@dmac0tx_dlar`] module*/
#[doc(alias = "DMAC0TxDLAR")]
pub type DMAC0TX_DLAR = crate::Reg<dmac0tx_dlar::DMAC0TX_DLARrs>;
///Channel i Tx descriptor list address register
pub mod dmac0tx_dlar;
/**DMAC1TxDLAR (rw) register accessor: Channel i Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxDLAR)

For information about available fields see [`mod@dmac1tx_dlar`] module*/
#[doc(alias = "DMAC1TxDLAR")]
pub type DMAC1TX_DLAR = crate::Reg<dmac1tx_dlar::DMAC1TX_DLARrs>;
///Channel i Tx descriptor list address register
pub mod dmac1tx_dlar;
/**DMAC0RxDLAR (rw) register accessor: Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxDLAR)

For information about available fields see [`mod@dmac0rx_dlar`] module*/
#[doc(alias = "DMAC0RxDLAR")]
pub type DMAC0RX_DLAR = crate::Reg<dmac0rx_dlar::DMAC0RX_DLARrs>;
///Channel Rx descriptor list address register
pub mod dmac0rx_dlar;
/**DMAC0TxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0tx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0tx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0TxDTPR)

For information about available fields see [`mod@dmac0tx_dtpr`] module*/
#[doc(alias = "DMAC0TxDTPR")]
pub type DMAC0TX_DTPR = crate::Reg<dmac0tx_dtpr::DMAC0TX_DTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod dmac0tx_dtpr;
/**DMAC1TxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxDTPR)

For information about available fields see [`mod@dmac1tx_dtpr`] module*/
#[doc(alias = "DMAC1TxDTPR")]
pub type DMAC1TX_DTPR = crate::Reg<dmac1tx_dtpr::DMAC1TX_DTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod dmac1tx_dtpr;
/**DMAC0RxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxDTPR)

For information about available fields see [`mod@dmac0rx_dtpr`] module*/
#[doc(alias = "DMAC0RxDTPR")]
pub type DMAC0RX_DTPR = crate::Reg<dmac0rx_dtpr::DMAC0RX_DTPRrs>;
///Channel Rx descriptor tail pointer register
pub mod dmac0rx_dtpr;
/**DMAC0TxRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0tx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0tx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0TxRLR)

For information about available fields see [`mod@dmac0tx_rlr`] module*/
#[doc(alias = "DMAC0TxRLR")]
pub type DMAC0TX_RLR = crate::Reg<dmac0tx_rlr::DMAC0TX_RLRrs>;
///Channel Tx descriptor ring length register
pub mod dmac0tx_rlr;
/**DMAC1TxRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac1tx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1tx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1TxRLR)

For information about available fields see [`mod@dmac1tx_rlr`] module*/
#[doc(alias = "DMAC1TxRLR")]
pub type DMAC1TX_RLR = crate::Reg<dmac1tx_rlr::DMAC1TX_RLRrs>;
///Channel Tx descriptor ring length register
pub mod dmac1tx_rlr;
/**DMAC0RxRLR (rw) register accessor: Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxRLR)

For information about available fields see [`mod@dmac0rx_rlr`] module*/
#[doc(alias = "DMAC0RxRLR")]
pub type DMAC0RX_RLR = crate::Reg<dmac0rx_rlr::DMAC0RX_RLRrs>;
///Channel Rx descriptor ring length register
pub mod dmac0rx_rlr;
/**DMAC0IER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmac0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0IER)

For information about available fields see [`mod@dmac0ier`] module*/
pub type DMAC0IER = crate::Reg<dmac0ier::DMAC0IERrs>;
///Channel interrupt enable register
pub mod dmac0ier;
/**DMAC1IER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmac1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1IER)

For information about available fields see [`mod@dmac1ier`] module*/
pub type DMAC1IER = crate::Reg<dmac1ier::DMAC1IERrs>;
///Channel interrupt enable register
pub mod dmac1ier;
/**DMAC0RxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_iwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_iwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxIWTR)

For information about available fields see [`mod@dmac0rx_iwtr`] module*/
#[doc(alias = "DMAC0RxIWTR")]
pub type DMAC0RX_IWTR = crate::Reg<dmac0rx_iwtr::DMAC0RX_IWTRrs>;
///Channel Rx interrupt watchdog timer register
pub mod dmac0rx_iwtr;
/**DMAC0SFCSR (rw) register accessor: Channel i slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac0sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0SFCSR)

For information about available fields see [`mod@dmac0sfcsr`] module*/
pub type DMAC0SFCSR = crate::Reg<dmac0sfcsr::DMAC0SFCSRrs>;
///Channel i slot function control status register
pub mod dmac0sfcsr;
/**DMAC1SFCSR (rw) register accessor: Channel i slot function control status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sfcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sfcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1SFCSR)

For information about available fields see [`mod@dmac1sfcsr`] module*/
pub type DMAC1SFCSR = crate::Reg<dmac1sfcsr::DMAC1SFCSRrs>;
///Channel i slot function control status register
pub mod dmac1sfcsr;
/**DMAC0CATxDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0catx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CATxDR)

For information about available fields see [`mod@dmac0catx_dr`] module*/
#[doc(alias = "DMAC0CATxDR")]
pub type DMAC0CATX_DR = crate::Reg<dmac0catx_dr::DMAC0CATX_DRrs>;
///Channel current application transmit descriptor register
pub mod dmac0catx_dr;
/**DMAC1CATxDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac1catx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1CATxDR)

For information about available fields see [`mod@dmac1catx_dr`] module*/
#[doc(alias = "DMAC1CATxDR")]
pub type DMAC1CATX_DR = crate::Reg<dmac1catx_dr::DMAC1CATX_DRrs>;
///Channel current application transmit descriptor register
pub mod dmac1catx_dr;
/**DMAC0CARxDR (r) register accessor: Channel 0 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0carx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CARxDR)

For information about available fields see [`mod@dmac0carx_dr`] module*/
#[doc(alias = "DMAC0CARxDR")]
pub type DMAC0CARX_DR = crate::Reg<dmac0carx_dr::DMAC0CARX_DRrs>;
///Channel 0 current application receive descriptor register
pub mod dmac0carx_dr;
/**DMAC0CATxBR (r) register accessor: Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac0catx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CATxBR)

For information about available fields see [`mod@dmac0catx_br`] module*/
#[doc(alias = "DMAC0CATxBR")]
pub type DMAC0CATX_BR = crate::Reg<dmac0catx_br::DMAC0CATX_BRrs>;
///Channel 0 current application transmit buffer register
pub mod dmac0catx_br;
/**DMAC1CATxBR (r) register accessor: Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1catx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1CATxBR)

For information about available fields see [`mod@dmac1catx_br`] module*/
#[doc(alias = "DMAC1CATxBR")]
pub type DMAC1CATX_BR = crate::Reg<dmac1catx_br::DMAC1CATX_BRrs>;
///Channel 0 current application transmit buffer register
pub mod dmac1catx_br;
/**DMAC0CARxBR (r) register accessor: Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac0carx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CARxBR)

For information about available fields see [`mod@dmac0carx_br`] module*/
#[doc(alias = "DMAC0CARxBR")]
pub type DMAC0CARX_BR = crate::Reg<dmac0carx_br::DMAC0CARX_BRrs>;
///Channel current application receive buffer register
pub mod dmac0carx_br;
/**DMAC0SR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`dmac0sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0SR)

For information about available fields see [`mod@dmac0sr`] module*/
pub type DMAC0SR = crate::Reg<dmac0sr::DMAC0SRrs>;
///Channel status register
pub mod dmac0sr;
/**DMAC1SR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`dmac1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1SR)

For information about available fields see [`mod@dmac1sr`] module*/
pub type DMAC1SR = crate::Reg<dmac1sr::DMAC1SRrs>;
///Channel status register
pub mod dmac1sr;
/**DMAC0MFCR (r) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac0mfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0MFCR)

For information about available fields see [`mod@dmac0mfcr`] module*/
pub type DMAC0MFCR = crate::Reg<dmac0mfcr::DMAC0MFCRrs>;
///Channel missed frame count register
pub mod dmac0mfcr;
/**DMAC1MFCR (r) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmac1mfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC1MFCR)

For information about available fields see [`mod@dmac1mfcr`] module*/
pub type DMAC1MFCR = crate::Reg<dmac1mfcr::DMAC1MFCRrs>;
///Channel missed frame count register
pub mod dmac1mfcr;
