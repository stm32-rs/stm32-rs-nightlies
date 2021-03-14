#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM16/TIM17 control register 1"]
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM16/TIM17 control register 2"]
    pub timx_cr2: TIMX_CR2,
    _reserved2: [u8; 6usize],
    #[doc = "0x0c - TIM16/TIM17 DMA/interrupt enable register"]
    pub timx_dier: TIMX_DIER,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - TIM16/TIM17 status register"]
    pub timx_sr: TIMX_SR,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - TIM16/TIM17 capture/compare enable register"]
    pub timx_ccer: TIMX_CCER,
    _reserved6: [u8; 2usize],
    #[doc = "0x24 - TIM16/TIM17 counter"]
    pub timx_cnt: TIMX_CNT,
    #[doc = "0x28 - TIM16/TIM17 prescaler"]
    pub timx_psc: TIMX_PSC,
    _reserved8: [u8; 2usize],
    #[doc = "0x2c - TIM16/TIM17 auto-reload register"]
    pub timx_arr: TIMX_ARR,
    _reserved9: [u8; 2usize],
    #[doc = "0x30 - TIM16/TIM17 repetition counter register"]
    pub timx_rcr: TIMX_RCR,
    _reserved10: [u8; 2usize],
    #[doc = "0x34 - TIM16/TIM17 capture/compare register 1"]
    pub timx_ccr1: TIMX_CCR1,
    _reserved11: [u8; 14usize],
    #[doc = "0x44 - As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub timx_bdtr: TIMX_BDTR,
    #[doc = "0x48 - TIM16/TIM17 DMA control register"]
    pub timx_dcr: TIMX_DCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x4c - TIM16/TIM17 DMA address for full transfer"]
    pub timx_dmar: TIMX_DMAR,
    _reserved14: [u8; 18usize],
    #[doc = "0x60 - TIM17 alternate function register 1"]
    pub timx_af1: TIMX_AF1,
    _reserved15: [u8; 4usize],
    #[doc = "0x68 - TIM17 input selection register"]
    pub timx_tisel: TIMX_TISEL,
}
#[doc = "TIM16/TIM17 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr1](timx_cr1) module"]
pub type TIMX_CR1 = crate::Reg<u16, _TIMX_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR1;
#[doc = "`read()` method returns [timx_cr1::R](timx_cr1::R) reader structure"]
impl crate::Readable for TIMX_CR1 {}
#[doc = "`write(|w| ..)` method takes [timx_cr1::W](timx_cr1::W) writer structure"]
impl crate::Writable for TIMX_CR1 {}
#[doc = "TIM16/TIM17 control register 1"]
pub mod timx_cr1;
#[doc = "TIM16/TIM17 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr2](timx_cr2) module"]
pub type TIMX_CR2 = crate::Reg<u16, _TIMX_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR2;
#[doc = "`read()` method returns [timx_cr2::R](timx_cr2::R) reader structure"]
impl crate::Readable for TIMX_CR2 {}
#[doc = "`write(|w| ..)` method takes [timx_cr2::W](timx_cr2::W) writer structure"]
impl crate::Writable for TIMX_CR2 {}
#[doc = "TIM16/TIM17 control register 2"]
pub mod timx_cr2;
#[doc = "TIM16/TIM17 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dier](timx_dier) module"]
pub type TIMX_DIER = crate::Reg<u16, _TIMX_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DIER;
#[doc = "`read()` method returns [timx_dier::R](timx_dier::R) reader structure"]
impl crate::Readable for TIMX_DIER {}
#[doc = "`write(|w| ..)` method takes [timx_dier::W](timx_dier::W) writer structure"]
impl crate::Writable for TIMX_DIER {}
#[doc = "TIM16/TIM17 DMA/interrupt enable register"]
pub mod timx_dier;
#[doc = "TIM16/TIM17 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_sr](timx_sr) module"]
pub type TIMX_SR = crate::Reg<u16, _TIMX_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SR;
#[doc = "`read()` method returns [timx_sr::R](timx_sr::R) reader structure"]
impl crate::Readable for TIMX_SR {}
#[doc = "`write(|w| ..)` method takes [timx_sr::W](timx_sr::W) writer structure"]
impl crate::Writable for TIMX_SR {}
#[doc = "TIM16/TIM17 status register"]
pub mod timx_sr;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](timx_egr) module"]
pub type TIMX_EGR = crate::Reg<u32, _TIMX_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_EGR;
#[doc = "`write(|w| ..)` method takes [timx_egr::W](timx_egr::W) writer structure"]
impl crate::Writable for TIMX_EGR {}
#[doc = "event generation register"]
pub mod timx_egr;
#[doc = "TIM16/TIM17 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccer](timx_ccer) module"]
pub type TIMX_CCER = crate::Reg<u16, _TIMX_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCER;
#[doc = "`read()` method returns [timx_ccer::R](timx_ccer::R) reader structure"]
impl crate::Readable for TIMX_CCER {}
#[doc = "`write(|w| ..)` method takes [timx_ccer::W](timx_ccer::W) writer structure"]
impl crate::Writable for TIMX_CCER {}
#[doc = "TIM16/TIM17 capture/compare enable register"]
pub mod timx_ccer;
#[doc = "TIM16/TIM17 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cnt](timx_cnt) module"]
pub type TIMX_CNT = crate::Reg<u32, _TIMX_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CNT;
#[doc = "`read()` method returns [timx_cnt::R](timx_cnt::R) reader structure"]
impl crate::Readable for TIMX_CNT {}
#[doc = "`write(|w| ..)` method takes [timx_cnt::W](timx_cnt::W) writer structure"]
impl crate::Writable for TIMX_CNT {}
#[doc = "TIM16/TIM17 counter"]
pub mod timx_cnt;
#[doc = "TIM16/TIM17 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_psc](timx_psc) module"]
pub type TIMX_PSC = crate::Reg<u16, _TIMX_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_PSC;
#[doc = "`read()` method returns [timx_psc::R](timx_psc::R) reader structure"]
impl crate::Readable for TIMX_PSC {}
#[doc = "`write(|w| ..)` method takes [timx_psc::W](timx_psc::W) writer structure"]
impl crate::Writable for TIMX_PSC {}
#[doc = "TIM16/TIM17 prescaler"]
pub mod timx_psc;
#[doc = "TIM16/TIM17 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_arr](timx_arr) module"]
pub type TIMX_ARR = crate::Reg<u16, _TIMX_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_ARR;
#[doc = "`read()` method returns [timx_arr::R](timx_arr::R) reader structure"]
impl crate::Readable for TIMX_ARR {}
#[doc = "`write(|w| ..)` method takes [timx_arr::W](timx_arr::W) writer structure"]
impl crate::Writable for TIMX_ARR {}
#[doc = "TIM16/TIM17 auto-reload register"]
pub mod timx_arr;
#[doc = "TIM16/TIM17 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_rcr](timx_rcr) module"]
pub type TIMX_RCR = crate::Reg<u16, _TIMX_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_RCR;
#[doc = "`read()` method returns [timx_rcr::R](timx_rcr::R) reader structure"]
impl crate::Readable for TIMX_RCR {}
#[doc = "`write(|w| ..)` method takes [timx_rcr::W](timx_rcr::W) writer structure"]
impl crate::Writable for TIMX_RCR {}
#[doc = "TIM16/TIM17 repetition counter register"]
pub mod timx_rcr;
#[doc = "TIM16/TIM17 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr1](timx_ccr1) module"]
pub type TIMX_CCR1 = crate::Reg<u16, _TIMX_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR1;
#[doc = "`read()` method returns [timx_ccr1::R](timx_ccr1::R) reader structure"]
impl crate::Readable for TIMX_CCR1 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr1::W](timx_ccr1::W) writer structure"]
impl crate::Writable for TIMX_CCR1 {}
#[doc = "TIM16/TIM17 capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_bdtr](timx_bdtr) module"]
pub type TIMX_BDTR = crate::Reg<u32, _TIMX_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_BDTR;
#[doc = "`read()` method returns [timx_bdtr::R](timx_bdtr::R) reader structure"]
impl crate::Readable for TIMX_BDTR {}
#[doc = "`write(|w| ..)` method takes [timx_bdtr::W](timx_bdtr::W) writer structure"]
impl crate::Writable for TIMX_BDTR {}
#[doc = "As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIM16/TIM17 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dcr](timx_dcr) module"]
pub type TIMX_DCR = crate::Reg<u16, _TIMX_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DCR;
#[doc = "`read()` method returns [timx_dcr::R](timx_dcr::R) reader structure"]
impl crate::Readable for TIMX_DCR {}
#[doc = "`write(|w| ..)` method takes [timx_dcr::W](timx_dcr::W) writer structure"]
impl crate::Writable for TIMX_DCR {}
#[doc = "TIM16/TIM17 DMA control register"]
pub mod timx_dcr;
#[doc = "TIM16/TIM17 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dmar](timx_dmar) module"]
pub type TIMX_DMAR = crate::Reg<u16, _TIMX_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DMAR;
#[doc = "`read()` method returns [timx_dmar::R](timx_dmar::R) reader structure"]
impl crate::Readable for TIMX_DMAR {}
#[doc = "`write(|w| ..)` method takes [timx_dmar::W](timx_dmar::W) writer structure"]
impl crate::Writable for TIMX_DMAR {}
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod timx_dmar;
#[doc = "TIM17 alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_af1](timx_af1) module"]
pub type TIMX_AF1 = crate::Reg<u32, _TIMX_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_AF1;
#[doc = "`read()` method returns [timx_af1::R](timx_af1::R) reader structure"]
impl crate::Readable for TIMX_AF1 {}
#[doc = "`write(|w| ..)` method takes [timx_af1::W](timx_af1::W) writer structure"]
impl crate::Writable for TIMX_AF1 {}
#[doc = "TIM17 alternate function register 1"]
pub mod timx_af1;
#[doc = "TIM17 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](timx_tisel) module"]
pub type TIMX_TISEL = crate::Reg<u32, _TIMX_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_TISEL;
#[doc = "`read()` method returns [timx_tisel::R](timx_tisel::R) reader structure"]
impl crate::Readable for TIMX_TISEL {}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](timx_tisel::W) writer structure"]
impl crate::Writable for TIMX_TISEL {}
#[doc = "TIM17 input selection register"]
pub mod timx_tisel;
