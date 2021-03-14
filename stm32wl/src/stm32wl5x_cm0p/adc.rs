#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub adc_isr: ADC_ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub adc_ier: ADC_IER,
    #[doc = "0x08 - ADC control register"]
    pub adc_cr: ADC_CR,
    #[doc = "0x0c - ADC configuration register 1"]
    pub adc_cfgr1: ADC_CFGR1,
    #[doc = "0x10 - ADC configuration register 2"]
    pub adc_cfgr2: ADC_CFGR2,
    #[doc = "0x14 - ADC sampling time register"]
    pub adc_smpr: ADC_SMPR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - ADC watchdog threshold register"]
    pub adc_awd1tr: ADC_AWD1TR,
    #[doc = "0x24 - ADC watchdog threshold register"]
    pub adc_awd2tr: ADC_AWD2TR,
    _reserved_8_adc_chselr: [u8; 4usize],
    #[doc = "0x2c - ADC watchdog threshold register"]
    pub adc_awd3tr: ADC_AWD3TR,
    _reserved10: [u8; 16usize],
    #[doc = "0x40 - ADC data register"]
    pub adc_dr: ADC_DR,
    _reserved11: [u8; 92usize],
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    pub adc_awd2cr: ADC_AWD2CR,
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    pub adc_awd3cr: ADC_AWD3CR,
    _reserved13: [u8; 12usize],
    #[doc = "0xb4 - ADC Calibration factor"]
    pub adc_calfact: ADC_CALFACT,
    _reserved14: [u8; 592usize],
    #[doc = "0x308 - ADC common configuration register"]
    pub adc_ccr: ADC_CCR,
}
impl RegisterBlock {
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn adc_chselr1(&self) -> &ADC_CHSELR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const ADC_CHSELR1) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn adc_chselr1_mut(&self) -> &mut ADC_CHSELR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut ADC_CHSELR1) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn adc_chselr0(&self) -> &ADC_CHSELR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const ADC_CHSELR0) }
    }
    #[doc = "0x28 - channel selection register"]
    #[inline(always)]
    pub fn adc_chselr0_mut(&self) -> &mut ADC_CHSELR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut ADC_CHSELR0) }
    }
}
#[doc = "ADC interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_isr](adc_isr) module"]
pub type ADC_ISR = crate::Reg<u32, _ADC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_ISR;
#[doc = "`read()` method returns [adc_isr::R](adc_isr::R) reader structure"]
impl crate::Readable for ADC_ISR {}
#[doc = "`write(|w| ..)` method takes [adc_isr::W](adc_isr::W) writer structure"]
impl crate::Writable for ADC_ISR {}
#[doc = "ADC interrupt and status register"]
pub mod adc_isr;
#[doc = "ADC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ier](adc_ier) module"]
pub type ADC_IER = crate::Reg<u32, _ADC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IER;
#[doc = "`read()` method returns [adc_ier::R](adc_ier::R) reader structure"]
impl crate::Readable for ADC_IER {}
#[doc = "`write(|w| ..)` method takes [adc_ier::W](adc_ier::W) writer structure"]
impl crate::Writable for ADC_IER {}
#[doc = "ADC interrupt enable register"]
pub mod adc_ier;
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cr](adc_cr) module"]
pub type ADC_CR = crate::Reg<u32, _ADC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CR;
#[doc = "`read()` method returns [adc_cr::R](adc_cr::R) reader structure"]
impl crate::Readable for ADC_CR {}
#[doc = "`write(|w| ..)` method takes [adc_cr::W](adc_cr::W) writer structure"]
impl crate::Writable for ADC_CR {}
#[doc = "ADC control register"]
pub mod adc_cr;
#[doc = "ADC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr1](adc_cfgr1) module"]
pub type ADC_CFGR1 = crate::Reg<u32, _ADC_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CFGR1;
#[doc = "`read()` method returns [adc_cfgr1::R](adc_cfgr1::R) reader structure"]
impl crate::Readable for ADC_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [adc_cfgr1::W](adc_cfgr1::W) writer structure"]
impl crate::Writable for ADC_CFGR1 {}
#[doc = "ADC configuration register 1"]
pub mod adc_cfgr1;
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr2](adc_cfgr2) module"]
pub type ADC_CFGR2 = crate::Reg<u32, _ADC_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CFGR2;
#[doc = "`read()` method returns [adc_cfgr2::R](adc_cfgr2::R) reader structure"]
impl crate::Readable for ADC_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [adc_cfgr2::W](adc_cfgr2::W) writer structure"]
impl crate::Writable for ADC_CFGR2 {}
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr](adc_smpr) module"]
pub type ADC_SMPR = crate::Reg<u32, _ADC_SMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SMPR;
#[doc = "`read()` method returns [adc_smpr::R](adc_smpr::R) reader structure"]
impl crate::Readable for ADC_SMPR {}
#[doc = "`write(|w| ..)` method takes [adc_smpr::W](adc_smpr::W) writer structure"]
impl crate::Writable for ADC_SMPR {}
#[doc = "ADC sampling time register"]
pub mod adc_smpr;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd1tr](adc_awd1tr) module"]
pub type ADC_AWD1TR = crate::Reg<u32, _ADC_AWD1TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD1TR;
#[doc = "`read()` method returns [adc_awd1tr::R](adc_awd1tr::R) reader structure"]
impl crate::Readable for ADC_AWD1TR {}
#[doc = "`write(|w| ..)` method takes [adc_awd1tr::W](adc_awd1tr::W) writer structure"]
impl crate::Writable for ADC_AWD1TR {}
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd1tr;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd2tr](adc_awd2tr) module"]
pub type ADC_AWD2TR = crate::Reg<u32, _ADC_AWD2TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD2TR;
#[doc = "`read()` method returns [adc_awd2tr::R](adc_awd2tr::R) reader structure"]
impl crate::Readable for ADC_AWD2TR {}
#[doc = "`write(|w| ..)` method takes [adc_awd2tr::W](adc_awd2tr::W) writer structure"]
impl crate::Writable for ADC_AWD2TR {}
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd2tr;
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_chselr0](adc_chselr0) module"]
pub type ADC_CHSELR0 = crate::Reg<u32, _ADC_CHSELR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CHSELR0;
#[doc = "`read()` method returns [adc_chselr0::R](adc_chselr0::R) reader structure"]
impl crate::Readable for ADC_CHSELR0 {}
#[doc = "`write(|w| ..)` method takes [adc_chselr0::W](adc_chselr0::W) writer structure"]
impl crate::Writable for ADC_CHSELR0 {}
#[doc = "channel selection register"]
pub mod adc_chselr0;
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_chselr1](adc_chselr1) module"]
pub type ADC_CHSELR1 = crate::Reg<u32, _ADC_CHSELR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CHSELR1;
#[doc = "`read()` method returns [adc_chselr1::R](adc_chselr1::R) reader structure"]
impl crate::Readable for ADC_CHSELR1 {}
#[doc = "`write(|w| ..)` method takes [adc_chselr1::W](adc_chselr1::W) writer structure"]
impl crate::Writable for ADC_CHSELR1 {}
#[doc = "channel selection register"]
pub mod adc_chselr1;
#[doc = "ADC watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd3tr](adc_awd3tr) module"]
pub type ADC_AWD3TR = crate::Reg<u32, _ADC_AWD3TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD3TR;
#[doc = "`read()` method returns [adc_awd3tr::R](adc_awd3tr::R) reader structure"]
impl crate::Readable for ADC_AWD3TR {}
#[doc = "`write(|w| ..)` method takes [adc_awd3tr::W](adc_awd3tr::W) writer structure"]
impl crate::Writable for ADC_AWD3TR {}
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd3tr;
#[doc = "ADC data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr](adc_dr) module"]
pub type ADC_DR = crate::Reg<u32, _ADC_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR;
#[doc = "`read()` method returns [adc_dr::R](adc_dr::R) reader structure"]
impl crate::Readable for ADC_DR {}
#[doc = "ADC data register"]
pub mod adc_dr;
#[doc = "ADC Analog Watchdog 2 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd2cr](adc_awd2cr) module"]
pub type ADC_AWD2CR = crate::Reg<u32, _ADC_AWD2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD2CR;
#[doc = "`read()` method returns [adc_awd2cr::R](adc_awd2cr::R) reader structure"]
impl crate::Readable for ADC_AWD2CR {}
#[doc = "`write(|w| ..)` method takes [adc_awd2cr::W](adc_awd2cr::W) writer structure"]
impl crate::Writable for ADC_AWD2CR {}
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC Analog Watchdog 3 Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd3cr](adc_awd3cr) module"]
pub type ADC_AWD3CR = crate::Reg<u32, _ADC_AWD3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD3CR;
#[doc = "`read()` method returns [adc_awd3cr::R](adc_awd3cr::R) reader structure"]
impl crate::Readable for ADC_AWD3CR {}
#[doc = "`write(|w| ..)` method takes [adc_awd3cr::W](adc_awd3cr::W) writer structure"]
impl crate::Writable for ADC_AWD3CR {}
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC Calibration factor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact](adc_calfact) module"]
pub type ADC_CALFACT = crate::Reg<u32, _ADC_CALFACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CALFACT;
#[doc = "`read()` method returns [adc_calfact::R](adc_calfact::R) reader structure"]
impl crate::Readable for ADC_CALFACT {}
#[doc = "`write(|w| ..)` method takes [adc_calfact::W](adc_calfact::W) writer structure"]
impl crate::Writable for ADC_CALFACT {}
#[doc = "ADC Calibration factor"]
pub mod adc_calfact;
#[doc = "ADC common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ccr](adc_ccr) module"]
pub type ADC_CCR = crate::Reg<u32, _ADC_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CCR;
#[doc = "`read()` method returns [adc_ccr::R](adc_ccr::R) reader structure"]
impl crate::Readable for ADC_CCR {}
#[doc = "`write(|w| ..)` method takes [adc_ccr::W](adc_ccr::W) writer structure"]
impl crate::Writable for ADC_CCR {}
#[doc = "ADC common configuration register"]
pub mod adc_ccr;
