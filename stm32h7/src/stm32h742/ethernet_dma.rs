#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmamr: DMAMR,
    dmasbmr: DMASBMR,
    dmaisr: DMAISR,
    dmadsr: DMADSR,
    _reserved4: [u8; 0xf0],
    dmaccr: DMACCR,
    dmactx_cr: DMACTX_CR,
    dmacrx_cr: DMACRX_CR,
    _reserved7: [u8; 0x08],
    dmactx_dlar: DMACTX_DLAR,
    _reserved8: [u8; 0x04],
    dmacrx_dlar: DMACRX_DLAR,
    dmactx_dtpr: DMACTX_DTPR,
    _reserved10: [u8; 0x04],
    dmacrx_dtpr: DMACRX_DTPR,
    dmactx_rlr: DMACTX_RLR,
    dmacrx_rlr: DMACRX_RLR,
    dmacier: DMACIER,
    dmacrx_iwtr: DMACRX_IWTR,
    _reserved15: [u8; 0x08],
    dmaccatx_dr: DMACCATX_DR,
    _reserved16: [u8; 0x04],
    dmaccarx_dr: DMACCARX_DR,
    _reserved17: [u8; 0x04],
    dmaccatx_br: DMACCATX_BR,
    _reserved18: [u8; 0x04],
    dmaccarx_br: DMACCARX_BR,
    dmacsr: DMACSR,
    _reserved20: [u8; 0x08],
    dmacmfcr: DMACMFCR,
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
    ///0x100 - Channel control register
    #[inline(always)]
    pub const fn dmaccr(&self) -> &DMACCR {
        &self.dmaccr
    }
    ///0x104 - Channel transmit control register
    #[inline(always)]
    pub const fn dmactx_cr(&self) -> &DMACTX_CR {
        &self.dmactx_cr
    }
    ///0x108 - Channel receive control register
    #[inline(always)]
    pub const fn dmacrx_cr(&self) -> &DMACRX_CR {
        &self.dmacrx_cr
    }
    ///0x114 - Channel Tx descriptor list address register
    #[inline(always)]
    pub const fn dmactx_dlar(&self) -> &DMACTX_DLAR {
        &self.dmactx_dlar
    }
    ///0x11c - Channel Rx descriptor list address register
    #[inline(always)]
    pub const fn dmacrx_dlar(&self) -> &DMACRX_DLAR {
        &self.dmacrx_dlar
    }
    ///0x120 - Channel Tx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmactx_dtpr(&self) -> &DMACTX_DTPR {
        &self.dmactx_dtpr
    }
    ///0x128 - Channel Rx descriptor tail pointer register
    #[inline(always)]
    pub const fn dmacrx_dtpr(&self) -> &DMACRX_DTPR {
        &self.dmacrx_dtpr
    }
    ///0x12c - Channel Tx descriptor ring length register
    #[inline(always)]
    pub const fn dmactx_rlr(&self) -> &DMACTX_RLR {
        &self.dmactx_rlr
    }
    ///0x130 - Channel Rx descriptor ring length register
    #[inline(always)]
    pub const fn dmacrx_rlr(&self) -> &DMACRX_RLR {
        &self.dmacrx_rlr
    }
    ///0x134 - Channel interrupt enable register
    #[inline(always)]
    pub const fn dmacier(&self) -> &DMACIER {
        &self.dmacier
    }
    ///0x138 - Channel Rx interrupt watchdog timer register
    #[inline(always)]
    pub const fn dmacrx_iwtr(&self) -> &DMACRX_IWTR {
        &self.dmacrx_iwtr
    }
    ///0x144 - Channel current application transmit descriptor register
    #[inline(always)]
    pub const fn dmaccatx_dr(&self) -> &DMACCATX_DR {
        &self.dmaccatx_dr
    }
    ///0x14c - Channel current application receive descriptor register
    #[inline(always)]
    pub const fn dmaccarx_dr(&self) -> &DMACCARX_DR {
        &self.dmaccarx_dr
    }
    ///0x154 - Channel current application transmit buffer register
    #[inline(always)]
    pub const fn dmaccatx_br(&self) -> &DMACCATX_BR {
        &self.dmaccatx_br
    }
    ///0x15c - Channel current application receive buffer register
    #[inline(always)]
    pub const fn dmaccarx_br(&self) -> &DMACCARX_BR {
        &self.dmaccarx_br
    }
    ///0x160 - Channel status register
    #[inline(always)]
    pub const fn dmacsr(&self) -> &DMACSR {
        &self.dmacsr
    }
    ///0x16c - Channel missed frame count register
    #[inline(always)]
    pub const fn dmacmfcr(&self) -> &DMACMFCR {
        &self.dmacmfcr
    }
}
/**DMAMR (rw) register accessor: DMA mode register

You can [`read`](crate::Reg::read) this register and get [`dmamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMAMR)

For information about available fields see [`mod@dmamr`] module*/
pub type DMAMR = crate::Reg<dmamr::DMAMRrs>;
///DMA mode register
pub mod dmamr;
/**DMASBMR (rw) register accessor: System bus mode register

You can [`read`](crate::Reg::read) this register and get [`dmasbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMASBMR)

For information about available fields see [`mod@dmasbmr`] module*/
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMRrs>;
///System bus mode register
pub mod dmasbmr;
/**DMAISR (r) register accessor: Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`dmaisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMAISR)

For information about available fields see [`mod@dmaisr`] module*/
pub type DMAISR = crate::Reg<dmaisr::DMAISRrs>;
///Interrupt status register
pub mod dmaisr;
/**DMADSR (r) register accessor: Debug status register

You can [`read`](crate::Reg::read) this register and get [`dmadsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMADSR)

For information about available fields see [`mod@dmadsr`] module*/
pub type DMADSR = crate::Reg<dmadsr::DMADSRrs>;
///Debug status register
pub mod dmadsr;
/**DMACCR (rw) register accessor: Channel control register

You can [`read`](crate::Reg::read) this register and get [`dmaccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACCR)

For information about available fields see [`mod@dmaccr`] module*/
pub type DMACCR = crate::Reg<dmaccr::DMACCRrs>;
///Channel control register
pub mod dmaccr;
/**DMACTxCR (rw) register accessor: Channel transmit control register

You can [`read`](crate::Reg::read) this register and get [`dmactx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACTxCR)

For information about available fields see [`mod@dmactx_cr`] module*/
#[doc(alias = "DMACTxCR")]
pub type DMACTX_CR = crate::Reg<dmactx_cr::DMACTX_CRrs>;
///Channel transmit control register
pub mod dmactx_cr;
/**DMACRxCR (rw) register accessor: Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACRxCR)

For information about available fields see [`mod@dmacrx_cr`] module*/
#[doc(alias = "DMACRxCR")]
pub type DMACRX_CR = crate::Reg<dmacrx_cr::DMACRX_CRrs>;
///Channel receive control register
pub mod dmacrx_cr;
/**DMACTxDLAR (rw) register accessor: Channel Tx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmactx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACTxDLAR)

For information about available fields see [`mod@dmactx_dlar`] module*/
#[doc(alias = "DMACTxDLAR")]
pub type DMACTX_DLAR = crate::Reg<dmactx_dlar::DMACTX_DLARrs>;
///Channel Tx descriptor list address register
pub mod dmactx_dlar;
/**DMACRxDLAR (rw) register accessor: Channel Rx descriptor list address register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_dlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACRxDLAR)

For information about available fields see [`mod@dmacrx_dlar`] module*/
#[doc(alias = "DMACRxDLAR")]
pub type DMACRX_DLAR = crate::Reg<dmacrx_dlar::DMACRX_DLARrs>;
///Channel Rx descriptor list address register
pub mod dmacrx_dlar;
/**DMACTxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmactx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACTxDTPR)

For information about available fields see [`mod@dmactx_dtpr`] module*/
#[doc(alias = "DMACTxDTPR")]
pub type DMACTX_DTPR = crate::Reg<dmactx_dtpr::DMACTX_DTPRrs>;
///Channel Tx descriptor tail pointer register
pub mod dmactx_dtpr;
/**DMACRxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_dtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACRxDTPR)

For information about available fields see [`mod@dmacrx_dtpr`] module*/
#[doc(alias = "DMACRxDTPR")]
pub type DMACRX_DTPR = crate::Reg<dmacrx_dtpr::DMACRX_DTPRrs>;
///Channel Rx descriptor tail pointer register
pub mod dmacrx_dtpr;
/**DMACTxRLR (rw) register accessor: Channel Tx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmactx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACTxRLR)

For information about available fields see [`mod@dmactx_rlr`] module*/
#[doc(alias = "DMACTxRLR")]
pub type DMACTX_RLR = crate::Reg<dmactx_rlr::DMACTX_RLRrs>;
///Channel Tx descriptor ring length register
pub mod dmactx_rlr;
/**DMACRxRLR (rw) register accessor: Channel Rx descriptor ring length register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACRxRLR)

For information about available fields see [`mod@dmacrx_rlr`] module*/
#[doc(alias = "DMACRxRLR")]
pub type DMACRX_RLR = crate::Reg<dmacrx_rlr::DMACRX_RLRrs>;
///Channel Rx descriptor ring length register
pub mod dmacrx_rlr;
/**DMACIER (rw) register accessor: Channel interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dmacier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACIER)

For information about available fields see [`mod@dmacier`] module*/
pub type DMACIER = crate::Reg<dmacier::DMACIERrs>;
///Channel interrupt enable register
pub mod dmacier;
/**DMACRxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_iwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACRxIWTR)

For information about available fields see [`mod@dmacrx_iwtr`] module*/
#[doc(alias = "DMACRxIWTR")]
pub type DMACRX_IWTR = crate::Reg<dmacrx_iwtr::DMACRX_IWTRrs>;
///Channel Rx interrupt watchdog timer register
pub mod dmacrx_iwtr;
/**DMACCATxDR (r) register accessor: Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccatx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACCATxDR)

For information about available fields see [`mod@dmaccatx_dr`] module*/
#[doc(alias = "DMACCATxDR")]
pub type DMACCATX_DR = crate::Reg<dmaccatx_dr::DMACCATX_DRrs>;
///Channel current application transmit descriptor register
pub mod dmaccatx_dr;
/**DMACCARxDR (r) register accessor: Channel current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACCARxDR)

For information about available fields see [`mod@dmaccarx_dr`] module*/
#[doc(alias = "DMACCARxDR")]
pub type DMACCARX_DR = crate::Reg<dmaccarx_dr::DMACCARX_DRrs>;
///Channel current application receive descriptor register
pub mod dmaccarx_dr;
/**DMACCATxBR (r) register accessor: Channel current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccatx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACCATxBR)

For information about available fields see [`mod@dmaccatx_br`] module*/
#[doc(alias = "DMACCATxBR")]
pub type DMACCATX_BR = crate::Reg<dmaccatx_br::DMACCATX_BRrs>;
///Channel current application transmit buffer register
pub mod dmaccatx_br;
/**DMACCARxBR (r) register accessor: Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_br::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACCARxBR)

For information about available fields see [`mod@dmaccarx_br`] module*/
#[doc(alias = "DMACCARxBR")]
pub type DMACCARX_BR = crate::Reg<dmaccarx_br::DMACCARX_BRrs>;
///Channel current application receive buffer register
pub mod dmaccarx_br;
/**DMACSR (rw) register accessor: Channel status register

You can [`read`](crate::Reg::read) this register and get [`dmacsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACSR)

For information about available fields see [`mod@dmacsr`] module*/
pub type DMACSR = crate::Reg<dmacsr::DMACSRrs>;
///Channel status register
pub mod dmacsr;
/**DMACMFCR (r) register accessor: Channel missed frame count register

You can [`read`](crate::Reg::read) this register and get [`dmacmfcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_DMA:DMACMFCR)

For information about available fields see [`mod@dmacmfcr`] module*/
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCRrs>;
///Channel missed frame count register
pub mod dmacmfcr;
