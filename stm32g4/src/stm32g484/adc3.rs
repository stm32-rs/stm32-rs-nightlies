#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - configuration register"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x18 - sample time register 2"]
    pub smpr2: SMPR2,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - watchdog threshold register 1"]
    pub tr1: TR1,
    #[doc = "0x24 - watchdog threshold register"]
    pub tr2: TR2,
    #[doc = "0x28 - watchdog threshold register 3"]
    pub tr3: TR3,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x34 - regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x38 - regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x3c - regular sequence register 4"]
    pub sqr4: SQR4,
    #[doc = "0x40 - regular Data Register"]
    pub dr: DR,
    _reserved15: [u8; 8usize],
    #[doc = "0x4c - injected sequence register"]
    pub jsqr: JSQR,
    _reserved16: [u8; 16usize],
    #[doc = "0x60 - offset register 1"]
    pub ofr1: OFR1,
    #[doc = "0x64 - offset register 2"]
    pub ofr2: OFR2,
    #[doc = "0x68 - offset register 3"]
    pub ofr3: OFR3,
    #[doc = "0x6c - offset register 4"]
    pub ofr4: OFR4,
    _reserved20: [u8; 16usize],
    #[doc = "0x80 - injected data register 1"]
    pub jdr1: JDR1,
    #[doc = "0x84 - injected data register 2"]
    pub jdr2: JDR2,
    #[doc = "0x88 - injected data register 3"]
    pub jdr3: JDR3,
    #[doc = "0x8c - injected data register 4"]
    pub jdr4: JDR4,
    _reserved24: [u8; 16usize],
    #[doc = "0xa0 - Analog Watchdog 2 Configuration Register"]
    pub awd2cr: AWD2CR,
    #[doc = "0xa4 - Analog Watchdog 3 Configuration Register"]
    pub awd3cr: AWD3CR,
    _reserved26: [u8; 8usize],
    #[doc = "0xb0 - Differential Mode Selection Register 2"]
    pub difsel: DIFSEL,
    #[doc = "0xb4 - Calibration Factors"]
    pub calfact: CALFACT,
    _reserved28: [u8; 8usize],
    #[doc = "0xc0 - Gain compensation Register"]
    pub gcomp: GCOMP,
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "control register"]
pub mod cr;
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](cfgr) module"]
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
#[doc = "`read()` method returns [cfgr::R](cfgr::R) reader structure"]
impl crate::Readable for CFGR {}
#[doc = "`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure"]
impl crate::Writable for CFGR {}
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "configuration register"]
pub mod cfgr2;
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](smpr1) module"]
pub type SMPR1 = crate::Reg<u32, _SMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR1;
#[doc = "`read()` method returns [smpr1::R](smpr1::R) reader structure"]
impl crate::Readable for SMPR1 {}
#[doc = "`write(|w| ..)` method takes [smpr1::W](smpr1::W) writer structure"]
impl crate::Writable for SMPR1 {}
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](smpr2) module"]
pub type SMPR2 = crate::Reg<u32, _SMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR2;
#[doc = "`read()` method returns [smpr2::R](smpr2::R) reader structure"]
impl crate::Readable for SMPR2 {}
#[doc = "`write(|w| ..)` method takes [smpr2::W](smpr2::W) writer structure"]
impl crate::Writable for SMPR2 {}
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "watchdog threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr1](tr1) module"]
pub type TR1 = crate::Reg<u32, _TR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR1;
#[doc = "`read()` method returns [tr1::R](tr1::R) reader structure"]
impl crate::Readable for TR1 {}
#[doc = "`write(|w| ..)` method takes [tr1::W](tr1::W) writer structure"]
impl crate::Writable for TR1 {}
#[doc = "watchdog threshold register 1"]
pub mod tr1;
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr2](tr2) module"]
pub type TR2 = crate::Reg<u32, _TR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR2;
#[doc = "`read()` method returns [tr2::R](tr2::R) reader structure"]
impl crate::Readable for TR2 {}
#[doc = "`write(|w| ..)` method takes [tr2::W](tr2::W) writer structure"]
impl crate::Writable for TR2 {}
#[doc = "watchdog threshold register"]
pub mod tr2;
#[doc = "watchdog threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr3](tr3) module"]
pub type TR3 = crate::Reg<u32, _TR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR3;
#[doc = "`read()` method returns [tr3::R](tr3::R) reader structure"]
impl crate::Readable for TR3 {}
#[doc = "`write(|w| ..)` method takes [tr3::W](tr3::W) writer structure"]
impl crate::Writable for TR3 {}
#[doc = "watchdog threshold register 3"]
pub mod tr3;
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](sqr1) module"]
pub type SQR1 = crate::Reg<u32, _SQR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR1;
#[doc = "`read()` method returns [sqr1::R](sqr1::R) reader structure"]
impl crate::Readable for SQR1 {}
#[doc = "`write(|w| ..)` method takes [sqr1::W](sqr1::W) writer structure"]
impl crate::Writable for SQR1 {}
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](sqr2) module"]
pub type SQR2 = crate::Reg<u32, _SQR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR2;
#[doc = "`read()` method returns [sqr2::R](sqr2::R) reader structure"]
impl crate::Readable for SQR2 {}
#[doc = "`write(|w| ..)` method takes [sqr2::W](sqr2::W) writer structure"]
impl crate::Writable for SQR2 {}
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr3](sqr3) module"]
pub type SQR3 = crate::Reg<u32, _SQR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR3;
#[doc = "`read()` method returns [sqr3::R](sqr3::R) reader structure"]
impl crate::Readable for SQR3 {}
#[doc = "`write(|w| ..)` method takes [sqr3::W](sqr3::W) writer structure"]
impl crate::Writable for SQR3 {}
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "regular sequence register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr4](sqr4) module"]
pub type SQR4 = crate::Reg<u32, _SQR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR4;
#[doc = "`read()` method returns [sqr4::R](sqr4::R) reader structure"]
impl crate::Readable for SQR4 {}
#[doc = "`write(|w| ..)` method takes [sqr4::W](sqr4::W) writer structure"]
impl crate::Writable for SQR4 {}
#[doc = "regular sequence register 4"]
pub mod sqr4;
#[doc = "regular Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "regular Data Register"]
pub mod dr;
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jsqr](jsqr) module"]
pub type JSQR = crate::Reg<u32, _JSQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JSQR;
#[doc = "`read()` method returns [jsqr::R](jsqr::R) reader structure"]
impl crate::Readable for JSQR {}
#[doc = "`write(|w| ..)` method takes [jsqr::W](jsqr::W) writer structure"]
impl crate::Writable for JSQR {}
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "offset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr1](ofr1) module"]
pub type OFR1 = crate::Reg<u32, _OFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR1;
#[doc = "`read()` method returns [ofr1::R](ofr1::R) reader structure"]
impl crate::Readable for OFR1 {}
#[doc = "`write(|w| ..)` method takes [ofr1::W](ofr1::W) writer structure"]
impl crate::Writable for OFR1 {}
#[doc = "offset register 1"]
pub mod ofr1;
#[doc = "offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](ofr2) module"]
pub type OFR2 = crate::Reg<u32, _OFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR2;
#[doc = "`read()` method returns [ofr2::R](ofr2::R) reader structure"]
impl crate::Readable for OFR2 {}
#[doc = "`write(|w| ..)` method takes [ofr2::W](ofr2::W) writer structure"]
impl crate::Writable for OFR2 {}
#[doc = "offset register 2"]
pub mod ofr2;
#[doc = "offset register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr3](ofr3) module"]
pub type OFR3 = crate::Reg<u32, _OFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR3;
#[doc = "`read()` method returns [ofr3::R](ofr3::R) reader structure"]
impl crate::Readable for OFR3 {}
#[doc = "`write(|w| ..)` method takes [ofr3::W](ofr3::W) writer structure"]
impl crate::Writable for OFR3 {}
#[doc = "offset register 3"]
pub mod ofr3;
#[doc = "offset register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr4](ofr4) module"]
pub type OFR4 = crate::Reg<u32, _OFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR4;
#[doc = "`read()` method returns [ofr4::R](ofr4::R) reader structure"]
impl crate::Readable for OFR4 {}
#[doc = "`write(|w| ..)` method takes [ofr4::W](ofr4::W) writer structure"]
impl crate::Writable for OFR4 {}
#[doc = "offset register 4"]
pub mod ofr4;
#[doc = "injected data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr1](jdr1) module"]
pub type JDR1 = crate::Reg<u32, _JDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR1;
#[doc = "`read()` method returns [jdr1::R](jdr1::R) reader structure"]
impl crate::Readable for JDR1 {}
#[doc = "injected data register 1"]
pub mod jdr1;
#[doc = "injected data register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr2](jdr2) module"]
pub type JDR2 = crate::Reg<u32, _JDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR2;
#[doc = "`read()` method returns [jdr2::R](jdr2::R) reader structure"]
impl crate::Readable for JDR2 {}
#[doc = "injected data register 2"]
pub mod jdr2;
#[doc = "injected data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr3](jdr3) module"]
pub type JDR3 = crate::Reg<u32, _JDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR3;
#[doc = "`read()` method returns [jdr3::R](jdr3::R) reader structure"]
impl crate::Readable for JDR3 {}
#[doc = "injected data register 3"]
pub mod jdr3;
#[doc = "injected data register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr4](jdr4) module"]
pub type JDR4 = crate::Reg<u32, _JDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR4;
#[doc = "`read()` method returns [jdr4::R](jdr4::R) reader structure"]
impl crate::Readable for JDR4 {}
#[doc = "injected data register 4"]
pub mod jdr4;
#[doc = "Analog Watchdog 2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2cr](awd2cr) module"]
pub type AWD2CR = crate::Reg<u32, _AWD2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD2CR;
#[doc = "`read()` method returns [awd2cr::R](awd2cr::R) reader structure"]
impl crate::Readable for AWD2CR {}
#[doc = "`write(|w| ..)` method takes [awd2cr::W](awd2cr::W) writer structure"]
impl crate::Writable for AWD2CR {}
#[doc = "Analog Watchdog 2 Configuration Register"]
pub mod awd2cr;
#[doc = "Analog Watchdog 3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3cr](awd3cr) module"]
pub type AWD3CR = crate::Reg<u32, _AWD3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD3CR;
#[doc = "`read()` method returns [awd3cr::R](awd3cr::R) reader structure"]
impl crate::Readable for AWD3CR {}
#[doc = "`write(|w| ..)` method takes [awd3cr::W](awd3cr::W) writer structure"]
impl crate::Writable for AWD3CR {}
#[doc = "Analog Watchdog 3 Configuration Register"]
pub mod awd3cr;
#[doc = "Differential Mode Selection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsel](difsel) module"]
pub type DIFSEL = crate::Reg<u32, _DIFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIFSEL;
#[doc = "`read()` method returns [difsel::R](difsel::R) reader structure"]
impl crate::Readable for DIFSEL {}
#[doc = "`write(|w| ..)` method takes [difsel::W](difsel::W) writer structure"]
impl crate::Writable for DIFSEL {}
#[doc = "Differential Mode Selection Register 2"]
pub mod difsel;
#[doc = "Calibration Factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact](calfact) module"]
pub type CALFACT = crate::Reg<u32, _CALFACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALFACT;
#[doc = "`read()` method returns [calfact::R](calfact::R) reader structure"]
impl crate::Readable for CALFACT {}
#[doc = "`write(|w| ..)` method takes [calfact::W](calfact::W) writer structure"]
impl crate::Writable for CALFACT {}
#[doc = "Calibration Factors"]
pub mod calfact;
#[doc = "Gain compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcomp](gcomp) module"]
pub type GCOMP = crate::Reg<u32, _GCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCOMP;
#[doc = "`read()` method returns [gcomp::R](gcomp::R) reader structure"]
impl crate::Readable for GCOMP {}
#[doc = "`write(|w| ..)` method takes [gcomp::W](gcomp::W) writer structure"]
impl crate::Writable for GCOMP {}
#[doc = "Gain compensation Register"]
pub mod gcomp;
