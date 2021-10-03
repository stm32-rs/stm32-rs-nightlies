#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
    pub eth_mtlomr: crate::Reg<eth_mtlomr::ETH_MTLOMR_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
    pub eth_mtlisr: crate::Reg<eth_mtlisr::ETH_MTLISR_SPEC>,
    _reserved2: [u8; 0xdc],
    #[doc = "0x100 - Tx queue 0 operating mode Register"]
    pub eth_mtltx_q0omr: crate::Reg<eth_mtltx_q0omr::ETH_MTLTXQ0OMR_SPEC>,
    #[doc = "0x104 - Tx queue 0 underflow register"]
    pub eth_mtltx_q0ur: crate::Reg<eth_mtltx_q0ur::ETH_MTLTXQ0UR_SPEC>,
    #[doc = "0x108 - Tx queue 0 underflow register"]
    pub eth_mtltx_q0dr: crate::Reg<eth_mtltx_q0dr::ETH_MTLTXQ0DR_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x114 - Tx queue x ETS status Register"]
    pub eth_mtltx_q0esr: crate::Reg<eth_mtltx_q0esr::ETH_MTLTXQ0ESR_SPEC>,
    _reserved6: [u8; 0x14],
    #[doc = "0x12c - Queue 0 interrupt control status Register"]
    pub eth_mtlq0icsr: crate::Reg<eth_mtlq0icsr::ETH_MTLQ0ICSR_SPEC>,
    #[doc = "0x130 - Rx queue 0 operating mode register"]
    pub eth_mtlrx_q0omr: crate::Reg<eth_mtlrx_q0omr::ETH_MTLRXQ0OMR_SPEC>,
    #[doc = "0x134 - Rx queue 0 missed packet and overflow counter register"]
    pub eth_mtlrx_q0mpocr: crate::Reg<eth_mtlrx_q0mpocr::ETH_MTLRXQ0MPOCR_SPEC>,
    #[doc = "0x138 - Rx queue i debug register"]
    pub eth_mtlrx_q0dr: crate::Reg<eth_mtlrx_q0dr::ETH_MTLRXQ0DR_SPEC>,
    #[doc = "0x13c - Rx queue 0 control register"]
    pub eth_mtlrx_q0cr: crate::Reg<eth_mtlrx_q0cr::ETH_MTLRXQ0CR_SPEC>,
    #[doc = "0x140 - Tx queue 1 operating mode Register"]
    pub eth_mtltx_q1omr: crate::Reg<eth_mtltx_q1omr::ETH_MTLTXQ1OMR_SPEC>,
    #[doc = "0x144 - Tx queue 1 underflow register"]
    pub eth_mtltx_q1ur: crate::Reg<eth_mtltx_q1ur::ETH_MTLTXQ1UR_SPEC>,
    #[doc = "0x148 - Tx queue 1 underflow register"]
    pub eth_mtltx_q1dr: crate::Reg<eth_mtltx_q1dr::ETH_MTLTXQ1DR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x150 - The Queue ETS Control register controls the enhanced transmission selection operation."]
    pub eth_mtltx_q1ecr: crate::Reg<eth_mtltx_q1ecr::ETH_MTLTXQ1ECR_SPEC>,
    #[doc = "0x154 - Tx queue x ETS status Register"]
    pub eth_mtltx_q1esr: crate::Reg<eth_mtltx_q1esr::ETH_MTLTXQ1ESR_SPEC>,
    #[doc = "0x158 - This register provides the average traffic transmitted on queue 1."]
    pub eth_mtltx_q1qwr: crate::Reg<eth_mtltx_q1qwr::ETH_MTLTXQ1QWR_SPEC>,
    #[doc = "0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1sscr: crate::Reg<eth_mtltx_q1sscr::ETH_MTLTXQ1SSCR_SPEC>,
    #[doc = "0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1hcr: crate::Reg<eth_mtltx_q1hcr::ETH_MTLTXQ1HCR_SPEC>,
    #[doc = "0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1lcr: crate::Reg<eth_mtltx_q1lcr::ETH_MTLTXQ1LCR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x16c - Queue 1 interrupt control status Register"]
    pub eth_mtlq1icsr: crate::Reg<eth_mtlq1icsr::ETH_MTLQ1ICSR_SPEC>,
    #[doc = "0x170 - Rx queue 1 operating mode register"]
    pub eth_mtlrx_q1omr: crate::Reg<eth_mtlrx_q1omr::ETH_MTLRXQ1OMR_SPEC>,
    #[doc = "0x174 - Rx queue 1 missed packet and overflow counter register"]
    pub eth_mtlrx_q1mpocr: crate::Reg<eth_mtlrx_q1mpocr::ETH_MTLRXQ1MPOCR_SPEC>,
    #[doc = "0x178 - Rx queue i debug register"]
    pub eth_mtlrx_q1dr: crate::Reg<eth_mtlrx_q1dr::ETH_MTLRXQ1DR_SPEC>,
    #[doc = "0x17c - Rx queue 1 control register"]
    pub eth_mtlrx_q1cr: crate::Reg<eth_mtlrx_q1cr::ETH_MTLRXQ1CR_SPEC>,
}
#[doc = "ETH_MTLOMR register accessor: an alias for `Reg<ETH_MTLOMR_SPEC>`"]
pub type ETH_MTLOMR = crate::Reg<eth_mtlomr::ETH_MTLOMR_SPEC>;
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
pub mod eth_mtlomr;
#[doc = "ETH_MTLISR register accessor: an alias for `Reg<ETH_MTLISR_SPEC>`"]
pub type ETH_MTLISR = crate::Reg<eth_mtlisr::ETH_MTLISR_SPEC>;
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
pub mod eth_mtlisr;
#[doc = "ETH_MTLTxQ0OMR register accessor: an alias for `Reg<ETH_MTLTXQ0OMR_SPEC>`"]
pub type ETH_MTLTXQ0OMR = crate::Reg<eth_mtltx_q0omr::ETH_MTLTXQ0OMR_SPEC>;
#[doc = "Tx queue 0 operating mode Register"]
pub mod eth_mtltx_q0omr;
#[doc = "ETH_MTLTxQ1OMR register accessor: an alias for `Reg<ETH_MTLTXQ1OMR_SPEC>`"]
pub type ETH_MTLTXQ1OMR = crate::Reg<eth_mtltx_q1omr::ETH_MTLTXQ1OMR_SPEC>;
#[doc = "Tx queue 1 operating mode Register"]
pub mod eth_mtltx_q1omr;
#[doc = "ETH_MTLTxQ0UR register accessor: an alias for `Reg<ETH_MTLTXQ0UR_SPEC>`"]
pub type ETH_MTLTXQ0UR = crate::Reg<eth_mtltx_q0ur::ETH_MTLTXQ0UR_SPEC>;
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0ur;
#[doc = "ETH_MTLTxQ1UR register accessor: an alias for `Reg<ETH_MTLTXQ1UR_SPEC>`"]
pub type ETH_MTLTXQ1UR = crate::Reg<eth_mtltx_q1ur::ETH_MTLTXQ1UR_SPEC>;
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1ur;
#[doc = "ETH_MTLTxQ0DR register accessor: an alias for `Reg<ETH_MTLTXQ0DR_SPEC>`"]
pub type ETH_MTLTXQ0DR = crate::Reg<eth_mtltx_q0dr::ETH_MTLTXQ0DR_SPEC>;
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0dr;
#[doc = "ETH_MTLTxQ1DR register accessor: an alias for `Reg<ETH_MTLTXQ1DR_SPEC>`"]
pub type ETH_MTLTXQ1DR = crate::Reg<eth_mtltx_q1dr::ETH_MTLTXQ1DR_SPEC>;
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1dr;
#[doc = "ETH_MTLTxQ0ESR register accessor: an alias for `Reg<ETH_MTLTXQ0ESR_SPEC>`"]
pub type ETH_MTLTXQ0ESR = crate::Reg<eth_mtltx_q0esr::ETH_MTLTXQ0ESR_SPEC>;
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q0esr;
#[doc = "ETH_MTLTxQ1ESR register accessor: an alias for `Reg<ETH_MTLTXQ1ESR_SPEC>`"]
pub type ETH_MTLTXQ1ESR = crate::Reg<eth_mtltx_q1esr::ETH_MTLTXQ1ESR_SPEC>;
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q1esr;
#[doc = "ETH_MTLQ0ICSR register accessor: an alias for `Reg<ETH_MTLQ0ICSR_SPEC>`"]
pub type ETH_MTLQ0ICSR = crate::Reg<eth_mtlq0icsr::ETH_MTLQ0ICSR_SPEC>;
#[doc = "Queue 0 interrupt control status Register"]
pub mod eth_mtlq0icsr;
#[doc = "ETH_MTLQ1ICSR register accessor: an alias for `Reg<ETH_MTLQ1ICSR_SPEC>`"]
pub type ETH_MTLQ1ICSR = crate::Reg<eth_mtlq1icsr::ETH_MTLQ1ICSR_SPEC>;
#[doc = "Queue 1 interrupt control status Register"]
pub mod eth_mtlq1icsr;
#[doc = "ETH_MTLRxQ0OMR register accessor: an alias for `Reg<ETH_MTLRXQ0OMR_SPEC>`"]
pub type ETH_MTLRXQ0OMR = crate::Reg<eth_mtlrx_q0omr::ETH_MTLRXQ0OMR_SPEC>;
#[doc = "Rx queue 0 operating mode register"]
pub mod eth_mtlrx_q0omr;
#[doc = "ETH_MTLRxQ1OMR register accessor: an alias for `Reg<ETH_MTLRXQ1OMR_SPEC>`"]
pub type ETH_MTLRXQ1OMR = crate::Reg<eth_mtlrx_q1omr::ETH_MTLRXQ1OMR_SPEC>;
#[doc = "Rx queue 1 operating mode register"]
pub mod eth_mtlrx_q1omr;
#[doc = "ETH_MTLRxQ0MPOCR register accessor: an alias for `Reg<ETH_MTLRXQ0MPOCR_SPEC>`"]
pub type ETH_MTLRXQ0MPOCR = crate::Reg<eth_mtlrx_q0mpocr::ETH_MTLRXQ0MPOCR_SPEC>;
#[doc = "Rx queue 0 missed packet and overflow counter register"]
pub mod eth_mtlrx_q0mpocr;
#[doc = "ETH_MTLRxQ1MPOCR register accessor: an alias for `Reg<ETH_MTLRXQ1MPOCR_SPEC>`"]
pub type ETH_MTLRXQ1MPOCR = crate::Reg<eth_mtlrx_q1mpocr::ETH_MTLRXQ1MPOCR_SPEC>;
#[doc = "Rx queue 1 missed packet and overflow counter register"]
pub mod eth_mtlrx_q1mpocr;
#[doc = "ETH_MTLRxQ0DR register accessor: an alias for `Reg<ETH_MTLRXQ0DR_SPEC>`"]
pub type ETH_MTLRXQ0DR = crate::Reg<eth_mtlrx_q0dr::ETH_MTLRXQ0DR_SPEC>;
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q0dr;
#[doc = "ETH_MTLRxQ1DR register accessor: an alias for `Reg<ETH_MTLRXQ1DR_SPEC>`"]
pub type ETH_MTLRXQ1DR = crate::Reg<eth_mtlrx_q1dr::ETH_MTLRXQ1DR_SPEC>;
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q1dr;
#[doc = "ETH_MTLRxQ0CR register accessor: an alias for `Reg<ETH_MTLRXQ0CR_SPEC>`"]
pub type ETH_MTLRXQ0CR = crate::Reg<eth_mtlrx_q0cr::ETH_MTLRXQ0CR_SPEC>;
#[doc = "Rx queue 0 control register"]
pub mod eth_mtlrx_q0cr;
#[doc = "ETH_MTLRxQ1CR register accessor: an alias for `Reg<ETH_MTLRXQ1CR_SPEC>`"]
pub type ETH_MTLRXQ1CR = crate::Reg<eth_mtlrx_q1cr::ETH_MTLRXQ1CR_SPEC>;
#[doc = "Rx queue 1 control register"]
pub mod eth_mtlrx_q1cr;
#[doc = "ETH_MTLTxQ1ECR register accessor: an alias for `Reg<ETH_MTLTXQ1ECR_SPEC>`"]
pub type ETH_MTLTXQ1ECR = crate::Reg<eth_mtltx_q1ecr::ETH_MTLTXQ1ECR_SPEC>;
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation."]
pub mod eth_mtltx_q1ecr;
#[doc = "ETH_MTLTxQ1QWR register accessor: an alias for `Reg<ETH_MTLTXQ1QWR_SPEC>`"]
pub type ETH_MTLTXQ1QWR = crate::Reg<eth_mtltx_q1qwr::ETH_MTLTXQ1QWR_SPEC>;
#[doc = "This register provides the average traffic transmitted on queue 1."]
pub mod eth_mtltx_q1qwr;
#[doc = "ETH_MTLTxQ1SSCR register accessor: an alias for `Reg<ETH_MTLTXQ1SSCR_SPEC>`"]
pub type ETH_MTLTXQ1SSCR = crate::Reg<eth_mtltx_q1sscr::ETH_MTLTXQ1SSCR_SPEC>;
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1sscr;
#[doc = "ETH_MTLTxQ1HCR register accessor: an alias for `Reg<ETH_MTLTXQ1HCR_SPEC>`"]
pub type ETH_MTLTXQ1HCR = crate::Reg<eth_mtltx_q1hcr::ETH_MTLTXQ1HCR_SPEC>;
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1hcr;
#[doc = "ETH_MTLTxQ1LCR register accessor: an alias for `Reg<ETH_MTLTXQ1LCR_SPEC>`"]
pub type ETH_MTLTXQ1LCR = crate::Reg<eth_mtltx_q1lcr::ETH_MTLTXQ1LCR_SPEC>;
#[doc = "The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1lcr;
