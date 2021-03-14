#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM8 control register 1"]
    pub tim8_cr1: TIM8_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM8 control register 2"]
    pub tim8_cr2: TIM8_CR2,
    #[doc = "0x08 - TIM8 slave mode control register"]
    pub tim8_smcr: TIM8_SMCR,
    #[doc = "0x0c - TIM8 DMA/interrupt enable register"]
    pub tim8_dier: TIM8_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM8 status register"]
    pub tim8_sr: TIM8_SR,
    #[doc = "0x14 - TIM8 event generation register"]
    pub tim8_egr: TIM8_EGR,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim8_ccmr1alternate8: TIM8_CCMR1ALTERNATE8,
    #[doc = "0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
    pub tim8_ccmr2alternate24: TIM8_CCMR2ALTERNATE24,
    #[doc = "0x20 - TIM8 capture/compare enable register"]
    pub tim8_ccer: TIM8_CCER,
    #[doc = "0x24 - TIM8 counter"]
    pub tim8_cnt: TIM8_CNT,
    #[doc = "0x28 - TIM8 prescaler"]
    pub tim8_psc: TIM8_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM8 auto-reload register"]
    pub tim8_arr: TIM8_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM8 repetition counter register"]
    pub tim8_rcr: TIM8_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM8 capture/compare register 1"]
    pub tim8_ccr1: TIM8_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM8 capture/compare register 2"]
    pub tim8_ccr2: TIM8_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM8 capture/compare register 3"]
    pub tim8_ccr3: TIM8_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM8 capture/compare register 4"]
    pub tim8_ccr4: TIM8_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub tim8_bdtr: TIM8_BDTR,
    #[doc = "0x48 - TIM8 DMA control register"]
    pub tim8_dcr: TIM8_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - TIM8 DMA address for full transfer"]
    pub tim8_dmar: TIM8_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:"]
    pub tim8_ccmr3: TIM8_CCMR3,
    #[doc = "0x58 - TIM8 capture/compare register 5"]
    pub tim8_ccr5: TIM8_CCR5,
    #[doc = "0x5c - TIM8 capture/compare register 6"]
    pub tim8_ccr6: TIM8_CCR6,
    _reserved23: [u8; 2usize],
    #[doc = "0x60 - TIM8 Alternate function option register 1"]
    pub tim8_af1: TIM8_AF1,
    #[doc = "0x64 - TIM8 Alternate function option register 2"]
    pub tim8_af2: TIM8_AF2,
    #[doc = "0x68 - TIM8 timer input selection register"]
    pub tim8_tisel: TIM8_TISEL,
}
#[doc = "TIM8 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_cr1](tim8_cr1) module"]
pub type TIM8_CR1 = crate::Reg<u16, _TIM8_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CR1;
#[doc = "`read()` method returns [tim8_cr1::R](tim8_cr1::R) reader structure"]
impl crate::Readable for TIM8_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim8_cr1::W](tim8_cr1::W) writer structure"]
impl crate::Writable for TIM8_CR1 {}
#[doc = "TIM8 control register 1"]
pub mod tim8_cr1;
#[doc = "TIM8 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_cr2](tim8_cr2) module"]
pub type TIM8_CR2 = crate::Reg<u32, _TIM8_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CR2;
#[doc = "`read()` method returns [tim8_cr2::R](tim8_cr2::R) reader structure"]
impl crate::Readable for TIM8_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim8_cr2::W](tim8_cr2::W) writer structure"]
impl crate::Writable for TIM8_CR2 {}
#[doc = "TIM8 control register 2"]
pub mod tim8_cr2;
#[doc = "TIM8 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_smcr](tim8_smcr) module"]
pub type TIM8_SMCR = crate::Reg<u32, _TIM8_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_SMCR;
#[doc = "`read()` method returns [tim8_smcr::R](tim8_smcr::R) reader structure"]
impl crate::Readable for TIM8_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim8_smcr::W](tim8_smcr::W) writer structure"]
impl crate::Writable for TIM8_SMCR {}
#[doc = "TIM8 slave mode control register"]
pub mod tim8_smcr;
#[doc = "TIM8 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_dier](tim8_dier) module"]
pub type TIM8_DIER = crate::Reg<u16, _TIM8_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_DIER;
#[doc = "`read()` method returns [tim8_dier::R](tim8_dier::R) reader structure"]
impl crate::Readable for TIM8_DIER {}
#[doc = "`write(|w| ..)` method takes [tim8_dier::W](tim8_dier::W) writer structure"]
impl crate::Writable for TIM8_DIER {}
#[doc = "TIM8 DMA/interrupt enable register"]
pub mod tim8_dier;
#[doc = "TIM8 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_sr](tim8_sr) module"]
pub type TIM8_SR = crate::Reg<u32, _TIM8_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_SR;
#[doc = "`read()` method returns [tim8_sr::R](tim8_sr::R) reader structure"]
impl crate::Readable for TIM8_SR {}
#[doc = "`write(|w| ..)` method takes [tim8_sr::W](tim8_sr::W) writer structure"]
impl crate::Writable for TIM8_SR {}
#[doc = "TIM8 status register"]
pub mod tim8_sr;
#[doc = "TIM8 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_egr](tim8_egr) module"]
pub type TIM8_EGR = crate::Reg<u16, _TIM8_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_EGR;
#[doc = "`write(|w| ..)` method takes [tim8_egr::W](tim8_egr::W) writer structure"]
impl crate::Writable for TIM8_EGR {}
#[doc = "TIM8 event generation register"]
pub mod tim8_egr;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccmr1alternate8](tim8_ccmr1alternate8) module"]
pub type TIM8_CCMR1ALTERNATE8 = crate::Reg<u32, _TIM8_CCMR1ALTERNATE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCMR1ALTERNATE8;
#[doc = "`read()` method returns [tim8_ccmr1alternate8::R](tim8_ccmr1alternate8::R) reader structure"]
impl crate::Readable for TIM8_CCMR1ALTERNATE8 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccmr1alternate8::W](tim8_ccmr1alternate8::W) writer structure"]
impl crate::Writable for TIM8_CCMR1ALTERNATE8 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim8_ccmr1alternate8;
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccmr2alternate24](tim8_ccmr2alternate24) module"]
pub type TIM8_CCMR2ALTERNATE24 = crate::Reg<u32, _TIM8_CCMR2ALTERNATE24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCMR2ALTERNATE24;
#[doc = "`read()` method returns [tim8_ccmr2alternate24::R](tim8_ccmr2alternate24::R) reader structure"]
impl crate::Readable for TIM8_CCMR2ALTERNATE24 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccmr2alternate24::W](tim8_ccmr2alternate24::W) writer structure"]
impl crate::Writable for TIM8_CCMR2ALTERNATE24 {}
#[doc = "The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:"]
pub mod tim8_ccmr2alternate24;
#[doc = "TIM8 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccer](tim8_ccer) module"]
pub type TIM8_CCER = crate::Reg<u32, _TIM8_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCER;
#[doc = "`read()` method returns [tim8_ccer::R](tim8_ccer::R) reader structure"]
impl crate::Readable for TIM8_CCER {}
#[doc = "`write(|w| ..)` method takes [tim8_ccer::W](tim8_ccer::W) writer structure"]
impl crate::Writable for TIM8_CCER {}
#[doc = "TIM8 capture/compare enable register"]
pub mod tim8_ccer;
#[doc = "TIM8 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_cnt](tim8_cnt) module"]
pub type TIM8_CNT = crate::Reg<u32, _TIM8_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CNT;
#[doc = "`read()` method returns [tim8_cnt::R](tim8_cnt::R) reader structure"]
impl crate::Readable for TIM8_CNT {}
#[doc = "`write(|w| ..)` method takes [tim8_cnt::W](tim8_cnt::W) writer structure"]
impl crate::Writable for TIM8_CNT {}
#[doc = "TIM8 counter"]
pub mod tim8_cnt;
#[doc = "TIM8 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_psc](tim8_psc) module"]
pub type TIM8_PSC = crate::Reg<u16, _TIM8_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_PSC;
#[doc = "`read()` method returns [tim8_psc::R](tim8_psc::R) reader structure"]
impl crate::Readable for TIM8_PSC {}
#[doc = "`write(|w| ..)` method takes [tim8_psc::W](tim8_psc::W) writer structure"]
impl crate::Writable for TIM8_PSC {}
#[doc = "TIM8 prescaler"]
pub mod tim8_psc;
#[doc = "TIM8 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_arr](tim8_arr) module"]
pub type TIM8_ARR = crate::Reg<u16, _TIM8_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_ARR;
#[doc = "`read()` method returns [tim8_arr::R](tim8_arr::R) reader structure"]
impl crate::Readable for TIM8_ARR {}
#[doc = "`write(|w| ..)` method takes [tim8_arr::W](tim8_arr::W) writer structure"]
impl crate::Writable for TIM8_ARR {}
#[doc = "TIM8 auto-reload register"]
pub mod tim8_arr;
#[doc = "TIM8 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_rcr](tim8_rcr) module"]
pub type TIM8_RCR = crate::Reg<u16, _TIM8_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_RCR;
#[doc = "`read()` method returns [tim8_rcr::R](tim8_rcr::R) reader structure"]
impl crate::Readable for TIM8_RCR {}
#[doc = "`write(|w| ..)` method takes [tim8_rcr::W](tim8_rcr::W) writer structure"]
impl crate::Writable for TIM8_RCR {}
#[doc = "TIM8 repetition counter register"]
pub mod tim8_rcr;
#[doc = "TIM8 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr1](tim8_ccr1) module"]
pub type TIM8_CCR1 = crate::Reg<u16, _TIM8_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR1;
#[doc = "`read()` method returns [tim8_ccr1::R](tim8_ccr1::R) reader structure"]
impl crate::Readable for TIM8_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr1::W](tim8_ccr1::W) writer structure"]
impl crate::Writable for TIM8_CCR1 {}
#[doc = "TIM8 capture/compare register 1"]
pub mod tim8_ccr1;
#[doc = "TIM8 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr2](tim8_ccr2) module"]
pub type TIM8_CCR2 = crate::Reg<u16, _TIM8_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR2;
#[doc = "`read()` method returns [tim8_ccr2::R](tim8_ccr2::R) reader structure"]
impl crate::Readable for TIM8_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr2::W](tim8_ccr2::W) writer structure"]
impl crate::Writable for TIM8_CCR2 {}
#[doc = "TIM8 capture/compare register 2"]
pub mod tim8_ccr2;
#[doc = "TIM8 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr3](tim8_ccr3) module"]
pub type TIM8_CCR3 = crate::Reg<u16, _TIM8_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR3;
#[doc = "`read()` method returns [tim8_ccr3::R](tim8_ccr3::R) reader structure"]
impl crate::Readable for TIM8_CCR3 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr3::W](tim8_ccr3::W) writer structure"]
impl crate::Writable for TIM8_CCR3 {}
#[doc = "TIM8 capture/compare register 3"]
pub mod tim8_ccr3;
#[doc = "TIM8 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr4](tim8_ccr4) module"]
pub type TIM8_CCR4 = crate::Reg<u16, _TIM8_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR4;
#[doc = "`read()` method returns [tim8_ccr4::R](tim8_ccr4::R) reader structure"]
impl crate::Readable for TIM8_CCR4 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr4::W](tim8_ccr4::W) writer structure"]
impl crate::Writable for TIM8_CCR4 {}
#[doc = "TIM8 capture/compare register 4"]
pub mod tim8_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_bdtr](tim8_bdtr) module"]
pub type TIM8_BDTR = crate::Reg<u32, _TIM8_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_BDTR;
#[doc = "`read()` method returns [tim8_bdtr::R](tim8_bdtr::R) reader structure"]
impl crate::Readable for TIM8_BDTR {}
#[doc = "`write(|w| ..)` method takes [tim8_bdtr::W](tim8_bdtr::W) writer structure"]
impl crate::Writable for TIM8_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod tim8_bdtr;
#[doc = "TIM8 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_dcr](tim8_dcr) module"]
pub type TIM8_DCR = crate::Reg<u16, _TIM8_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_DCR;
#[doc = "`read()` method returns [tim8_dcr::R](tim8_dcr::R) reader structure"]
impl crate::Readable for TIM8_DCR {}
#[doc = "`write(|w| ..)` method takes [tim8_dcr::W](tim8_dcr::W) writer structure"]
impl crate::Writable for TIM8_DCR {}
#[doc = "TIM8 DMA control register"]
pub mod tim8_dcr;
#[doc = "TIM8 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_dmar](tim8_dmar) module"]
pub type TIM8_DMAR = crate::Reg<u32, _TIM8_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_DMAR;
#[doc = "`read()` method returns [tim8_dmar::R](tim8_dmar::R) reader structure"]
impl crate::Readable for TIM8_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim8_dmar::W](tim8_dmar::W) writer structure"]
impl crate::Writable for TIM8_DMAR {}
#[doc = "TIM8 DMA address for full transfer"]
pub mod tim8_dmar;
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccmr3](tim8_ccmr3) module"]
pub type TIM8_CCMR3 = crate::Reg<u32, _TIM8_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCMR3;
#[doc = "`read()` method returns [tim8_ccmr3::R](tim8_ccmr3::R) reader structure"]
impl crate::Readable for TIM8_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccmr3::W](tim8_ccmr3::W) writer structure"]
impl crate::Writable for TIM8_CCMR3 {}
#[doc = "The channels 5 and 6 can only be configured in output. Output compare mode:"]
pub mod tim8_ccmr3;
#[doc = "TIM8 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr5](tim8_ccr5) module"]
pub type TIM8_CCR5 = crate::Reg<u32, _TIM8_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR5;
#[doc = "`read()` method returns [tim8_ccr5::R](tim8_ccr5::R) reader structure"]
impl crate::Readable for TIM8_CCR5 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr5::W](tim8_ccr5::W) writer structure"]
impl crate::Writable for TIM8_CCR5 {}
#[doc = "TIM8 capture/compare register 5"]
pub mod tim8_ccr5;
#[doc = "TIM8 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_ccr6](tim8_ccr6) module"]
pub type TIM8_CCR6 = crate::Reg<u16, _TIM8_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_CCR6;
#[doc = "`read()` method returns [tim8_ccr6::R](tim8_ccr6::R) reader structure"]
impl crate::Readable for TIM8_CCR6 {}
#[doc = "`write(|w| ..)` method takes [tim8_ccr6::W](tim8_ccr6::W) writer structure"]
impl crate::Writable for TIM8_CCR6 {}
#[doc = "TIM8 capture/compare register 6"]
pub mod tim8_ccr6;
#[doc = "TIM8 Alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_af1](tim8_af1) module"]
pub type TIM8_AF1 = crate::Reg<u32, _TIM8_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_AF1;
#[doc = "`read()` method returns [tim8_af1::R](tim8_af1::R) reader structure"]
impl crate::Readable for TIM8_AF1 {}
#[doc = "`write(|w| ..)` method takes [tim8_af1::W](tim8_af1::W) writer structure"]
impl crate::Writable for TIM8_AF1 {}
#[doc = "TIM8 Alternate function option register 1"]
pub mod tim8_af1;
#[doc = "TIM8 Alternate function option register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_af2](tim8_af2) module"]
pub type TIM8_AF2 = crate::Reg<u32, _TIM8_AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_AF2;
#[doc = "`read()` method returns [tim8_af2::R](tim8_af2::R) reader structure"]
impl crate::Readable for TIM8_AF2 {}
#[doc = "`write(|w| ..)` method takes [tim8_af2::W](tim8_af2::W) writer structure"]
impl crate::Writable for TIM8_AF2 {}
#[doc = "TIM8 Alternate function option register 2"]
pub mod tim8_af2;
#[doc = "TIM8 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_tisel](tim8_tisel) module"]
pub type TIM8_TISEL = crate::Reg<u32, _TIM8_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM8_TISEL;
#[doc = "`read()` method returns [tim8_tisel::R](tim8_tisel::R) reader structure"]
impl crate::Readable for TIM8_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim8_tisel::W](tim8_tisel::W) writer structure"]
impl crate::Writable for TIM8_TISEL {}
#[doc = "TIM8 timer input selection register"]
pub mod tim8_tisel;
