#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec control register"]
    pub confr0: CONFR0,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub confr1: CONFR1,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub confr2: CONFR2,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub confr3: CONFR3,
    #[doc = "0x10 - JPEG codec configuration register 4-7"]
    pub confrn1: CONFRN1,
    #[doc = "0x14 - JPEG codec configuration register 4-7"]
    pub confrn2: CONFRN2,
    #[doc = "0x18 - JPEG codec configuration register 4-7"]
    pub confrn3: CONFRN3,
    #[doc = "0x1c - JPEG codec configuration register 4-7"]
    pub confrn4: CONFRN4,
    _reserved8: [u8; 16usize],
    #[doc = "0x30 - JPEG control register"]
    pub cr: CR,
    #[doc = "0x34 - JPEG status register"]
    pub sr: SR,
    #[doc = "0x38 - JPEG clear flag register"]
    pub cfr: CFR,
    _reserved11: [u8; 4usize],
    #[doc = "0x40 - JPEG data input register"]
    pub dir: DIR,
    #[doc = "0x44 - JPEG data output register"]
    pub dor: DOR,
}
#[doc = "JPEG codec control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr0](confr0) module"]
pub type CONFR0 = crate::Reg<u32, _CONFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFR0;
#[doc = "`write(|w| ..)` method takes [confr0::W](confr0::W) writer structure"]
impl crate::Writable for CONFR0 {}
#[doc = "JPEG codec control register"]
pub mod confr0;
#[doc = "JPEG codec configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr1](confr1) module"]
pub type CONFR1 = crate::Reg<u32, _CONFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFR1;
#[doc = "`read()` method returns [confr1::R](confr1::R) reader structure"]
impl crate::Readable for CONFR1 {}
#[doc = "`write(|w| ..)` method takes [confr1::W](confr1::W) writer structure"]
impl crate::Writable for CONFR1 {}
#[doc = "JPEG codec configuration register 1"]
pub mod confr1;
#[doc = "JPEG codec configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr2](confr2) module"]
pub type CONFR2 = crate::Reg<u32, _CONFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFR2;
#[doc = "`read()` method returns [confr2::R](confr2::R) reader structure"]
impl crate::Readable for CONFR2 {}
#[doc = "`write(|w| ..)` method takes [confr2::W](confr2::W) writer structure"]
impl crate::Writable for CONFR2 {}
#[doc = "JPEG codec configuration register 2"]
pub mod confr2;
#[doc = "JPEG codec configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr3](confr3) module"]
pub type CONFR3 = crate::Reg<u32, _CONFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFR3;
#[doc = "`read()` method returns [confr3::R](confr3::R) reader structure"]
impl crate::Readable for CONFR3 {}
#[doc = "`write(|w| ..)` method takes [confr3::W](confr3::W) writer structure"]
impl crate::Writable for CONFR3 {}
#[doc = "JPEG codec configuration register 3"]
pub mod confr3;
#[doc = "JPEG codec configuration register 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confrn1](confrn1) module"]
pub type CONFRN1 = crate::Reg<u32, _CONFRN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFRN1;
#[doc = "`read()` method returns [confrn1::R](confrn1::R) reader structure"]
impl crate::Readable for CONFRN1 {}
#[doc = "`write(|w| ..)` method takes [confrn1::W](confrn1::W) writer structure"]
impl crate::Writable for CONFRN1 {}
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn1;
#[doc = "JPEG codec configuration register 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confrn2](confrn2) module"]
pub type CONFRN2 = crate::Reg<u32, _CONFRN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFRN2;
#[doc = "`read()` method returns [confrn2::R](confrn2::R) reader structure"]
impl crate::Readable for CONFRN2 {}
#[doc = "`write(|w| ..)` method takes [confrn2::W](confrn2::W) writer structure"]
impl crate::Writable for CONFRN2 {}
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn2;
#[doc = "JPEG codec configuration register 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confrn3](confrn3) module"]
pub type CONFRN3 = crate::Reg<u32, _CONFRN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFRN3;
#[doc = "`read()` method returns [confrn3::R](confrn3::R) reader structure"]
impl crate::Readable for CONFRN3 {}
#[doc = "`write(|w| ..)` method takes [confrn3::W](confrn3::W) writer structure"]
impl crate::Writable for CONFRN3 {}
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn3;
#[doc = "JPEG codec configuration register 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confrn4](confrn4) module"]
pub type CONFRN4 = crate::Reg<u32, _CONFRN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFRN4;
#[doc = "`read()` method returns [confrn4::R](confrn4::R) reader structure"]
impl crate::Readable for CONFRN4 {}
#[doc = "`write(|w| ..)` method takes [confrn4::W](confrn4::W) writer structure"]
impl crate::Writable for CONFRN4 {}
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn4;
#[doc = "JPEG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "JPEG control register"]
pub mod cr;
#[doc = "JPEG status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "JPEG status register"]
pub mod sr;
#[doc = "JPEG clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](cfr) module"]
pub type CFR = crate::Reg<u32, _CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFR;
#[doc = "`read()` method returns [cfr::R](cfr::R) reader structure"]
impl crate::Readable for CFR {}
#[doc = "`write(|w| ..)` method takes [cfr::W](cfr::W) writer structure"]
impl crate::Writable for CFR {}
#[doc = "JPEG clear flag register"]
pub mod cfr;
#[doc = "JPEG data input register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "JPEG data input register"]
pub mod dir;
#[doc = "JPEG data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor](dor) module"]
pub type DOR = crate::Reg<u32, _DOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOR;
#[doc = "`read()` method returns [dor::R](dor::R) reader structure"]
impl crate::Readable for DOR {}
#[doc = "JPEG data output register"]
pub mod dor;
