#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
    pub eth_mtlomr: ETH_MTLOMR,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
    pub eth_mtlisr: ETH_MTLISR,
    _reserved2: [u8; 220usize],
    #[doc = "0x100 - Tx queue 0 operating mode Register"]
    pub eth_mtltx_q0omr: ETH_MTLTXQ0OMR,
    #[doc = "0x104 - Tx queue 0 underflow register"]
    pub eth_mtltx_q0ur: ETH_MTLTXQ0UR,
    #[doc = "0x108 - Tx queue 0 underflow register"]
    pub eth_mtltx_q0dr: ETH_MTLTXQ0DR,
    _reserved5: [u8; 8usize],
    #[doc = "0x114 - Tx queue x ETS status Register"]
    pub eth_mtltx_q0esr: ETH_MTLTXQ0ESR,
    _reserved6: [u8; 20usize],
    #[doc = "0x12c - Queue 0 interrupt control status Register"]
    pub eth_mtlq0icsr: ETH_MTLQ0ICSR,
    #[doc = "0x130 - Rx queue 0 operating mode register"]
    pub eth_mtlrx_q0omr: ETH_MTLRXQ0OMR,
    #[doc = "0x134 - Rx queue 0 missed packet and overflow counter register"]
    pub eth_mtlrx_q0mpocr: ETH_MTLRXQ0MPOCR,
    #[doc = "0x138 - Rx queue i debug register"]
    pub eth_mtlrx_q0dr: ETH_MTLRXQ0DR,
    #[doc = "0x13c - Rx queue 0 control register"]
    pub eth_mtlrx_q0cr: ETH_MTLRXQ0CR,
    #[doc = "0x140 - Tx queue 1 operating mode Register"]
    pub eth_mtltx_q1omr: ETH_MTLTXQ1OMR,
    #[doc = "0x144 - Tx queue 1 underflow register"]
    pub eth_mtltx_q1ur: ETH_MTLTXQ1UR,
    #[doc = "0x148 - Tx queue 1 underflow register"]
    pub eth_mtltx_q1dr: ETH_MTLTXQ1DR,
    _reserved14: [u8; 4usize],
    #[doc = "0x150 - The Queue ETS Control register controls the enhanced transmission selection operation."]
    pub eth_mtltx_q1ecr: ETH_MTLTXQ1ECR,
    #[doc = "0x154 - Tx queue x ETS status Register"]
    pub eth_mtltx_q1esr: ETH_MTLTXQ1ESR,
    #[doc = "0x158 - This register provides the average traffic transmitted on queue 1."]
    pub eth_mtltx_q1qwr: ETH_MTLTXQ1QWR,
    #[doc = "0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1sscr: ETH_MTLTXQ1SSCR,
    #[doc = "0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1hcr: ETH_MTLTXQ1HCR,
    #[doc = "0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
    pub eth_mtltx_q1lcr: ETH_MTLTXQ1LCR,
    _reserved20: [u8; 4usize],
    #[doc = "0x16c - Queue 1 interrupt control status Register"]
    pub eth_mtlq1icsr: ETH_MTLQ1ICSR,
    #[doc = "0x170 - Rx queue 1 operating mode register"]
    pub eth_mtlrx_q1omr: ETH_MTLRXQ1OMR,
    #[doc = "0x174 - Rx queue 1 missed packet and overflow counter register"]
    pub eth_mtlrx_q1mpocr: ETH_MTLRXQ1MPOCR,
    #[doc = "0x178 - Rx queue i debug register"]
    pub eth_mtlrx_q1dr: ETH_MTLRXQ1DR,
    #[doc = "0x17c - Rx queue 1 control register"]
    pub eth_mtlrx_q1cr: ETH_MTLRXQ1CR,
}
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlomr](eth_mtlomr) module"]
pub type ETH_MTLOMR = crate::Reg<u32, _ETH_MTLOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLOMR;
#[doc = "`read()` method returns [eth_mtlomr::R](eth_mtlomr::R) reader structure"]
impl crate::Readable for ETH_MTLOMR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlomr::W](eth_mtlomr::W) writer structure"]
impl crate::Writable for ETH_MTLOMR {}
#[doc = "The Operating Mode register establishes the Transmit and Receive operating modes and commands."]
pub mod eth_mtlomr;
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlisr](eth_mtlisr) module"]
pub type ETH_MTLISR = crate::Reg<u32, _ETH_MTLISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLISR;
#[doc = "`read()` method returns [eth_mtlisr::R](eth_mtlisr::R) reader structure"]
impl crate::Readable for ETH_MTLISR {}
#[doc = "The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC."]
pub mod eth_mtlisr;
#[doc = "Tx queue 0 operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0omr](eth_mtltx_q0omr) module"]
pub type ETH_MTLTXQ0OMR = crate::Reg<u32, _ETH_MTLTXQ0OMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ0OMR;
#[doc = "`read()` method returns [eth_mtltx_q0omr::R](eth_mtltx_q0omr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0OMR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q0omr::W](eth_mtltx_q0omr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ0OMR {}
#[doc = "Tx queue 0 operating mode Register"]
pub mod eth_mtltx_q0omr;
#[doc = "Tx queue 1 operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1omr](eth_mtltx_q1omr) module"]
pub type ETH_MTLTXQ1OMR = crate::Reg<u32, _ETH_MTLTXQ1OMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1OMR;
#[doc = "`read()` method returns [eth_mtltx_q1omr::R](eth_mtltx_q1omr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1OMR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1omr::W](eth_mtltx_q1omr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1OMR {}
#[doc = "Tx queue 1 operating mode Register"]
pub mod eth_mtltx_q1omr;
#[doc = "Tx queue 0 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0ur](eth_mtltx_q0ur) module"]
pub type ETH_MTLTXQ0UR = crate::Reg<u32, _ETH_MTLTXQ0UR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ0UR;
#[doc = "`read()` method returns [eth_mtltx_q0ur::R](eth_mtltx_q0ur::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0UR {}
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0ur;
#[doc = "Tx queue 1 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1ur](eth_mtltx_q1ur) module"]
pub type ETH_MTLTXQ1UR = crate::Reg<u32, _ETH_MTLTXQ1UR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1UR;
#[doc = "`read()` method returns [eth_mtltx_q1ur::R](eth_mtltx_q1ur::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1UR {}
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1ur;
#[doc = "Tx queue 0 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0dr](eth_mtltx_q0dr) module"]
pub type ETH_MTLTXQ0DR = crate::Reg<u32, _ETH_MTLTXQ0DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ0DR;
#[doc = "`read()` method returns [eth_mtltx_q0dr::R](eth_mtltx_q0dr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0DR {}
#[doc = "Tx queue 0 underflow register"]
pub mod eth_mtltx_q0dr;
#[doc = "Tx queue 1 underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1dr](eth_mtltx_q1dr) module"]
pub type ETH_MTLTXQ1DR = crate::Reg<u32, _ETH_MTLTXQ1DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1DR;
#[doc = "`read()` method returns [eth_mtltx_q1dr::R](eth_mtltx_q1dr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1DR {}
#[doc = "Tx queue 1 underflow register"]
pub mod eth_mtltx_q1dr;
#[doc = "Tx queue x ETS status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0esr](eth_mtltx_q0esr) module"]
pub type ETH_MTLTXQ0ESR = crate::Reg<u32, _ETH_MTLTXQ0ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ0ESR;
#[doc = "`read()` method returns [eth_mtltx_q0esr::R](eth_mtltx_q0esr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0ESR {}
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q0esr;
#[doc = "Tx queue x ETS status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1esr](eth_mtltx_q1esr) module"]
pub type ETH_MTLTXQ1ESR = crate::Reg<u32, _ETH_MTLTXQ1ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1ESR;
#[doc = "`read()` method returns [eth_mtltx_q1esr::R](eth_mtltx_q1esr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1ESR {}
#[doc = "Tx queue x ETS status Register"]
pub mod eth_mtltx_q1esr;
#[doc = "Queue 0 interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlq0icsr](eth_mtlq0icsr) module"]
pub type ETH_MTLQ0ICSR = crate::Reg<u32, _ETH_MTLQ0ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLQ0ICSR;
#[doc = "`read()` method returns [eth_mtlq0icsr::R](eth_mtlq0icsr::R) reader structure"]
impl crate::Readable for ETH_MTLQ0ICSR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlq0icsr::W](eth_mtlq0icsr::W) writer structure"]
impl crate::Writable for ETH_MTLQ0ICSR {}
#[doc = "Queue 0 interrupt control status Register"]
pub mod eth_mtlq0icsr;
#[doc = "Queue 1 interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlq1icsr](eth_mtlq1icsr) module"]
pub type ETH_MTLQ1ICSR = crate::Reg<u32, _ETH_MTLQ1ICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLQ1ICSR;
#[doc = "`read()` method returns [eth_mtlq1icsr::R](eth_mtlq1icsr::R) reader structure"]
impl crate::Readable for ETH_MTLQ1ICSR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlq1icsr::W](eth_mtlq1icsr::W) writer structure"]
impl crate::Writable for ETH_MTLQ1ICSR {}
#[doc = "Queue 1 interrupt control status Register"]
pub mod eth_mtlq1icsr;
#[doc = "Rx queue 0 operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0omr](eth_mtlrx_q0omr) module"]
pub type ETH_MTLRXQ0OMR = crate::Reg<u32, _ETH_MTLRXQ0OMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ0OMR;
#[doc = "`read()` method returns [eth_mtlrx_q0omr::R](eth_mtlrx_q0omr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0OMR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q0omr::W](eth_mtlrx_q0omr::W) writer structure"]
impl crate::Writable for ETH_MTLRXQ0OMR {}
#[doc = "Rx queue 0 operating mode register"]
pub mod eth_mtlrx_q0omr;
#[doc = "Rx queue 1 operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1omr](eth_mtlrx_q1omr) module"]
pub type ETH_MTLRXQ1OMR = crate::Reg<u32, _ETH_MTLRXQ1OMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ1OMR;
#[doc = "`read()` method returns [eth_mtlrx_q1omr::R](eth_mtlrx_q1omr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ1OMR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q1omr::W](eth_mtlrx_q1omr::W) writer structure"]
impl crate::Writable for ETH_MTLRXQ1OMR {}
#[doc = "Rx queue 1 operating mode register"]
pub mod eth_mtlrx_q1omr;
#[doc = "Rx queue 0 missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0mpocr](eth_mtlrx_q0mpocr) module"]
pub type ETH_MTLRXQ0MPOCR = crate::Reg<u32, _ETH_MTLRXQ0MPOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ0MPOCR;
#[doc = "`read()` method returns [eth_mtlrx_q0mpocr::R](eth_mtlrx_q0mpocr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0MPOCR {}
#[doc = "Rx queue 0 missed packet and overflow counter register"]
pub mod eth_mtlrx_q0mpocr;
#[doc = "Rx queue 1 missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1mpocr](eth_mtlrx_q1mpocr) module"]
pub type ETH_MTLRXQ1MPOCR = crate::Reg<u32, _ETH_MTLRXQ1MPOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ1MPOCR;
#[doc = "`read()` method returns [eth_mtlrx_q1mpocr::R](eth_mtlrx_q1mpocr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ1MPOCR {}
#[doc = "Rx queue 1 missed packet and overflow counter register"]
pub mod eth_mtlrx_q1mpocr;
#[doc = "Rx queue i debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0dr](eth_mtlrx_q0dr) module"]
pub type ETH_MTLRXQ0DR = crate::Reg<u32, _ETH_MTLRXQ0DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ0DR;
#[doc = "`read()` method returns [eth_mtlrx_q0dr::R](eth_mtlrx_q0dr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0DR {}
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q0dr;
#[doc = "Rx queue i debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1dr](eth_mtlrx_q1dr) module"]
pub type ETH_MTLRXQ1DR = crate::Reg<u32, _ETH_MTLRXQ1DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ1DR;
#[doc = "`read()` method returns [eth_mtlrx_q1dr::R](eth_mtlrx_q1dr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ1DR {}
#[doc = "Rx queue i debug register"]
pub mod eth_mtlrx_q1dr;
#[doc = "Rx queue 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q0cr](eth_mtlrx_q0cr) module"]
pub type ETH_MTLRXQ0CR = crate::Reg<u32, _ETH_MTLRXQ0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ0CR;
#[doc = "`read()` method returns [eth_mtlrx_q0cr::R](eth_mtlrx_q0cr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ0CR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q0cr::W](eth_mtlrx_q0cr::W) writer structure"]
impl crate::Writable for ETH_MTLRXQ0CR {}
#[doc = "Rx queue 0 control register"]
pub mod eth_mtlrx_q0cr;
#[doc = "Rx queue 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1cr](eth_mtlrx_q1cr) module"]
pub type ETH_MTLRXQ1CR = crate::Reg<u32, _ETH_MTLRXQ1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLRXQ1CR;
#[doc = "`read()` method returns [eth_mtlrx_q1cr::R](eth_mtlrx_q1cr::R) reader structure"]
impl crate::Readable for ETH_MTLRXQ1CR {}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q1cr::W](eth_mtlrx_q1cr::W) writer structure"]
impl crate::Writable for ETH_MTLRXQ1CR {}
#[doc = "Rx queue 1 control register"]
pub mod eth_mtlrx_q1cr;
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1ecr](eth_mtltx_q1ecr) module"]
pub type ETH_MTLTXQ1ECR = crate::Reg<u32, _ETH_MTLTXQ1ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1ECR;
#[doc = "`read()` method returns [eth_mtltx_q1ecr::R](eth_mtltx_q1ecr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1ECR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1ecr::W](eth_mtltx_q1ecr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1ECR {}
#[doc = "The Queue ETS Control register controls the enhanced transmission selection operation."]
pub mod eth_mtltx_q1ecr;
#[doc = "This register provides the average traffic transmitted on queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1qwr](eth_mtltx_q1qwr) module"]
pub type ETH_MTLTXQ1QWR = crate::Reg<u32, _ETH_MTLTXQ1QWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1QWR;
#[doc = "`read()` method returns [eth_mtltx_q1qwr::R](eth_mtltx_q1qwr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1QWR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1qwr::W](eth_mtltx_q1qwr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1QWR {}
#[doc = "This register provides the average traffic transmitted on queue 1."]
pub mod eth_mtltx_q1qwr;
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1sscr](eth_mtltx_q1sscr) module"]
pub type ETH_MTLTXQ1SSCR = crate::Reg<u32, _ETH_MTLTXQ1SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1SSCR;
#[doc = "`read()` method returns [eth_mtltx_q1sscr::R](eth_mtltx_q1sscr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1SSCR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1sscr::W](eth_mtltx_q1sscr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1SSCR {}
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1sscr;
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1hcr](eth_mtltx_q1hcr) module"]
pub type ETH_MTLTXQ1HCR = crate::Reg<u32, _ETH_MTLTXQ1HCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1HCR;
#[doc = "`read()` method returns [eth_mtltx_q1hcr::R](eth_mtltx_q1hcr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1HCR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1hcr::W](eth_mtltx_q1hcr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1HCR {}
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1hcr;
#[doc = "The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1lcr](eth_mtltx_q1lcr) module"]
pub type ETH_MTLTXQ1LCR = crate::Reg<u32, _ETH_MTLTXQ1LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETH_MTLTXQ1LCR;
#[doc = "`read()` method returns [eth_mtltx_q1lcr::R](eth_mtltx_q1lcr::R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1LCR {}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1lcr::W](eth_mtltx_q1lcr::W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1LCR {}
#[doc = "The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue."]
pub mod eth_mtltx_q1lcr;
