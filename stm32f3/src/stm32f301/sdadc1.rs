#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x0c - interrupt and status clear register"]
    pub clrisr: CLRISR,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - injected channel group selection register"]
    pub jchgr: JCHGR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - configuration 0 register"]
    pub conf0r: CONF0R,
    #[doc = "0x24 - configuration 1 register"]
    pub conf1r: CONF1R,
    #[doc = "0x28 - configuration 2 register"]
    pub conf2r: CONF2R,
    _reserved8: [u8; 20usize],
    #[doc = "0x40 - channel configuration register 1"]
    pub confchr1: CONFCHR1,
    #[doc = "0x44 - channel configuration register 2"]
    pub confchr2: CONFCHR2,
    _reserved10: [u8; 24usize],
    #[doc = "0x60 - data register for injected group"]
    pub jdatar: JDATAR,
    #[doc = "0x64 - data register for the regular channel"]
    pub rdatar: RDATAR,
    _reserved12: [u8; 8usize],
    #[doc = "0x70 - SDADC1 and SDADC2 injected data register"]
    pub jdata12r: JDATA12R,
    #[doc = "0x74 - SDADC1 and SDADC2 regular data register"]
    pub rdata12r: RDATA12R,
    #[doc = "0x78 - SDADC1 and SDADC3 injected data register"]
    pub jdata13r: JDATA13R,
    #[doc = "0x7c - SDADC1 and SDADC3 regular data register"]
    pub rdata13r: RDATA13R,
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt and status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrisr](clrisr) module"]
pub type CLRISR = crate::Reg<u32, _CLRISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRISR;
#[doc = "`read()` method returns [clrisr::R](clrisr::R) reader structure"]
impl crate::Readable for CLRISR {}
#[doc = "`write(|w| ..)` method takes [clrisr::W](clrisr::W) writer structure"]
impl crate::Writable for CLRISR {}
#[doc = "interrupt and status clear register"]
pub mod clrisr;
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jchgr](jchgr) module"]
pub type JCHGR = crate::Reg<u32, _JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JCHGR;
#[doc = "`read()` method returns [jchgr::R](jchgr::R) reader structure"]
impl crate::Readable for JCHGR {}
#[doc = "`write(|w| ..)` method takes [jchgr::W](jchgr::W) writer structure"]
impl crate::Writable for JCHGR {}
#[doc = "injected channel group selection register"]
pub mod jchgr;
#[doc = "configuration 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0r](conf0r) module"]
pub type CONF0R = crate::Reg<u32, _CONF0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF0R;
#[doc = "`read()` method returns [conf0r::R](conf0r::R) reader structure"]
impl crate::Readable for CONF0R {}
#[doc = "`write(|w| ..)` method takes [conf0r::W](conf0r::W) writer structure"]
impl crate::Writable for CONF0R {}
#[doc = "configuration 0 register"]
pub mod conf0r;
#[doc = "configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1r](conf1r) module"]
pub type CONF1R = crate::Reg<u32, _CONF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF1R;
#[doc = "`read()` method returns [conf1r::R](conf1r::R) reader structure"]
impl crate::Readable for CONF1R {}
#[doc = "`write(|w| ..)` method takes [conf1r::W](conf1r::W) writer structure"]
impl crate::Writable for CONF1R {}
#[doc = "configuration 1 register"]
pub mod conf1r;
#[doc = "configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2r](conf2r) module"]
pub type CONF2R = crate::Reg<u32, _CONF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF2R;
#[doc = "`read()` method returns [conf2r::R](conf2r::R) reader structure"]
impl crate::Readable for CONF2R {}
#[doc = "`write(|w| ..)` method takes [conf2r::W](conf2r::W) writer structure"]
impl crate::Writable for CONF2R {}
#[doc = "configuration 2 register"]
pub mod conf2r;
#[doc = "channel configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr1](confchr1) module"]
pub type CONFCHR1 = crate::Reg<u32, _CONFCHR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFCHR1;
#[doc = "`read()` method returns [confchr1::R](confchr1::R) reader structure"]
impl crate::Readable for CONFCHR1 {}
#[doc = "`write(|w| ..)` method takes [confchr1::W](confchr1::W) writer structure"]
impl crate::Writable for CONFCHR1 {}
#[doc = "channel configuration register 1"]
pub mod confchr1;
#[doc = "channel configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confchr2](confchr2) module"]
pub type CONFCHR2 = crate::Reg<u32, _CONFCHR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFCHR2;
#[doc = "`read()` method returns [confchr2::R](confchr2::R) reader structure"]
impl crate::Readable for CONFCHR2 {}
#[doc = "`write(|w| ..)` method takes [confchr2::W](confchr2::W) writer structure"]
impl crate::Writable for CONFCHR2 {}
#[doc = "channel configuration register 2"]
pub mod confchr2;
#[doc = "data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdatar](jdatar) module"]
pub type JDATAR = crate::Reg<u32, _JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDATAR;
#[doc = "`read()` method returns [jdatar::R](jdatar::R) reader structure"]
impl crate::Readable for JDATAR {}
#[doc = "data register for injected group"]
pub mod jdatar;
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdatar](rdatar) module"]
pub type RDATAR = crate::Reg<u32, _RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATAR;
#[doc = "`read()` method returns [rdatar::R](rdatar::R) reader structure"]
impl crate::Readable for RDATAR {}
#[doc = "data register for the regular channel"]
pub mod rdatar;
#[doc = "SDADC1 and SDADC2 injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdata12r](jdata12r) module"]
pub type JDATA12R = crate::Reg<u32, _JDATA12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDATA12R;
#[doc = "`read()` method returns [jdata12r::R](jdata12r::R) reader structure"]
impl crate::Readable for JDATA12R {}
#[doc = "SDADC1 and SDADC2 injected data register"]
pub mod jdata12r;
#[doc = "SDADC1 and SDADC2 regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata12r](rdata12r) module"]
pub type RDATA12R = crate::Reg<u32, _RDATA12R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA12R;
#[doc = "`read()` method returns [rdata12r::R](rdata12r::R) reader structure"]
impl crate::Readable for RDATA12R {}
#[doc = "SDADC1 and SDADC2 regular data register"]
pub mod rdata12r;
#[doc = "SDADC1 and SDADC3 injected data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdata13r](jdata13r) module"]
pub type JDATA13R = crate::Reg<u32, _JDATA13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDATA13R;
#[doc = "`read()` method returns [jdata13r::R](jdata13r::R) reader structure"]
impl crate::Readable for JDATA13R {}
#[doc = "SDADC1 and SDADC3 injected data register"]
pub mod jdata13r;
#[doc = "SDADC1 and SDADC3 regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata13r](rdata13r) module"]
pub type RDATA13R = crate::Reg<u32, _RDATA13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA13R;
#[doc = "`read()` method returns [rdata13r::R](rdata13r::R) reader structure"]
impl crate::Readable for RDATA13R {}
#[doc = "SDADC1 and SDADC3 regular data register"]
pub mod rdata13r;
