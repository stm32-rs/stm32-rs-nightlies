#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM5 control register 1"]
    pub tim5_cr1: TIM5_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM5 control register 2"]
    pub tim5_cr2: TIM5_CR2,
    #[doc = "0x08 - TIM5 slave mode control register"]
    pub tim5_smcr: TIM5_SMCR,
    #[doc = "0x0c - TIM5 DMA/interrupt enable register"]
    pub tim5_dier: TIM5_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM5 status register"]
    pub tim5_sr: TIM5_SR,
    #[doc = "0x14 - TIM5 event generation register"]
    pub tim5_egr: TIM5_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim5_ccmr1alternate5: TIM5_CCMR1ALTERNATE5,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim5_ccmr2alternate21: TIM5_CCMR2ALTERNATE21,
    #[doc = "0x20 - TIM5 capture/compare enable register"]
    pub tim5_ccer: TIM5_CCER,
    #[doc = "0x24 - TIM5 counter"]
    pub tim5_cnt: TIM5_CNT,
    #[doc = "0x28 - TIM5 prescaler"]
    pub tim5_psc: TIM5_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM5 auto-reload register"]
    pub tim5_arr: TIM5_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM5 repetition counter register"]
    pub tim5_rcr: TIM5_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM5 capture/compare register 1"]
    pub tim5_ccr1: TIM5_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM5 capture/compare register 2"]
    pub tim5_ccr2: TIM5_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM5 capture/compare register 3"]
    pub tim5_ccr3: TIM5_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM5 capture/compare register 4"]
    pub tim5_ccr4: TIM5_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim5_bdtr: TIM5_BDTR,
    #[doc = "0x48 - TIM5 DMA control register"]
    pub tim5_dcr: TIM5_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM5 DMA address for full transfer"]
    pub tim5_dmar: TIM5_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim5_ccmr3: TIM5_CCMR3,
    #[doc = "0x58 - TIM5 capture/compare register 5"]
    pub tim5_ccr5: TIM5_CCR5,
    #[doc = "0x5c - TIM5 capture/compare register 6"]
    pub tim5_ccr6: TIM5_CCR6,
}
#[doc = "TIM5 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_cr1](tim5_cr1) module"]
pub type TIM5_CR1 = crate::Reg<u16, _TIM5_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CR1;
#[doc = "`read()` method returns [tim5_cr1::R](tim5_cr1::R) reader structure"]
impl crate::Readable for TIM5_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim5_cr1::W](tim5_cr1::W) writer structure"]
impl crate::Writable for TIM5_CR1 {}
#[doc = "TIM5 control register 1"]
pub mod tim5_cr1;
#[doc = "TIM5 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_cr2](tim5_cr2) module"]
pub type TIM5_CR2 = crate::Reg<u32, _TIM5_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CR2;
#[doc = "`read()` method returns [tim5_cr2::R](tim5_cr2::R) reader structure"]
impl crate::Readable for TIM5_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim5_cr2::W](tim5_cr2::W) writer structure"]
impl crate::Writable for TIM5_CR2 {}
#[doc = "TIM5 control register 2"]
pub mod tim5_cr2;
#[doc = "TIM5 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_smcr](tim5_smcr) module"]
pub type TIM5_SMCR = crate::Reg<u32, _TIM5_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_SMCR;
#[doc = "`read()` method returns [tim5_smcr::R](tim5_smcr::R) reader structure"]
impl crate::Readable for TIM5_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim5_smcr::W](tim5_smcr::W) writer structure"]
impl crate::Writable for TIM5_SMCR {}
#[doc = "TIM5 slave mode control register"]
pub mod tim5_smcr;
#[doc = "TIM5 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_dier](tim5_dier) module"]
pub type TIM5_DIER = crate::Reg<u16, _TIM5_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_DIER;
#[doc = "`read()` method returns [tim5_dier::R](tim5_dier::R) reader structure"]
impl crate::Readable for TIM5_DIER {}
#[doc = "`write(|w| ..)` method takes [tim5_dier::W](tim5_dier::W) writer structure"]
impl crate::Writable for TIM5_DIER {}
#[doc = "TIM5 DMA/interrupt enable register"]
pub mod tim5_dier;
#[doc = "TIM5 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_sr](tim5_sr) module"]
pub type TIM5_SR = crate::Reg<u32, _TIM5_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_SR;
#[doc = "`read()` method returns [tim5_sr::R](tim5_sr::R) reader structure"]
impl crate::Readable for TIM5_SR {}
#[doc = "`write(|w| ..)` method takes [tim5_sr::W](tim5_sr::W) writer structure"]
impl crate::Writable for TIM5_SR {}
#[doc = "TIM5 status register"]
pub mod tim5_sr;
#[doc = "TIM5 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_egr](tim5_egr) module"]
pub type TIM5_EGR = crate::Reg<u16, _TIM5_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_EGR;
#[doc = "`write(|w| ..)` method takes [tim5_egr::W](tim5_egr::W) writer structure"]
impl crate::Writable for TIM5_EGR {}
#[doc = "TIM5 event generation register"]
pub mod tim5_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccmr1alternate5](tim5_ccmr1alternate5) module"]
pub type TIM5_CCMR1ALTERNATE5 = crate::Reg<u32, _TIM5_CCMR1ALTERNATE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCMR1ALTERNATE5;
#[doc = "`read()` method returns [tim5_ccmr1alternate5::R](tim5_ccmr1alternate5::R) reader structure"]
impl crate::Readable for TIM5_CCMR1ALTERNATE5 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccmr1alternate5::W](tim5_ccmr1alternate5::W) writer structure"]
impl crate::Writable for TIM5_CCMR1ALTERNATE5 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim5_ccmr1alternate5;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccmr2alternate21](tim5_ccmr2alternate21) module"]
pub type TIM5_CCMR2ALTERNATE21 = crate::Reg<u32, _TIM5_CCMR2ALTERNATE21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCMR2ALTERNATE21;
#[doc = "`read()` method returns [tim5_ccmr2alternate21::R](tim5_ccmr2alternate21::R) reader structure"]
impl crate::Readable for TIM5_CCMR2ALTERNATE21 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccmr2alternate21::W](tim5_ccmr2alternate21::W) writer structure"]
impl crate::Writable for TIM5_CCMR2ALTERNATE21 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim5_ccmr2alternate21;
#[doc = "TIM5 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccer](tim5_ccer) module"]
pub type TIM5_CCER = crate::Reg<u32, _TIM5_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCER;
#[doc = "`read()` method returns [tim5_ccer::R](tim5_ccer::R) reader structure"]
impl crate::Readable for TIM5_CCER {}
#[doc = "`write(|w| ..)` method takes [tim5_ccer::W](tim5_ccer::W) writer structure"]
impl crate::Writable for TIM5_CCER {}
#[doc = "TIM5 capture/compare enable register"]
pub mod tim5_ccer;
#[doc = "TIM5 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_cnt](tim5_cnt) module"]
pub type TIM5_CNT = crate::Reg<u32, _TIM5_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CNT;
#[doc = "`read()` method returns [tim5_cnt::R](tim5_cnt::R) reader structure"]
impl crate::Readable for TIM5_CNT {}
#[doc = "`write(|w| ..)` method takes [tim5_cnt::W](tim5_cnt::W) writer structure"]
impl crate::Writable for TIM5_CNT {}
#[doc = "TIM5 counter"]
pub mod tim5_cnt;
#[doc = "TIM5 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_psc](tim5_psc) module"]
pub type TIM5_PSC = crate::Reg<u16, _TIM5_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_PSC;
#[doc = "`read()` method returns [tim5_psc::R](tim5_psc::R) reader structure"]
impl crate::Readable for TIM5_PSC {}
#[doc = "`write(|w| ..)` method takes [tim5_psc::W](tim5_psc::W) writer structure"]
impl crate::Writable for TIM5_PSC {}
#[doc = "TIM5 prescaler"]
pub mod tim5_psc;
#[doc = "TIM5 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_arr](tim5_arr) module"]
pub type TIM5_ARR = crate::Reg<u16, _TIM5_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_ARR;
#[doc = "`read()` method returns [tim5_arr::R](tim5_arr::R) reader structure"]
impl crate::Readable for TIM5_ARR {}
#[doc = "`write(|w| ..)` method takes [tim5_arr::W](tim5_arr::W) writer structure"]
impl crate::Writable for TIM5_ARR {}
#[doc = "TIM5 auto-reload register"]
pub mod tim5_arr;
#[doc = "TIM5 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_rcr](tim5_rcr) module"]
pub type TIM5_RCR = crate::Reg<u16, _TIM5_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_RCR;
#[doc = "`read()` method returns [tim5_rcr::R](tim5_rcr::R) reader structure"]
impl crate::Readable for TIM5_RCR {}
#[doc = "`write(|w| ..)` method takes [tim5_rcr::W](tim5_rcr::W) writer structure"]
impl crate::Writable for TIM5_RCR {}
#[doc = "TIM5 repetition counter register"]
pub mod tim5_rcr;
#[doc = "TIM5 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr1](tim5_ccr1) module"]
pub type TIM5_CCR1 = crate::Reg<u16, _TIM5_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR1;
#[doc = "`read()` method returns [tim5_ccr1::R](tim5_ccr1::R) reader structure"]
impl crate::Readable for TIM5_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr1::W](tim5_ccr1::W) writer structure"]
impl crate::Writable for TIM5_CCR1 {}
#[doc = "TIM5 capture/compare register 1"]
pub mod tim5_ccr1;
#[doc = "TIM5 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr2](tim5_ccr2) module"]
pub type TIM5_CCR2 = crate::Reg<u16, _TIM5_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR2;
#[doc = "`read()` method returns [tim5_ccr2::R](tim5_ccr2::R) reader structure"]
impl crate::Readable for TIM5_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr2::W](tim5_ccr2::W) writer structure"]
impl crate::Writable for TIM5_CCR2 {}
#[doc = "TIM5 capture/compare register 2"]
pub mod tim5_ccr2;
#[doc = "TIM5 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr3](tim5_ccr3) module"]
pub type TIM5_CCR3 = crate::Reg<u16, _TIM5_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR3;
#[doc = "`read()` method returns [tim5_ccr3::R](tim5_ccr3::R) reader structure"]
impl crate::Readable for TIM5_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr3::W](tim5_ccr3::W) writer structure"]
impl crate::Writable for TIM5_CCR3 {}
#[doc = "TIM5 capture/compare register 3"]
pub mod tim5_ccr3;
#[doc = "TIM5 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr4](tim5_ccr4) module"]
pub type TIM5_CCR4 = crate::Reg<u16, _TIM5_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR4;
#[doc = "`read()` method returns [tim5_ccr4::R](tim5_ccr4::R) reader structure"]
impl crate::Readable for TIM5_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr4::W](tim5_ccr4::W) writer structure"]
impl crate::Writable for TIM5_CCR4 {}
#[doc = "TIM5 capture/compare register 4"]
pub mod tim5_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_bdtr](tim5_bdtr) module"]
pub type TIM5_BDTR = crate::Reg<u32, _TIM5_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_BDTR;
#[doc = "`read()` method returns [tim5_bdtr::R](tim5_bdtr::R) reader structure"]
impl crate::Readable for TIM5_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim5_bdtr::W](tim5_bdtr::W) writer structure"]
impl crate::Writable for TIM5_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim5_bdtr;
#[doc = "TIM5 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_dcr](tim5_dcr) module"]
pub type TIM5_DCR = crate::Reg<u16, _TIM5_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_DCR;
#[doc = "`read()` method returns [tim5_dcr::R](tim5_dcr::R) reader structure"]
impl crate::Readable for TIM5_DCR {}
#[doc = "`write(|w| ..)` method takes [tim5_dcr::W](tim5_dcr::W) writer structure"]
impl crate::Writable for TIM5_DCR {}
#[doc = "TIM5 DMA control register"]
pub mod tim5_dcr;
#[doc = "TIM5 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_dmar](tim5_dmar) module"]
pub type TIM5_DMAR = crate::Reg<u32, _TIM5_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_DMAR;
#[doc = "`read()` method returns [tim5_dmar::R](tim5_dmar::R) reader structure"]
impl crate::Readable for TIM5_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim5_dmar::W](tim5_dmar::W) writer structure"]
impl crate::Writable for TIM5_DMAR {}
#[doc = "TIM5 DMA address for full transfer"]
pub mod tim5_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccmr3](tim5_ccmr3) module"]
pub type TIM5_CCMR3 = crate::Reg<u32, _TIM5_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCMR3;
#[doc = "`read()` method returns [tim5_ccmr3::R](tim5_ccmr3::R) reader structure"]
impl crate::Readable for TIM5_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccmr3::W](tim5_ccmr3::W) writer structure"]
impl crate::Writable for TIM5_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim5_ccmr3;
#[doc = "TIM5 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr5](tim5_ccr5) module"]
pub type TIM5_CCR5 = crate::Reg<u32, _TIM5_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR5;
#[doc = "`read()` method returns [tim5_ccr5::R](tim5_ccr5::R) reader structure"]
impl crate::Readable for TIM5_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr5::W](tim5_ccr5::W) writer structure"]
impl crate::Writable for TIM5_CCR5 {}
#[doc = "TIM5 capture/compare register 5"]
pub mod tim5_ccr5;
#[doc = "TIM5 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_ccr6](tim5_ccr6) module"]
pub type TIM5_CCR6 = crate::Reg<u16, _TIM5_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM5_CCR6;
#[doc = "`read()` method returns [tim5_ccr6::R](tim5_ccr6::R) reader structure"]
impl crate::Readable for TIM5_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim5_ccr6::W](tim5_ccr6::W) writer structure"]
impl crate::Writable for TIM5_CCR6 {}
#[doc = "TIM5 capture/compare register 6"]
pub mod tim5_ccr6;
