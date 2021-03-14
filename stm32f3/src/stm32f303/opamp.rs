#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 56usize],
    #[doc = "0x38 - OPAMP1 control register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x3c - OPAMP2 control register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x40 - OPAMP3 control register"]
    pub opamp3_csr: OPAMP3_CSR,
    #[doc = "0x44 - OPAMP4 control register"]
    pub opamp4_csr: OPAMP4_CSR,
}
#[doc = "OPAMP2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](opamp2_csr) module"]
pub type OPAMP2_CSR = crate::Reg<u32, _OPAMP2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_CSR;
#[doc = "`read()` method returns [opamp2_csr::R](opamp2_csr::R) reader structure"]
impl crate::Readable for OPAMP2_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](opamp2_csr::W) writer structure"]
impl crate::Writable for OPAMP2_CSR {}
#[doc = "OPAMP2 control register"]
pub mod opamp2_csr;
#[doc = "OPAMP3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp3_csr](opamp3_csr) module"]
pub type OPAMP3_CSR = crate::Reg<u32, _OPAMP3_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP3_CSR;
#[doc = "`read()` method returns [opamp3_csr::R](opamp3_csr::R) reader structure"]
impl crate::Readable for OPAMP3_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp3_csr::W](opamp3_csr::W) writer structure"]
impl crate::Writable for OPAMP3_CSR {}
#[doc = "OPAMP3 control register"]
pub mod opamp3_csr;
#[doc = "OPAMP4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp4_csr](opamp4_csr) module"]
pub type OPAMP4_CSR = crate::Reg<u32, _OPAMP4_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP4_CSR;
#[doc = "`read()` method returns [opamp4_csr::R](opamp4_csr::R) reader structure"]
impl crate::Readable for OPAMP4_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp4_csr::W](opamp4_csr::W) writer structure"]
impl crate::Writable for OPAMP4_CSR {}
#[doc = "OPAMP4 control register"]
pub mod opamp4_csr;
#[doc = "OPAMP1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_csr](opamp1_csr) module"]
pub type OPAMP1_CSR = crate::Reg<u32, _OPAMP1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_CSR;
#[doc = "`read()` method returns [opamp1_csr::R](opamp1_csr::R) reader structure"]
impl crate::Readable for OPAMP1_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp1_csr::W](opamp1_csr::W) writer structure"]
impl crate::Writable for OPAMP1_CSR {}
#[doc = "OPAMP1 control register"]
pub mod opamp1_csr;
