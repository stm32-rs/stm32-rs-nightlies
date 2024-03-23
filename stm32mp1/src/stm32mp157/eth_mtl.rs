#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eth_mtlomr: ETH_MTLOMR,
    _reserved1: [u8; 0x1c],
    eth_mtlisr: ETH_MTLISR,
    _reserved2: [u8; 0xdc],
    eth_mtltx_q0omr: ETH_MTLTX_Q0OMR,
    eth_mtltx_q0ur: ETH_MTLTX_Q0UR,
    eth_mtltx_q0dr: ETH_MTLTX_Q0DR,
    _reserved5: [u8; 0x08],
    eth_mtltx_q0esr: ETH_MTLTX_Q0ESR,
    _reserved6: [u8; 0x14],
    eth_mtlq0icsr: ETH_MTLQ0ICSR,
    eth_mtlrx_q0omr: ETH_MTLRX_Q0OMR,
    eth_mtlrx_q0mpocr: ETH_MTLRX_Q0MPOCR,
    eth_mtlrx_q0dr: ETH_MTLRX_Q0DR,
    eth_mtlrx_q0cr: ETH_MTLRX_Q0CR,
    eth_mtltx_q1omr: ETH_MTLTX_Q1OMR,
    eth_mtltx_q1ur: ETH_MTLTX_Q1UR,
    eth_mtltx_q1dr: ETH_MTLTX_Q1DR,
    _reserved14: [u8; 0x04],
    eth_mtltx_q1ecr: ETH_MTLTX_Q1ECR,
    eth_mtltx_q1esr: ETH_MTLTX_Q1ESR,
    eth_mtltx_q1qwr: ETH_MTLTX_Q1QWR,
    eth_mtltx_q1sscr: ETH_MTLTX_Q1SSCR,
    eth_mtltx_q1hcr: ETH_MTLTX_Q1HCR,
    eth_mtltx_q1lcr: ETH_MTLTX_Q1LCR,
    _reserved20: [u8; 0x04],
    eth_mtlq1icsr: ETH_MTLQ1ICSR,
    eth_mtlrx_q1omr: ETH_MTLRX_Q1OMR,
    eth_mtlrx_q1mpocr: ETH_MTLRX_Q1MPOCR,
    eth_mtlrx_q1dr: ETH_MTLRX_Q1DR,
    eth_mtlrx_q1cr: ETH_MTLRX_Q1CR,
}
impl RegisterBlock {
    #[doc = "0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
    #[inline(always)]
    pub const fn eth_mtlomr(&self) -> &ETH_MTLOMR {
        &self.eth_mtlomr
    }
    #[doc = "0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
    #[inline(always)]
    pub const fn eth_mtlisr(&self) -> &ETH_MTLISR {
        &self.eth_mtlisr
    }
    #[doc = "0x100 - Tx queue 0 operating mode Register"]
    #[inline(always)]
    pub const fn eth_mtltx_q0omr(&self) -> &ETH_MTLTX_Q0OMR {
        &self.eth_mtltx_q0omr
    }
    #[doc = "0x104 - Tx queue 0 underflow register"]
    #[inline(always)]
    pub const fn eth_mtltx_q0ur(&self) -> &ETH_MTLTX_Q0UR {
        &self.eth_mtltx_q0ur
    }
    #[doc = "0x108 - Tx queue 0 underflow register"]
    #[inline(always)]
    pub const fn eth_mtltx_q0dr(&self) -> &ETH_MTLTX_Q0DR {
        &self.eth_mtltx_q0dr
    }
    #[doc = "0x114 - Tx queue x ETS status Register"]
    #[inline(always)]
    pub const fn eth_mtltx_q0esr(&self) -> &ETH_MTLTX_Q0ESR {
        &self.eth_mtltx_q0esr
    }
    #[doc = "0x12c - Queue 0 interrupt control status Register"]
    #[inline(always)]
    pub const fn eth_mtlq0icsr(&self) -> &ETH_MTLQ0ICSR {
        &self.eth_mtlq0icsr
    }
    #[doc = "0x130 - Rx queue 0 operating mode register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q0omr(&self) -> &ETH_MTLRX_Q0OMR {
        &self.eth_mtlrx_q0omr
    }
    #[doc = "0x134 - Rx queue 0 missed packet and overflow counter register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q0mpocr(&self) -> &ETH_MTLRX_Q0MPOCR {
        &self.eth_mtlrx_q0mpocr
    }
    #[doc = "0x138 - Rx queue i debug register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q0dr(&self) -> &ETH_MTLRX_Q0DR {
        &self.eth_mtlrx_q0dr
    }
    #[doc = "0x13c - Rx queue 0 control register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q0cr(&self) -> &ETH_MTLRX_Q0CR {
        &self.eth_mtlrx_q0cr
    }
    #[doc = "0x140 - Tx queue 1 operating mode Register"]
    #[inline(always)]
    pub const fn eth_mtltx_q1omr(&self) -> &ETH_MTLTX_Q1OMR {
        &self.eth_mtltx_q1omr
    }
    #[doc = "0x144 - Tx queue 1 underflow register"]
    #[inline(always)]
    pub const fn eth_mtltx_q1ur(&self) -> &ETH_MTLTX_Q1UR {
        &self.eth_mtltx_q1ur
    }
    #[doc = "0x148 - Tx queue 1 underflow register"]
    #[inline(always)]
    pub const fn eth_mtltx_q1dr(&self) -> &ETH_MTLTX_Q1DR {
        &self.eth_mtltx_q1dr
    }
    #[doc = "0x150 - The Queue ETS Control register controls the enhanced transmission selection operation."]
    #[inline(always)]
    pub const fn eth_mtltx_q1ecr(&self) -> &ETH_MTLTX_Q1ECR {
        &self.eth_mtltx_q1ecr
    }
    #[doc = "0x154 - Tx queue x ETS status Register"]
    #[inline(always)]
    pub const fn eth_mtltx_q1esr(&self) -> &ETH_MTLTX_Q1ESR {
        &self.eth_mtltx_q1esr
    }
    #[doc = "0x158 - This register provides the average traffic transmitted on queue 1."]
    #[inline(always)]
    pub const fn eth_mtltx_q1qwr(&self) -> &ETH_MTLTX_Q1QWR {
        &self.eth_mtltx_q1qwr
    }
    #[doc = "0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
    #[inline(always)]
    pub const fn eth_mtltx_q1sscr(&self) -> &ETH_MTLTX_Q1SSCR {
        &self.eth_mtltx_q1sscr
    }
    #[doc = "0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
    #[inline(always)]
    pub const fn eth_mtltx_q1hcr(&self) -> &ETH_MTLTX_Q1HCR {
        &self.eth_mtltx_q1hcr
    }
    #[doc = "0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
    #[inline(always)]
    pub const fn eth_mtltx_q1lcr(&self) -> &ETH_MTLTX_Q1LCR {
        &self.eth_mtltx_q1lcr
    }
    #[doc = "0x16c - Queue 1 interrupt control status Register"]
    #[inline(always)]
    pub const fn eth_mtlq1icsr(&self) -> &ETH_MTLQ1ICSR {
        &self.eth_mtlq1icsr
    }
    #[doc = "0x170 - Rx queue 1 operating mode register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q1omr(&self) -> &ETH_MTLRX_Q1OMR {
        &self.eth_mtlrx_q1omr
    }
    #[doc = "0x174 - Rx queue 1 missed packet and overflow counter register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q1mpocr(&self) -> &ETH_MTLRX_Q1MPOCR {
        &self.eth_mtlrx_q1mpocr
    }
    #[doc = "0x178 - Rx queue i debug register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q1dr(&self) -> &ETH_MTLRX_Q1DR {
        &self.eth_mtlrx_q1dr
    }
    #[doc = "0x17c - Rx queue 1 control register"]
    #[inline(always)]
    pub const fn eth_mtlrx_q1cr(&self) -> &ETH_MTLRX_Q1CR {
        &self.eth_mtlrx_q1cr
    }
}
#[doc = "ETH_MTLOMR (rw) register accessor: The Operating Mode register establishes the Transmit and Receive operating modes and commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlomr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlomr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlomr`]
module"]
pub type ETH_MTLOMR = crate::Reg<eth_mtlomr::ETH_MTLOMRrs>;
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
pub mod eth_mtlomr;
#[doc = "ETH_MTLISR (r) register accessor: The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlisr`]
module"]
pub type ETH_MTLISR = crate::Reg<eth_mtlisr::ETH_MTLISRrs>;
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
pub mod eth_mtlisr;
#[doc = "ETH_MTLTxQ0OMR (rw) register accessor: Tx queue 0 operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0omr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q0omr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q0omr`]
module"]
#[doc(alias = "ETH_MTLTxQ0OMR")]
pub type ETH_MTLTX_Q0OMR = crate::Reg<eth_mtltx_q0omr::ETH_MTLTX_Q0OMRrs>;
#[doc = "Tx queue 0 operating mode Register"]
pub mod eth_mtltx_q0omr;
#[doc = "ETH_MTLTxQ1OMR (rw) register accessor: Tx queue 1 operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1omr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1omr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1omr`]
module"]
#[doc(alias = "ETH_MTLTxQ1OMR")]
pub type ETH_MTLTX_Q1OMR = crate::Reg<eth_mtltx_q1omr::ETH_MTLTX_Q1OMRrs>;
#[doc = "Tx queue 1 operating mode Register"]
pub mod eth_mtltx_q1omr;
#[doc = "ETH_MTLTxQ0UR (r) register accessor: Tx queue 0 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0ur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q0ur`]
module"]
#[doc(alias = "ETH_MTLTxQ0UR")]
pub type ETH_MTLTX_Q0UR = crate::Reg<eth_mtltx_q0ur::ETH_MTLTX_Q0URrs>;
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0ur;
#[doc = "ETH_MTLTxQ1UR (r) register accessor: Tx queue 1 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1ur::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1ur`]
module"]
#[doc(alias = "ETH_MTLTxQ1UR")]
pub type ETH_MTLTX_Q1UR = crate::Reg<eth_mtltx_q1ur::ETH_MTLTX_Q1URrs>;
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1ur;
#[doc = "ETH_MTLTxQ0DR (r) register accessor: Tx queue 0 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q0dr`]
module"]
#[doc(alias = "ETH_MTLTxQ0DR")]
pub type ETH_MTLTX_Q0DR = crate::Reg<eth_mtltx_q0dr::ETH_MTLTX_Q0DRrs>;
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0dr;
#[doc = "ETH_MTLTxQ1DR (r) register accessor: Tx queue 1 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1dr`]
module"]
#[doc(alias = "ETH_MTLTxQ1DR")]
pub type ETH_MTLTX_Q1DR = crate::Reg<eth_mtltx_q1dr::ETH_MTLTX_Q1DRrs>;
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1dr;
#[doc = "ETH_MTLTxQ0ESR (r) register accessor: Tx queue x ETS status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q0esr`]
module"]
#[doc(alias = "ETH_MTLTxQ0ESR")]
pub type ETH_MTLTX_Q0ESR = crate::Reg<eth_mtltx_q0esr::ETH_MTLTX_Q0ESRrs>;
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q0esr;
#[doc = "ETH_MTLTxQ1ESR (r) register accessor: Tx queue x ETS status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1esr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1esr`]
module"]
#[doc(alias = "ETH_MTLTxQ1ESR")]
pub type ETH_MTLTX_Q1ESR = crate::Reg<eth_mtltx_q1esr::ETH_MTLTX_Q1ESRrs>;
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q1esr;
#[doc = "ETH_MTLQ0ICSR (rw) register accessor: Queue 0 interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlq0icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlq0icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlq0icsr`]
module"]
pub type ETH_MTLQ0ICSR = crate::Reg<eth_mtlq0icsr::ETH_MTLQ0ICSRrs>;
#[doc = "Queue 0 interrupt control status Register"]
pub mod eth_mtlq0icsr;
#[doc = "ETH_MTLQ1ICSR (rw) register accessor: Queue 1 interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlq1icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlq1icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlq1icsr`]
module"]
pub type ETH_MTLQ1ICSR = crate::Reg<eth_mtlq1icsr::ETH_MTLQ1ICSRrs>;
#[doc = "Queue 1 interrupt control status Register"]
pub mod eth_mtlq1icsr;
#[doc = "ETH_MTLRxQ0OMR (rw) register accessor: Rx queue 0 operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0omr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlrx_q0omr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q0omr`]
module"]
#[doc(alias = "ETH_MTLRxQ0OMR")]
pub type ETH_MTLRX_Q0OMR = crate::Reg<eth_mtlrx_q0omr::ETH_MTLRX_Q0OMRrs>;
#[doc = "Rx queue 0 operating mode register"]
pub mod eth_mtlrx_q0omr;
#[doc = "ETH_MTLRxQ1OMR (rw) register accessor: Rx queue 1 operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q1omr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlrx_q1omr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q1omr`]
module"]
#[doc(alias = "ETH_MTLRxQ1OMR")]
pub type ETH_MTLRX_Q1OMR = crate::Reg<eth_mtlrx_q1omr::ETH_MTLRX_Q1OMRrs>;
#[doc = "Rx queue 1 operating mode register"]
pub mod eth_mtlrx_q1omr;
#[doc = "ETH_MTLRxQ0MPOCR (r) register accessor: Rx queue 0 missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0mpocr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q0mpocr`]
module"]
#[doc(alias = "ETH_MTLRxQ0MPOCR")]
pub type ETH_MTLRX_Q0MPOCR = crate::Reg<eth_mtlrx_q0mpocr::ETH_MTLRX_Q0MPOCRrs>;
#[doc = "Rx queue 0 missed packet and overflow counter register"]
pub mod eth_mtlrx_q0mpocr;
#[doc = "ETH_MTLRxQ1MPOCR (r) register accessor: Rx queue 1 missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q1mpocr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q1mpocr`]
module"]
#[doc(alias = "ETH_MTLRxQ1MPOCR")]
pub type ETH_MTLRX_Q1MPOCR = crate::Reg<eth_mtlrx_q1mpocr::ETH_MTLRX_Q1MPOCRrs>;
#[doc = "Rx queue 1 missed packet and overflow counter register"]
pub mod eth_mtlrx_q1mpocr;
#[doc = "ETH_MTLRxQ0DR (r) register accessor: Rx queue i debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q0dr`]
module"]
#[doc(alias = "ETH_MTLRxQ0DR")]
pub type ETH_MTLRX_Q0DR = crate::Reg<eth_mtlrx_q0dr::ETH_MTLRX_Q0DRrs>;
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q0dr;
#[doc = "ETH_MTLRxQ1DR (r) register accessor: Rx queue i debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q1dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q1dr`]
module"]
#[doc(alias = "ETH_MTLRxQ1DR")]
pub type ETH_MTLRX_Q1DR = crate::Reg<eth_mtlrx_q1dr::ETH_MTLRX_Q1DRrs>;
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q1dr;
#[doc = "ETH_MTLRxQ0CR (rw) register accessor: Rx queue 0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q0cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlrx_q0cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q0cr`]
module"]
#[doc(alias = "ETH_MTLRxQ0CR")]
pub type ETH_MTLRX_Q0CR = crate::Reg<eth_mtlrx_q0cr::ETH_MTLRX_Q0CRrs>;
#[doc = "Rx queue 0 control register"]
pub mod eth_mtlrx_q0cr;
#[doc = "ETH_MTLRxQ1CR (rw) register accessor: Rx queue 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlrx_q1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlrx_q1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtlrx_q1cr`]
module"]
#[doc(alias = "ETH_MTLRxQ1CR")]
pub type ETH_MTLRX_Q1CR = crate::Reg<eth_mtlrx_q1cr::ETH_MTLRX_Q1CRrs>;
#[doc = "Rx queue 1 control register"]
pub mod eth_mtlrx_q1cr;
#[doc = "ETH_MTLTxQ1ECR (rw) register accessor: The Queue ETS Control register controls the enhanced transmission selection operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1ecr`]
module"]
#[doc(alias = "ETH_MTLTxQ1ECR")]
pub type ETH_MTLTX_Q1ECR = crate::Reg<eth_mtltx_q1ecr::ETH_MTLTX_Q1ECRrs>;
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation."]
pub mod eth_mtltx_q1ecr;
#[doc = "ETH_MTLTxQ1QWR (rw) register accessor: This register provides the average traffic transmitted on queue 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1qwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1qwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1qwr`]
module"]
#[doc(alias = "ETH_MTLTxQ1QWR")]
pub type ETH_MTLTX_Q1QWR = crate::Reg<eth_mtltx_q1qwr::ETH_MTLTX_Q1QWRrs>;
#[doc = "This register provides the average traffic transmitted on queue 1."]
pub mod eth_mtltx_q1qwr;
#[doc = "ETH_MTLTxQ1SSCR (rw) register accessor: The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1sscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1sscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1sscr`]
module"]
#[doc(alias = "ETH_MTLTxQ1SSCR")]
pub type ETH_MTLTX_Q1SSCR = crate::Reg<eth_mtltx_q1sscr::ETH_MTLTX_Q1SSCRrs>;
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1sscr;
#[doc = "ETH_MTLTxQ1HCR (rw) register accessor: The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1hcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1hcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1hcr`]
module"]
#[doc(alias = "ETH_MTLTxQ1HCR")]
pub type ETH_MTLTX_Q1HCR = crate::Reg<eth_mtltx_q1hcr::ETH_MTLTX_Q1HCRrs>;
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1hcr;
#[doc = "ETH_MTLTxQ1LCR (rw) register accessor: The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q1lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtltx_q1lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mtltx_q1lcr`]
module"]
#[doc(alias = "ETH_MTLTxQ1LCR")]
pub type ETH_MTLTX_Q1LCR = crate::Reg<eth_mtltx_q1lcr::ETH_MTLTX_Q1LCRrs>;
#[doc = "The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1lcr;
