#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub adc_isr: ADC_ISR,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub adc_ier: ADC_IER,
    #[doc = "0x08 - ADC control register"]
    pub adc_cr: ADC_CR,
    #[doc = "0x0c - ADC configuration register"]
    pub adc_cfgr: ADC_CFGR,
    #[doc = "0x10 - ADC configuration register 2"]
    pub adc_cfgr2: ADC_CFGR2,
    #[doc = "0x14 - ADC sample time register 1"]
    pub adc_smpr1: ADC_SMPR1,
    #[doc = "0x18 - ADC sample time register 2"]
    pub adc_smpr2: ADC_SMPR2,
    #[doc = "0x1c - ADC channel preselection register"]
    pub adc_pcsel: ADC_PCSEL,
    #[doc = "0x20 - ADC watchdog threshold register 1"]
    pub adc_ltr1: ADC_LTR1,
    #[doc = "0x24 - ADC watchdog threshold register 1"]
    pub adc_htr1: ADC_HTR1,
    _reserved10: [u8; 8usize],
    #[doc = "0x30 - ADC regular sequence register 1"]
    pub adc_sqr1: ADC_SQR1,
    #[doc = "0x34 - ADC regular sequence register 2"]
    pub adc_sqr2: ADC_SQR2,
    #[doc = "0x38 - ADC regular sequence register 3"]
    pub adc_sqr3: ADC_SQR3,
    #[doc = "0x3c - ADC regular sequence register 4"]
    pub adc_sqr4: ADC_SQR4,
    #[doc = "0x40 - ADC regular Data Register"]
    pub adc_dr: ADC_DR,
    _reserved15: [u8; 8usize],
    #[doc = "0x4c - ADC injected sequence register"]
    pub adc_jsqr: ADC_JSQR,
    _reserved16: [u8; 16usize],
    #[doc = "0x60 - ADC offset register"]
    pub adc_ofr1: ADC_OFR1,
    #[doc = "0x64 - ADC offset register"]
    pub adc_ofr2: ADC_OFR2,
    #[doc = "0x68 - ADC offset register"]
    pub adc_ofr3: ADC_OFR3,
    #[doc = "0x6c - ADC offset register"]
    pub adc_ofr4: ADC_OFR4,
    _reserved20: [u8; 16usize],
    #[doc = "0x80 - ADC injected data register"]
    pub adc_jdr1: ADC_JDR1,
    #[doc = "0x84 - ADC injected data register"]
    pub adc_jdr2: ADC_JDR2,
    #[doc = "0x88 - ADC injected data register"]
    pub adc_jdr3: ADC_JDR3,
    #[doc = "0x8c - ADC injected data register"]
    pub adc_jdr4: ADC_JDR4,
    _reserved24: [u8; 16usize],
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    pub adc_awd2cr: ADC_AWD2CR,
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    pub adc_awd3cr: ADC_AWD3CR,
    _reserved26: [u8; 8usize],
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    pub adc_ltr2: ADC_LTR2,
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    pub adc_htr2: ADC_HTR2,
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    pub adc_ltr3: ADC_LTR3,
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    pub adc_htr3: ADC_HTR3,
    #[doc = "0xc0 - ADC differential mode selection register"]
    pub adc_difsel: ADC_DIFSEL,
    #[doc = "0xc4 - ADC calibration factors register"]
    pub adc_calfact: ADC_CALFACT,
    #[doc = "0xc8 - ADC calibration factor register 2"]
    pub adc_calfact2: ADC_CALFACT2,
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
#[doc = "ADC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr](adc_cfgr) module"]
pub type ADC_CFGR = crate::Reg<u32, _ADC_CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CFGR;
#[doc = "`read()` method returns [adc_cfgr::R](adc_cfgr::R) reader structure"]
impl crate::Readable for ADC_CFGR {}
#[doc = "`write(|w| ..)` method takes [adc_cfgr::W](adc_cfgr::W) writer structure"]
impl crate::Writable for ADC_CFGR {}
#[doc = "ADC configuration register"]
pub mod adc_cfgr;
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
#[doc = "ADC sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr1](adc_smpr1) module"]
pub type ADC_SMPR1 = crate::Reg<u32, _ADC_SMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SMPR1;
#[doc = "`read()` method returns [adc_smpr1::R](adc_smpr1::R) reader structure"]
impl crate::Readable for ADC_SMPR1 {}
#[doc = "`write(|w| ..)` method takes [adc_smpr1::W](adc_smpr1::W) writer structure"]
impl crate::Writable for ADC_SMPR1 {}
#[doc = "ADC sample time register 1"]
pub mod adc_smpr1;
#[doc = "ADC sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr2](adc_smpr2) module"]
pub type ADC_SMPR2 = crate::Reg<u32, _ADC_SMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SMPR2;
#[doc = "`read()` method returns [adc_smpr2::R](adc_smpr2::R) reader structure"]
impl crate::Readable for ADC_SMPR2 {}
#[doc = "`write(|w| ..)` method takes [adc_smpr2::W](adc_smpr2::W) writer structure"]
impl crate::Writable for ADC_SMPR2 {}
#[doc = "ADC sample time register 2"]
pub mod adc_smpr2;
#[doc = "ADC channel preselection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_pcsel](adc_pcsel) module"]
pub type ADC_PCSEL = crate::Reg<u32, _ADC_PCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_PCSEL;
#[doc = "`read()` method returns [adc_pcsel::R](adc_pcsel::R) reader structure"]
impl crate::Readable for ADC_PCSEL {}
#[doc = "`write(|w| ..)` method takes [adc_pcsel::W](adc_pcsel::W) writer structure"]
impl crate::Writable for ADC_PCSEL {}
#[doc = "ADC channel preselection register"]
pub mod adc_pcsel;
#[doc = "ADC watchdog threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr1](adc_ltr1) module"]
pub type ADC_LTR1 = crate::Reg<u32, _ADC_LTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LTR1;
#[doc = "`read()` method returns [adc_ltr1::R](adc_ltr1::R) reader structure"]
impl crate::Readable for ADC_LTR1 {}
#[doc = "`write(|w| ..)` method takes [adc_ltr1::W](adc_ltr1::W) writer structure"]
impl crate::Writable for ADC_LTR1 {}
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_ltr1;
#[doc = "ADC watchdog threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htr1](adc_htr1) module"]
pub type ADC_HTR1 = crate::Reg<u32, _ADC_HTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HTR1;
#[doc = "`read()` method returns [adc_htr1::R](adc_htr1::R) reader structure"]
impl crate::Readable for ADC_HTR1 {}
#[doc = "`write(|w| ..)` method takes [adc_htr1::W](adc_htr1::W) writer structure"]
impl crate::Writable for ADC_HTR1 {}
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_htr1;
#[doc = "ADC regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sqr1](adc_sqr1) module"]
pub type ADC_SQR1 = crate::Reg<u32, _ADC_SQR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SQR1;
#[doc = "`read()` method returns [adc_sqr1::R](adc_sqr1::R) reader structure"]
impl crate::Readable for ADC_SQR1 {}
#[doc = "`write(|w| ..)` method takes [adc_sqr1::W](adc_sqr1::W) writer structure"]
impl crate::Writable for ADC_SQR1 {}
#[doc = "ADC regular sequence register 1"]
pub mod adc_sqr1;
#[doc = "ADC regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sqr2](adc_sqr2) module"]
pub type ADC_SQR2 = crate::Reg<u32, _ADC_SQR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SQR2;
#[doc = "`read()` method returns [adc_sqr2::R](adc_sqr2::R) reader structure"]
impl crate::Readable for ADC_SQR2 {}
#[doc = "`write(|w| ..)` method takes [adc_sqr2::W](adc_sqr2::W) writer structure"]
impl crate::Writable for ADC_SQR2 {}
#[doc = "ADC regular sequence register 2"]
pub mod adc_sqr2;
#[doc = "ADC regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sqr3](adc_sqr3) module"]
pub type ADC_SQR3 = crate::Reg<u32, _ADC_SQR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SQR3;
#[doc = "`read()` method returns [adc_sqr3::R](adc_sqr3::R) reader structure"]
impl crate::Readable for ADC_SQR3 {}
#[doc = "`write(|w| ..)` method takes [adc_sqr3::W](adc_sqr3::W) writer structure"]
impl crate::Writable for ADC_SQR3 {}
#[doc = "ADC regular sequence register 3"]
pub mod adc_sqr3;
#[doc = "ADC regular sequence register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sqr4](adc_sqr4) module"]
pub type ADC_SQR4 = crate::Reg<u32, _ADC_SQR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_SQR4;
#[doc = "`read()` method returns [adc_sqr4::R](adc_sqr4::R) reader structure"]
impl crate::Readable for ADC_SQR4 {}
#[doc = "`write(|w| ..)` method takes [adc_sqr4::W](adc_sqr4::W) writer structure"]
impl crate::Writable for ADC_SQR4 {}
#[doc = "ADC regular sequence register 4"]
pub mod adc_sqr4;
#[doc = "ADC regular Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr](adc_dr) module"]
pub type ADC_DR = crate::Reg<u32, _ADC_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR;
#[doc = "`read()` method returns [adc_dr::R](adc_dr::R) reader structure"]
impl crate::Readable for ADC_DR {}
#[doc = "ADC regular Data Register"]
pub mod adc_dr;
#[doc = "ADC injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jsqr](adc_jsqr) module"]
pub type ADC_JSQR = crate::Reg<u32, _ADC_JSQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_JSQR;
#[doc = "`read()` method returns [adc_jsqr::R](adc_jsqr::R) reader structure"]
impl crate::Readable for ADC_JSQR {}
#[doc = "`write(|w| ..)` method takes [adc_jsqr::W](adc_jsqr::W) writer structure"]
impl crate::Writable for ADC_JSQR {}
#[doc = "ADC injected sequence register"]
pub mod adc_jsqr;
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr1](adc_ofr1) module"]
pub type ADC_OFR1 = crate::Reg<u32, _ADC_OFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR1;
#[doc = "`read()` method returns [adc_ofr1::R](adc_ofr1::R) reader structure"]
impl crate::Readable for ADC_OFR1 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr1::W](adc_ofr1::W) writer structure"]
impl crate::Writable for ADC_OFR1 {}
#[doc = "ADC offset register"]
pub mod adc_ofr1;
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr2](adc_ofr2) module"]
pub type ADC_OFR2 = crate::Reg<u32, _ADC_OFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR2;
#[doc = "`read()` method returns [adc_ofr2::R](adc_ofr2::R) reader structure"]
impl crate::Readable for ADC_OFR2 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr2::W](adc_ofr2::W) writer structure"]
impl crate::Writable for ADC_OFR2 {}
#[doc = "ADC offset register"]
pub mod adc_ofr2;
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr3](adc_ofr3) module"]
pub type ADC_OFR3 = crate::Reg<u32, _ADC_OFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR3;
#[doc = "`read()` method returns [adc_ofr3::R](adc_ofr3::R) reader structure"]
impl crate::Readable for ADC_OFR3 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr3::W](adc_ofr3::W) writer structure"]
impl crate::Writable for ADC_OFR3 {}
#[doc = "ADC offset register"]
pub mod adc_ofr3;
#[doc = "ADC offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr4](adc_ofr4) module"]
pub type ADC_OFR4 = crate::Reg<u32, _ADC_OFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_OFR4;
#[doc = "`read()` method returns [adc_ofr4::R](adc_ofr4::R) reader structure"]
impl crate::Readable for ADC_OFR4 {}
#[doc = "`write(|w| ..)` method takes [adc_ofr4::W](adc_ofr4::W) writer structure"]
impl crate::Writable for ADC_OFR4 {}
#[doc = "ADC offset register"]
pub mod adc_ofr4;
#[doc = "ADC injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdr1](adc_jdr1) module"]
pub type ADC_JDR1 = crate::Reg<u32, _ADC_JDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_JDR1;
#[doc = "`read()` method returns [adc_jdr1::R](adc_jdr1::R) reader structure"]
impl crate::Readable for ADC_JDR1 {}
#[doc = "ADC injected data register"]
pub mod adc_jdr1;
#[doc = "ADC injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdr2](adc_jdr2) module"]
pub type ADC_JDR2 = crate::Reg<u32, _ADC_JDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_JDR2;
#[doc = "`read()` method returns [adc_jdr2::R](adc_jdr2::R) reader structure"]
impl crate::Readable for ADC_JDR2 {}
#[doc = "ADC injected data register"]
pub mod adc_jdr2;
#[doc = "ADC injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdr3](adc_jdr3) module"]
pub type ADC_JDR3 = crate::Reg<u32, _ADC_JDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_JDR3;
#[doc = "`read()` method returns [adc_jdr3::R](adc_jdr3::R) reader structure"]
impl crate::Readable for ADC_JDR3 {}
#[doc = "ADC injected data register"]
pub mod adc_jdr3;
#[doc = "ADC injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdr4](adc_jdr4) module"]
pub type ADC_JDR4 = crate::Reg<u32, _ADC_JDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_JDR4;
#[doc = "`read()` method returns [adc_jdr4::R](adc_jdr4::R) reader structure"]
impl crate::Readable for ADC_JDR4 {}
#[doc = "ADC injected data register"]
pub mod adc_jdr4;
#[doc = "ADC analog watchdog 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd2cr](adc_awd2cr) module"]
pub type ADC_AWD2CR = crate::Reg<u32, _ADC_AWD2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD2CR;
#[doc = "`read()` method returns [adc_awd2cr::R](adc_awd2cr::R) reader structure"]
impl crate::Readable for ADC_AWD2CR {}
#[doc = "`write(|w| ..)` method takes [adc_awd2cr::W](adc_awd2cr::W) writer structure"]
impl crate::Writable for ADC_AWD2CR {}
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC analog watchdog 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_awd3cr](adc_awd3cr) module"]
pub type ADC_AWD3CR = crate::Reg<u32, _ADC_AWD3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_AWD3CR;
#[doc = "`read()` method returns [adc_awd3cr::R](adc_awd3cr::R) reader structure"]
impl crate::Readable for ADC_AWD3CR {}
#[doc = "`write(|w| ..)` method takes [adc_awd3cr::W](adc_awd3cr::W) writer structure"]
impl crate::Writable for ADC_AWD3CR {}
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC watchdog lower threshold register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr2](adc_ltr2) module"]
pub type ADC_LTR2 = crate::Reg<u32, _ADC_LTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LTR2;
#[doc = "`read()` method returns [adc_ltr2::R](adc_ltr2::R) reader structure"]
impl crate::Readable for ADC_LTR2 {}
#[doc = "`write(|w| ..)` method takes [adc_ltr2::W](adc_ltr2::W) writer structure"]
impl crate::Writable for ADC_LTR2 {}
#[doc = "ADC watchdog lower threshold register 2"]
pub mod adc_ltr2;
#[doc = "ADC watchdog higher threshold register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htr2](adc_htr2) module"]
pub type ADC_HTR2 = crate::Reg<u32, _ADC_HTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HTR2;
#[doc = "`read()` method returns [adc_htr2::R](adc_htr2::R) reader structure"]
impl crate::Readable for ADC_HTR2 {}
#[doc = "`write(|w| ..)` method takes [adc_htr2::W](adc_htr2::W) writer structure"]
impl crate::Writable for ADC_HTR2 {}
#[doc = "ADC watchdog higher threshold register 2"]
pub mod adc_htr2;
#[doc = "ADC watchdog lower threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr3](adc_ltr3) module"]
pub type ADC_LTR3 = crate::Reg<u32, _ADC_LTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LTR3;
#[doc = "`read()` method returns [adc_ltr3::R](adc_ltr3::R) reader structure"]
impl crate::Readable for ADC_LTR3 {}
#[doc = "`write(|w| ..)` method takes [adc_ltr3::W](adc_ltr3::W) writer structure"]
impl crate::Writable for ADC_LTR3 {}
#[doc = "ADC watchdog lower threshold register 3"]
pub mod adc_ltr3;
#[doc = "ADC watchdog higher threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htr3](adc_htr3) module"]
pub type ADC_HTR3 = crate::Reg<u32, _ADC_HTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_HTR3;
#[doc = "`read()` method returns [adc_htr3::R](adc_htr3::R) reader structure"]
impl crate::Readable for ADC_HTR3 {}
#[doc = "`write(|w| ..)` method takes [adc_htr3::W](adc_htr3::W) writer structure"]
impl crate::Writable for ADC_HTR3 {}
#[doc = "ADC watchdog higher threshold register 3"]
pub mod adc_htr3;
#[doc = "ADC differential mode selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_difsel](adc_difsel) module"]
pub type ADC_DIFSEL = crate::Reg<u32, _ADC_DIFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DIFSEL;
#[doc = "`read()` method returns [adc_difsel::R](adc_difsel::R) reader structure"]
impl crate::Readable for ADC_DIFSEL {}
#[doc = "`write(|w| ..)` method takes [adc_difsel::W](adc_difsel::W) writer structure"]
impl crate::Writable for ADC_DIFSEL {}
#[doc = "ADC differential mode selection register"]
pub mod adc_difsel;
#[doc = "ADC calibration factors register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact](adc_calfact) module"]
pub type ADC_CALFACT = crate::Reg<u32, _ADC_CALFACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CALFACT;
#[doc = "`read()` method returns [adc_calfact::R](adc_calfact::R) reader structure"]
impl crate::Readable for ADC_CALFACT {}
#[doc = "`write(|w| ..)` method takes [adc_calfact::W](adc_calfact::W) writer structure"]
impl crate::Writable for ADC_CALFACT {}
#[doc = "ADC calibration factors register"]
pub mod adc_calfact;
#[doc = "ADC calibration factor register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact2](adc_calfact2) module"]
pub type ADC_CALFACT2 = crate::Reg<u32, _ADC_CALFACT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CALFACT2;
#[doc = "`read()` method returns [adc_calfact2::R](adc_calfact2::R) reader structure"]
impl crate::Readable for ADC_CALFACT2 {}
#[doc = "`write(|w| ..)` method takes [adc_calfact2::W](adc_calfact2::W) writer structure"]
impl crate::Writable for ADC_CALFACT2 {}
#[doc = "ADC calibration factor register 2"]
pub mod adc_calfact2;
