#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands.
    #[inline(always)]
    pub const fn eth_mtlomr(&self) -> &ETH_MTLOMR {
        &self.eth_mtlomr
    }
    ///0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
    #[inline(always)]
    pub const fn eth_mtlisr(&self) -> &ETH_MTLISR {
        &self.eth_mtlisr
    }
    ///0x100 - Tx queue 0 operating mode Register
    #[inline(always)]
    pub const fn eth_mtltx_q0omr(&self) -> &ETH_MTLTX_Q0OMR {
        &self.eth_mtltx_q0omr
    }
    ///0x104 - Tx queue 0 underflow register
    #[inline(always)]
    pub const fn eth_mtltx_q0ur(&self) -> &ETH_MTLTX_Q0UR {
        &self.eth_mtltx_q0ur
    }
    ///0x108 - Tx queue 0 underflow register
    #[inline(always)]
    pub const fn eth_mtltx_q0dr(&self) -> &ETH_MTLTX_Q0DR {
        &self.eth_mtltx_q0dr
    }
    ///0x114 - Tx queue x ETS status Register
    #[inline(always)]
    pub const fn eth_mtltx_q0esr(&self) -> &ETH_MTLTX_Q0ESR {
        &self.eth_mtltx_q0esr
    }
    ///0x12c - Queue 0 interrupt control status Register
    #[inline(always)]
    pub const fn eth_mtlq0icsr(&self) -> &ETH_MTLQ0ICSR {
        &self.eth_mtlq0icsr
    }
    ///0x130 - Rx queue 0 operating mode register
    #[inline(always)]
    pub const fn eth_mtlrx_q0omr(&self) -> &ETH_MTLRX_Q0OMR {
        &self.eth_mtlrx_q0omr
    }
    ///0x134 - Rx queue 0 missed packet and overflow counter register
    #[inline(always)]
    pub const fn eth_mtlrx_q0mpocr(&self) -> &ETH_MTLRX_Q0MPOCR {
        &self.eth_mtlrx_q0mpocr
    }
    ///0x138 - Rx queue i debug register
    #[inline(always)]
    pub const fn eth_mtlrx_q0dr(&self) -> &ETH_MTLRX_Q0DR {
        &self.eth_mtlrx_q0dr
    }
    ///0x13c - Rx queue 0 control register
    #[inline(always)]
    pub const fn eth_mtlrx_q0cr(&self) -> &ETH_MTLRX_Q0CR {
        &self.eth_mtlrx_q0cr
    }
    ///0x140 - Tx queue 1 operating mode Register
    #[inline(always)]
    pub const fn eth_mtltx_q1omr(&self) -> &ETH_MTLTX_Q1OMR {
        &self.eth_mtltx_q1omr
    }
    ///0x144 - Tx queue 1 underflow register
    #[inline(always)]
    pub const fn eth_mtltx_q1ur(&self) -> &ETH_MTLTX_Q1UR {
        &self.eth_mtltx_q1ur
    }
    ///0x148 - Tx queue 1 underflow register
    #[inline(always)]
    pub const fn eth_mtltx_q1dr(&self) -> &ETH_MTLTX_Q1DR {
        &self.eth_mtltx_q1dr
    }
    ///0x150 - The Queue ETS Control register controls the enhanced transmission selection operation.
    #[inline(always)]
    pub const fn eth_mtltx_q1ecr(&self) -> &ETH_MTLTX_Q1ECR {
        &self.eth_mtltx_q1ecr
    }
    ///0x154 - Tx queue x ETS status Register
    #[inline(always)]
    pub const fn eth_mtltx_q1esr(&self) -> &ETH_MTLTX_Q1ESR {
        &self.eth_mtltx_q1esr
    }
    ///0x158 - This register provides the average traffic transmitted on queue 1.
    #[inline(always)]
    pub const fn eth_mtltx_q1qwr(&self) -> &ETH_MTLTX_Q1QWR {
        &self.eth_mtltx_q1qwr
    }
    ///0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn eth_mtltx_q1sscr(&self) -> &ETH_MTLTX_Q1SSCR {
        &self.eth_mtltx_q1sscr
    }
    ///0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn eth_mtltx_q1hcr(&self) -> &ETH_MTLTX_Q1HCR {
        &self.eth_mtltx_q1hcr
    }
    ///0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
    #[inline(always)]
    pub const fn eth_mtltx_q1lcr(&self) -> &ETH_MTLTX_Q1LCR {
        &self.eth_mtltx_q1lcr
    }
    ///0x16c - Queue 1 interrupt control status Register
    #[inline(always)]
    pub const fn eth_mtlq1icsr(&self) -> &ETH_MTLQ1ICSR {
        &self.eth_mtlq1icsr
    }
    ///0x170 - Rx queue 1 operating mode register
    #[inline(always)]
    pub const fn eth_mtlrx_q1omr(&self) -> &ETH_MTLRX_Q1OMR {
        &self.eth_mtlrx_q1omr
    }
    ///0x174 - Rx queue 1 missed packet and overflow counter register
    #[inline(always)]
    pub const fn eth_mtlrx_q1mpocr(&self) -> &ETH_MTLRX_Q1MPOCR {
        &self.eth_mtlrx_q1mpocr
    }
    ///0x178 - Rx queue i debug register
    #[inline(always)]
    pub const fn eth_mtlrx_q1dr(&self) -> &ETH_MTLRX_Q1DR {
        &self.eth_mtlrx_q1dr
    }
    ///0x17c - Rx queue 1 control register
    #[inline(always)]
    pub const fn eth_mtlrx_q1cr(&self) -> &ETH_MTLRX_Q1CR {
        &self.eth_mtlrx_q1cr
    }
}
/**ETH_MTLOMR (rw) register accessor: The Operating Mode register establishes the Transmit and Receive operating modes and commands.

You can [`read`](crate::Reg::read) this register and get [`eth_mtlomr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlomr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLOMR)

For information about available fields see [`mod@eth_mtlomr`]
module*/
pub type ETH_MTLOMR = crate::Reg<eth_mtlomr::ETH_MTLOMRrs>;
///The Operating Mode register establishes the Transmit and Receive operating modes and commands.
pub mod eth_mtlomr;
/**ETH_MTLISR (r) register accessor: The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.

You can [`read`](crate::Reg::read) this register and get [`eth_mtlisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLISR)

For information about available fields see [`mod@eth_mtlisr`]
module*/
pub type ETH_MTLISR = crate::Reg<eth_mtlisr::ETH_MTLISRrs>;
///The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
pub mod eth_mtlisr;
/**ETH_MTLTxQ0OMR (rw) register accessor: Tx queue 0 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ0OMR)

For information about available fields see [`mod@eth_mtltx_q0omr`]
module*/
#[doc(alias = "ETH_MTLTxQ0OMR")]
pub type ETH_MTLTX_Q0OMR = crate::Reg<eth_mtltx_q0omr::ETH_MTLTX_Q0OMRrs>;
///Tx queue 0 operating mode Register
pub mod eth_mtltx_q0omr;
/**ETH_MTLTxQ1OMR (rw) register accessor: Tx queue 1 operating mode Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1OMR)

For information about available fields see [`mod@eth_mtltx_q1omr`]
module*/
#[doc(alias = "ETH_MTLTxQ1OMR")]
pub type ETH_MTLTX_Q1OMR = crate::Reg<eth_mtltx_q1omr::ETH_MTLTX_Q1OMRrs>;
///Tx queue 1 operating mode Register
pub mod eth_mtltx_q1omr;
/**ETH_MTLTxQ0UR (r) register accessor: Tx queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q0ur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ0UR)

For information about available fields see [`mod@eth_mtltx_q0ur`]
module*/
#[doc(alias = "ETH_MTLTxQ0UR")]
pub type ETH_MTLTX_Q0UR = crate::Reg<eth_mtltx_q0ur::ETH_MTLTX_Q0URrs>;
///Tx queue 0 underflow register
pub mod eth_mtltx_q0ur;
/**ETH_MTLTxQ1UR (r) register accessor: Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1ur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1UR)

For information about available fields see [`mod@eth_mtltx_q1ur`]
module*/
#[doc(alias = "ETH_MTLTxQ1UR")]
pub type ETH_MTLTX_Q1UR = crate::Reg<eth_mtltx_q1ur::ETH_MTLTX_Q1URrs>;
///Tx queue 1 underflow register
pub mod eth_mtltx_q1ur;
/**ETH_MTLTxQ0DR (r) register accessor: Tx queue 0 underflow register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ0DR)

For information about available fields see [`mod@eth_mtltx_q0dr`]
module*/
#[doc(alias = "ETH_MTLTxQ0DR")]
pub type ETH_MTLTX_Q0DR = crate::Reg<eth_mtltx_q0dr::ETH_MTLTX_Q0DRrs>;
///Tx queue 0 underflow register
pub mod eth_mtltx_q0dr;
/**ETH_MTLTxQ1DR (r) register accessor: Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1DR)

For information about available fields see [`mod@eth_mtltx_q1dr`]
module*/
#[doc(alias = "ETH_MTLTxQ1DR")]
pub type ETH_MTLTX_Q1DR = crate::Reg<eth_mtltx_q1dr::ETH_MTLTX_Q1DRrs>;
///Tx queue 1 underflow register
pub mod eth_mtltx_q1dr;
/**ETH_MTLTxQ0ESR (r) register accessor: Tx queue x ETS status Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ0ESR)

For information about available fields see [`mod@eth_mtltx_q0esr`]
module*/
#[doc(alias = "ETH_MTLTxQ0ESR")]
pub type ETH_MTLTX_Q0ESR = crate::Reg<eth_mtltx_q0esr::ETH_MTLTX_Q0ESRrs>;
///Tx queue x ETS status Register
pub mod eth_mtltx_q0esr;
/**ETH_MTLTxQ1ESR (r) register accessor: Tx queue x ETS status Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1ESR)

For information about available fields see [`mod@eth_mtltx_q1esr`]
module*/
#[doc(alias = "ETH_MTLTxQ1ESR")]
pub type ETH_MTLTX_Q1ESR = crate::Reg<eth_mtltx_q1esr::ETH_MTLTX_Q1ESRrs>;
///Tx queue x ETS status Register
pub mod eth_mtltx_q1esr;
/**ETH_MTLQ0ICSR (rw) register accessor: Queue 0 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlq0icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlq0icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLQ0ICSR)

For information about available fields see [`mod@eth_mtlq0icsr`]
module*/
pub type ETH_MTLQ0ICSR = crate::Reg<eth_mtlq0icsr::ETH_MTLQ0ICSRrs>;
///Queue 0 interrupt control status Register
pub mod eth_mtlq0icsr;
/**ETH_MTLQ1ICSR (rw) register accessor: Queue 1 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlq1icsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlq1icsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLQ1ICSR)

For information about available fields see [`mod@eth_mtlq1icsr`]
module*/
pub type ETH_MTLQ1ICSR = crate::Reg<eth_mtlq1icsr::ETH_MTLQ1ICSRrs>;
///Queue 1 interrupt control status Register
pub mod eth_mtlq1icsr;
/**ETH_MTLRxQ0OMR (rw) register accessor: Rx queue 0 operating mode register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q0omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlrx_q0omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ0OMR)

For information about available fields see [`mod@eth_mtlrx_q0omr`]
module*/
#[doc(alias = "ETH_MTLRxQ0OMR")]
pub type ETH_MTLRX_Q0OMR = crate::Reg<eth_mtlrx_q0omr::ETH_MTLRX_Q0OMRrs>;
///Rx queue 0 operating mode register
pub mod eth_mtlrx_q0omr;
/**ETH_MTLRxQ1OMR (rw) register accessor: Rx queue 1 operating mode register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q1omr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlrx_q1omr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ1OMR)

For information about available fields see [`mod@eth_mtlrx_q1omr`]
module*/
#[doc(alias = "ETH_MTLRxQ1OMR")]
pub type ETH_MTLRX_Q1OMR = crate::Reg<eth_mtlrx_q1omr::ETH_MTLRX_Q1OMRrs>;
///Rx queue 1 operating mode register
pub mod eth_mtlrx_q1omr;
/**ETH_MTLRxQ0MPOCR (r) register accessor: Rx queue 0 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q0mpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ0MPOCR)

For information about available fields see [`mod@eth_mtlrx_q0mpocr`]
module*/
#[doc(alias = "ETH_MTLRxQ0MPOCR")]
pub type ETH_MTLRX_Q0MPOCR = crate::Reg<eth_mtlrx_q0mpocr::ETH_MTLRX_Q0MPOCRrs>;
///Rx queue 0 missed packet and overflow counter register
pub mod eth_mtlrx_q0mpocr;
/**ETH_MTLRxQ1MPOCR (r) register accessor: Rx queue 1 missed packet and overflow counter register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q1mpocr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ1MPOCR)

For information about available fields see [`mod@eth_mtlrx_q1mpocr`]
module*/
#[doc(alias = "ETH_MTLRxQ1MPOCR")]
pub type ETH_MTLRX_Q1MPOCR = crate::Reg<eth_mtlrx_q1mpocr::ETH_MTLRX_Q1MPOCRrs>;
///Rx queue 1 missed packet and overflow counter register
pub mod eth_mtlrx_q1mpocr;
/**ETH_MTLRxQ0DR (r) register accessor: Rx queue i debug register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ0DR)

For information about available fields see [`mod@eth_mtlrx_q0dr`]
module*/
#[doc(alias = "ETH_MTLRxQ0DR")]
pub type ETH_MTLRX_Q0DR = crate::Reg<eth_mtlrx_q0dr::ETH_MTLRX_Q0DRrs>;
///Rx queue i debug register
pub mod eth_mtlrx_q0dr;
/**ETH_MTLRxQ1DR (r) register accessor: Rx queue i debug register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ1DR)

For information about available fields see [`mod@eth_mtlrx_q1dr`]
module*/
#[doc(alias = "ETH_MTLRxQ1DR")]
pub type ETH_MTLRX_Q1DR = crate::Reg<eth_mtlrx_q1dr::ETH_MTLRX_Q1DRrs>;
///Rx queue i debug register
pub mod eth_mtlrx_q1dr;
/**ETH_MTLRxQ0CR (rw) register accessor: Rx queue 0 control register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlrx_q0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ0CR)

For information about available fields see [`mod@eth_mtlrx_q0cr`]
module*/
#[doc(alias = "ETH_MTLRxQ0CR")]
pub type ETH_MTLRX_Q0CR = crate::Reg<eth_mtlrx_q0cr::ETH_MTLRX_Q0CRrs>;
///Rx queue 0 control register
pub mod eth_mtlrx_q0cr;
/**ETH_MTLRxQ1CR (rw) register accessor: Rx queue 1 control register

You can [`read`](crate::Reg::read) this register and get [`eth_mtlrx_q1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtlrx_q1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLRxQ1CR)

For information about available fields see [`mod@eth_mtlrx_q1cr`]
module*/
#[doc(alias = "ETH_MTLRxQ1CR")]
pub type ETH_MTLRX_Q1CR = crate::Reg<eth_mtlrx_q1cr::ETH_MTLRX_Q1CRrs>;
///Rx queue 1 control register
pub mod eth_mtlrx_q1cr;
/**ETH_MTLTxQ1ECR (rw) register accessor: The Queue ETS Control register controls the enhanced transmission selection operation.

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1ECR)

For information about available fields see [`mod@eth_mtltx_q1ecr`]
module*/
#[doc(alias = "ETH_MTLTxQ1ECR")]
pub type ETH_MTLTX_Q1ECR = crate::Reg<eth_mtltx_q1ecr::ETH_MTLTX_Q1ECRrs>;
///The Queue ETS Control register controls the enhanced transmission selection operation.
pub mod eth_mtltx_q1ecr;
/**ETH_MTLTxQ1QWR (rw) register accessor: This register provides the average traffic transmitted on queue 1.

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1qwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1qwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1QWR)

For information about available fields see [`mod@eth_mtltx_q1qwr`]
module*/
#[doc(alias = "ETH_MTLTxQ1QWR")]
pub type ETH_MTLTX_Q1QWR = crate::Reg<eth_mtltx_q1qwr::ETH_MTLTX_Q1QWRrs>;
///This register provides the average traffic transmitted on queue 1.
pub mod eth_mtltx_q1qwr;
/**ETH_MTLTxQ1SSCR (rw) register accessor: The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1SSCR)

For information about available fields see [`mod@eth_mtltx_q1sscr`]
module*/
#[doc(alias = "ETH_MTLTxQ1SSCR")]
pub type ETH_MTLTX_Q1SSCR = crate::Reg<eth_mtltx_q1sscr::ETH_MTLTX_Q1SSCRrs>;
///The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1sscr;
/**ETH_MTLTxQ1HCR (rw) register accessor: The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1HCR)

For information about available fields see [`mod@eth_mtltx_q1hcr`]
module*/
#[doc(alias = "ETH_MTLTxQ1HCR")]
pub type ETH_MTLTX_Q1HCR = crate::Reg<eth_mtltx_q1hcr::ETH_MTLTX_Q1HCRrs>;
///The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1hcr;
/**ETH_MTLTxQ1LCR (rw) register accessor: The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mtltx_q1lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:ETH_MTLTxQ1LCR)

For information about available fields see [`mod@eth_mtltx_q1lcr`]
module*/
#[doc(alias = "ETH_MTLTxQ1LCR")]
pub type ETH_MTLTX_Q1LCR = crate::Reg<eth_mtltx_q1lcr::ETH_MTLTX_Q1LCRrs>;
///The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1lcr;
