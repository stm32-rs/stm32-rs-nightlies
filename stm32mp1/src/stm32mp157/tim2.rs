#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM2 control register 1"]
    pub tim2_cr1: TIM2_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM2 control register 2"]
    pub tim2_cr2: TIM2_CR2,
    #[doc = "0x08 - TIM2 slave mode control register"]
    pub tim2_smcr: TIM2_SMCR,
    #[doc = "0x0c - TIM2 DMA/interrupt enable register"]
    pub tim2_dier: TIM2_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM2 status register"]
    pub tim2_sr: TIM2_SR,
    #[doc = "0x14 - TIM2 event generation register"]
    pub tim2_egr: TIM2_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim2_ccmr1alternate2: TIM2_CCMR1ALTERNATE2,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim2_ccmr2alternate18: TIM2_CCMR2ALTERNATE18,
    #[doc = "0x20 - TIM2 capture/compare enable register"]
    pub tim2_ccer: TIM2_CCER,
    #[doc = "0x24 - TIM2 counter"]
    pub tim2_cnt: TIM2_CNT,
    #[doc = "0x28 - TIM2 prescaler"]
    pub tim2_psc: TIM2_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM2 auto-reload register"]
    pub tim2_arr: TIM2_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM2 repetition counter register"]
    pub tim2_rcr: TIM2_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM2 capture/compare register 1"]
    pub tim2_ccr1: TIM2_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM2 capture/compare register 2"]
    pub tim2_ccr2: TIM2_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM2 capture/compare register 3"]
    pub tim2_ccr3: TIM2_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM2 capture/compare register 4"]
    pub tim2_ccr4: TIM2_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim2_bdtr: TIM2_BDTR,
    #[doc = "0x48 - TIM2 DMA control register"]
    pub tim2_dcr: TIM2_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM2 DMA address for full transfer"]
    pub tim2_dmar: TIM2_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim2_ccmr3: TIM2_CCMR3,
    #[doc = "0x58 - TIM2 capture/compare register 5"]
    pub tim2_ccr5: TIM2_CCR5,
    #[doc = "0x5c - TIM2 capture/compare register 6"]
    pub tim2_ccr6: TIM2_CCR6,
}
#[doc = "TIM2 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_cr1](tim2_cr1) module"]
pub type TIM2_CR1 = crate::Reg<u16, _TIM2_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CR1;
#[doc = "`read()` method returns [tim2_cr1::R](tim2_cr1::R) reader structure"]
impl crate::Readable for TIM2_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim2_cr1::W](tim2_cr1::W) writer structure"]
impl crate::Writable for TIM2_CR1 {}
#[doc = "TIM2 control register 1"]
pub mod tim2_cr1;
#[doc = "TIM2 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_cr2](tim2_cr2) module"]
pub type TIM2_CR2 = crate::Reg<u32, _TIM2_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CR2;
#[doc = "`read()` method returns [tim2_cr2::R](tim2_cr2::R) reader structure"]
impl crate::Readable for TIM2_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim2_cr2::W](tim2_cr2::W) writer structure"]
impl crate::Writable for TIM2_CR2 {}
#[doc = "TIM2 control register 2"]
pub mod tim2_cr2;
#[doc = "TIM2 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_smcr](tim2_smcr) module"]
pub type TIM2_SMCR = crate::Reg<u32, _TIM2_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_SMCR;
#[doc = "`read()` method returns [tim2_smcr::R](tim2_smcr::R) reader structure"]
impl crate::Readable for TIM2_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim2_smcr::W](tim2_smcr::W) writer structure"]
impl crate::Writable for TIM2_SMCR {}
#[doc = "TIM2 slave mode control register"]
pub mod tim2_smcr;
#[doc = "TIM2 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_dier](tim2_dier) module"]
pub type TIM2_DIER = crate::Reg<u16, _TIM2_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_DIER;
#[doc = "`read()` method returns [tim2_dier::R](tim2_dier::R) reader structure"]
impl crate::Readable for TIM2_DIER {}
#[doc = "`write(|w| ..)` method takes [tim2_dier::W](tim2_dier::W) writer structure"]
impl crate::Writable for TIM2_DIER {}
#[doc = "TIM2 DMA/interrupt enable register"]
pub mod tim2_dier;
#[doc = "TIM2 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_sr](tim2_sr) module"]
pub type TIM2_SR = crate::Reg<u32, _TIM2_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_SR;
#[doc = "`read()` method returns [tim2_sr::R](tim2_sr::R) reader structure"]
impl crate::Readable for TIM2_SR {}
#[doc = "`write(|w| ..)` method takes [tim2_sr::W](tim2_sr::W) writer structure"]
impl crate::Writable for TIM2_SR {}
#[doc = "TIM2 status register"]
pub mod tim2_sr;
#[doc = "TIM2 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_egr](tim2_egr) module"]
pub type TIM2_EGR = crate::Reg<u16, _TIM2_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_EGR;
#[doc = "`write(|w| ..)` method takes [tim2_egr::W](tim2_egr::W) writer structure"]
impl crate::Writable for TIM2_EGR {}
#[doc = "TIM2 event generation register"]
pub mod tim2_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccmr1alternate2](tim2_ccmr1alternate2) module"]
pub type TIM2_CCMR1ALTERNATE2 = crate::Reg<u32, _TIM2_CCMR1ALTERNATE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCMR1ALTERNATE2;
#[doc = "`read()` method returns [tim2_ccmr1alternate2::R](tim2_ccmr1alternate2::R) reader structure"]
impl crate::Readable for TIM2_CCMR1ALTERNATE2 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccmr1alternate2::W](tim2_ccmr1alternate2::W) writer structure"]
impl crate::Writable for TIM2_CCMR1ALTERNATE2 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim2_ccmr1alternate2;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccmr2alternate18](tim2_ccmr2alternate18) module"]
pub type TIM2_CCMR2ALTERNATE18 = crate::Reg<u32, _TIM2_CCMR2ALTERNATE18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCMR2ALTERNATE18;
#[doc = "`read()` method returns [tim2_ccmr2alternate18::R](tim2_ccmr2alternate18::R) reader structure"]
impl crate::Readable for TIM2_CCMR2ALTERNATE18 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccmr2alternate18::W](tim2_ccmr2alternate18::W) writer structure"]
impl crate::Writable for TIM2_CCMR2ALTERNATE18 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim2_ccmr2alternate18;
#[doc = "TIM2 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccer](tim2_ccer) module"]
pub type TIM2_CCER = crate::Reg<u32, _TIM2_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCER;
#[doc = "`read()` method returns [tim2_ccer::R](tim2_ccer::R) reader structure"]
impl crate::Readable for TIM2_CCER {}
#[doc = "`write(|w| ..)` method takes [tim2_ccer::W](tim2_ccer::W) writer structure"]
impl crate::Writable for TIM2_CCER {}
#[doc = "TIM2 capture/compare enable register"]
pub mod tim2_ccer;
#[doc = "TIM2 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_cnt](tim2_cnt) module"]
pub type TIM2_CNT = crate::Reg<u32, _TIM2_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CNT;
#[doc = "`read()` method returns [tim2_cnt::R](tim2_cnt::R) reader structure"]
impl crate::Readable for TIM2_CNT {}
#[doc = "`write(|w| ..)` method takes [tim2_cnt::W](tim2_cnt::W) writer structure"]
impl crate::Writable for TIM2_CNT {}
#[doc = "TIM2 counter"]
pub mod tim2_cnt;
#[doc = "TIM2 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_psc](tim2_psc) module"]
pub type TIM2_PSC = crate::Reg<u16, _TIM2_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_PSC;
#[doc = "`read()` method returns [tim2_psc::R](tim2_psc::R) reader structure"]
impl crate::Readable for TIM2_PSC {}
#[doc = "`write(|w| ..)` method takes [tim2_psc::W](tim2_psc::W) writer structure"]
impl crate::Writable for TIM2_PSC {}
#[doc = "TIM2 prescaler"]
pub mod tim2_psc;
#[doc = "TIM2 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_arr](tim2_arr) module"]
pub type TIM2_ARR = crate::Reg<u16, _TIM2_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_ARR;
#[doc = "`read()` method returns [tim2_arr::R](tim2_arr::R) reader structure"]
impl crate::Readable for TIM2_ARR {}
#[doc = "`write(|w| ..)` method takes [tim2_arr::W](tim2_arr::W) writer structure"]
impl crate::Writable for TIM2_ARR {}
#[doc = "TIM2 auto-reload register"]
pub mod tim2_arr;
#[doc = "TIM2 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_rcr](tim2_rcr) module"]
pub type TIM2_RCR = crate::Reg<u16, _TIM2_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_RCR;
#[doc = "`read()` method returns [tim2_rcr::R](tim2_rcr::R) reader structure"]
impl crate::Readable for TIM2_RCR {}
#[doc = "`write(|w| ..)` method takes [tim2_rcr::W](tim2_rcr::W) writer structure"]
impl crate::Writable for TIM2_RCR {}
#[doc = "TIM2 repetition counter register"]
pub mod tim2_rcr;
#[doc = "TIM2 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr1](tim2_ccr1) module"]
pub type TIM2_CCR1 = crate::Reg<u16, _TIM2_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR1;
#[doc = "`read()` method returns [tim2_ccr1::R](tim2_ccr1::R) reader structure"]
impl crate::Readable for TIM2_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr1::W](tim2_ccr1::W) writer structure"]
impl crate::Writable for TIM2_CCR1 {}
#[doc = "TIM2 capture/compare register 1"]
pub mod tim2_ccr1;
#[doc = "TIM2 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr2](tim2_ccr2) module"]
pub type TIM2_CCR2 = crate::Reg<u16, _TIM2_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR2;
#[doc = "`read()` method returns [tim2_ccr2::R](tim2_ccr2::R) reader structure"]
impl crate::Readable for TIM2_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr2::W](tim2_ccr2::W) writer structure"]
impl crate::Writable for TIM2_CCR2 {}
#[doc = "TIM2 capture/compare register 2"]
pub mod tim2_ccr2;
#[doc = "TIM2 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr3](tim2_ccr3) module"]
pub type TIM2_CCR3 = crate::Reg<u16, _TIM2_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR3;
#[doc = "`read()` method returns [tim2_ccr3::R](tim2_ccr3::R) reader structure"]
impl crate::Readable for TIM2_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr3::W](tim2_ccr3::W) writer structure"]
impl crate::Writable for TIM2_CCR3 {}
#[doc = "TIM2 capture/compare register 3"]
pub mod tim2_ccr3;
#[doc = "TIM2 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr4](tim2_ccr4) module"]
pub type TIM2_CCR4 = crate::Reg<u16, _TIM2_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR4;
#[doc = "`read()` method returns [tim2_ccr4::R](tim2_ccr4::R) reader structure"]
impl crate::Readable for TIM2_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr4::W](tim2_ccr4::W) writer structure"]
impl crate::Writable for TIM2_CCR4 {}
#[doc = "TIM2 capture/compare register 4"]
pub mod tim2_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_bdtr](tim2_bdtr) module"]
pub type TIM2_BDTR = crate::Reg<u32, _TIM2_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_BDTR;
#[doc = "`read()` method returns [tim2_bdtr::R](tim2_bdtr::R) reader structure"]
impl crate::Readable for TIM2_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim2_bdtr::W](tim2_bdtr::W) writer structure"]
impl crate::Writable for TIM2_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim2_bdtr;
#[doc = "TIM2 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_dcr](tim2_dcr) module"]
pub type TIM2_DCR = crate::Reg<u16, _TIM2_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_DCR;
#[doc = "`read()` method returns [tim2_dcr::R](tim2_dcr::R) reader structure"]
impl crate::Readable for TIM2_DCR {}
#[doc = "`write(|w| ..)` method takes [tim2_dcr::W](tim2_dcr::W) writer structure"]
impl crate::Writable for TIM2_DCR {}
#[doc = "TIM2 DMA control register"]
pub mod tim2_dcr;
#[doc = "TIM2 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_dmar](tim2_dmar) module"]
pub type TIM2_DMAR = crate::Reg<u32, _TIM2_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_DMAR;
#[doc = "`read()` method returns [tim2_dmar::R](tim2_dmar::R) reader structure"]
impl crate::Readable for TIM2_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim2_dmar::W](tim2_dmar::W) writer structure"]
impl crate::Writable for TIM2_DMAR {}
#[doc = "TIM2 DMA address for full transfer"]
pub mod tim2_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccmr3](tim2_ccmr3) module"]
pub type TIM2_CCMR3 = crate::Reg<u32, _TIM2_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCMR3;
#[doc = "`read()` method returns [tim2_ccmr3::R](tim2_ccmr3::R) reader structure"]
impl crate::Readable for TIM2_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccmr3::W](tim2_ccmr3::W) writer structure"]
impl crate::Writable for TIM2_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim2_ccmr3;
#[doc = "TIM2 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr5](tim2_ccr5) module"]
pub type TIM2_CCR5 = crate::Reg<u32, _TIM2_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR5;
#[doc = "`read()` method returns [tim2_ccr5::R](tim2_ccr5::R) reader structure"]
impl crate::Readable for TIM2_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr5::W](tim2_ccr5::W) writer structure"]
impl crate::Writable for TIM2_CCR5 {}
#[doc = "TIM2 capture/compare register 5"]
pub mod tim2_ccr5;
#[doc = "TIM2 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_ccr6](tim2_ccr6) module"]
pub type TIM2_CCR6 = crate::Reg<u16, _TIM2_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM2_CCR6;
#[doc = "`read()` method returns [tim2_ccr6::R](tim2_ccr6::R) reader structure"]
impl crate::Readable for TIM2_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim2_ccr6::W](tim2_ccr6::W) writer structure"]
impl crate::Writable for TIM2_CCR6 {}
#[doc = "TIM2 capture/compare register 6"]
pub mod tim2_ccr6;
