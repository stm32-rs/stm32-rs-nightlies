#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - ADC control register"]
    pub cr: CR,
    #[doc = "0x0c - ADC configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - ADC sampling time register"]
    pub smpr: SMPR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - ADC watchdog threshold register"]
    pub awd1tr: AWD1TR,
    #[doc = "0x24 - ADC watchdog threshold register"]
    pub awd2tr: AWD2TR,
    _reserved_8_chselr0: [u8; 4usize],
    #[doc = "0x2c - ADC watchdog threshold register"]
    pub awd3tr: AWD3TR,
    _reserved10: [u8; 16usize],
    #[doc = "0x40 - ADC data register"]
    pub dr: DR,
    _reserved11: [u8; 92usize],
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    pub awd2cr: AWD2CR,
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    pub awd3cr: AWD3CR,
    _reserved13: [u8; 12usize],
    #[doc = "0xb4 - ADC Calibration factor"]
    pub calfact: CALFACT,
    _reserved14: [u8; 592usize],
    #[doc = "0x308 - ADC common configuration register"]
    pub ccr: CCR,
}
impl RegisterBlock {
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const CHSELR1) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn chselr1_mut(&self) -> &mut CHSELR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut CHSELR1) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const CHSELR0) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn chselr0_mut(&self) -> &mut CHSELR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut CHSELR0) }
    }
}
#[doc = "ADC interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "ADC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "ADC control register"]
pub mod cr;
#[doc = "ADC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "ADC configuration register 1"]
pub mod cfgr1;
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](smpr) module"]
pub type SMPR = crate::Reg<u32, _SMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR;
#[doc = "`read()` method returns [smpr::R](smpr::R) reader structure"]
impl crate::Readable for SMPR {}
#[doc = "`write(|w| ..)` method takes [smpr::W](smpr::W) writer structure"]
impl crate::Writable for SMPR {}
#[doc = "ADC sampling time register"]
pub mod smpr;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd1tr](awd1tr) module"]
pub type AWD1TR = crate::Reg<u32, _AWD1TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD1TR;
#[doc = "`read()` method returns [awd1tr::R](awd1tr::R) reader structure"]
impl crate::Readable for AWD1TR {}
#[doc = "`write(|w| ..)` method takes [awd1tr::W](awd1tr::W) writer structure"]
impl crate::Writable for AWD1TR {}
#[doc = "ADC watchdog threshold register"]
pub mod awd1tr;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2tr](awd2tr) module"]
pub type AWD2TR = crate::Reg<u32, _AWD2TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD2TR;
#[doc = "`read()` method returns [awd2tr::R](awd2tr::R) reader structure"]
impl crate::Readable for AWD2TR {}
#[doc = "`write(|w| ..)` method takes [awd2tr::W](awd2tr::W) writer structure"]
impl crate::Writable for AWD2TR {}
#[doc = "ADC watchdog threshold register"]
pub mod awd2tr;
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr0](chselr0) module"]
pub type CHSELR0 = crate::Reg<u32, _CHSELR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSELR0;
#[doc = "`read()` method returns [chselr0::R](chselr0::R) reader structure"]
impl crate::Readable for CHSELR0 {}
#[doc = "`write(|w| ..)` method takes [chselr0::W](chselr0::W) writer structure"]
impl crate::Writable for CHSELR0 {}
#[doc = "channel selection register"]
pub mod chselr0;
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr1](chselr1) module"]
pub type CHSELR1 = crate::Reg<u32, _CHSELR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSELR1;
#[doc = "`read()` method returns [chselr1::R](chselr1::R) reader structure"]
impl crate::Readable for CHSELR1 {}
#[doc = "`write(|w| ..)` method takes [chselr1::W](chselr1::W) writer structure"]
impl crate::Writable for CHSELR1 {}
#[doc = "channel selection register"]
pub mod chselr1;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3tr](awd3tr) module"]
pub type AWD3TR = crate::Reg<u32, _AWD3TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD3TR;
#[doc = "`read()` method returns [awd3tr::R](awd3tr::R) reader structure"]
impl crate::Readable for AWD3TR {}
#[doc = "`write(|w| ..)` method takes [awd3tr::W](awd3tr::W) writer structure"]
impl crate::Writable for AWD3TR {}
#[doc = "ADC watchdog threshold register"]
pub mod awd3tr;
#[doc = "ADC data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "ADC data register"]
pub mod dr;
#[doc = "ADC Analog Watchdog 2 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2cr](awd2cr) module"]
pub type AWD2CR = crate::Reg<u32, _AWD2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD2CR;
#[doc = "`read()` method returns [awd2cr::R](awd2cr::R) reader structure"]
impl crate::Readable for AWD2CR {}
#[doc = "`write(|w| ..)` method takes [awd2cr::W](awd2cr::W) writer structure"]
impl crate::Writable for AWD2CR {}
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod awd2cr;
#[doc = "ADC Analog Watchdog 3 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3cr](awd3cr) module"]
pub type AWD3CR = crate::Reg<u32, _AWD3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD3CR;
#[doc = "`read()` method returns [awd3cr::R](awd3cr::R) reader structure"]
impl crate::Readable for AWD3CR {}
#[doc = "`write(|w| ..)` method takes [awd3cr::W](awd3cr::W) writer structure"]
impl crate::Writable for AWD3CR {}
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod awd3cr;
#[doc = "ADC Calibration factor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact](calfact) module"]
pub type CALFACT = crate::Reg<u32, _CALFACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALFACT;
#[doc = "`read()` method returns [calfact::R](calfact::R) reader structure"]
impl crate::Readable for CALFACT {}
#[doc = "`write(|w| ..)` method takes [calfact::W](calfact::W) writer structure"]
impl crate::Writable for CALFACT {}
#[doc = "ADC Calibration factor"]
pub mod calfact;
#[doc = "ADC common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "ADC common configuration register"]
pub mod ccr;
