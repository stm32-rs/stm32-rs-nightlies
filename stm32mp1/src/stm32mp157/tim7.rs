#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM7 control register 1"]
    pub tim7_cr1: TIM7_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM7 control register 2"]
    pub tim7_cr2: TIM7_CR2,
    #[doc = "0x08 - TIM7 slave mode control register"]
    pub tim7_smcr: TIM7_SMCR,
    #[doc = "0x0c - TIM7 DMA/interrupt enable register"]
    pub tim7_dier: TIM7_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM7 status register"]
    pub tim7_sr: TIM7_SR,
    #[doc = "0x14 - TIM7 event generation register"]
    pub tim7_egr: TIM7_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim7_ccmr1alternate7: TIM7_CCMR1ALTERNATE7,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim7_ccmr2alternate23: TIM7_CCMR2ALTERNATE23,
    #[doc = "0x20 - TIM7 capture/compare enable register"]
    pub tim7_ccer: TIM7_CCER,
    #[doc = "0x24 - TIM7 counter"]
    pub tim7_cnt: TIM7_CNT,
    #[doc = "0x28 - TIM7 prescaler"]
    pub tim7_psc: TIM7_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM7 auto-reload register"]
    pub tim7_arr: TIM7_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM7 repetition counter register"]
    pub tim7_rcr: TIM7_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM7 capture/compare register 1"]
    pub tim7_ccr1: TIM7_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM7 capture/compare register 2"]
    pub tim7_ccr2: TIM7_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM7 capture/compare register 3"]
    pub tim7_ccr3: TIM7_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM7 capture/compare register 4"]
    pub tim7_ccr4: TIM7_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim7_bdtr: TIM7_BDTR,
    #[doc = "0x48 - TIM7 DMA control register"]
    pub tim7_dcr: TIM7_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM7 DMA address for full transfer"]
    pub tim7_dmar: TIM7_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim7_ccmr3: TIM7_CCMR3,
    #[doc = "0x58 - TIM7 capture/compare register 5"]
    pub tim7_ccr5: TIM7_CCR5,
    #[doc = "0x5c - TIM7 capture/compare register 6"]
    pub tim7_ccr6: TIM7_CCR6,
}
#[doc = "TIM7 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_cr1](tim7_cr1) module"]
pub type TIM7_CR1 = crate::Reg<u16, _TIM7_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CR1;
#[doc = "`read()` method returns [tim7_cr1::R](tim7_cr1::R) reader structure"]
impl crate::Readable for TIM7_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim7_cr1::W](tim7_cr1::W) writer structure"]
impl crate::Writable for TIM7_CR1 {}
#[doc = "TIM7 control register 1"]
pub mod tim7_cr1;
#[doc = "TIM7 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_cr2](tim7_cr2) module"]
pub type TIM7_CR2 = crate::Reg<u32, _TIM7_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CR2;
#[doc = "`read()` method returns [tim7_cr2::R](tim7_cr2::R) reader structure"]
impl crate::Readable for TIM7_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim7_cr2::W](tim7_cr2::W) writer structure"]
impl crate::Writable for TIM7_CR2 {}
#[doc = "TIM7 control register 2"]
pub mod tim7_cr2;
#[doc = "TIM7 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_smcr](tim7_smcr) module"]
pub type TIM7_SMCR = crate::Reg<u32, _TIM7_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_SMCR;
#[doc = "`read()` method returns [tim7_smcr::R](tim7_smcr::R) reader structure"]
impl crate::Readable for TIM7_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim7_smcr::W](tim7_smcr::W) writer structure"]
impl crate::Writable for TIM7_SMCR {}
#[doc = "TIM7 slave mode control register"]
pub mod tim7_smcr;
#[doc = "TIM7 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_dier](tim7_dier) module"]
pub type TIM7_DIER = crate::Reg<u16, _TIM7_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_DIER;
#[doc = "`read()` method returns [tim7_dier::R](tim7_dier::R) reader structure"]
impl crate::Readable for TIM7_DIER {}
#[doc = "`write(|w| ..)` method takes [tim7_dier::W](tim7_dier::W) writer structure"]
impl crate::Writable for TIM7_DIER {}
#[doc = "TIM7 DMA/interrupt enable register"]
pub mod tim7_dier;
#[doc = "TIM7 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_sr](tim7_sr) module"]
pub type TIM7_SR = crate::Reg<u32, _TIM7_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_SR;
#[doc = "`read()` method returns [tim7_sr::R](tim7_sr::R) reader structure"]
impl crate::Readable for TIM7_SR {}
#[doc = "`write(|w| ..)` method takes [tim7_sr::W](tim7_sr::W) writer structure"]
impl crate::Writable for TIM7_SR {}
#[doc = "TIM7 status register"]
pub mod tim7_sr;
#[doc = "TIM7 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_egr](tim7_egr) module"]
pub type TIM7_EGR = crate::Reg<u16, _TIM7_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_EGR;
#[doc = "`write(|w| ..)` method takes [tim7_egr::W](tim7_egr::W) writer structure"]
impl crate::Writable for TIM7_EGR {}
#[doc = "TIM7 event generation register"]
pub mod tim7_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccmr1alternate7](tim7_ccmr1alternate7) module"]
pub type TIM7_CCMR1ALTERNATE7 = crate::Reg<u32, _TIM7_CCMR1ALTERNATE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCMR1ALTERNATE7;
#[doc = "`read()` method returns [tim7_ccmr1alternate7::R](tim7_ccmr1alternate7::R) reader structure"]
impl crate::Readable for TIM7_CCMR1ALTERNATE7 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccmr1alternate7::W](tim7_ccmr1alternate7::W) writer structure"]
impl crate::Writable for TIM7_CCMR1ALTERNATE7 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim7_ccmr1alternate7;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccmr2alternate23](tim7_ccmr2alternate23) module"]
pub type TIM7_CCMR2ALTERNATE23 = crate::Reg<u32, _TIM7_CCMR2ALTERNATE23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCMR2ALTERNATE23;
#[doc = "`read()` method returns [tim7_ccmr2alternate23::R](tim7_ccmr2alternate23::R) reader structure"]
impl crate::Readable for TIM7_CCMR2ALTERNATE23 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccmr2alternate23::W](tim7_ccmr2alternate23::W) writer structure"]
impl crate::Writable for TIM7_CCMR2ALTERNATE23 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim7_ccmr2alternate23;
#[doc = "TIM7 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccer](tim7_ccer) module"]
pub type TIM7_CCER = crate::Reg<u32, _TIM7_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCER;
#[doc = "`read()` method returns [tim7_ccer::R](tim7_ccer::R) reader structure"]
impl crate::Readable for TIM7_CCER {}
#[doc = "`write(|w| ..)` method takes [tim7_ccer::W](tim7_ccer::W) writer structure"]
impl crate::Writable for TIM7_CCER {}
#[doc = "TIM7 capture/compare enable register"]
pub mod tim7_ccer;
#[doc = "TIM7 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_cnt](tim7_cnt) module"]
pub type TIM7_CNT = crate::Reg<u32, _TIM7_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CNT;
#[doc = "`read()` method returns [tim7_cnt::R](tim7_cnt::R) reader structure"]
impl crate::Readable for TIM7_CNT {}
#[doc = "`write(|w| ..)` method takes [tim7_cnt::W](tim7_cnt::W) writer structure"]
impl crate::Writable for TIM7_CNT {}
#[doc = "TIM7 counter"]
pub mod tim7_cnt;
#[doc = "TIM7 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_psc](tim7_psc) module"]
pub type TIM7_PSC = crate::Reg<u16, _TIM7_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_PSC;
#[doc = "`read()` method returns [tim7_psc::R](tim7_psc::R) reader structure"]
impl crate::Readable for TIM7_PSC {}
#[doc = "`write(|w| ..)` method takes [tim7_psc::W](tim7_psc::W) writer structure"]
impl crate::Writable for TIM7_PSC {}
#[doc = "TIM7 prescaler"]
pub mod tim7_psc;
#[doc = "TIM7 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_arr](tim7_arr) module"]
pub type TIM7_ARR = crate::Reg<u16, _TIM7_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_ARR;
#[doc = "`read()` method returns [tim7_arr::R](tim7_arr::R) reader structure"]
impl crate::Readable for TIM7_ARR {}
#[doc = "`write(|w| ..)` method takes [tim7_arr::W](tim7_arr::W) writer structure"]
impl crate::Writable for TIM7_ARR {}
#[doc = "TIM7 auto-reload register"]
pub mod tim7_arr;
#[doc = "TIM7 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_rcr](tim7_rcr) module"]
pub type TIM7_RCR = crate::Reg<u16, _TIM7_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_RCR;
#[doc = "`read()` method returns [tim7_rcr::R](tim7_rcr::R) reader structure"]
impl crate::Readable for TIM7_RCR {}
#[doc = "`write(|w| ..)` method takes [tim7_rcr::W](tim7_rcr::W) writer structure"]
impl crate::Writable for TIM7_RCR {}
#[doc = "TIM7 repetition counter register"]
pub mod tim7_rcr;
#[doc = "TIM7 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr1](tim7_ccr1) module"]
pub type TIM7_CCR1 = crate::Reg<u16, _TIM7_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR1;
#[doc = "`read()` method returns [tim7_ccr1::R](tim7_ccr1::R) reader structure"]
impl crate::Readable for TIM7_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr1::W](tim7_ccr1::W) writer structure"]
impl crate::Writable for TIM7_CCR1 {}
#[doc = "TIM7 capture/compare register 1"]
pub mod tim7_ccr1;
#[doc = "TIM7 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr2](tim7_ccr2) module"]
pub type TIM7_CCR2 = crate::Reg<u16, _TIM7_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR2;
#[doc = "`read()` method returns [tim7_ccr2::R](tim7_ccr2::R) reader structure"]
impl crate::Readable for TIM7_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr2::W](tim7_ccr2::W) writer structure"]
impl crate::Writable for TIM7_CCR2 {}
#[doc = "TIM7 capture/compare register 2"]
pub mod tim7_ccr2;
#[doc = "TIM7 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr3](tim7_ccr3) module"]
pub type TIM7_CCR3 = crate::Reg<u16, _TIM7_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR3;
#[doc = "`read()` method returns [tim7_ccr3::R](tim7_ccr3::R) reader structure"]
impl crate::Readable for TIM7_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr3::W](tim7_ccr3::W) writer structure"]
impl crate::Writable for TIM7_CCR3 {}
#[doc = "TIM7 capture/compare register 3"]
pub mod tim7_ccr3;
#[doc = "TIM7 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr4](tim7_ccr4) module"]
pub type TIM7_CCR4 = crate::Reg<u16, _TIM7_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR4;
#[doc = "`read()` method returns [tim7_ccr4::R](tim7_ccr4::R) reader structure"]
impl crate::Readable for TIM7_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr4::W](tim7_ccr4::W) writer structure"]
impl crate::Writable for TIM7_CCR4 {}
#[doc = "TIM7 capture/compare register 4"]
pub mod tim7_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_bdtr](tim7_bdtr) module"]
pub type TIM7_BDTR = crate::Reg<u32, _TIM7_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_BDTR;
#[doc = "`read()` method returns [tim7_bdtr::R](tim7_bdtr::R) reader structure"]
impl crate::Readable for TIM7_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim7_bdtr::W](tim7_bdtr::W) writer structure"]
impl crate::Writable for TIM7_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim7_bdtr;
#[doc = "TIM7 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_dcr](tim7_dcr) module"]
pub type TIM7_DCR = crate::Reg<u16, _TIM7_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_DCR;
#[doc = "`read()` method returns [tim7_dcr::R](tim7_dcr::R) reader structure"]
impl crate::Readable for TIM7_DCR {}
#[doc = "`write(|w| ..)` method takes [tim7_dcr::W](tim7_dcr::W) writer structure"]
impl crate::Writable for TIM7_DCR {}
#[doc = "TIM7 DMA control register"]
pub mod tim7_dcr;
#[doc = "TIM7 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_dmar](tim7_dmar) module"]
pub type TIM7_DMAR = crate::Reg<u32, _TIM7_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_DMAR;
#[doc = "`read()` method returns [tim7_dmar::R](tim7_dmar::R) reader structure"]
impl crate::Readable for TIM7_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim7_dmar::W](tim7_dmar::W) writer structure"]
impl crate::Writable for TIM7_DMAR {}
#[doc = "TIM7 DMA address for full transfer"]
pub mod tim7_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccmr3](tim7_ccmr3) module"]
pub type TIM7_CCMR3 = crate::Reg<u32, _TIM7_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCMR3;
#[doc = "`read()` method returns [tim7_ccmr3::R](tim7_ccmr3::R) reader structure"]
impl crate::Readable for TIM7_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccmr3::W](tim7_ccmr3::W) writer structure"]
impl crate::Writable for TIM7_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim7_ccmr3;
#[doc = "TIM7 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr5](tim7_ccr5) module"]
pub type TIM7_CCR5 = crate::Reg<u32, _TIM7_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR5;
#[doc = "`read()` method returns [tim7_ccr5::R](tim7_ccr5::R) reader structure"]
impl crate::Readable for TIM7_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr5::W](tim7_ccr5::W) writer structure"]
impl crate::Writable for TIM7_CCR5 {}
#[doc = "TIM7 capture/compare register 5"]
pub mod tim7_ccr5;
#[doc = "TIM7 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim7_ccr6](tim7_ccr6) module"]
pub type TIM7_CCR6 = crate::Reg<u16, _TIM7_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM7_CCR6;
#[doc = "`read()` method returns [tim7_ccr6::R](tim7_ccr6::R) reader structure"]
impl crate::Readable for TIM7_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim7_ccr6::W](tim7_ccr6::W) writer structure"]
impl crate::Writable for TIM7_CCR6 {}
#[doc = "TIM7 capture/compare register 6"]
pub mod tim7_ccr6;
