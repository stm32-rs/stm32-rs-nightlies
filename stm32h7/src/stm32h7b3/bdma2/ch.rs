#[doc = "DMA channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "DMA channel x configuration register"]
pub mod cr;
#[doc = "DMA channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndtr](ndtr) module"]
pub type NDTR = crate::Reg<u32, _NDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDTR;
#[doc = "`read()` method returns [ndtr::R](ndtr::R) reader structure"]
impl crate::Readable for NDTR {}
#[doc = "`write(|w| ..)` method takes [ndtr::W](ndtr::W) writer structure"]
impl crate::Writable for NDTR {}
#[doc = "DMA channel x number of data register"]
pub mod ndtr;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [par](par) module"]
pub type PAR = crate::Reg<u32, _PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAR;
#[doc = "`read()` method returns [par::R](par::R) reader structure"]
impl crate::Readable for PAR {}
#[doc = "`write(|w| ..)` method takes [par::W](par::W) writer structure"]
impl crate::Writable for PAR {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod par;
#[doc = "This register must not be written when the channel is enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0ar](m0ar) module"]
pub type M0AR = crate::Reg<u32, _M0AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0AR;
#[doc = "`read()` method returns [m0ar::R](m0ar::R) reader structure"]
impl crate::Readable for M0AR {}
#[doc = "`write(|w| ..)` method takes [m0ar::W](m0ar::W) writer structure"]
impl crate::Writable for M0AR {}
#[doc = "This register must not be written when the channel is enabled."]
pub mod m0ar;
#[doc = "This register must not be written when the channel is enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1ar](m1ar) module"]
pub type M1AR = crate::Reg<u32, _M1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1AR;
#[doc = "`read()` method returns [m1ar::R](m1ar::R) reader structure"]
impl crate::Readable for M1AR {}
#[doc = "`write(|w| ..)` method takes [m1ar::W](m1ar::W) writer structure"]
impl crate::Writable for M1AR {}
#[doc = "This register must not be written when the channel is enabled"]
pub mod m1ar;
