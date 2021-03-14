#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM15 control register 1"]
    pub tim15_cr1: TIM15_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM15 control register 2"]
    pub tim15_cr2: TIM15_CR2,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - slave mode control register"]
    pub timx_smcr: TIMX_SMCR,
    #[doc = "0x0c - TIM15 DMA/interrupt enable register"]
    pub tim15_dier: TIM15_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM15 status register"]
    pub tim15_sr: TIM15_SR,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved_6_timx_ccmr1: [u8; 4usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - TIM15 capture/compare enable register"]
    pub tim15_ccer: TIM15_CCER,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - TIM15 counter"]
    pub tim15_cnt: TIM15_CNT,
    #[doc = "0x28 - TIM15 prescaler"]
    pub tim15_psc: TIM15_PSC,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - TIM15 auto-reload register"]
    pub tim15_arr: TIM15_ARR,
    _reserved11: [u8; 2usize],
    #[doc = "0x30 - TIM15 repetition counter register"]
    pub tim15_rcr: TIM15_RCR,
    _reserved12: [u8; 2usize],
    #[doc = "0x34 - TIM15 capture/compare register 1"]
    pub tim15_ccr1: TIM15_CCR1,
    _reserved13: [u8; 2usize],
    #[doc = "0x38 - TIM15 capture/compare register 2"]
    pub tim15_ccr2: TIM15_CCR2,
    _reserved14: [u8; 10usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub timx_bdtr: TIMX_BDTR,
    #[doc = "0x48 - TIM15 DMA control register"]
    pub tim15_dcr: TIM15_DCR,
    _reserved16: [u8; 2usize],
    #[doc = "0x4c - TIM15 DMA address for full transfer"]
    pub tim15_dmar: TIM15_DMAR,
    _reserved17: [u8; 18usize],
    #[doc = "0x60 - TIM15 alternate register 1"]
    pub tim15_af1: TIM15_AF1,
    _reserved18: [u8; 4usize],
    #[doc = "0x68 - TIM15 input selection register"]
    pub tim15_tisel: TIM15_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input(&self) -> &TIMX_CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMX_CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input_mut(&self) -> &mut TIMX_CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMX_CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_output(&self) -> &TIMX_CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMX_CCMR1_OUTPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_output_mut(&self) -> &mut TIMX_CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMX_CCMR1_OUTPUT) }
    }
}
#[doc = "TIM15 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_cr1](tim15_cr1) module"]
pub type TIM15_CR1 = crate::Reg<u16, _TIM15_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CR1;
#[doc = "`read()` method returns [tim15_cr1::R](tim15_cr1::R) reader structure"]
impl crate::Readable for TIM15_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim15_cr1::W](tim15_cr1::W) writer structure"]
impl crate::Writable for TIM15_CR1 {}
#[doc = "TIM15 control register 1"]
pub mod tim15_cr1;
#[doc = "TIM15 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_cr2](tim15_cr2) module"]
pub type TIM15_CR2 = crate::Reg<u16, _TIM15_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CR2;
#[doc = "`read()` method returns [tim15_cr2::R](tim15_cr2::R) reader structure"]
impl crate::Readable for TIM15_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim15_cr2::W](tim15_cr2::W) writer structure"]
impl crate::Writable for TIM15_CR2 {}
#[doc = "TIM15 control register 2"]
pub mod tim15_cr2;
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_smcr](timx_smcr) module"]
pub type TIMX_SMCR = crate::Reg<u32, _TIMX_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SMCR;
#[doc = "`read()` method returns [timx_smcr::R](timx_smcr::R) reader structure"]
impl crate::Readable for TIMX_SMCR {}
#[doc = "`write(|w| ..)` method takes [timx_smcr::W](timx_smcr::W) writer structure"]
impl crate::Writable for TIMX_SMCR {}
#[doc = "slave mode control register"]
pub mod timx_smcr;
#[doc = "TIM15 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_dier](tim15_dier) module"]
pub type TIM15_DIER = crate::Reg<u16, _TIM15_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_DIER;
#[doc = "`read()` method returns [tim15_dier::R](tim15_dier::R) reader structure"]
impl crate::Readable for TIM15_DIER {}
#[doc = "`write(|w| ..)` method takes [tim15_dier::W](tim15_dier::W) writer structure"]
impl crate::Writable for TIM15_DIER {}
#[doc = "TIM15 DMA/interrupt enable register"]
pub mod tim15_dier;
#[doc = "TIM15 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_sr](tim15_sr) module"]
pub type TIM15_SR = crate::Reg<u16, _TIM15_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_SR;
#[doc = "`read()` method returns [tim15_sr::R](tim15_sr::R) reader structure"]
impl crate::Readable for TIM15_SR {}
#[doc = "`write(|w| ..)` method takes [tim15_sr::W](tim15_sr::W) writer structure"]
impl crate::Writable for TIM15_SR {}
#[doc = "TIM15 status register"]
pub mod tim15_sr;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](timx_egr) module"]
pub type TIMX_EGR = crate::Reg<u32, _TIMX_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_EGR;
#[doc = "`write(|w| ..)` method takes [timx_egr::W](timx_egr::W) writer structure"]
impl crate::Writable for TIMX_EGR {}
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr1_output](timx_ccmr1_output) module"]
pub type TIMX_CCMR1_OUTPUT = crate::Reg<u32, _TIMX_CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR1_OUTPUT;
#[doc = "`read()` method returns [timx_ccmr1_output::R](timx_ccmr1_output::R) reader structure"]
impl crate::Readable for TIMX_CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr1_output::W](timx_ccmr1_output::W) writer structure"]
impl crate::Writable for TIMX_CCMR1_OUTPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod timx_ccmr1_output;
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr1_input](timx_ccmr1_input) module"]
pub type TIMX_CCMR1_INPUT = crate::Reg<u32, _TIMX_CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR1_INPUT;
#[doc = "`read()` method returns [timx_ccmr1_input::R](timx_ccmr1_input::R) reader structure"]
impl crate::Readable for TIMX_CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr1_input::W](timx_ccmr1_input::W) writer structure"]
impl crate::Writable for TIMX_CCMR1_INPUT {}
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod timx_ccmr1_input;
#[doc = "TIM15 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_ccer](tim15_ccer) module"]
pub type TIM15_CCER = crate::Reg<u16, _TIM15_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CCER;
#[doc = "`read()` method returns [tim15_ccer::R](tim15_ccer::R) reader structure"]
impl crate::Readable for TIM15_CCER {}
#[doc = "`write(|w| ..)` method takes [tim15_ccer::W](tim15_ccer::W) writer structure"]
impl crate::Writable for TIM15_CCER {}
#[doc = "TIM15 capture/compare enable register"]
pub mod tim15_ccer;
#[doc = "TIM15 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_cnt](tim15_cnt) module"]
pub type TIM15_CNT = crate::Reg<u32, _TIM15_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CNT;
#[doc = "`read()` method returns [tim15_cnt::R](tim15_cnt::R) reader structure"]
impl crate::Readable for TIM15_CNT {}
#[doc = "`write(|w| ..)` method takes [tim15_cnt::W](tim15_cnt::W) writer structure"]
impl crate::Writable for TIM15_CNT {}
#[doc = "TIM15 counter"]
pub mod tim15_cnt;
#[doc = "TIM15 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_psc](tim15_psc) module"]
pub type TIM15_PSC = crate::Reg<u16, _TIM15_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_PSC;
#[doc = "`read()` method returns [tim15_psc::R](tim15_psc::R) reader structure"]
impl crate::Readable for TIM15_PSC {}
#[doc = "`write(|w| ..)` method takes [tim15_psc::W](tim15_psc::W) writer structure"]
impl crate::Writable for TIM15_PSC {}
#[doc = "TIM15 prescaler"]
pub mod tim15_psc;
#[doc = "TIM15 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_arr](tim15_arr) module"]
pub type TIM15_ARR = crate::Reg<u16, _TIM15_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_ARR;
#[doc = "`read()` method returns [tim15_arr::R](tim15_arr::R) reader structure"]
impl crate::Readable for TIM15_ARR {}
#[doc = "`write(|w| ..)` method takes [tim15_arr::W](tim15_arr::W) writer structure"]
impl crate::Writable for TIM15_ARR {}
#[doc = "TIM15 auto-reload register"]
pub mod tim15_arr;
#[doc = "TIM15 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_rcr](tim15_rcr) module"]
pub type TIM15_RCR = crate::Reg<u16, _TIM15_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_RCR;
#[doc = "`read()` method returns [tim15_rcr::R](tim15_rcr::R) reader structure"]
impl crate::Readable for TIM15_RCR {}
#[doc = "`write(|w| ..)` method takes [tim15_rcr::W](tim15_rcr::W) writer structure"]
impl crate::Writable for TIM15_RCR {}
#[doc = "TIM15 repetition counter register"]
pub mod tim15_rcr;
#[doc = "TIM15 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_ccr1](tim15_ccr1) module"]
pub type TIM15_CCR1 = crate::Reg<u16, _TIM15_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CCR1;
#[doc = "`read()` method returns [tim15_ccr1::R](tim15_ccr1::R) reader structure"]
impl crate::Readable for TIM15_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim15_ccr1::W](tim15_ccr1::W) writer structure"]
impl crate::Writable for TIM15_CCR1 {}
#[doc = "TIM15 capture/compare register 1"]
pub mod tim15_ccr1;
#[doc = "TIM15 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_ccr2](tim15_ccr2) module"]
pub type TIM15_CCR2 = crate::Reg<u16, _TIM15_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_CCR2;
#[doc = "`read()` method returns [tim15_ccr2::R](tim15_ccr2::R) reader structure"]
impl crate::Readable for TIM15_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim15_ccr2::W](tim15_ccr2::W) writer structure"]
impl crate::Writable for TIM15_CCR2 {}
#[doc = "TIM15 capture/compare register 2"]
pub mod tim15_ccr2;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_bdtr](timx_bdtr) module"]
pub type TIMX_BDTR = crate::Reg<u32, _TIMX_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_BDTR;
#[doc = "`read()` method returns [timx_bdtr::R](timx_bdtr::R) reader structure"]
impl crate::Readable for TIMX_BDTR {}
#[doc = "`write(|w| ..)` method takes [timx_bdtr::W](timx_bdtr::W) writer structure"]
impl crate::Writable for TIMX_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIM15 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_dcr](tim15_dcr) module"]
pub type TIM15_DCR = crate::Reg<u16, _TIM15_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_DCR;
#[doc = "`read()` method returns [tim15_dcr::R](tim15_dcr::R) reader structure"]
impl crate::Readable for TIM15_DCR {}
#[doc = "`write(|w| ..)` method takes [tim15_dcr::W](tim15_dcr::W) writer structure"]
impl crate::Writable for TIM15_DCR {}
#[doc = "TIM15 DMA control register"]
pub mod tim15_dcr;
#[doc = "TIM15 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_dmar](tim15_dmar) module"]
pub type TIM15_DMAR = crate::Reg<u16, _TIM15_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_DMAR;
#[doc = "`read()` method returns [tim15_dmar::R](tim15_dmar::R) reader structure"]
impl crate::Readable for TIM15_DMAR {}
#[doc = "`write(|w| ..)` method takes [tim15_dmar::W](tim15_dmar::W) writer structure"]
impl crate::Writable for TIM15_DMAR {}
#[doc = "TIM15 DMA address for full transfer"]
pub mod tim15_dmar;
#[doc = "TIM15 alternate register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_af1](tim15_af1) module"]
pub type TIM15_AF1 = crate::Reg<u32, _TIM15_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_AF1;
#[doc = "`read()` method returns [tim15_af1::R](tim15_af1::R) reader structure"]
impl crate::Readable for TIM15_AF1 {}
#[doc = "`write(|w| ..)` method takes [tim15_af1::W](tim15_af1::W) writer structure"]
impl crate::Writable for TIM15_AF1 {}
#[doc = "TIM15 alternate register 1"]
pub mod tim15_af1;
#[doc = "TIM15 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_tisel](tim15_tisel) module"]
pub type TIM15_TISEL = crate::Reg<u32, _TIM15_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM15_TISEL;
#[doc = "`read()` method returns [tim15_tisel::R](tim15_tisel::R) reader structure"]
impl crate::Readable for TIM15_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim15_tisel::W](tim15_tisel::W) writer structure"]
impl crate::Writable for TIM15_TISEL {}
#[doc = "TIM15 input selection register"]
pub mod tim15_tisel;
