#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM6 control register 1"]
    pub tim6_cr1: TIM6_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM6 control register 2"]
    pub tim6_cr2: TIM6_CR2,
    #[doc = "0x08 - TIM6 slave mode control register"]
    pub tim6_smcr: TIM6_SMCR,
    #[doc = "0x0c - TIM6 DMA/interrupt enable register"]
    pub tim6_dier: TIM6_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM6 status register"]
    pub tim6_sr: TIM6_SR,
    #[doc = "0x14 - TIM6 event generation register"]
    pub tim6_egr: TIM6_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim6_ccmr1alternate6: TIM6_CCMR1ALTERNATE6,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim6_ccmr2alternate22: TIM6_CCMR2ALTERNATE22,
    #[doc = "0x20 - TIM6 capture/compare enable register"]
    pub tim6_ccer: TIM6_CCER,
    #[doc = "0x24 - TIM6 counter"]
    pub tim6_cnt: TIM6_CNT,
    #[doc = "0x28 - TIM6 prescaler"]
    pub tim6_psc: TIM6_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM6 auto-reload register"]
    pub tim6_arr: TIM6_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM6 repetition counter register"]
    pub tim6_rcr: TIM6_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM6 capture/compare register 1"]
    pub tim6_ccr1: TIM6_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM6 capture/compare register 2"]
    pub tim6_ccr2: TIM6_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM6 capture/compare register 3"]
    pub tim6_ccr3: TIM6_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM6 capture/compare register 4"]
    pub tim6_ccr4: TIM6_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim6_bdtr: TIM6_BDTR,
    #[doc = "0x48 - TIM6 DMA control register"]
    pub tim6_dcr: TIM6_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM6 DMA address for full transfer"]
    pub tim6_dmar: TIM6_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim6_ccmr3: TIM6_CCMR3,
    #[doc = "0x58 - TIM6 capture/compare register 5"]
    pub tim6_ccr5: TIM6_CCR5,
    #[doc = "0x5c - TIM6 capture/compare register 6"]
    pub tim6_ccr6: TIM6_CCR6,
}
#[doc = "TIM6 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_cr1](tim6_cr1) module"]
pub type TIM6_CR1 = crate::Reg<u16, _TIM6_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CR1;
#[doc = "`read()` method returns [tim6_cr1::R](tim6_cr1::R) reader structure"]
impl crate::Readable for TIM6_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim6_cr1::W](tim6_cr1::W) writer structure"]
impl crate::Writable for TIM6_CR1 {}
#[doc = "TIM6 control register 1"]
pub mod tim6_cr1;
#[doc = "TIM6 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_cr2](tim6_cr2) module"]
pub type TIM6_CR2 = crate::Reg<u32, _TIM6_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CR2;
#[doc = "`read()` method returns [tim6_cr2::R](tim6_cr2::R) reader structure"]
impl crate::Readable for TIM6_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim6_cr2::W](tim6_cr2::W) writer structure"]
impl crate::Writable for TIM6_CR2 {}
#[doc = "TIM6 control register 2"]
pub mod tim6_cr2;
#[doc = "TIM6 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_smcr](tim6_smcr) module"]
pub type TIM6_SMCR = crate::Reg<u32, _TIM6_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_SMCR;
#[doc = "`read()` method returns [tim6_smcr::R](tim6_smcr::R) reader structure"]
impl crate::Readable for TIM6_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim6_smcr::W](tim6_smcr::W) writer structure"]
impl crate::Writable for TIM6_SMCR {}
#[doc = "TIM6 slave mode control register"]
pub mod tim6_smcr;
#[doc = "TIM6 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_dier](tim6_dier) module"]
pub type TIM6_DIER = crate::Reg<u16, _TIM6_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_DIER;
#[doc = "`read()` method returns [tim6_dier::R](tim6_dier::R) reader structure"]
impl crate::Readable for TIM6_DIER {}
#[doc = "`write(|w| ..)` method takes [tim6_dier::W](tim6_dier::W) writer structure"]
impl crate::Writable for TIM6_DIER {}
#[doc = "TIM6 DMA/interrupt enable register"]
pub mod tim6_dier;
#[doc = "TIM6 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_sr](tim6_sr) module"]
pub type TIM6_SR = crate::Reg<u32, _TIM6_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_SR;
#[doc = "`read()` method returns [tim6_sr::R](tim6_sr::R) reader structure"]
impl crate::Readable for TIM6_SR {}
#[doc = "`write(|w| ..)` method takes [tim6_sr::W](tim6_sr::W) writer structure"]
impl crate::Writable for TIM6_SR {}
#[doc = "TIM6 status register"]
pub mod tim6_sr;
#[doc = "TIM6 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_egr](tim6_egr) module"]
pub type TIM6_EGR = crate::Reg<u16, _TIM6_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_EGR;
#[doc = "`write(|w| ..)` method takes [tim6_egr::W](tim6_egr::W) writer structure"]
impl crate::Writable for TIM6_EGR {}
#[doc = "TIM6 event generation register"]
pub mod tim6_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccmr1alternate6](tim6_ccmr1alternate6) module"]
pub type TIM6_CCMR1ALTERNATE6 = crate::Reg<u32, _TIM6_CCMR1ALTERNATE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCMR1ALTERNATE6;
#[doc = "`read()` method returns [tim6_ccmr1alternate6::R](tim6_ccmr1alternate6::R) reader structure"]
impl crate::Readable for TIM6_CCMR1ALTERNATE6 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccmr1alternate6::W](tim6_ccmr1alternate6::W) writer structure"]
impl crate::Writable for TIM6_CCMR1ALTERNATE6 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim6_ccmr1alternate6;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccmr2alternate22](tim6_ccmr2alternate22) module"]
pub type TIM6_CCMR2ALTERNATE22 = crate::Reg<u32, _TIM6_CCMR2ALTERNATE22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCMR2ALTERNATE22;
#[doc = "`read()` method returns [tim6_ccmr2alternate22::R](tim6_ccmr2alternate22::R) reader structure"]
impl crate::Readable for TIM6_CCMR2ALTERNATE22 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccmr2alternate22::W](tim6_ccmr2alternate22::W) writer structure"]
impl crate::Writable for TIM6_CCMR2ALTERNATE22 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim6_ccmr2alternate22;
#[doc = "TIM6 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccer](tim6_ccer) module"]
pub type TIM6_CCER = crate::Reg<u32, _TIM6_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCER;
#[doc = "`read()` method returns [tim6_ccer::R](tim6_ccer::R) reader structure"]
impl crate::Readable for TIM6_CCER {}
#[doc = "`write(|w| ..)` method takes [tim6_ccer::W](tim6_ccer::W) writer structure"]
impl crate::Writable for TIM6_CCER {}
#[doc = "TIM6 capture/compare enable register"]
pub mod tim6_ccer;
#[doc = "TIM6 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_cnt](tim6_cnt) module"]
pub type TIM6_CNT = crate::Reg<u32, _TIM6_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CNT;
#[doc = "`read()` method returns [tim6_cnt::R](tim6_cnt::R) reader structure"]
impl crate::Readable for TIM6_CNT {}
#[doc = "`write(|w| ..)` method takes [tim6_cnt::W](tim6_cnt::W) writer structure"]
impl crate::Writable for TIM6_CNT {}
#[doc = "TIM6 counter"]
pub mod tim6_cnt;
#[doc = "TIM6 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_psc](tim6_psc) module"]
pub type TIM6_PSC = crate::Reg<u16, _TIM6_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_PSC;
#[doc = "`read()` method returns [tim6_psc::R](tim6_psc::R) reader structure"]
impl crate::Readable for TIM6_PSC {}
#[doc = "`write(|w| ..)` method takes [tim6_psc::W](tim6_psc::W) writer structure"]
impl crate::Writable for TIM6_PSC {}
#[doc = "TIM6 prescaler"]
pub mod tim6_psc;
#[doc = "TIM6 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_arr](tim6_arr) module"]
pub type TIM6_ARR = crate::Reg<u16, _TIM6_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_ARR;
#[doc = "`read()` method returns [tim6_arr::R](tim6_arr::R) reader structure"]
impl crate::Readable for TIM6_ARR {}
#[doc = "`write(|w| ..)` method takes [tim6_arr::W](tim6_arr::W) writer structure"]
impl crate::Writable for TIM6_ARR {}
#[doc = "TIM6 auto-reload register"]
pub mod tim6_arr;
#[doc = "TIM6 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_rcr](tim6_rcr) module"]
pub type TIM6_RCR = crate::Reg<u16, _TIM6_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_RCR;
#[doc = "`read()` method returns [tim6_rcr::R](tim6_rcr::R) reader structure"]
impl crate::Readable for TIM6_RCR {}
#[doc = "`write(|w| ..)` method takes [tim6_rcr::W](tim6_rcr::W) writer structure"]
impl crate::Writable for TIM6_RCR {}
#[doc = "TIM6 repetition counter register"]
pub mod tim6_rcr;
#[doc = "TIM6 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr1](tim6_ccr1) module"]
pub type TIM6_CCR1 = crate::Reg<u16, _TIM6_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR1;
#[doc = "`read()` method returns [tim6_ccr1::R](tim6_ccr1::R) reader structure"]
impl crate::Readable for TIM6_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr1::W](tim6_ccr1::W) writer structure"]
impl crate::Writable for TIM6_CCR1 {}
#[doc = "TIM6 capture/compare register 1"]
pub mod tim6_ccr1;
#[doc = "TIM6 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr2](tim6_ccr2) module"]
pub type TIM6_CCR2 = crate::Reg<u16, _TIM6_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR2;
#[doc = "`read()` method returns [tim6_ccr2::R](tim6_ccr2::R) reader structure"]
impl crate::Readable for TIM6_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr2::W](tim6_ccr2::W) writer structure"]
impl crate::Writable for TIM6_CCR2 {}
#[doc = "TIM6 capture/compare register 2"]
pub mod tim6_ccr2;
#[doc = "TIM6 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr3](tim6_ccr3) module"]
pub type TIM6_CCR3 = crate::Reg<u16, _TIM6_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR3;
#[doc = "`read()` method returns [tim6_ccr3::R](tim6_ccr3::R) reader structure"]
impl crate::Readable for TIM6_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr3::W](tim6_ccr3::W) writer structure"]
impl crate::Writable for TIM6_CCR3 {}
#[doc = "TIM6 capture/compare register 3"]
pub mod tim6_ccr3;
#[doc = "TIM6 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr4](tim6_ccr4) module"]
pub type TIM6_CCR4 = crate::Reg<u16, _TIM6_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR4;
#[doc = "`read()` method returns [tim6_ccr4::R](tim6_ccr4::R) reader structure"]
impl crate::Readable for TIM6_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr4::W](tim6_ccr4::W) writer structure"]
impl crate::Writable for TIM6_CCR4 {}
#[doc = "TIM6 capture/compare register 4"]
pub mod tim6_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_bdtr](tim6_bdtr) module"]
pub type TIM6_BDTR = crate::Reg<u32, _TIM6_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_BDTR;
#[doc = "`read()` method returns [tim6_bdtr::R](tim6_bdtr::R) reader structure"]
impl crate::Readable for TIM6_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim6_bdtr::W](tim6_bdtr::W) writer structure"]
impl crate::Writable for TIM6_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim6_bdtr;
#[doc = "TIM6 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_dcr](tim6_dcr) module"]
pub type TIM6_DCR = crate::Reg<u16, _TIM6_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_DCR;
#[doc = "`read()` method returns [tim6_dcr::R](tim6_dcr::R) reader structure"]
impl crate::Readable for TIM6_DCR {}
#[doc = "`write(|w| ..)` method takes [tim6_dcr::W](tim6_dcr::W) writer structure"]
impl crate::Writable for TIM6_DCR {}
#[doc = "TIM6 DMA control register"]
pub mod tim6_dcr;
#[doc = "TIM6 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_dmar](tim6_dmar) module"]
pub type TIM6_DMAR = crate::Reg<u32, _TIM6_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_DMAR;
#[doc = "`read()` method returns [tim6_dmar::R](tim6_dmar::R) reader structure"]
impl crate::Readable for TIM6_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim6_dmar::W](tim6_dmar::W) writer structure"]
impl crate::Writable for TIM6_DMAR {}
#[doc = "TIM6 DMA address for full transfer"]
pub mod tim6_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccmr3](tim6_ccmr3) module"]
pub type TIM6_CCMR3 = crate::Reg<u32, _TIM6_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCMR3;
#[doc = "`read()` method returns [tim6_ccmr3::R](tim6_ccmr3::R) reader structure"]
impl crate::Readable for TIM6_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccmr3::W](tim6_ccmr3::W) writer structure"]
impl crate::Writable for TIM6_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim6_ccmr3;
#[doc = "TIM6 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr5](tim6_ccr5) module"]
pub type TIM6_CCR5 = crate::Reg<u32, _TIM6_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR5;
#[doc = "`read()` method returns [tim6_ccr5::R](tim6_ccr5::R) reader structure"]
impl crate::Readable for TIM6_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr5::W](tim6_ccr5::W) writer structure"]
impl crate::Writable for TIM6_CCR5 {}
#[doc = "TIM6 capture/compare register 5"]
pub mod tim6_ccr5;
#[doc = "TIM6 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr6](tim6_ccr6) module"]
pub type TIM6_CCR6 = crate::Reg<u16, _TIM6_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM6_CCR6;
#[doc = "`read()` method returns [tim6_ccr6::R](tim6_ccr6::R) reader structure"]
impl crate::Readable for TIM6_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim6_ccr6::W](tim6_ccr6::W) writer structure"]
impl crate::Writable for TIM6_CCR6 {}
#[doc = "TIM6 capture/compare register 6"]
pub mod tim6_ccr6;
