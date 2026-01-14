#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mtlomr: MTLOMR,
    _reserved1: [u8; 0x1c],
    mtlisr: MTLISR,
    _reserved2: [u8; 0xdc],
    mtltx_q0omr: MTLTX_Q0OMR,
    mtltx_q0ur: MTLTX_Q0UR,
    mtltx_q0dr: MTLTX_Q0DR,
    _reserved5: [u8; 0x08],
    mtltx_q0esr: MTLTX_Q0ESR,
    _reserved6: [u8; 0x14],
    mtlq0icsr: MTLQ0ICSR,
    mtlrx_q0omr: MTLRX_Q0OMR,
    mtlrx_q0mpocr: MTLRX_Q0MPOCR,
    mtlrx_q0dr: MTLRX_Q0DR,
    mtlrx_q0cr: MTLRX_Q0CR,
    mtltx_q1omr: MTLTX_Q1OMR,
    mtltx_q1ur: MTLTX_Q1UR,
    mtltx_q1dr: MTLTX_Q1DR,
    _reserved14: [u8; 0x04],
    mtltx_q1ecr: MTLTX_Q1ECR,
    mtltx_q1esr: MTLTX_Q1ESR,
    mtltx_q1qwr: MTLTX_Q1QWR,
    mtltx_q1sscr: MTLTX_Q1SSCR,
    mtltx_q1hcr: MTLTX_Q1HCR,
    mtltx_q1lcr: MTLTX_Q1LCR,
    _reserved20: [u8; 0x04],
    mtlq1icsr: MTLQ1ICSR,
    mtlrx_q1omr: MTLRX_Q1OMR,
    mtlrx_q1mpocr: MTLRX_Q1MPOCR,
    mtlrx_q1dr: MTLRX_Q1DR,
    mtlrx_q1cr: MTLRX_Q1CR,
}
impl RegisterBlock {
    ///0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands.
    #[inline(always)]
    pub const fn mtlomr(&self) -> &MTLOMR {
        &self.mtlomr
    }
    ///0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
    #[inline(always)]
    pub const fn mtlisr(&self) -> &MTLISR {
        &self.mtlisr
    }
    ///0x100 - Tx queue 0 operating mode Register
    #[inline(always)]
    pub const fn mtltx_q0omr(&self) -> &MTLTX_Q0OMR {
        &self.mtltx_q0omr
    }
    ///0x104 - Tx queue 0 underflow register
    #[inline(always)]
    pub const fn mtltx_q0ur(&self) -> &MTLTX_Q0UR {
        &self.mtltx_q0ur
    }
    ///0x108 - Tx queue 0 underflow register
    #[inline(always)]
    pub const fn mtltx_q0dr(&self) -> &MTLTX_Q0DR {
        &self.mtltx_q0dr
    }
    ///0x114 - Tx queue x ETS status Register
    #[inline(always)]
    pub const fn mtltx_q0esr(&self) -> &MTLTX_Q0ESR {
        &self.mtltx_q0esr
    }
    ///0x12c - Queue 0 interrupt control status Register
    #[inline(always)]
    pub const fn mtlq0icsr(&self) -> &MTLQ0ICSR {
        &self.mtlq0icsr
    }
    ///0x130 - Rx queue 0 operating mode register
    #[inline(always)]
    pub const fn mtlrx_q0omr(&self) -> &MTLRX_Q0OMR {
        &self.mtlrx_q0omr
    }
    ///0x134 - Rx queue 0 missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrx_q0mpocr(&self) -> &MTLRX_Q0MPOCR {
        &self.mtlrx_q0mpocr
    }
    ///0x138 - Rx queue i debug register
    #[inline(always)]
    pub const fn mtlrx_q0dr(&self) -> &MTLRX_Q0DR {
        &self.mtlrx_q0dr
    }
    ///0x13c - Rx queue 0 control register
    #[inline(always)]
    pub const fn mtlrx_q0cr(&self) -> &MTLRX_Q0CR {
        &self.mtlrx_q0cr
    }
    ///0x140 - Tx queue 1 operating mode Register
    #[inline(always)]
    pub const fn mtltx_q1omr(&self) -> &MTLTX_Q1OMR {
        &self.mtltx_q1omr
    }
    ///0x144 - Tx queue 1 underflow register
    #[inline(always)]
    pub const fn mtltx_q1ur(&self) -> &MTLTX_Q1UR {
        &self.mtltx_q1ur
    }
    ///0x148 - Tx queue 1 underflow register
    #[inline(always)]
    pub const fn mtltx_q1dr(&self) -> &MTLTX_Q1DR {
        &self.mtltx_q1dr
    }
    ///0x150 - The Queue ETS Control register controls the enhanced transmission selection operation.
    #[inline(always)]
    pub const fn mtltx_q1ecr(&self) -> &MTLTX_Q1ECR {
        &self.mtltx_q1ecr
    }
    ///0x154 - Tx queue x ETS status Register
    #[inline(always)]
    pub const fn mtltx_q1esr(&self) -> &MTLTX_Q1ESR {
        &self.mtltx_q1esr
    }
    ///0x158 - This register provides the average traffic transmitted on queue 1.
    #[inline(always)]
    pub const fn mtltx_q1qwr(&self) -> &MTLTX_Q1QWR {
        &self.mtltx_q1qwr
    }
    ///0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn mtltx_q1sscr(&self) -> &MTLTX_Q1SSCR {
        &self.mtltx_q1sscr
    }
    ///0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn mtltx_q1hcr(&self) -> &MTLTX_Q1HCR {
        &self.mtltx_q1hcr
    }
    ///0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn mtltx_q1lcr(&self) -> &MTLTX_Q1LCR {
        &self.mtltx_q1lcr
    }
    ///0x16c - Queue 1 interrupt control status Register
    #[inline(always)]
    pub const fn mtlq1icsr(&self) -> &MTLQ1ICSR {
        &self.mtlq1icsr
    }
    ///0x170 - Rx queue 1 operating mode register
    #[inline(always)]
    pub const fn mtlrx_q1omr(&self) -> &MTLRX_Q1OMR {
        &self.mtlrx_q1omr
    }
    ///0x174 - Rx queue 1 missed packet and overflow counter register
    #[inline(always)]
    pub const fn mtlrx_q1mpocr(&self) -> &MTLRX_Q1MPOCR {
        &self.mtlrx_q1mpocr
    }
    ///0x178 - Rx queue i debug register
    #[inline(always)]
    pub const fn mtlrx_q1dr(&self) -> &MTLRX_Q1DR {
        &self.mtlrx_q1dr
    }
    ///0x17c - Rx queue 1 control register
    #[inline(always)]
    pub const fn mtlrx_q1cr(&self) -> &MTLRX_Q1CR {
        &self.mtlrx_q1cr
    }
}
/**MTLOMR (rw) register accessor: The Operating Mode register establishes the Transmit and Receive operating modes and commands.

You can [`read`](crate::Reg::read) this register and get [`mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLOMR)

For information about available fields see [`mod@mtlomr`] module*/
pub type MTLOMR = crate::Reg<mtlomr::MTLOMRrs>;
///The Operating Mode register establishes the Transmit and Receive operating modes and commands.
pub mod mtlomr;
/**MTLISR (r) register accessor: The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLISR)

For information about available fields see [`mod@mtlisr`] module*/
pub type MTLISR = crate::Reg<mtlisr::MTLISRrs>;
///The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
pub mod mtlisr;
/**MTLTxQ0OMR (rw) register accessor: Tx queue 0 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ0OMR)

For information about available fields see [`mod@mtltx_q0omr`] module*/
#[doc(alias = "MTLTxQ0OMR")]
pub type MTLTX_Q0OMR = crate::Reg<mtltx_q0omr::MTLTX_Q0OMRrs>;
///Tx queue 0 operating mode Register
pub mod mtltx_q0omr;
/**MTLTxQ1OMR (rw) register accessor: Tx queue 1 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1OMR)

For information about available fields see [`mod@mtltx_q1omr`] module*/
#[doc(alias = "MTLTxQ1OMR")]
pub type MTLTX_Q1OMR = crate::Reg<mtltx_q1omr::MTLTX_Q1OMRrs>;
///Tx queue 1 operating mode Register
pub mod mtltx_q1omr;
/**MTLTxQ0UR (r) register accessor: Tx queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q0ur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ0UR)

For information about available fields see [`mod@mtltx_q0ur`] module*/
#[doc(alias = "MTLTxQ0UR")]
pub type MTLTX_Q0UR = crate::Reg<mtltx_q0ur::MTLTX_Q0URrs>;
///Tx queue 0 underflow register
pub mod mtltx_q0ur;
/**MTLTxQ1UR (r) register accessor: Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1ur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1UR)

For information about available fields see [`mod@mtltx_q1ur`] module*/
#[doc(alias = "MTLTxQ1UR")]
pub type MTLTX_Q1UR = crate::Reg<mtltx_q1ur::MTLTX_Q1URrs>;
///Tx queue 1 underflow register
pub mod mtltx_q1ur;
/**MTLTxQ0DR (r) register accessor: Tx queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ0DR)

For information about available fields see [`mod@mtltx_q0dr`] module*/
#[doc(alias = "MTLTxQ0DR")]
pub type MTLTX_Q0DR = crate::Reg<mtltx_q0dr::MTLTX_Q0DRrs>;
///Tx queue 0 underflow register
pub mod mtltx_q0dr;
/**MTLTxQ1DR (r) register accessor: Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1DR)

For information about available fields see [`mod@mtltx_q1dr`] module*/
#[doc(alias = "MTLTxQ1DR")]
pub type MTLTX_Q1DR = crate::Reg<mtltx_q1dr::MTLTX_Q1DRrs>;
///Tx queue 1 underflow register
pub mod mtltx_q1dr;
/**MTLTxQ0ESR (r) register accessor: Tx queue x ETS status Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ0ESR)

For information about available fields see [`mod@mtltx_q0esr`] module*/
#[doc(alias = "MTLTxQ0ESR")]
pub type MTLTX_Q0ESR = crate::Reg<mtltx_q0esr::MTLTX_Q0ESRrs>;
///Tx queue x ETS status Register
pub mod mtltx_q0esr;
/**MTLTxQ1ESR (r) register accessor: Tx queue x ETS status Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1ESR)

For information about available fields see [`mod@mtltx_q1esr`] module*/
#[doc(alias = "MTLTxQ1ESR")]
pub type MTLTX_Q1ESR = crate::Reg<mtltx_q1esr::MTLTX_Q1ESRrs>;
///Tx queue x ETS status Register
pub mod mtltx_q1esr;
/**MTLQ0ICSR (rw) register accessor: Queue 0 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq0icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq0icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLQ0ICSR)

For information about available fields see [`mod@mtlq0icsr`] module*/
pub type MTLQ0ICSR = crate::Reg<mtlq0icsr::MTLQ0ICSRrs>;
///Queue 0 interrupt control status Register
pub mod mtlq0icsr;
/**MTLQ1ICSR (rw) register accessor: Queue 1 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq1icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq1icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLQ1ICSR)

For information about available fields see [`mod@mtlq1icsr`] module*/
pub type MTLQ1ICSR = crate::Reg<mtlq1icsr::MTLQ1ICSRrs>;
///Queue 1 interrupt control status Register
pub mod mtlq1icsr;
/**MTLRxQ0OMR (rw) register accessor: Rx queue 0 operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ0OMR)

For information about available fields see [`mod@mtlrx_q0omr`] module*/
#[doc(alias = "MTLRxQ0OMR")]
pub type MTLRX_Q0OMR = crate::Reg<mtlrx_q0omr::MTLRX_Q0OMRrs>;
///Rx queue 0 operating mode register
pub mod mtlrx_q0omr;
/**MTLRxQ1OMR (rw) register accessor: Rx queue 1 operating mode register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ1OMR)

For information about available fields see [`mod@mtlrx_q1omr`] module*/
#[doc(alias = "MTLRxQ1OMR")]
pub type MTLRX_Q1OMR = crate::Reg<mtlrx_q1omr::MTLRX_Q1OMRrs>;
///Rx queue 1 operating mode register
pub mod mtlrx_q1omr;
/**MTLRxQ0MPOCR (r) register accessor: Rx queue 0 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0mpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ0MPOCR)

For information about available fields see [`mod@mtlrx_q0mpocr`] module*/
#[doc(alias = "MTLRxQ0MPOCR")]
pub type MTLRX_Q0MPOCR = crate::Reg<mtlrx_q0mpocr::MTLRX_Q0MPOCRrs>;
///Rx queue 0 missed packet and overflow counter register
pub mod mtlrx_q0mpocr;
/**MTLRxQ1MPOCR (r) register accessor: Rx queue 1 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q1mpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ1MPOCR)

For information about available fields see [`mod@mtlrx_q1mpocr`] module*/
#[doc(alias = "MTLRxQ1MPOCR")]
pub type MTLRX_Q1MPOCR = crate::Reg<mtlrx_q1mpocr::MTLRX_Q1MPOCRrs>;
///Rx queue 1 missed packet and overflow counter register
pub mod mtlrx_q1mpocr;
/**MTLRxQ0DR (r) register accessor: Rx queue i debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ0DR)

For information about available fields see [`mod@mtlrx_q0dr`] module*/
#[doc(alias = "MTLRxQ0DR")]
pub type MTLRX_Q0DR = crate::Reg<mtlrx_q0dr::MTLRX_Q0DRrs>;
///Rx queue i debug register
pub mod mtlrx_q0dr;
/**MTLRxQ1DR (r) register accessor: Rx queue i debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ1DR)

For information about available fields see [`mod@mtlrx_q1dr`] module*/
#[doc(alias = "MTLRxQ1DR")]
pub type MTLRX_Q1DR = crate::Reg<mtlrx_q1dr::MTLRX_Q1DRrs>;
///Rx queue i debug register
pub mod mtlrx_q1dr;
/**MTLRxQ0CR (rw) register accessor: Rx queue 0 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ0CR)

For information about available fields see [`mod@mtlrx_q0cr`] module*/
#[doc(alias = "MTLRxQ0CR")]
pub type MTLRX_Q0CR = crate::Reg<mtlrx_q0cr::MTLRX_Q0CRrs>;
///Rx queue 0 control register
pub mod mtlrx_q0cr;
/**MTLRxQ1CR (rw) register accessor: Rx queue 1 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_q1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_q1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLRxQ1CR)

For information about available fields see [`mod@mtlrx_q1cr`] module*/
#[doc(alias = "MTLRxQ1CR")]
pub type MTLRX_Q1CR = crate::Reg<mtlrx_q1cr::MTLRX_Q1CRrs>;
///Rx queue 1 control register
pub mod mtlrx_q1cr;
/**MTLTxQ1ECR (rw) register accessor: The Queue ETS Control register controls the enhanced transmission selection operation.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1ECR)

For information about available fields see [`mod@mtltx_q1ecr`] module*/
#[doc(alias = "MTLTxQ1ECR")]
pub type MTLTX_Q1ECR = crate::Reg<mtltx_q1ecr::MTLTX_Q1ECRrs>;
///The Queue ETS Control register controls the enhanced transmission selection operation.
pub mod mtltx_q1ecr;
/**MTLTxQ1QWR (rw) register accessor: This register provides the average traffic transmitted on queue 1.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1qwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1qwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1QWR)

For information about available fields see [`mod@mtltx_q1qwr`] module*/
#[doc(alias = "MTLTxQ1QWR")]
pub type MTLTX_Q1QWR = crate::Reg<mtltx_q1qwr::MTLTX_Q1QWRrs>;
///This register provides the average traffic transmitted on queue 1.
pub mod mtltx_q1qwr;
/**MTLTxQ1SSCR (rw) register accessor: The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1SSCR)

For information about available fields see [`mod@mtltx_q1sscr`] module*/
#[doc(alias = "MTLTxQ1SSCR")]
pub type MTLTX_Q1SSCR = crate::Reg<mtltx_q1sscr::MTLTX_Q1SSCRrs>;
///The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
pub mod mtltx_q1sscr;
/**MTLTxQ1HCR (rw) register accessor: The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1HCR)

For information about available fields see [`mod@mtltx_q1hcr`] module*/
#[doc(alias = "MTLTxQ1HCR")]
pub type MTLTX_Q1HCR = crate::Reg<mtltx_q1hcr::MTLTX_Q1HCRrs>;
///The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
pub mod mtltx_q1hcr;
/**MTLTxQ1LCR (rw) register accessor: The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_q1lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1LCR)

For information about available fields see [`mod@mtltx_q1lcr`] module*/
#[doc(alias = "MTLTxQ1LCR")]
pub type MTLTX_Q1LCR = crate::Reg<mtltx_q1lcr::MTLTX_Q1LCRrs>;
///The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
pub mod mtltx_q1lcr;
