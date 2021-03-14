#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM13 control register 1"]
    pub tim13_cr1: TIM13_CR1,
    _reserved1: [u8; 10usize],
    #[doc = "0x0c - TIM13 Interrupt enable register"]
    pub tim13_dier: TIM13_DIER,
    _reserved2: [u8; 2usize],
    #[doc = "0x10 - TIM13 status register"]
    pub tim13_sr: TIM13_SR,
    _reserved3: [u8; 2usize],
    #[doc = "0x14 - TIM13 event generation register"]
    pub tim13_egr: TIM13_EGR,
    _reserved4: [u8; 2usize],
    #[doc = "0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
    pub tim13_ccmr1: TIM13_CCMR1,
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - TIM13 capture/compare enable register"]
    pub tim13_ccer: TIM13_CCER,
    _reserved6: [u8; 2usize],
    #[doc = "0x24 - TIM13 counter"]
    pub tim13_cnt: TIM13_CNT,
    #[doc = "0x28 - TIM13 prescaler"]
    pub tim13_psc: TIM13_PSC,
    _reserved8: [u8; 2usize],
    #[doc = "0x2c - TIM13 auto-reload register"]
    pub tim13_arr: TIM13_ARR,
    _reserved9: [u8; 6usize],
    #[doc = "0x34 - TIM13 capture/compare register 1"]
    pub tim13_ccr1: TIM13_CCR1,
    _reserved10: [u8; 50usize],
    #[doc = "0x68 - TIM13 timer input selection register"]
    pub tim13_tisel: TIM13_TISEL,
}
#[doc = "TIM13 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_cr1](tim13_cr1) module"]
pub type TIM13_CR1 = crate::Reg<u16, _TIM13_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_CR1;
#[doc = "`read()` method returns [tim13_cr1::R](tim13_cr1::R) reader structure"]
impl crate::Readable for TIM13_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim13_cr1::W](tim13_cr1::W) writer structure"]
impl crate::Writable for TIM13_CR1 {}
#[doc = "TIM13 control register 1"]
pub mod tim13_cr1;
#[doc = "TIM13 Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_dier](tim13_dier) module"]
pub type TIM13_DIER = crate::Reg<u16, _TIM13_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_DIER;
#[doc = "`read()` method returns [tim13_dier::R](tim13_dier::R) reader structure"]
impl crate::Readable for TIM13_DIER {}
#[doc = "`write(|w| ..)` method takes [tim13_dier::W](tim13_dier::W) writer structure"]
impl crate::Writable for TIM13_DIER {}
#[doc = "TIM13 Interrupt enable register"]
pub mod tim13_dier;
#[doc = "TIM13 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_sr](tim13_sr) module"]
pub type TIM13_SR = crate::Reg<u16, _TIM13_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_SR;
#[doc = "`read()` method returns [tim13_sr::R](tim13_sr::R) reader structure"]
impl crate::Readable for TIM13_SR {}
#[doc = "`write(|w| ..)` method takes [tim13_sr::W](tim13_sr::W) writer structure"]
impl crate::Writable for TIM13_SR {}
#[doc = "TIM13 status register"]
pub mod tim13_sr;
#[doc = "TIM13 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_egr](tim13_egr) module"]
pub type TIM13_EGR = crate::Reg<u16, _TIM13_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_EGR;
#[doc = "`write(|w| ..)` method takes [tim13_egr::W](tim13_egr::W) writer structure"]
impl crate::Writable for TIM13_EGR {}
#[doc = "TIM13 event generation register"]
pub mod tim13_egr;
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_ccmr1](tim13_ccmr1) module"]
pub type TIM13_CCMR1 = crate::Reg<u32, _TIM13_CCMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_CCMR1;
#[doc = "`read()` method returns [tim13_ccmr1::R](tim13_ccmr1::R) reader structure"]
impl crate::Readable for TIM13_CCMR1 {}
#[doc = "`write(|w| ..)` method takes [tim13_ccmr1::W](tim13_ccmr1::W) writer structure"]
impl crate::Writable for TIM13_CCMR1 {}
#[doc = "The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode"]
pub mod tim13_ccmr1;
#[doc = "TIM13 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_ccer](tim13_ccer) module"]
pub type TIM13_CCER = crate::Reg<u16, _TIM13_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_CCER;
#[doc = "`read()` method returns [tim13_ccer::R](tim13_ccer::R) reader structure"]
impl crate::Readable for TIM13_CCER {}
#[doc = "`write(|w| ..)` method takes [tim13_ccer::W](tim13_ccer::W) writer structure"]
impl crate::Writable for TIM13_CCER {}
#[doc = "TIM13 capture/compare enable register"]
pub mod tim13_ccer;
#[doc = "TIM13 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_cnt](tim13_cnt) module"]
pub type TIM13_CNT = crate::Reg<u32, _TIM13_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_CNT;
#[doc = "`read()` method returns [tim13_cnt::R](tim13_cnt::R) reader structure"]
impl crate::Readable for TIM13_CNT {}
#[doc = "`write(|w| ..)` method takes [tim13_cnt::W](tim13_cnt::W) writer structure"]
impl crate::Writable for TIM13_CNT {}
#[doc = "TIM13 counter"]
pub mod tim13_cnt;
#[doc = "TIM13 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_psc](tim13_psc) module"]
pub type TIM13_PSC = crate::Reg<u16, _TIM13_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_PSC;
#[doc = "`read()` method returns [tim13_psc::R](tim13_psc::R) reader structure"]
impl crate::Readable for TIM13_PSC {}
#[doc = "`write(|w| ..)` method takes [tim13_psc::W](tim13_psc::W) writer structure"]
impl crate::Writable for TIM13_PSC {}
#[doc = "TIM13 prescaler"]
pub mod tim13_psc;
#[doc = "TIM13 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_arr](tim13_arr) module"]
pub type TIM13_ARR = crate::Reg<u16, _TIM13_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_ARR;
#[doc = "`read()` method returns [tim13_arr::R](tim13_arr::R) reader structure"]
impl crate::Readable for TIM13_ARR {}
#[doc = "`write(|w| ..)` method takes [tim13_arr::W](tim13_arr::W) writer structure"]
impl crate::Writable for TIM13_ARR {}
#[doc = "TIM13 auto-reload register"]
pub mod tim13_arr;
#[doc = "TIM13 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_ccr1](tim13_ccr1) module"]
pub type TIM13_CCR1 = crate::Reg<u16, _TIM13_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_CCR1;
#[doc = "`read()` method returns [tim13_ccr1::R](tim13_ccr1::R) reader structure"]
impl crate::Readable for TIM13_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim13_ccr1::W](tim13_ccr1::W) writer structure"]
impl crate::Writable for TIM13_CCR1 {}
#[doc = "TIM13 capture/compare register 1"]
pub mod tim13_ccr1;
#[doc = "TIM13 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim13_tisel](tim13_tisel) module"]
pub type TIM13_TISEL = crate::Reg<u16, _TIM13_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM13_TISEL;
#[doc = "`read()` method returns [tim13_tisel::R](tim13_tisel::R) reader structure"]
impl crate::Readable for TIM13_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim13_tisel::W](tim13_tisel::W) writer structure"]
impl crate::Writable for TIM13_TISEL {}
#[doc = "TIM13 timer input selection register"]
pub mod tim13_tisel;
