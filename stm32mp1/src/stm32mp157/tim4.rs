#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM4 control register 1"]
    pub tim4_cr1: TIM4_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM4 control register 2"]
    pub tim4_cr2: TIM4_CR2,
    #[doc = "0x08 - TIM4 slave mode control register"]
    pub tim4_smcr: TIM4_SMCR,
    #[doc = "0x0c - TIM4 DMA/interrupt enable register"]
    pub tim4_dier: TIM4_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM4 status register"]
    pub tim4_sr: TIM4_SR,
    #[doc = "0x14 - TIM4 event generation register"]
    pub tim4_egr: TIM4_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim4_ccmr1alternate4: TIM4_CCMR1ALTERNATE4,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim4_ccmr2alternate20: TIM4_CCMR2ALTERNATE20,
    #[doc = "0x20 - TIM4 capture/compare enable register"]
    pub tim4_ccer: TIM4_CCER,
    #[doc = "0x24 - TIM4 counter"]
    pub tim4_cnt: TIM4_CNT,
    #[doc = "0x28 - TIM4 prescaler"]
    pub tim4_psc: TIM4_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM4 auto-reload register"]
    pub tim4_arr: TIM4_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM4 repetition counter register"]
    pub tim4_rcr: TIM4_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM4 capture/compare register 1"]
    pub tim4_ccr1: TIM4_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM4 capture/compare register 2"]
    pub tim4_ccr2: TIM4_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM4 capture/compare register 3"]
    pub tim4_ccr3: TIM4_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM4 capture/compare register 4"]
    pub tim4_ccr4: TIM4_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim4_bdtr: TIM4_BDTR,
    #[doc = "0x48 - TIM4 DMA control register"]
    pub tim4_dcr: TIM4_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM4 DMA address for full transfer"]
    pub tim4_dmar: TIM4_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim4_ccmr3: TIM4_CCMR3,
    #[doc = "0x58 - TIM4 capture/compare register 5"]
    pub tim4_ccr5: TIM4_CCR5,
    #[doc = "0x5c - TIM4 capture/compare register 6"]
    pub tim4_ccr6: TIM4_CCR6,
}
#[doc = "TIM4 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_cr1](tim4_cr1) module"]
pub type TIM4_CR1 = crate::Reg<u16, _TIM4_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CR1;
#[doc = "`read()` method returns [tim4_cr1::R](tim4_cr1::R) reader structure"]
impl crate::Readable for TIM4_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim4_cr1::W](tim4_cr1::W) writer structure"]
impl crate::Writable for TIM4_CR1 {}
#[doc = "TIM4 control register 1"]
pub mod tim4_cr1;
#[doc = "TIM4 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_cr2](tim4_cr2) module"]
pub type TIM4_CR2 = crate::Reg<u32, _TIM4_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CR2;
#[doc = "`read()` method returns [tim4_cr2::R](tim4_cr2::R) reader structure"]
impl crate::Readable for TIM4_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim4_cr2::W](tim4_cr2::W) writer structure"]
impl crate::Writable for TIM4_CR2 {}
#[doc = "TIM4 control register 2"]
pub mod tim4_cr2;
#[doc = "TIM4 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_smcr](tim4_smcr) module"]
pub type TIM4_SMCR = crate::Reg<u32, _TIM4_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_SMCR;
#[doc = "`read()` method returns [tim4_smcr::R](tim4_smcr::R) reader structure"]
impl crate::Readable for TIM4_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim4_smcr::W](tim4_smcr::W) writer structure"]
impl crate::Writable for TIM4_SMCR {}
#[doc = "TIM4 slave mode control register"]
pub mod tim4_smcr;
#[doc = "TIM4 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_dier](tim4_dier) module"]
pub type TIM4_DIER = crate::Reg<u16, _TIM4_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_DIER;
#[doc = "`read()` method returns [tim4_dier::R](tim4_dier::R) reader structure"]
impl crate::Readable for TIM4_DIER {}
#[doc = "`write(|w| ..)` method takes [tim4_dier::W](tim4_dier::W) writer structure"]
impl crate::Writable for TIM4_DIER {}
#[doc = "TIM4 DMA/interrupt enable register"]
pub mod tim4_dier;
#[doc = "TIM4 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_sr](tim4_sr) module"]
pub type TIM4_SR = crate::Reg<u32, _TIM4_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_SR;
#[doc = "`read()` method returns [tim4_sr::R](tim4_sr::R) reader structure"]
impl crate::Readable for TIM4_SR {}
#[doc = "`write(|w| ..)` method takes [tim4_sr::W](tim4_sr::W) writer structure"]
impl crate::Writable for TIM4_SR {}
#[doc = "TIM4 status register"]
pub mod tim4_sr;
#[doc = "TIM4 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_egr](tim4_egr) module"]
pub type TIM4_EGR = crate::Reg<u16, _TIM4_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_EGR;
#[doc = "`write(|w| ..)` method takes [tim4_egr::W](tim4_egr::W) writer structure"]
impl crate::Writable for TIM4_EGR {}
#[doc = "TIM4 event generation register"]
pub mod tim4_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccmr1alternate4](tim4_ccmr1alternate4) module"]
pub type TIM4_CCMR1ALTERNATE4 = crate::Reg<u32, _TIM4_CCMR1ALTERNATE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCMR1ALTERNATE4;
#[doc = "`read()` method returns [tim4_ccmr1alternate4::R](tim4_ccmr1alternate4::R) reader structure"]
impl crate::Readable for TIM4_CCMR1ALTERNATE4 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccmr1alternate4::W](tim4_ccmr1alternate4::W) writer structure"]
impl crate::Writable for TIM4_CCMR1ALTERNATE4 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr1alternate4;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccmr2alternate20](tim4_ccmr2alternate20) module"]
pub type TIM4_CCMR2ALTERNATE20 = crate::Reg<u32, _TIM4_CCMR2ALTERNATE20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCMR2ALTERNATE20;
#[doc = "`read()` method returns [tim4_ccmr2alternate20::R](tim4_ccmr2alternate20::R) reader structure"]
impl crate::Readable for TIM4_CCMR2ALTERNATE20 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccmr2alternate20::W](tim4_ccmr2alternate20::W) writer structure"]
impl crate::Writable for TIM4_CCMR2ALTERNATE20 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim4_ccmr2alternate20;
#[doc = "TIM4 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccer](tim4_ccer) module"]
pub type TIM4_CCER = crate::Reg<u32, _TIM4_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCER;
#[doc = "`read()` method returns [tim4_ccer::R](tim4_ccer::R) reader structure"]
impl crate::Readable for TIM4_CCER {}
#[doc = "`write(|w| ..)` method takes [tim4_ccer::W](tim4_ccer::W) writer structure"]
impl crate::Writable for TIM4_CCER {}
#[doc = "TIM4 capture/compare enable register"]
pub mod tim4_ccer;
#[doc = "TIM4 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_cnt](tim4_cnt) module"]
pub type TIM4_CNT = crate::Reg<u32, _TIM4_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CNT;
#[doc = "`read()` method returns [tim4_cnt::R](tim4_cnt::R) reader structure"]
impl crate::Readable for TIM4_CNT {}
#[doc = "`write(|w| ..)` method takes [tim4_cnt::W](tim4_cnt::W) writer structure"]
impl crate::Writable for TIM4_CNT {}
#[doc = "TIM4 counter"]
pub mod tim4_cnt;
#[doc = "TIM4 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_psc](tim4_psc) module"]
pub type TIM4_PSC = crate::Reg<u16, _TIM4_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_PSC;
#[doc = "`read()` method returns [tim4_psc::R](tim4_psc::R) reader structure"]
impl crate::Readable for TIM4_PSC {}
#[doc = "`write(|w| ..)` method takes [tim4_psc::W](tim4_psc::W) writer structure"]
impl crate::Writable for TIM4_PSC {}
#[doc = "TIM4 prescaler"]
pub mod tim4_psc;
#[doc = "TIM4 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_arr](tim4_arr) module"]
pub type TIM4_ARR = crate::Reg<u16, _TIM4_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_ARR;
#[doc = "`read()` method returns [tim4_arr::R](tim4_arr::R) reader structure"]
impl crate::Readable for TIM4_ARR {}
#[doc = "`write(|w| ..)` method takes [tim4_arr::W](tim4_arr::W) writer structure"]
impl crate::Writable for TIM4_ARR {}
#[doc = "TIM4 auto-reload register"]
pub mod tim4_arr;
#[doc = "TIM4 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_rcr](tim4_rcr) module"]
pub type TIM4_RCR = crate::Reg<u16, _TIM4_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_RCR;
#[doc = "`read()` method returns [tim4_rcr::R](tim4_rcr::R) reader structure"]
impl crate::Readable for TIM4_RCR {}
#[doc = "`write(|w| ..)` method takes [tim4_rcr::W](tim4_rcr::W) writer structure"]
impl crate::Writable for TIM4_RCR {}
#[doc = "TIM4 repetition counter register"]
pub mod tim4_rcr;
#[doc = "TIM4 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr1](tim4_ccr1) module"]
pub type TIM4_CCR1 = crate::Reg<u16, _TIM4_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR1;
#[doc = "`read()` method returns [tim4_ccr1::R](tim4_ccr1::R) reader structure"]
impl crate::Readable for TIM4_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr1::W](tim4_ccr1::W) writer structure"]
impl crate::Writable for TIM4_CCR1 {}
#[doc = "TIM4 capture/compare register 1"]
pub mod tim4_ccr1;
#[doc = "TIM4 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr2](tim4_ccr2) module"]
pub type TIM4_CCR2 = crate::Reg<u16, _TIM4_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR2;
#[doc = "`read()` method returns [tim4_ccr2::R](tim4_ccr2::R) reader structure"]
impl crate::Readable for TIM4_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr2::W](tim4_ccr2::W) writer structure"]
impl crate::Writable for TIM4_CCR2 {}
#[doc = "TIM4 capture/compare register 2"]
pub mod tim4_ccr2;
#[doc = "TIM4 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr3](tim4_ccr3) module"]
pub type TIM4_CCR3 = crate::Reg<u16, _TIM4_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR3;
#[doc = "`read()` method returns [tim4_ccr3::R](tim4_ccr3::R) reader structure"]
impl crate::Readable for TIM4_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr3::W](tim4_ccr3::W) writer structure"]
impl crate::Writable for TIM4_CCR3 {}
#[doc = "TIM4 capture/compare register 3"]
pub mod tim4_ccr3;
#[doc = "TIM4 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr4](tim4_ccr4) module"]
pub type TIM4_CCR4 = crate::Reg<u16, _TIM4_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR4;
#[doc = "`read()` method returns [tim4_ccr4::R](tim4_ccr4::R) reader structure"]
impl crate::Readable for TIM4_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr4::W](tim4_ccr4::W) writer structure"]
impl crate::Writable for TIM4_CCR4 {}
#[doc = "TIM4 capture/compare register 4"]
pub mod tim4_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_bdtr](tim4_bdtr) module"]
pub type TIM4_BDTR = crate::Reg<u32, _TIM4_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_BDTR;
#[doc = "`read()` method returns [tim4_bdtr::R](tim4_bdtr::R) reader structure"]
impl crate::Readable for TIM4_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim4_bdtr::W](tim4_bdtr::W) writer structure"]
impl crate::Writable for TIM4_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim4_bdtr;
#[doc = "TIM4 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_dcr](tim4_dcr) module"]
pub type TIM4_DCR = crate::Reg<u16, _TIM4_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_DCR;
#[doc = "`read()` method returns [tim4_dcr::R](tim4_dcr::R) reader structure"]
impl crate::Readable for TIM4_DCR {}
#[doc = "`write(|w| ..)` method takes [tim4_dcr::W](tim4_dcr::W) writer structure"]
impl crate::Writable for TIM4_DCR {}
#[doc = "TIM4 DMA control register"]
pub mod tim4_dcr;
#[doc = "TIM4 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_dmar](tim4_dmar) module"]
pub type TIM4_DMAR = crate::Reg<u32, _TIM4_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_DMAR;
#[doc = "`read()` method returns [tim4_dmar::R](tim4_dmar::R) reader structure"]
impl crate::Readable for TIM4_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim4_dmar::W](tim4_dmar::W) writer structure"]
impl crate::Writable for TIM4_DMAR {}
#[doc = "TIM4 DMA address for full transfer"]
pub mod tim4_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccmr3](tim4_ccmr3) module"]
pub type TIM4_CCMR3 = crate::Reg<u32, _TIM4_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCMR3;
#[doc = "`read()` method returns [tim4_ccmr3::R](tim4_ccmr3::R) reader structure"]
impl crate::Readable for TIM4_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccmr3::W](tim4_ccmr3::W) writer structure"]
impl crate::Writable for TIM4_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim4_ccmr3;
#[doc = "TIM4 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr5](tim4_ccr5) module"]
pub type TIM4_CCR5 = crate::Reg<u32, _TIM4_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR5;
#[doc = "`read()` method returns [tim4_ccr5::R](tim4_ccr5::R) reader structure"]
impl crate::Readable for TIM4_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr5::W](tim4_ccr5::W) writer structure"]
impl crate::Writable for TIM4_CCR5 {}
#[doc = "TIM4 capture/compare register 5"]
pub mod tim4_ccr5;
#[doc = "TIM4 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr6](tim4_ccr6) module"]
pub type TIM4_CCR6 = crate::Reg<u16, _TIM4_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM4_CCR6;
#[doc = "`read()` method returns [tim4_ccr6::R](tim4_ccr6::R) reader structure"]
impl crate::Readable for TIM4_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim4_ccr6::W](tim4_ccr6::W) writer structure"]
impl crate::Writable for TIM4_CCR6 {}
#[doc = "TIM4 capture/compare register 6"]
pub mod tim4_ccr6;
