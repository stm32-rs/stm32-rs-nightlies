#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub cr: CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub wrfr: WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub cwrfr: CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub rdfr: RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub crdfr: CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub sr: SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub clrfr: CLRFR,
    #[doc = "0x1c - MDIOS input data register 0"]
    pub dinr: [DINR; 32],
    #[doc = "0x9c - MDIOS output data register 0"]
    pub doutr: [DOUTR; 32],
}
#[doc = "MDIOS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "MDIOS configuration register"]
pub mod cr;
#[doc = "MDIOS write flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrfr](wrfr) module"]
pub type WRFR = crate::Reg<u32, _WRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRFR;
#[doc = "`read()` method returns [wrfr::R](wrfr::R) reader structure"]
impl crate::Readable for WRFR {}
#[doc = "MDIOS write flag register"]
pub mod wrfr;
#[doc = "MDIOS clear write flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cwrfr](cwrfr) module"]
pub type CWRFR = crate::Reg<u32, _CWRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CWRFR;
#[doc = "`read()` method returns [cwrfr::R](cwrfr::R) reader structure"]
impl crate::Readable for CWRFR {}
#[doc = "`write(|w| ..)` method takes [cwrfr::W](cwrfr::W) writer structure"]
impl crate::Writable for CWRFR {}
#[doc = "MDIOS clear write flag register"]
pub mod cwrfr;
#[doc = "MDIOS read flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdfr](rdfr) module"]
pub type RDFR = crate::Reg<u32, _RDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDFR;
#[doc = "`read()` method returns [rdfr::R](rdfr::R) reader structure"]
impl crate::Readable for RDFR {}
#[doc = "MDIOS read flag register"]
pub mod rdfr;
#[doc = "MDIOS clear read flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crdfr](crdfr) module"]
pub type CRDFR = crate::Reg<u32, _CRDFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRDFR;
#[doc = "`read()` method returns [crdfr::R](crdfr::R) reader structure"]
impl crate::Readable for CRDFR {}
#[doc = "`write(|w| ..)` method takes [crdfr::W](crdfr::W) writer structure"]
impl crate::Writable for CRDFR {}
#[doc = "MDIOS clear read flag register"]
pub mod crdfr;
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "MDIOS status register"]
pub mod sr;
#[doc = "MDIOS clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](clrfr) module"]
pub type CLRFR = crate::Reg<u32, _CLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRFR;
#[doc = "`read()` method returns [clrfr::R](clrfr::R) reader structure"]
impl crate::Readable for CLRFR {}
#[doc = "`write(|w| ..)` method takes [clrfr::W](clrfr::W) writer structure"]
impl crate::Writable for CLRFR {}
#[doc = "MDIOS clear flag register"]
pub mod clrfr;
#[doc = "MDIOS input data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](dinr) module"]
pub type DINR = crate::Reg<u32, _DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DINR;
#[doc = "`read()` method returns [dinr::R](dinr::R) reader structure"]
impl crate::Readable for DINR {}
#[doc = "MDIOS input data register 0"]
pub mod dinr;
#[doc = "MDIOS output data register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr](doutr) module"]
pub type DOUTR = crate::Reg<u32, _DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTR;
#[doc = "`read()` method returns [doutr::R](doutr::R) reader structure"]
impl crate::Readable for DOUTR {}
#[doc = "`write(|w| ..)` method takes [doutr::W](doutr::W) writer structure"]
impl crate::Writable for DOUTR {}
#[doc = "MDIOS output data register 0"]
pub mod doutr;
