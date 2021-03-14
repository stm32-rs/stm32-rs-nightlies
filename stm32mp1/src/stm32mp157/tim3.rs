#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM3 control register 1"]
    pub tim3_cr1: TIM3_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM3 control register 2"]
    pub tim3_cr2: TIM3_CR2,
    #[doc = "0x08 - TIM3 slave mode control register"]
    pub tim3_smcr: TIM3_SMCR,
    #[doc = "0x0c - TIM3 DMA/interrupt enable register"]
    pub tim3_dier: TIM3_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM3 status register"]
    pub tim3_sr: TIM3_SR,
    #[doc = "0x14 - TIM3 event generation register"]
    pub tim3_egr: TIM3_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim3_ccmr1alternate3: TIM3_CCMR1ALTERNATE3,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim3_ccmr2alternate19: TIM3_CCMR2ALTERNATE19,
    #[doc = "0x20 - TIM3 capture/compare enable register"]
    pub tim3_ccer: TIM3_CCER,
    #[doc = "0x24 - TIM3 counter"]
    pub tim3_cnt: TIM3_CNT,
    #[doc = "0x28 - TIM3 prescaler"]
    pub tim3_psc: TIM3_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM3 auto-reload register"]
    pub tim3_arr: TIM3_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM3 repetition counter register"]
    pub tim3_rcr: TIM3_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM3 capture/compare register 1"]
    pub tim3_ccr1: TIM3_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM3 capture/compare register 2"]
    pub tim3_ccr2: TIM3_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM3 capture/compare register 3"]
    pub tim3_ccr3: TIM3_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM3 capture/compare register 4"]
    pub tim3_ccr4: TIM3_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim3_bdtr: TIM3_BDTR,
    #[doc = "0x48 - TIM3 DMA control register"]
    pub tim3_dcr: TIM3_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM3 DMA address for full transfer"]
    pub tim3_dmar: TIM3_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim3_ccmr3: TIM3_CCMR3,
    #[doc = "0x58 - TIM3 capture/compare register 5"]
    pub tim3_ccr5: TIM3_CCR5,
    #[doc = "0x5c - TIM3 capture/compare register 6"]
    pub tim3_ccr6: TIM3_CCR6,
}
#[doc = "TIM3 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_cr1](tim3_cr1) module"]
pub type TIM3_CR1 = crate::Reg<u16, _TIM3_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CR1;
#[doc = "`read()` method returns [tim3_cr1::R](tim3_cr1::R) reader structure"]
impl crate::Readable for TIM3_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim3_cr1::W](tim3_cr1::W) writer structure"]
impl crate::Writable for TIM3_CR1 {}
#[doc = "TIM3 control register 1"]
pub mod tim3_cr1;
#[doc = "TIM3 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_cr2](tim3_cr2) module"]
pub type TIM3_CR2 = crate::Reg<u32, _TIM3_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CR2;
#[doc = "`read()` method returns [tim3_cr2::R](tim3_cr2::R) reader structure"]
impl crate::Readable for TIM3_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim3_cr2::W](tim3_cr2::W) writer structure"]
impl crate::Writable for TIM3_CR2 {}
#[doc = "TIM3 control register 2"]
pub mod tim3_cr2;
#[doc = "TIM3 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_smcr](tim3_smcr) module"]
pub type TIM3_SMCR = crate::Reg<u32, _TIM3_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_SMCR;
#[doc = "`read()` method returns [tim3_smcr::R](tim3_smcr::R) reader structure"]
impl crate::Readable for TIM3_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim3_smcr::W](tim3_smcr::W) writer structure"]
impl crate::Writable for TIM3_SMCR {}
#[doc = "TIM3 slave mode control register"]
pub mod tim3_smcr;
#[doc = "TIM3 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_dier](tim3_dier) module"]
pub type TIM3_DIER = crate::Reg<u16, _TIM3_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_DIER;
#[doc = "`read()` method returns [tim3_dier::R](tim3_dier::R) reader structure"]
impl crate::Readable for TIM3_DIER {}
#[doc = "`write(|w| ..)` method takes [tim3_dier::W](tim3_dier::W) writer structure"]
impl crate::Writable for TIM3_DIER {}
#[doc = "TIM3 DMA/interrupt enable register"]
pub mod tim3_dier;
#[doc = "TIM3 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_sr](tim3_sr) module"]
pub type TIM3_SR = crate::Reg<u32, _TIM3_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_SR;
#[doc = "`read()` method returns [tim3_sr::R](tim3_sr::R) reader structure"]
impl crate::Readable for TIM3_SR {}
#[doc = "`write(|w| ..)` method takes [tim3_sr::W](tim3_sr::W) writer structure"]
impl crate::Writable for TIM3_SR {}
#[doc = "TIM3 status register"]
pub mod tim3_sr;
#[doc = "TIM3 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_egr](tim3_egr) module"]
pub type TIM3_EGR = crate::Reg<u16, _TIM3_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_EGR;
#[doc = "`write(|w| ..)` method takes [tim3_egr::W](tim3_egr::W) writer structure"]
impl crate::Writable for TIM3_EGR {}
#[doc = "TIM3 event generation register"]
pub mod tim3_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccmr1alternate3](tim3_ccmr1alternate3) module"]
pub type TIM3_CCMR1ALTERNATE3 = crate::Reg<u32, _TIM3_CCMR1ALTERNATE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCMR1ALTERNATE3;
#[doc = "`read()` method returns [tim3_ccmr1alternate3::R](tim3_ccmr1alternate3::R) reader structure"]
impl crate::Readable for TIM3_CCMR1ALTERNATE3 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccmr1alternate3::W](tim3_ccmr1alternate3::W) writer structure"]
impl crate::Writable for TIM3_CCMR1ALTERNATE3 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr1alternate3;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccmr2alternate19](tim3_ccmr2alternate19) module"]
pub type TIM3_CCMR2ALTERNATE19 = crate::Reg<u32, _TIM3_CCMR2ALTERNATE19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCMR2ALTERNATE19;
#[doc = "`read()` method returns [tim3_ccmr2alternate19::R](tim3_ccmr2alternate19::R) reader structure"]
impl crate::Readable for TIM3_CCMR2ALTERNATE19 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccmr2alternate19::W](tim3_ccmr2alternate19::W) writer structure"]
impl crate::Writable for TIM3_CCMR2ALTERNATE19 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim3_ccmr2alternate19;
#[doc = "TIM3 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccer](tim3_ccer) module"]
pub type TIM3_CCER = crate::Reg<u32, _TIM3_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCER;
#[doc = "`read()` method returns [tim3_ccer::R](tim3_ccer::R) reader structure"]
impl crate::Readable for TIM3_CCER {}
#[doc = "`write(|w| ..)` method takes [tim3_ccer::W](tim3_ccer::W) writer structure"]
impl crate::Writable for TIM3_CCER {}
#[doc = "TIM3 capture/compare enable register"]
pub mod tim3_ccer;
#[doc = "TIM3 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_cnt](tim3_cnt) module"]
pub type TIM3_CNT = crate::Reg<u32, _TIM3_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CNT;
#[doc = "`read()` method returns [tim3_cnt::R](tim3_cnt::R) reader structure"]
impl crate::Readable for TIM3_CNT {}
#[doc = "`write(|w| ..)` method takes [tim3_cnt::W](tim3_cnt::W) writer structure"]
impl crate::Writable for TIM3_CNT {}
#[doc = "TIM3 counter"]
pub mod tim3_cnt;
#[doc = "TIM3 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_psc](tim3_psc) module"]
pub type TIM3_PSC = crate::Reg<u16, _TIM3_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_PSC;
#[doc = "`read()` method returns [tim3_psc::R](tim3_psc::R) reader structure"]
impl crate::Readable for TIM3_PSC {}
#[doc = "`write(|w| ..)` method takes [tim3_psc::W](tim3_psc::W) writer structure"]
impl crate::Writable for TIM3_PSC {}
#[doc = "TIM3 prescaler"]
pub mod tim3_psc;
#[doc = "TIM3 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_arr](tim3_arr) module"]
pub type TIM3_ARR = crate::Reg<u16, _TIM3_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_ARR;
#[doc = "`read()` method returns [tim3_arr::R](tim3_arr::R) reader structure"]
impl crate::Readable for TIM3_ARR {}
#[doc = "`write(|w| ..)` method takes [tim3_arr::W](tim3_arr::W) writer structure"]
impl crate::Writable for TIM3_ARR {}
#[doc = "TIM3 auto-reload register"]
pub mod tim3_arr;
#[doc = "TIM3 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_rcr](tim3_rcr) module"]
pub type TIM3_RCR = crate::Reg<u16, _TIM3_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_RCR;
#[doc = "`read()` method returns [tim3_rcr::R](tim3_rcr::R) reader structure"]
impl crate::Readable for TIM3_RCR {}
#[doc = "`write(|w| ..)` method takes [tim3_rcr::W](tim3_rcr::W) writer structure"]
impl crate::Writable for TIM3_RCR {}
#[doc = "TIM3 repetition counter register"]
pub mod tim3_rcr;
#[doc = "TIM3 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr1](tim3_ccr1) module"]
pub type TIM3_CCR1 = crate::Reg<u16, _TIM3_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR1;
#[doc = "`read()` method returns [tim3_ccr1::R](tim3_ccr1::R) reader structure"]
impl crate::Readable for TIM3_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr1::W](tim3_ccr1::W) writer structure"]
impl crate::Writable for TIM3_CCR1 {}
#[doc = "TIM3 capture/compare register 1"]
pub mod tim3_ccr1;
#[doc = "TIM3 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr2](tim3_ccr2) module"]
pub type TIM3_CCR2 = crate::Reg<u16, _TIM3_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR2;
#[doc = "`read()` method returns [tim3_ccr2::R](tim3_ccr2::R) reader structure"]
impl crate::Readable for TIM3_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr2::W](tim3_ccr2::W) writer structure"]
impl crate::Writable for TIM3_CCR2 {}
#[doc = "TIM3 capture/compare register 2"]
pub mod tim3_ccr2;
#[doc = "TIM3 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr3](tim3_ccr3) module"]
pub type TIM3_CCR3 = crate::Reg<u16, _TIM3_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR3;
#[doc = "`read()` method returns [tim3_ccr3::R](tim3_ccr3::R) reader structure"]
impl crate::Readable for TIM3_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr3::W](tim3_ccr3::W) writer structure"]
impl crate::Writable for TIM3_CCR3 {}
#[doc = "TIM3 capture/compare register 3"]
pub mod tim3_ccr3;
#[doc = "TIM3 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr4](tim3_ccr4) module"]
pub type TIM3_CCR4 = crate::Reg<u16, _TIM3_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR4;
#[doc = "`read()` method returns [tim3_ccr4::R](tim3_ccr4::R) reader structure"]
impl crate::Readable for TIM3_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr4::W](tim3_ccr4::W) writer structure"]
impl crate::Writable for TIM3_CCR4 {}
#[doc = "TIM3 capture/compare register 4"]
pub mod tim3_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_bdtr](tim3_bdtr) module"]
pub type TIM3_BDTR = crate::Reg<u32, _TIM3_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_BDTR;
#[doc = "`read()` method returns [tim3_bdtr::R](tim3_bdtr::R) reader structure"]
impl crate::Readable for TIM3_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim3_bdtr::W](tim3_bdtr::W) writer structure"]
impl crate::Writable for TIM3_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim3_bdtr;
#[doc = "TIM3 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_dcr](tim3_dcr) module"]
pub type TIM3_DCR = crate::Reg<u16, _TIM3_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_DCR;
#[doc = "`read()` method returns [tim3_dcr::R](tim3_dcr::R) reader structure"]
impl crate::Readable for TIM3_DCR {}
#[doc = "`write(|w| ..)` method takes [tim3_dcr::W](tim3_dcr::W) writer structure"]
impl crate::Writable for TIM3_DCR {}
#[doc = "TIM3 DMA control register"]
pub mod tim3_dcr;
#[doc = "TIM3 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_dmar](tim3_dmar) module"]
pub type TIM3_DMAR = crate::Reg<u32, _TIM3_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_DMAR;
#[doc = "`read()` method returns [tim3_dmar::R](tim3_dmar::R) reader structure"]
impl crate::Readable for TIM3_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim3_dmar::W](tim3_dmar::W) writer structure"]
impl crate::Writable for TIM3_DMAR {}
#[doc = "TIM3 DMA address for full transfer"]
pub mod tim3_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccmr3](tim3_ccmr3) module"]
pub type TIM3_CCMR3 = crate::Reg<u32, _TIM3_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCMR3;
#[doc = "`read()` method returns [tim3_ccmr3::R](tim3_ccmr3::R) reader structure"]
impl crate::Readable for TIM3_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccmr3::W](tim3_ccmr3::W) writer structure"]
impl crate::Writable for TIM3_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim3_ccmr3;
#[doc = "TIM3 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr5](tim3_ccr5) module"]
pub type TIM3_CCR5 = crate::Reg<u32, _TIM3_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR5;
#[doc = "`read()` method returns [tim3_ccr5::R](tim3_ccr5::R) reader structure"]
impl crate::Readable for TIM3_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr5::W](tim3_ccr5::W) writer structure"]
impl crate::Writable for TIM3_CCR5 {}
#[doc = "TIM3 capture/compare register 5"]
pub mod tim3_ccr5;
#[doc = "TIM3 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr6](tim3_ccr6) module"]
pub type TIM3_CCR6 = crate::Reg<u16, _TIM3_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM3_CCR6;
#[doc = "`read()` method returns [tim3_ccr6::R](tim3_ccr6::R) reader structure"]
impl crate::Readable for TIM3_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim3_ccr6::W](tim3_ccr6::W) writer structure"]
impl crate::Writable for TIM3_CCR6 {}
#[doc = "TIM3 capture/compare register 6"]
pub mod tim3_ccr6;
