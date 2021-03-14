#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREFBUF control and status register"]
    pub vrefbuf_csr: VREFBUF_CSR,
    #[doc = "0x04 - VREFBUF calibration control register"]
    pub vrefbuf_ccr: VREFBUF_CCR,
}
#[doc = "VREFBUF control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_csr](vrefbuf_csr) module"]
pub type VREFBUF_CSR = crate::Reg<u32, _VREFBUF_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREFBUF_CSR;
#[doc = "`read()` method returns [vrefbuf_csr::R](vrefbuf_csr::R) reader structure"]
impl crate::Readable for VREFBUF_CSR {}
#[doc = "`write(|w| ..)` method takes [vrefbuf_csr::W](vrefbuf_csr::W) writer structure"]
impl crate::Writable for VREFBUF_CSR {}
#[doc = "VREFBUF control and status register"]
pub mod vrefbuf_csr;
#[doc = "VREFBUF calibration control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_ccr](vrefbuf_ccr) module"]
pub type VREFBUF_CCR = crate::Reg<u32, _VREFBUF_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREFBUF_CCR;
#[doc = "`read()` method returns [vrefbuf_ccr::R](vrefbuf_ccr::R) reader structure"]
impl crate::Readable for VREFBUF_CCR {}
#[doc = "`write(|w| ..)` method takes [vrefbuf_ccr::W](vrefbuf_ccr::W) writer structure"]
impl crate::Writable for VREFBUF_CCR {}
#[doc = "VREFBUF calibration control register"]
pub mod vrefbuf_ccr;
