#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator 1 control and status register"]
    pub comp1_csr: COMP1_CSR,
    #[doc = "0x04 - Comparator 2 control and status register"]
    pub comp2_csr: COMP2_CSR,
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](comp1_csr) module"]
pub type COMP1_CSR = crate::Reg<u32, _COMP1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_CSR;
#[doc = "`read()` method returns [comp1_csr::R](comp1_csr::R) reader structure"]
impl crate::Readable for COMP1_CSR {}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](comp1_csr::W) writer structure"]
impl crate::Writable for COMP1_CSR {}
#[doc = "Comparator 1 control and status register"]
pub mod comp1_csr;
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](comp2_csr) module"]
pub type COMP2_CSR = crate::Reg<u32, _COMP2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2_CSR;
#[doc = "`read()` method returns [comp2_csr::R](comp2_csr::R) reader structure"]
impl crate::Readable for COMP2_CSR {}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](comp2_csr::W) writer structure"]
impl crate::Writable for COMP2_CSR {}
#[doc = "Comparator 2 control and status register"]
pub mod comp2_csr;
