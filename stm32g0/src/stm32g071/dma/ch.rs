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
#[doc = "DMA channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [par](par) module"]
pub type PAR = crate::Reg<u32, _PAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAR;
#[doc = "`read()` method returns [par::R](par::R) reader structure"]
impl crate::Readable for PAR {}
#[doc = "`write(|w| ..)` method takes [par::W](par::W) writer structure"]
impl crate::Writable for PAR {}
#[doc = "DMA channel x peripheral address register"]
pub mod par;
#[doc = "DMA channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mar](mar) module"]
pub type MAR = crate::Reg<u32, _MAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAR;
#[doc = "`read()` method returns [mar::R](mar::R) reader structure"]
impl crate::Readable for MAR {}
#[doc = "`write(|w| ..)` method takes [mar::W](mar::W) writer structure"]
impl crate::Writable for MAR {}
#[doc = "DMA channel x memory address register"]
pub mod mar;
