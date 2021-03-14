#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control/status register"]
    pub csr: CSR,
    #[doc = "0x04 - offset trimming register for normal mode"]
    pub otr: OTR,
    #[doc = "0x08 - OPAMP offset trimming register for low power mode"]
    pub lpotr: LPOTR,
}
#[doc = "control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "control/status register"]
pub mod csr;
#[doc = "offset trimming register for normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otr](otr) module"]
pub type OTR = crate::Reg<u32, _OTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTR;
#[doc = "`read()` method returns [otr::R](otr::R) reader structure"]
impl crate::Readable for OTR {}
#[doc = "`write(|w| ..)` method takes [otr::W](otr::W) writer structure"]
impl crate::Writable for OTR {}
#[doc = "offset trimming register for normal mode"]
pub mod otr;
#[doc = "OPAMP offset trimming register for low power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpotr](lpotr) module"]
pub type LPOTR = crate::Reg<u32, _LPOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPOTR;
#[doc = "`read()` method returns [lpotr::R](lpotr::R) reader structure"]
impl crate::Readable for LPOTR {}
#[doc = "`write(|w| ..)` method takes [lpotr::W](lpotr::W) writer structure"]
impl crate::Writable for LPOTR {}
#[doc = "OPAMP offset trimming register for low power mode"]
pub mod lpotr;
