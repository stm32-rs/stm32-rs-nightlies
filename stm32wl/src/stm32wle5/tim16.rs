#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM16/TIM17 control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - TIM16/TIM17 control register 2"]
    pub cr2: CR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - TIM16/TIM17 DMA/interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - TIM16/TIM17 status register"]
    pub sr: SR,
    #[doc = "0x14 - TIM16/TIM17 event generation register"]
    pub egr: EGR,
    _reserved_5_ccmr1: [u8; 4usize],
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - TIM16/TIM17 capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - TIM16/TIM17 counter"]
    pub cnt: CNT,
    #[doc = "0x28 - TIM16/TIM17 prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - TIM16/TIM17 auto-reload register"]
    pub arr: ARR,
    #[doc = "0x30 - TIM16/TIM17 repetition counter register"]
    pub rcr: RCR,
    #[doc = "0x34 - TIM16/TIM17 capture/compare register 1"]
    pub ccr1: CCR1,
    _reserved12: [u8; 12usize],
    #[doc = "0x44 - TIM16/TIM17 break and dead-time register"]
    pub bdtr: BDTR,
    #[doc = "0x48 - TIM16/TIM17 DMA control register"]
    pub dcr: DCR,
    #[doc = "0x4c - TIM16/TIM17 DMA address for full transfer"]
    pub dmar: DMAR,
    #[doc = "0x50 - TIM16 option register 1"]
    pub tim16_or1: TIM16_OR1,
    _reserved16: [u8; 12usize],
    #[doc = "0x60 - TIM16 alternate function register 1"]
    pub tim16_af1: TIM16_AF1,
    _reserved17: [u8; 4usize],
    #[doc = "0x68 - TIM16 input selection register"]
    pub tim16_tisel: TIM16_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_input_mut(&self) -> &mut CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_INPUT) }
    }
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    #[doc = "0x18 - TIM16/TIM17 capture/compare mode register 1"]
    #[inline(always)]
    pub fn ccmr1_output_mut(&self) -> &mut CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_OUTPUT) }
    }
}
#[doc = "TIM16/TIM17 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "TIM16/TIM17 control register 1"]
pub mod cr1;
#[doc = "TIM16/TIM17 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "TIM16/TIM17 control register 2"]
pub mod cr2;
#[doc = "TIM16/TIM17 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dier](dier) module"]
pub type DIER = crate::Reg<u32, _DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIER;
#[doc = "`read()` method returns [dier::R](dier::R) reader structure"]
impl crate::Readable for DIER {}
#[doc = "`write(|w| ..)` method takes [dier::W](dier::W) writer structure"]
impl crate::Writable for DIER {}
#[doc = "TIM16/TIM17 DMA/interrupt enable register"]
pub mod dier;
#[doc = "TIM16/TIM17 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "TIM16/TIM17 status register"]
pub mod sr;
#[doc = "TIM16/TIM17 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](egr) module"]
pub type EGR = crate::Reg<u32, _EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EGR;
#[doc = "`write(|w| ..)` method takes [egr::W](egr::W) writer structure"]
impl crate::Writable for EGR {}
#[doc = "TIM16/TIM17 event generation register"]
pub mod egr;
#[doc = "TIM16/TIM17 capture/compare mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_output](ccmr1_output) module"]
pub type CCMR1_OUTPUT = crate::Reg<u32, _CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_OUTPUT;
#[doc = "`read()` method returns [ccmr1_output::R](ccmr1_output::R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_output::W](ccmr1_output::W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT {}
#[doc = "TIM16/TIM17 capture/compare mode register 1"]
pub mod ccmr1_output;
#[doc = "TIM16/TIM17 capture/compare mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](ccmr1_input) module"]
pub type CCMR1_INPUT = crate::Reg<u32, _CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_INPUT;
#[doc = "`read()` method returns [ccmr1_input::R](ccmr1_input::R) reader structure"]
impl crate::Readable for CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](ccmr1_input::W) writer structure"]
impl crate::Writable for CCMR1_INPUT {}
#[doc = "TIM16/TIM17 capture/compare mode register 1"]
pub mod ccmr1_input;
#[doc = "TIM16/TIM17 capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](ccer) module"]
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
#[doc = "`read()` method returns [ccer::R](ccer::R) reader structure"]
impl crate::Readable for CCER {}
#[doc = "`write(|w| ..)` method takes [ccer::W](ccer::W) writer structure"]
impl crate::Writable for CCER {}
#[doc = "TIM16/TIM17 capture/compare enable register"]
pub mod ccer;
#[doc = "TIM16/TIM17 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "TIM16/TIM17 counter"]
pub mod cnt;
#[doc = "TIM16/TIM17 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "TIM16/TIM17 prescaler"]
pub mod psc;
#[doc = "TIM16/TIM17 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](arr) module"]
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
#[doc = "`read()` method returns [arr::R](arr::R) reader structure"]
impl crate::Readable for ARR {}
#[doc = "`write(|w| ..)` method takes [arr::W](arr::W) writer structure"]
impl crate::Writable for ARR {}
#[doc = "TIM16/TIM17 auto-reload register"]
pub mod arr;
#[doc = "TIM16/TIM17 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "TIM16/TIM17 repetition counter register"]
pub mod rcr;
#[doc = "TIM16/TIM17 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](ccr1) module"]
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
#[doc = "`read()` method returns [ccr1::R](ccr1::R) reader structure"]
impl crate::Readable for CCR1 {}
#[doc = "`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure"]
impl crate::Writable for CCR1 {}
#[doc = "TIM16/TIM17 capture/compare register 1"]
pub mod ccr1;
#[doc = "TIM16/TIM17 break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtr](bdtr) module"]
pub type BDTR = crate::Reg<u32, _BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTR;
#[doc = "`read()` method returns [bdtr::R](bdtr::R) reader structure"]
impl crate::Readable for BDTR {}
#[doc = "`write(|w| ..)` method takes [bdtr::W](bdtr::W) writer structure"]
impl crate::Writable for BDTR {}
#[doc = "TIM16/TIM17 break and dead-time register"]
pub mod bdtr;
#[doc = "TIM16/TIM17 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](dcr) module"]
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
#[doc = "`read()` method returns [dcr::R](dcr::R) reader structure"]
impl crate::Readable for DCR {}
#[doc = "`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure"]
impl crate::Writable for DCR {}
#[doc = "TIM16/TIM17 DMA control register"]
pub mod dcr;
#[doc = "TIM16/TIM17 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmar](dmar) module"]
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
#[doc = "`read()` method returns [dmar::R](dmar::R) reader structure"]
impl crate::Readable for DMAR {}
#[doc = "`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure"]
impl crate::Writable for DMAR {}
#[doc = "TIM16/TIM17 DMA address for full transfer"]
pub mod dmar;
#[doc = "TIM16 option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim16_or1](tim16_or1) module"]
pub type TIM16_OR1 = crate::Reg<u32, _TIM16_OR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM16_OR1;
#[doc = "`read()` method returns [tim16_or1::R](tim16_or1::R) reader structure"]
impl crate::Readable for TIM16_OR1 {}
#[doc = "`write(|w| ..)` method takes [tim16_or1::W](tim16_or1::W) writer structure"]
impl crate::Writable for TIM16_OR1 {}
#[doc = "TIM16 option register 1"]
pub mod tim16_or1;
#[doc = "TIM16 alternate function register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim16_af1](tim16_af1) module"]
pub type TIM16_AF1 = crate::Reg<u32, _TIM16_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM16_AF1;
#[doc = "`read()` method returns [tim16_af1::R](tim16_af1::R) reader structure"]
impl crate::Readable for TIM16_AF1 {}
#[doc = "`write(|w| ..)` method takes [tim16_af1::W](tim16_af1::W) writer structure"]
impl crate::Writable for TIM16_AF1 {}
#[doc = "TIM16 alternate function register 1"]
pub mod tim16_af1;
#[doc = "TIM16 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim16_tisel](tim16_tisel) module"]
pub type TIM16_TISEL = crate::Reg<u32, _TIM16_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIM16_TISEL;
#[doc = "`read()` method returns [tim16_tisel::R](tim16_tisel::R) reader structure"]
impl crate::Readable for TIM16_TISEL {}
#[doc = "`write(|w| ..)` method takes [tim16_tisel::W](tim16_tisel::W) writer structure"]
impl crate::Writable for TIM16_TISEL {}
#[doc = "TIM16 input selection register"]
pub mod tim16_tisel;
