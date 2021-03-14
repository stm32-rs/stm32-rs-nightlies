#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM1 control register 1"]
    pub tim1_cr1: TIM1_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM1 control register 2"]
    pub tim1_cr2: TIM1_CR2,
    #[doc = "0x08 - TIM1 slave mode control register"]
    pub tim1_smcr: TIM1_SMCR,
    #[doc = "0x0c - TIM1 DMA/interrupt enable register"]
    pub tim1_dier: TIM1_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM1 status register"]
    pub tim1_sr: TIM1_SR,
    #[doc = "0x14 - TIM1 event generation register"]
    pub tim1_egr: TIM1_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim1_ccmr1alternate1: TIM1_CCMR1ALTERNATE1,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim1_ccmr2alternate17: TIM1_CCMR2ALTERNATE17,
    #[doc = "0x20 - TIM1 capture/compare enable register"]
    pub tim1_ccer: TIM1_CCER,
    #[doc = "0x24 - TIM1 counter"]
    pub tim1_cnt: TIM1_CNT,
    #[doc = "0x28 - TIM1 prescaler"]
    pub tim1_psc: TIM1_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM1 auto-reload register"]
    pub tim1_arr: TIM1_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM1 repetition counter register"]
    pub tim1_rcr: TIM1_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM1 capture/compare register 1"]
    pub tim1_ccr1: TIM1_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM1 capture/compare register 2"]
    pub tim1_ccr2: TIM1_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM1 capture/compare register 3"]
    pub tim1_ccr3: TIM1_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM1 capture/compare register 4"]
    pub tim1_ccr4: TIM1_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim1_bdtr: TIM1_BDTR,
    #[doc = "0x48 - TIM1 DMA control register"]
    pub tim1_dcr: TIM1_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM1 DMA address for full transfer"]
    pub tim1_dmar: TIM1_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim1_ccmr3: TIM1_CCMR3,
    #[doc = "0x58 - TIM1 capture/compare register 5"]
    pub tim1_ccr5: TIM1_CCR5,
    #[doc = "0x5c - TIM1 capture/compare register 6"]
    pub tim1_ccr6: TIM1_CCR6,
    _reserved23: [u8; 2usize],
    #[doc = "0x60 - TIM1 alternate function option register 1"]
    pub tim1_af1: TIM1_AF1,
    #[doc = "0x64 - TIM1 Alternate function register 2"]
    pub tim1_af2: TIM1_AF2,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub tim1_tisel: TIM1_TISEL,
}
#[doc = "TIM1 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cr1](tim1_cr1) module"]
pub type TIM1_CR1 = crate::Reg<u16, _TIM1_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CR1;
#[doc = "`read()` method returns [tim1_cr1::R](tim1_cr1::R) reader structure"]
impl crate::Readable for TIM1_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim1_cr1::W](tim1_cr1::W) writer structure"]
impl crate::Writable for TIM1_CR1 {}
#[doc = "TIM1 control register 1"]
pub mod tim1_cr1;
#[doc = "TIM1 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cr2](tim1_cr2) module"]
pub type TIM1_CR2 = crate::Reg<u32, _TIM1_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CR2;
#[doc = "`read()` method returns [tim1_cr2::R](tim1_cr2::R) reader structure"]
impl crate::Readable for TIM1_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim1_cr2::W](tim1_cr2::W) writer structure"]
impl crate::Writable for TIM1_CR2 {}
#[doc = "TIM1 control register 2"]
pub mod tim1_cr2;
#[doc = "TIM1 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_smcr](tim1_smcr) module"]
pub type TIM1_SMCR = crate::Reg<u32, _TIM1_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_SMCR;
#[doc = "`read()` method returns [tim1_smcr::R](tim1_smcr::R) reader structure"]
impl crate::Readable for TIM1_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim1_smcr::W](tim1_smcr::W) writer structure"]
impl crate::Writable for TIM1_SMCR {}
#[doc = "TIM1 slave mode control register"]
pub mod tim1_smcr;
#[doc = "TIM1 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dier](tim1_dier) module"]
pub type TIM1_DIER = crate::Reg<u16, _TIM1_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DIER;
#[doc = "`read()` method returns [tim1_dier::R](tim1_dier::R) reader structure"]
impl crate::Readable for TIM1_DIER {}
#[doc = "`write(|w| ..)` method takes [tim1_dier::W](tim1_dier::W) writer structure"]
impl crate::Writable for TIM1_DIER {}
#[doc = "TIM1 DMA/interrupt enable register"]
pub mod tim1_dier;
#[doc = "TIM1 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_sr](tim1_sr) module"]
pub type TIM1_SR = crate::Reg<u32, _TIM1_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_SR;
#[doc = "`read()` method returns [tim1_sr::R](tim1_sr::R) reader structure"]
impl crate::Readable for TIM1_SR {}
#[doc = "`write(|w| ..)` method takes [tim1_sr::W](tim1_sr::W) writer structure"]
impl crate::Writable for TIM1_SR {}
#[doc = "TIM1 status register"]
pub mod tim1_sr;
#[doc = "TIM1 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_egr](tim1_egr) module"]
pub type TIM1_EGR = crate::Reg<u16, _TIM1_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_EGR;
#[doc = "`write(|w| ..)` method takes [tim1_egr::W](tim1_egr::W) writer structure"]
impl crate::Writable for TIM1_EGR {}
#[doc = "TIM1 event generation register"]
pub mod tim1_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccmr1alternate1](tim1_ccmr1alternate1) module"]
pub type TIM1_CCMR1ALTERNATE1 = crate::Reg<u32, _TIM1_CCMR1ALTERNATE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCMR1ALTERNATE1;
#[doc = "`read()` method returns [tim1_ccmr1alternate1::R](tim1_ccmr1alternate1::R) reader structure"]
impl crate::Readable for TIM1_CCMR1ALTERNATE1 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccmr1alternate1::W](tim1_ccmr1alternate1::W) writer structure"]
impl crate::Writable for TIM1_CCMR1ALTERNATE1 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim1_ccmr1alternate1;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccmr2alternate17](tim1_ccmr2alternate17) module"]
pub type TIM1_CCMR2ALTERNATE17 = crate::Reg<u32, _TIM1_CCMR2ALTERNATE17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCMR2ALTERNATE17;
#[doc = "`read()` method returns [tim1_ccmr2alternate17::R](tim1_ccmr2alternate17::R) reader structure"]
impl crate::Readable for TIM1_CCMR2ALTERNATE17 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccmr2alternate17::W](tim1_ccmr2alternate17::W) writer structure"]
impl crate::Writable for TIM1_CCMR2ALTERNATE17 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim1_ccmr2alternate17;
#[doc = "TIM1 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccer](tim1_ccer) module"]
pub type TIM1_CCER = crate::Reg<u32, _TIM1_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCER;
#[doc = "`read()` method returns [tim1_ccer::R](tim1_ccer::R) reader structure"]
impl crate::Readable for TIM1_CCER {}
#[doc = "`write(|w| ..)` method takes [tim1_ccer::W](tim1_ccer::W) writer structure"]
impl crate::Writable for TIM1_CCER {}
#[doc = "TIM1 capture/compare enable register"]
pub mod tim1_ccer;
#[doc = "TIM1 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cnt](tim1_cnt) module"]
pub type TIM1_CNT = crate::Reg<u32, _TIM1_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CNT;
#[doc = "`read()` method returns [tim1_cnt::R](tim1_cnt::R) reader structure"]
impl crate::Readable for TIM1_CNT {}
#[doc = "`write(|w| ..)` method takes [tim1_cnt::W](tim1_cnt::W) writer structure"]
impl crate::Writable for TIM1_CNT {}
#[doc = "TIM1 counter"]
pub mod tim1_cnt;
#[doc = "TIM1 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_psc](tim1_psc) module"]
pub type TIM1_PSC = crate::Reg<u16, _TIM1_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_PSC;
#[doc = "`read()` method returns [tim1_psc::R](tim1_psc::R) reader structure"]
impl crate::Readable for TIM1_PSC {}
#[doc = "`write(|w| ..)` method takes [tim1_psc::W](tim1_psc::W) writer structure"]
impl crate::Writable for TIM1_PSC {}
#[doc = "TIM1 prescaler"]
pub mod tim1_psc;
#[doc = "TIM1 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_arr](tim1_arr) module"]
pub type TIM1_ARR = crate::Reg<u16, _TIM1_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_ARR;
#[doc = "`read()` method returns [tim1_arr::R](tim1_arr::R) reader structure"]
impl crate::Readable for TIM1_ARR {}
#[doc = "`write(|w| ..)` method takes [tim1_arr::W](tim1_arr::W) writer structure"]
impl crate::Writable for TIM1_ARR {}
#[doc = "TIM1 auto-reload register"]
pub mod tim1_arr;
#[doc = "TIM1 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_rcr](tim1_rcr) module"]
pub type TIM1_RCR = crate::Reg<u16, _TIM1_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_RCR;
#[doc = "`read()` method returns [tim1_rcr::R](tim1_rcr::R) reader structure"]
impl crate::Readable for TIM1_RCR {}
#[doc = "`write(|w| ..)` method takes [tim1_rcr::W](tim1_rcr::W) writer structure"]
impl crate::Writable for TIM1_RCR {}
#[doc = "TIM1 repetition counter register"]
pub mod tim1_rcr;
#[doc = "TIM1 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr1](tim1_ccr1) module"]
pub type TIM1_CCR1 = crate::Reg<u16, _TIM1_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR1;
#[doc = "`read()` method returns [tim1_ccr1::R](tim1_ccr1::R) reader structure"]
impl crate::Readable for TIM1_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr1::W](tim1_ccr1::W) writer structure"]
impl crate::Writable for TIM1_CCR1 {}
#[doc = "TIM1 capture/compare register 1"]
pub mod tim1_ccr1;
#[doc = "TIM1 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr2](tim1_ccr2) module"]
pub type TIM1_CCR2 = crate::Reg<u16, _TIM1_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR2;
#[doc = "`read()` method returns [tim1_ccr2::R](tim1_ccr2::R) reader structure"]
impl crate::Readable for TIM1_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr2::W](tim1_ccr2::W) writer structure"]
impl crate::Writable for TIM1_CCR2 {}
#[doc = "TIM1 capture/compare register 2"]
pub mod tim1_ccr2;
#[doc = "TIM1 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr3](tim1_ccr3) module"]
pub type TIM1_CCR3 = crate::Reg<u16, _TIM1_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR3;
#[doc = "`read()` method returns [tim1_ccr3::R](tim1_ccr3::R) reader structure"]
impl crate::Readable for TIM1_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr3::W](tim1_ccr3::W) writer structure"]
impl crate::Writable for TIM1_CCR3 {}
#[doc = "TIM1 capture/compare register 3"]
pub mod tim1_ccr3;
#[doc = "TIM1 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr4](tim1_ccr4) module"]
pub type TIM1_CCR4 = crate::Reg<u16, _TIM1_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR4;
#[doc = "`read()` method returns [tim1_ccr4::R](tim1_ccr4::R) reader structure"]
impl crate::Readable for TIM1_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr4::W](tim1_ccr4::W) writer structure"]
impl crate::Writable for TIM1_CCR4 {}
#[doc = "TIM1 capture/compare register 4"]
pub mod tim1_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_bdtr](tim1_bdtr) module"]
pub type TIM1_BDTR = crate::Reg<u32, _TIM1_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_BDTR;
#[doc = "`read()` method returns [tim1_bdtr::R](tim1_bdtr::R) reader structure"]
impl crate::Readable for TIM1_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim1_bdtr::W](tim1_bdtr::W) writer structure"]
impl crate::Writable for TIM1_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim1_bdtr;
#[doc = "TIM1 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dcr](tim1_dcr) module"]
pub type TIM1_DCR = crate::Reg<u16, _TIM1_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DCR;
#[doc = "`read()` method returns [tim1_dcr::R](tim1_dcr::R) reader structure"]
impl crate::Readable for TIM1_DCR {}
#[doc = "`write(|w| ..)` method takes [tim1_dcr::W](tim1_dcr::W) writer structure"]
impl crate::Writable for TIM1_DCR {}
#[doc = "TIM1 DMA control register"]
pub mod tim1_dcr;
#[doc = "TIM1 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_dmar](tim1_dmar) module"]
pub type TIM1_DMAR = crate::Reg<u32, _TIM1_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_DMAR;
#[doc = "`read()` method returns [tim1_dmar::R](tim1_dmar::R) reader structure"]
impl crate::Readable for TIM1_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim1_dmar::W](tim1_dmar::W) writer structure"]
impl crate::Writable for TIM1_DMAR {}
#[doc = "TIM1 DMA address for full transfer"]
pub mod tim1_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccmr3](tim1_ccmr3) module"]
pub type TIM1_CCMR3 = crate::Reg<u32, _TIM1_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCMR3;
#[doc = "`read()` method returns [tim1_ccmr3::R](tim1_ccmr3::R) reader structure"]
impl crate::Readable for TIM1_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccmr3::W](tim1_ccmr3::W) writer structure"]
impl crate::Writable for TIM1_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim1_ccmr3;
#[doc = "TIM1 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr5](tim1_ccr5) module"]
pub type TIM1_CCR5 = crate::Reg<u32, _TIM1_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR5;
#[doc = "`read()` method returns [tim1_ccr5::R](tim1_ccr5::R) reader structure"]
impl crate::Readable for TIM1_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr5::W](tim1_ccr5::W) writer structure"]
impl crate::Writable for TIM1_CCR5 {}
#[doc = "TIM1 capture/compare register 5"]
pub mod tim1_ccr5;
#[doc = "TIM1 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_ccr6](tim1_ccr6) module"]
pub type TIM1_CCR6 = crate::Reg<u16, _TIM1_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_CCR6;
#[doc = "`read()` method returns [tim1_ccr6::R](tim1_ccr6::R) reader structure"]
impl crate::Readable for TIM1_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim1_ccr6::W](tim1_ccr6::W) writer structure"]
impl crate::Writable for TIM1_CCR6 {}
#[doc = "TIM1 capture/compare register 6"]
pub mod tim1_ccr6;
#[doc = "TIM1 alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af1](tim1_af1) module"]
pub type TIM1_AF1 = crate::Reg<u32, _TIM1_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_AF1;
#[doc = "`read()` method returns [tim1_af1::R](tim1_af1::R) reader structure"]
impl crate::Readable for TIM1_AF1 {}
#[doc = "`write(|w| ..)` method takes [tim1_af1::W](tim1_af1::W) writer structure"]
impl crate::Writable for TIM1_AF1 {}
#[doc = "TIM1 alternate function option register 1"]
pub mod tim1_af1;
#[doc = "TIM1 Alternate function register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_af2](tim1_af2) module"]
pub type TIM1_AF2 = crate::Reg<u32, _TIM1_AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_AF2;
#[doc = "`read()` method returns [tim1_af2::R](tim1_af2::R) reader structure"]
impl crate::Readable for TIM1_AF2 {}
#[doc = "`write(|w| ..)` method takes [tim1_af2::W](tim1_af2::W) writer structure"]
impl crate::Writable for TIM1_AF2 {}
#[doc = "TIM1 Alternate function register 2"]
pub mod tim1_af2;
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_tisel](tim1_tisel) module"]
pub type TIM1_TISEL = crate::Reg<u32, _TIM1_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM1_TISEL;
#[doc = "`read()` method returns [tim1_tisel::R](tim1_tisel::R) reader structure"]
impl crate::Readable for TIM1_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim1_tisel::W](tim1_tisel::W) writer structure"]
impl crate::Writable for TIM1_TISEL {}
#[doc = "TIM1 timer input selection register"]
pub mod tim1_tisel;
