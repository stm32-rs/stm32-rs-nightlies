#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM12 control register 1"]
    pub tim12_cr1: TIM12_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM12 control register 2"]
    pub tim12_cr2: TIM12_CR2,
    #[doc = "0x08 - TIM12 slave mode control register"]
    pub tim12_smcr: TIM12_SMCR,
    #[doc = "0x0c - TIM12 interrupt enable register"]
    pub tim12_dier: TIM12_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM12 status register"]
    pub tim12_sr: TIM12_SR,
    #[doc = "0x14 - TIM12 event generation register"]
    pub tim12_egr: TIM12_EGR,
    _reserved6: [u8; 2usize],
    _reserved_6_tim12_ccmr1: [u8; 4usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - TIM12 capture/compare enable register"]
    pub tim12_ccer: TIM12_CCER,
    #[doc = "0x24 - TIM12 counter"]
    pub tim12_cnt: TIM12_CNT,
    #[doc = "0x28 - TIM12 prescaler"]
    pub tim12_psc: TIM12_PSC,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - TIM12 auto-reload register"]
    pub tim12_arr: TIM12_ARR,
    _reserved11: [u8; 6usize],
    #[doc = "0x34 - TIM12 capture/compare register 1"]
    pub tim12_ccr1: TIM12_CCR1,
    _reserved12: [u8; 2usize],
    #[doc = "0x38 - TIM12 capture/compare register 2"]
    pub tim12_ccr2: TIM12_CCR2,
    _reserved13: [u8; 46usize],
    #[doc = "0x68 - TIM12 timer input selection register"]
    pub tim12_tisel: TIM12_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_output(&self) -> &TIM12_CCMR1_OUTPUT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize) as *const TIM12_CCMR1_OUTPUT)
        }
    }
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_output_mut(&self) -> &mut TIM12_CCMR1_OUTPUT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIM12_CCMR1_OUTPUT)
        }
    }
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_input(&self) -> &TIM12_CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIM12_CCMR1_INPUT) }
    }
    #[doc = "0x18 - TIM12 capture/compare mode register 1"]
    #[inline(always)]
    pub fn tim12_ccmr1_input_mut(&self) -> &mut TIM12_CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIM12_CCMR1_INPUT) }
    }
}
#[doc = "TIM12 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_cr1](tim12_cr1) module"]
pub type TIM12_CR1 = crate::Reg<u16, _TIM12_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CR1;
#[doc = "`read()` method returns [tim12_cr1::R](tim12_cr1::R) reader structure"]
impl crate::Readable for TIM12_CR1 {}
#[doc = "`write(|w| ..)` method takes [tim12_cr1::W](tim12_cr1::W) writer structure"]
impl crate::Writable for TIM12_CR1 {}
#[doc = "TIM12 control register 1"]
pub mod tim12_cr1;
#[doc = "TIM12 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_cr2](tim12_cr2) module"]
pub type TIM12_CR2 = crate::Reg<u32, _TIM12_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CR2;
#[doc = "`read()` method returns [tim12_cr2::R](tim12_cr2::R) reader structure"]
impl crate::Readable for TIM12_CR2 {}
#[doc = "`write(|w| ..)` method takes [tim12_cr2::W](tim12_cr2::W) writer structure"]
impl crate::Writable for TIM12_CR2 {}
#[doc = "TIM12 control register 2"]
pub mod tim12_cr2;
#[doc = "TIM12 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_smcr](tim12_smcr) module"]
pub type TIM12_SMCR = crate::Reg<u32, _TIM12_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_SMCR;
#[doc = "`read()` method returns [tim12_smcr::R](tim12_smcr::R) reader structure"]
impl crate::Readable for TIM12_SMCR {}
#[doc = "`write(|w| ..)` method takes [tim12_smcr::W](tim12_smcr::W) writer structure"]
impl crate::Writable for TIM12_SMCR {}
#[doc = "TIM12 slave mode control register"]
pub mod tim12_smcr;
#[doc = "TIM12 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_dier](tim12_dier) module"]
pub type TIM12_DIER = crate::Reg<u16, _TIM12_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_DIER;
#[doc = "`read()` method returns [tim12_dier::R](tim12_dier::R) reader structure"]
impl crate::Readable for TIM12_DIER {}
#[doc = "`write(|w| ..)` method takes [tim12_dier::W](tim12_dier::W) writer structure"]
impl crate::Writable for TIM12_DIER {}
#[doc = "TIM12 interrupt enable register"]
pub mod tim12_dier;
#[doc = "TIM12 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_sr](tim12_sr) module"]
pub type TIM12_SR = crate::Reg<u32, _TIM12_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_SR;
#[doc = "`read()` method returns [tim12_sr::R](tim12_sr::R) reader structure"]
impl crate::Readable for TIM12_SR {}
#[doc = "`write(|w| ..)` method takes [tim12_sr::W](tim12_sr::W) writer structure"]
impl crate::Writable for TIM12_SR {}
#[doc = "TIM12 status register"]
pub mod tim12_sr;
#[doc = "TIM12 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_egr](tim12_egr) module"]
pub type TIM12_EGR = crate::Reg<u16, _TIM12_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_EGR;
#[doc = "`write(|w| ..)` method takes [tim12_egr::W](tim12_egr::W) writer structure"]
impl crate::Writable for TIM12_EGR {}
#[doc = "TIM12 event generation register"]
pub mod tim12_egr;
#[doc = "TIM12 capture/compare mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_ccmr1_input](tim12_ccmr1_input) module"]
pub type TIM12_CCMR1_INPUT = crate::Reg<u32, _TIM12_CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CCMR1_INPUT;
#[doc = "`read()` method returns [tim12_ccmr1_input::R](tim12_ccmr1_input::R) reader structure"]
impl crate::Readable for TIM12_CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [tim12_ccmr1_input::W](tim12_ccmr1_input::W) writer structure"]
impl crate::Writable for TIM12_CCMR1_INPUT {}
#[doc = "TIM12 capture/compare mode register 1"]
pub mod tim12_ccmr1_input;
#[doc = "TIM12 capture/compare mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_ccmr1_output](tim12_ccmr1_output) module"]
pub type TIM12_CCMR1_OUTPUT = crate::Reg<u32, _TIM12_CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CCMR1_OUTPUT;
#[doc = "`read()` method returns [tim12_ccmr1_output::R](tim12_ccmr1_output::R) reader structure"]
impl crate::Readable for TIM12_CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [tim12_ccmr1_output::W](tim12_ccmr1_output::W) writer structure"]
impl crate::Writable for TIM12_CCMR1_OUTPUT {}
#[doc = "TIM12 capture/compare mode register 1"]
pub mod tim12_ccmr1_output;
#[doc = "TIM12 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_ccer](tim12_ccer) module"]
pub type TIM12_CCER = crate::Reg<u32, _TIM12_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CCER;
#[doc = "`read()` method returns [tim12_ccer::R](tim12_ccer::R) reader structure"]
impl crate::Readable for TIM12_CCER {}
#[doc = "`write(|w| ..)` method takes [tim12_ccer::W](tim12_ccer::W) writer structure"]
impl crate::Writable for TIM12_CCER {}
#[doc = "TIM12 capture/compare enable register"]
pub mod tim12_ccer;
#[doc = "TIM12 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_cnt](tim12_cnt) module"]
pub type TIM12_CNT = crate::Reg<u32, _TIM12_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CNT;
#[doc = "`read()` method returns [tim12_cnt::R](tim12_cnt::R) reader structure"]
impl crate::Readable for TIM12_CNT {}
#[doc = "`write(|w| ..)` method takes [tim12_cnt::W](tim12_cnt::W) writer structure"]
impl crate::Writable for TIM12_CNT {}
#[doc = "TIM12 counter"]
pub mod tim12_cnt;
#[doc = "TIM12 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_psc](tim12_psc) module"]
pub type TIM12_PSC = crate::Reg<u16, _TIM12_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_PSC;
#[doc = "`read()` method returns [tim12_psc::R](tim12_psc::R) reader structure"]
impl crate::Readable for TIM12_PSC {}
#[doc = "`write(|w| ..)` method takes [tim12_psc::W](tim12_psc::W) writer structure"]
impl crate::Writable for TIM12_PSC {}
#[doc = "TIM12 prescaler"]
pub mod tim12_psc;
#[doc = "TIM12 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_arr](tim12_arr) module"]
pub type TIM12_ARR = crate::Reg<u16, _TIM12_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_ARR;
#[doc = "`read()` method returns [tim12_arr::R](tim12_arr::R) reader structure"]
impl crate::Readable for TIM12_ARR {}
#[doc = "`write(|w| ..)` method takes [tim12_arr::W](tim12_arr::W) writer structure"]
impl crate::Writable for TIM12_ARR {}
#[doc = "TIM12 auto-reload register"]
pub mod tim12_arr;
#[doc = "TIM12 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_ccr1](tim12_ccr1) module"]
pub type TIM12_CCR1 = crate::Reg<u16, _TIM12_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CCR1;
#[doc = "`read()` method returns [tim12_ccr1::R](tim12_ccr1::R) reader structure"]
impl crate::Readable for TIM12_CCR1 {}
#[doc = "`write(|w| ..)` method takes [tim12_ccr1::W](tim12_ccr1::W) writer structure"]
impl crate::Writable for TIM12_CCR1 {}
#[doc = "TIM12 capture/compare register 1"]
pub mod tim12_ccr1;
#[doc = "TIM12 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_ccr2](tim12_ccr2) module"]
pub type TIM12_CCR2 = crate::Reg<u16, _TIM12_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_CCR2;
#[doc = "`read()` method returns [tim12_ccr2::R](tim12_ccr2::R) reader structure"]
impl crate::Readable for TIM12_CCR2 {}
#[doc = "`write(|w| ..)` method takes [tim12_ccr2::W](tim12_ccr2::W) writer structure"]
impl crate::Writable for TIM12_CCR2 {}
#[doc = "TIM12 capture/compare register 2"]
pub mod tim12_ccr2;
#[doc = "TIM12 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_tisel](tim12_tisel) module"]
pub type TIM12_TISEL = crate::Reg<u32, _TIM12_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM12_TISEL;
#[doc = "`read()` method returns [tim12_tisel::R](tim12_tisel::R) reader structure"]
impl crate::Readable for TIM12_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim12_tisel::W](tim12_tisel::W) writer structure"]
impl crate::Writable for TIM12_TISEL {}
#[doc = "TIM12 timer input selection register"]
pub mod tim12_tisel;
