#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM14 control register 1"]
    pub tim14_cr1: TIM14_CR1,
    _reserved1: [u8; 10usize],
    #[doc = "0x0c - TIM14 Interrupt enable register"]
    pub tim14_dier: TIM14_DIER,
    _reserved2: [u8; 2usize],
    #[doc = "0x10 - TIM14 status register"]
    pub tim14_sr: TIM14_SR,
    _reserved3: [u8; 2usize],
    #[doc = "0x14 - TIM14 event generation register"]
    pub tim14_egr: TIM14_EGR,
    _reserved4: [u8; 2usize],
    #[doc = "0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
    pub tim14_ccmr1: TIM14_CCMR1,
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - TIM14 capture/compare enable register"]
    pub tim14_ccer: TIM14_CCER,
    _reserved6: [u8; 2usize],
    #[doc = "0x24 - TIM14 counter"]
    pub tim14_cnt: TIM14_CNT,
    #[doc = "0x28 - TIM14 prescaler"]
    pub tim14_psc: TIM14_PSC,
    _reserved8: [u8; 2usize],
    #[doc = "0x2c - TIM14 auto-reload register"]
    pub tim14_arr: TIM14_ARR,
    _reserved9: [u8; 6usize],
    #[doc = "0x34 - TIM14 capture/compare register 1"]
    pub tim14_ccr1: TIM14_CCR1,
    _reserved10: [u8; 50usize],
    #[doc = "0x68 - TIM14 timer input selection register"]
    pub tim14_tisel: TIM14_TISEL,
}
#[doc = "TIM14 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_cr1](tim14_cr1) module"]
pub type TIM14_CR1 = crate::Reg<u16, _TIM14_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_CR1;
#[doc = "`read()` method returns [tim14_cr1::R](tim14_cr1::R) reader structure"]
impl crate::Readable for TIM14_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim14_cr1::W](tim14_cr1::W) writer structure"]
impl crate::Writable for TIM14_CR1 {}
#[doc = "TIM14 control register 1"]
pub mod tim14_cr1;
#[doc = "TIM14 Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_dier](tim14_dier) module"]
pub type TIM14_DIER = crate::Reg<u16, _TIM14_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_DIER;
#[doc = "`read()` method returns [tim14_dier::R](tim14_dier::R) reader structure"]
impl crate::Readable for TIM14_DIER {}
#[doc = "`write(|w| ..)` method takes [tim14_dier::W](tim14_dier::W) writer structure"]
impl crate::Writable for TIM14_DIER {}
#[doc = "TIM14 Interrupt enable register"]
pub mod tim14_dier;
#[doc = "TIM14 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_sr](tim14_sr) module"]
pub type TIM14_SR = crate::Reg<u16, _TIM14_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_SR;
#[doc = "`read()` method returns [tim14_sr::R](tim14_sr::R) reader structure"]
impl crate::Readable for TIM14_SR {}
#[doc = "`write(|w| ..)` method takes [tim14_sr::W](tim14_sr::W) writer structure"]
impl crate::Writable for TIM14_SR {}
#[doc = "TIM14 status register"]
pub mod tim14_sr;
#[doc = "TIM14 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_egr](tim14_egr) module"]
pub type TIM14_EGR = crate::Reg<u16, _TIM14_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_EGR;
#[doc = "`write(|w| ..)` method takes [tim14_egr::W](tim14_egr::W) writer structure"]
impl crate::Writable for TIM14_EGR {}
#[doc = "TIM14 event generation register"]
pub mod tim14_egr;
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_ccmr1](tim14_ccmr1) module"]
pub type TIM14_CCMR1 = crate::Reg<u32, _TIM14_CCMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_CCMR1;
#[doc = "`read()` method returns [tim14_ccmr1::R](tim14_ccmr1::R) reader structure"]
impl crate::Readable for TIM14_CCMR1 {}
#[doc = "`write(|w| ..)` method takes [tim14_ccmr1::W](tim14_ccmr1::W) writer structure"]
impl crate::Writable for TIM14_CCMR1 {}
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
pub mod tim14_ccmr1;
#[doc = "TIM14 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_ccer](tim14_ccer) module"]
pub type TIM14_CCER = crate::Reg<u16, _TIM14_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_CCER;
#[doc = "`read()` method returns [tim14_ccer::R](tim14_ccer::R) reader structure"]
impl crate::Readable for TIM14_CCER {}
#[doc = "`write(|w| ..)` method takes [tim14_ccer::W](tim14_ccer::W) writer structure"]
impl crate::Writable for TIM14_CCER {}
#[doc = "TIM14 capture/compare enable register"]
pub mod tim14_ccer;
#[doc = "TIM14 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_cnt](tim14_cnt) module"]
pub type TIM14_CNT = crate::Reg<u32, _TIM14_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_CNT;
#[doc = "`read()` method returns [tim14_cnt::R](tim14_cnt::R) reader structure"]
impl crate::Readable for TIM14_CNT {}
#[doc = "`write(|w| ..)` method takes [tim14_cnt::W](tim14_cnt::W) writer structure"]
impl crate::Writable for TIM14_CNT {}
#[doc = "TIM14 counter"]
pub mod tim14_cnt;
#[doc = "TIM14 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_psc](tim14_psc) module"]
pub type TIM14_PSC = crate::Reg<u16, _TIM14_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_PSC;
#[doc = "`read()` method returns [tim14_psc::R](tim14_psc::R) reader structure"]
impl crate::Readable for TIM14_PSC {}
#[doc = "`write(|w| ..)` method takes [tim14_psc::W](tim14_psc::W) writer structure"]
impl crate::Writable for TIM14_PSC {}
#[doc = "TIM14 prescaler"]
pub mod tim14_psc;
#[doc = "TIM14 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_arr](tim14_arr) module"]
pub type TIM14_ARR = crate::Reg<u16, _TIM14_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_ARR;
#[doc = "`read()` method returns [tim14_arr::R](tim14_arr::R) reader structure"]
impl crate::Readable for TIM14_ARR {}
#[doc = "`write(|w| ..)` method takes [tim14_arr::W](tim14_arr::W) writer structure"]
impl crate::Writable for TIM14_ARR {}
#[doc = "TIM14 auto-reload register"]
pub mod tim14_arr;
#[doc = "TIM14 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_ccr1](tim14_ccr1) module"]
pub type TIM14_CCR1 = crate::Reg<u16, _TIM14_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_CCR1;
#[doc = "`read()` method returns [tim14_ccr1::R](tim14_ccr1::R) reader structure"]
impl crate::Readable for TIM14_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim14_ccr1::W](tim14_ccr1::W) writer structure"]
impl crate::Writable for TIM14_CCR1 {}
#[doc = "TIM14 capture/compare register 1"]
pub mod tim14_ccr1;
#[doc = "TIM14 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_tisel](tim14_tisel) module"]
pub type TIM14_TISEL = crate::Reg<u16, _TIM14_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM14_TISEL;
#[doc = "`read()` method returns [tim14_tisel::R](tim14_tisel::R) reader structure"]
impl crate::Readable for TIM14_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim14_tisel::W](tim14_tisel::W) writer structure"]
impl crate::Writable for TIM14_TISEL {}
#[doc = "TIM14 timer input selection register"]
pub mod tim14_tisel;
