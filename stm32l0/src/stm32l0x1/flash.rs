#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Program/erase control register"]
    pub pecr: PECR,
    #[doc = "0x08 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x0c - Program/erase key register"]
    pub pekeyr: PEKEYR,
    #[doc = "0x10 - Program memory key register"]
    pub prgkeyr: PRGKEYR,
    #[doc = "0x14 - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x18 - Status register"]
    pub sr: SR,
    #[doc = "0x1c - Option byte register"]
    pub optr: OPTR,
    #[doc = "0x20 - Write Protection Register 1"]
    pub wrprot1: WRPROT1,
    _reserved9: [u8; 92usize],
    #[doc = "0x80 - Write Protection Register 2"]
    pub wrprot2: WRPROT2,
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Program/erase control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](pecr) module"]
pub type PECR = crate::Reg<u32, _PECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PECR;
#[doc = "`read()` method returns [pecr::R](pecr::R) reader structure"]
impl crate::Readable for PECR {}
#[doc = "`write(|w| ..)` method takes [pecr::W](pecr::W) writer structure"]
impl crate::Writable for PECR {}
#[doc = "Program/erase control register"]
pub mod pecr;
#[doc = "Power down key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdkeyr](pdkeyr) module"]
pub type PDKEYR = crate::Reg<u32, _PDKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDKEYR;
#[doc = "`write(|w| ..)` method takes [pdkeyr::W](pdkeyr::W) writer structure"]
impl crate::Writable for PDKEYR {}
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "Program/erase key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pekeyr](pekeyr) module"]
pub type PEKEYR = crate::Reg<u32, _PEKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEKEYR;
#[doc = "`write(|w| ..)` method takes [pekeyr::W](pekeyr::W) writer structure"]
impl crate::Writable for PEKEYR {}
#[doc = "Program/erase key register"]
pub mod pekeyr;
#[doc = "Program memory key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prgkeyr](prgkeyr) module"]
pub type PRGKEYR = crate::Reg<u32, _PRGKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRGKEYR;
#[doc = "`write(|w| ..)` method takes [prgkeyr::W](prgkeyr::W) writer structure"]
impl crate::Writable for PRGKEYR {}
#[doc = "Program memory key register"]
pub mod prgkeyr;
#[doc = "Option byte key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optkeyr](optkeyr) module"]
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
#[doc = "`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure"]
impl crate::Writable for OPTKEYR {}
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Status register"]
pub mod sr;
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](optr) module"]
pub type OPTR = crate::Reg<u32, _OPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTR;
#[doc = "`read()` method returns [optr::R](optr::R) reader structure"]
impl crate::Readable for OPTR {}
#[doc = "Option byte register"]
pub mod optr;
#[doc = "Write Protection Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprot1](wrprot1) module"]
pub type WRPROT1 = crate::Reg<u32, _WRPROT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRPROT1;
#[doc = "`read()` method returns [wrprot1::R](wrprot1::R) reader structure"]
impl crate::Readable for WRPROT1 {}
#[doc = "Write Protection Register 1"]
pub mod wrprot1;
#[doc = "Write Protection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprot2](wrprot2) module"]
pub type WRPROT2 = crate::Reg<u32, _WRPROT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRPROT2;
#[doc = "`read()` method returns [wrprot2::R](wrprot2::R) reader structure"]
impl crate::Readable for WRPROT2 {}
#[doc = "Write Protection Register 2"]
pub mod wrprot2;
