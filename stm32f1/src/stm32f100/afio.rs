#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control Register (AFIO_EVCR)"]
    pub evcr: EVCR,
    #[doc = "0x04 - AF remap and debug I/O configuration register (AFIO_MAPR)"]
    pub mapr: MAPR,
    #[doc = "0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)"]
    pub exticr4: EXTICR4,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - AF remap and debug I/O configuration register"]
    pub mapr2: MAPR2,
}
#[doc = "Event Control Register (AFIO_EVCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evcr](evcr) module"]
pub type EVCR = crate::Reg<u32, _EVCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVCR;
#[doc = "`read()` method returns [evcr::R](evcr::R) reader structure"]
impl crate::Readable for EVCR {}
#[doc = "`write(|w| ..)` method takes [evcr::W](evcr::W) writer structure"]
impl crate::Writable for EVCR {}
#[doc = "Event Control Register (AFIO_EVCR)"]
pub mod evcr;
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr](mapr) module"]
pub type MAPR = crate::Reg<u32, _MAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAPR;
#[doc = "`read()` method returns [mapr::R](mapr::R) reader structure"]
impl crate::Readable for MAPR {}
#[doc = "`write(|w| ..)` method takes [mapr::W](mapr::W) writer structure"]
impl crate::Writable for MAPR {}
#[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
pub mod mapr;
#[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
pub mod exticr1;
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)"]
pub mod exticr2;
#[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
pub mod exticr3;
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)"]
pub mod exticr4;
#[doc = "AF remap and debug I/O configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapr2](mapr2) module"]
pub type MAPR2 = crate::Reg<u32, _MAPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAPR2;
#[doc = "`read()` method returns [mapr2::R](mapr2::R) reader structure"]
impl crate::Readable for MAPR2 {}
#[doc = "`write(|w| ..)` method takes [mapr2::W](mapr2::W) writer structure"]
impl crate::Writable for MAPR2 {}
#[doc = "AF remap and debug I/O configuration register"]
pub mod mapr2;
