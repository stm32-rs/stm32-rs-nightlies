#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - DMA mode register"]
    #[inline(always)]
    pub const fn dmamr(&self) -> &DMAMR {
        &self.dmamr
    }
    #[doc = "0x04 - System bus mode register"]
    #[inline(always)]
    pub const fn dmasbmr(&self) -> &DMASBMR {
        &self.dmasbmr
    }
    #[doc = "0x08 - Interrupt status register"]
    #[inline(always)]
    pub const fn dmaisr(&self) -> &DMAISR {
        &self.dmaisr
    }
    #[doc = "0x0c - Debug status register"]
    #[inline(always)]
    pub const fn dmadsr(&self) -> &DMADSR {
        &self.dmadsr
    }
    #[doc = "0x100 - Channel control register"]
    #[inline(always)]
    pub const fn dmaccr(&self) -> &DMACCR {
        &self.dmaccr
    }
    #[doc = "0x104 - Channel transmit control register"]
    #[inline(always)]
    pub const fn dmactx_cr(&self) -> &DMACTX_CR {
        &self.dmactx_cr
    }
    #[doc = "0x108 - Channel receive control register"]
    #[inline(always)]
    pub const fn dmacrx_cr(&self) -> &DMACRX_CR {
        &self.dmacrx_cr
    }
    #[doc = "0x114 - Channel Tx descriptor list address register"]
    #[inline(always)]
    pub const fn dmactx_dlar(&self) -> &DMACTX_DLAR {
        &self.dmactx_dlar
    }
    #[doc = "0x11c - Channel Rx descriptor list address register"]
    #[inline(always)]
    pub const fn dmacrx_dlar(&self) -> &DMACRX_DLAR {
        &self.dmacrx_dlar
    }
    #[doc = "0x120 - Channel Tx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmactx_dtpr(&self) -> &DMACTX_DTPR {
        &self.dmactx_dtpr
    }
    #[doc = "0x128 - Channel Rx descriptor tail pointer register"]
    #[inline(always)]
    pub const fn dmacrx_dtpr(&self) -> &DMACRX_DTPR {
        &self.dmacrx_dtpr
    }
    #[doc = "0x12c - Channel Tx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmactx_rlr(&self) -> &DMACTX_RLR {
        &self.dmactx_rlr
    }
    #[doc = "0x130 - Channel Rx descriptor ring length register"]
    #[inline(always)]
    pub const fn dmacrx_rlr(&self) -> &DMACRX_RLR {
        &self.dmacrx_rlr
    }
    #[doc = "0x134 - Channel interrupt enable register"]
    #[inline(always)]
    pub const fn dmacier(&self) -> &DMACIER {
        &self.dmacier
    }
    #[doc = "0x138 - Channel Rx interrupt watchdog timer register"]
    #[inline(always)]
    pub const fn dmacrx_iwtr(&self) -> &DMACRX_IWTR {
        &self.dmacrx_iwtr
    }
    #[doc = "0x144 - Channel current application transmit descriptor register"]
    #[inline(always)]
    pub const fn dmaccatx_dr(&self) -> &DMACCATX_DR {
        &self.dmaccatx_dr
    }
    #[doc = "0x14c - Channel current application receive descriptor register"]
    #[inline(always)]
    pub const fn dmaccarx_dr(&self) -> &DMACCARX_DR {
        &self.dmaccarx_dr
    }
    #[doc = "0x154 - Channel current application transmit buffer register"]
    #[inline(always)]
    pub const fn dmaccatx_br(&self) -> &DMACCATX_BR {
        &self.dmaccatx_br
    }
    #[doc = "0x15c - Channel current application receive buffer register"]
    #[inline(always)]
    pub const fn dmaccarx_br(&self) -> &DMACCARX_BR {
        &self.dmaccarx_br
    }
    #[doc = "0x160 - Channel status register"]
    #[inline(always)]
    pub const fn dmacsr(&self) -> &DMACSR {
        &self.dmacsr
    }
    #[doc = "0x16c - Channel missed frame count register"]
    #[inline(always)]
    pub const fn dmacmfcr(&self) -> &DMACMFCR {
        &self.dmacmfcr
    }
}
#[doc = "DMAMR (rw) register accessor: DMA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmamr`]
module"]
pub type DMAMR = crate::Reg<dmamr::DMAMRrs>;
#[doc = "DMA mode register"]
pub mod dmamr;
#[doc = "DMASBMR (rw) register accessor: System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasbmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasbmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasbmr`]
module"]
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMRrs>;
#[doc = "System bus mode register"]
pub mod dmasbmr;
#[doc = "DMAISR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaisr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaisr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaisr`]
module"]
pub type DMAISR = crate::Reg<dmaisr::DMAISRrs>;
#[doc = "Interrupt status register"]
pub mod dmaisr;
#[doc = "DMADSR (rw) register accessor: Debug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmadsr`]
module"]
pub type DMADSR = crate::Reg<dmadsr::DMADSRrs>;
#[doc = "Debug status register"]
pub mod dmadsr;
#[doc = "DMACCR (rw) register accessor: Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccr`]
module"]
pub type DMACCR = crate::Reg<dmaccr::DMACCRrs>;
#[doc = "Channel control register"]
pub mod dmaccr;
#[doc = "DMACTxCR (rw) register accessor: Channel transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_cr`]
module"]
#[doc(alias = "DMACTxCR")]
pub type DMACTX_CR = crate::Reg<dmactx_cr::DMACTX_CRrs>;
#[doc = "Channel transmit control register"]
pub mod dmactx_cr;
#[doc = "DMACRxCR (rw) register accessor: Channel receive control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_cr`]
module"]
#[doc(alias = "DMACRxCR")]
pub type DMACRX_CR = crate::Reg<dmacrx_cr::DMACRX_CRrs>;
#[doc = "Channel receive control register"]
pub mod dmacrx_cr;
#[doc = "DMACTxDLAR (rw) register accessor: Channel Tx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_dlar`]
module"]
#[doc(alias = "DMACTxDLAR")]
pub type DMACTX_DLAR = crate::Reg<dmactx_dlar::DMACTX_DLARrs>;
#[doc = "Channel Tx descriptor list address register"]
pub mod dmactx_dlar;
#[doc = "DMACRxDLAR (rw) register accessor: Channel Rx descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_dlar`]
module"]
#[doc(alias = "DMACRxDLAR")]
pub type DMACRX_DLAR = crate::Reg<dmacrx_dlar::DMACRX_DLARrs>;
#[doc = "Channel Rx descriptor list address register"]
pub mod dmacrx_dlar;
#[doc = "DMACTxDTPR (rw) register accessor: Channel Tx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_dtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_dtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_dtpr`]
module"]
#[doc(alias = "DMACTxDTPR")]
pub type DMACTX_DTPR = crate::Reg<dmactx_dtpr::DMACTX_DTPRrs>;
#[doc = "Channel Tx descriptor tail pointer register"]
pub mod dmactx_dtpr;
#[doc = "DMACRxDTPR (rw) register accessor: Channel Rx descriptor tail pointer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_dtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_dtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_dtpr`]
module"]
#[doc(alias = "DMACRxDTPR")]
pub type DMACRX_DTPR = crate::Reg<dmacrx_dtpr::DMACRX_DTPRrs>;
#[doc = "Channel Rx descriptor tail pointer register"]
pub mod dmacrx_dtpr;
#[doc = "DMACTxRLR (rw) register accessor: Channel Tx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactx_rlr`]
module"]
#[doc(alias = "DMACTxRLR")]
pub type DMACTX_RLR = crate::Reg<dmactx_rlr::DMACTX_RLRrs>;
#[doc = "Channel Tx descriptor ring length register"]
pub mod dmactx_rlr;
#[doc = "DMACRxRLR (rw) register accessor: Channel Rx descriptor ring length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_rlr`]
module"]
#[doc(alias = "DMACRxRLR")]
pub type DMACRX_RLR = crate::Reg<dmacrx_rlr::DMACRX_RLRrs>;
#[doc = "Channel Rx descriptor ring length register"]
pub mod dmacrx_rlr;
#[doc = "DMACIER (rw) register accessor: Channel interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacier`]
module"]
pub type DMACIER = crate::Reg<dmacier::DMACIERrs>;
#[doc = "Channel interrupt enable register"]
pub mod dmacier;
#[doc = "DMACRxIWTR (rw) register accessor: Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacrx_iwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacrx_iwtr`]
module"]
#[doc(alias = "DMACRxIWTR")]
pub type DMACRX_IWTR = crate::Reg<dmacrx_iwtr::DMACRX_IWTRrs>;
#[doc = "Channel Rx interrupt watchdog timer register"]
pub mod dmacrx_iwtr;
#[doc = "DMACCATxDR (rw) register accessor: Channel current application transmit descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccatx_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccatx_dr`]
module"]
#[doc(alias = "DMACCATxDR")]
pub type DMACCATX_DR = crate::Reg<dmaccatx_dr::DMACCATX_DRrs>;
#[doc = "Channel current application transmit descriptor register"]
pub mod dmaccatx_dr;
#[doc = "DMACCARxDR (rw) register accessor: Channel current application receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccarx_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccarx_dr`]
module"]
#[doc(alias = "DMACCARxDR")]
pub type DMACCARX_DR = crate::Reg<dmaccarx_dr::DMACCARX_DRrs>;
#[doc = "Channel current application receive descriptor register"]
pub mod dmaccarx_dr;
#[doc = "DMACCATxBR (rw) register accessor: Channel current application transmit buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccatx_br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccatx_br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccatx_br`]
module"]
#[doc(alias = "DMACCATxBR")]
pub type DMACCATX_BR = crate::Reg<dmaccatx_br::DMACCATX_BRrs>;
#[doc = "Channel current application transmit buffer register"]
pub mod dmaccatx_br;
#[doc = "DMACCARxBR (rw) register accessor: Channel current application receive buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccarx_br::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccarx_br::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaccarx_br`]
module"]
#[doc(alias = "DMACCARxBR")]
pub type DMACCARX_BR = crate::Reg<dmaccarx_br::DMACCARX_BRrs>;
#[doc = "Channel current application receive buffer register"]
pub mod dmaccarx_br;
#[doc = "DMACSR (rw) register accessor: Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacsr`]
module"]
pub type DMACSR = crate::Reg<dmacsr::DMACSRrs>;
#[doc = "Channel status register"]
pub mod dmacsr;
#[doc = "DMACMFCR (rw) register accessor: Channel missed frame count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacmfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacmfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacmfcr`]
module"]
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCRrs>;
#[doc = "Channel missed frame count register"]
pub mod dmacmfcr;
