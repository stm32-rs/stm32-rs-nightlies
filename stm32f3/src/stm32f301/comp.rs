#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - control and status register"]
    pub comp2_csr: COMP2_CSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - control and status register"]
    pub comp4_csr: COMP4_CSR,
    _reserved2: [u8; 4usize],
    #[doc = "0x30 - control and status register"]
    pub comp6_csr: COMP6_CSR,
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](comp2_csr) module"]
pub type COMP2_CSR = crate::Reg<u32, _COMP2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP2_CSR;
#[doc = "`read()` method returns [comp2_csr::R](comp2_csr::R) reader structure"]
impl crate::Readable for COMP2_CSR {}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](comp2_csr::W) writer structure"]
impl crate::Writable for COMP2_CSR {}
#[doc = "control and status register"]
pub mod comp2_csr;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4_csr](comp4_csr) module"]
pub type COMP4_CSR = crate::Reg<u32, _COMP4_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP4_CSR;
#[doc = "`read()` method returns [comp4_csr::R](comp4_csr::R) reader structure"]
impl crate::Readable for COMP4_CSR {}
#[doc = "`write(|w| ..)` method takes [comp4_csr::W](comp4_csr::W) writer structure"]
impl crate::Writable for COMP4_CSR {}
#[doc = "control and status register"]
pub mod comp4_csr;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](comp6_csr) module"]
pub type COMP6_CSR = crate::Reg<u32, _COMP6_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP6_CSR;
#[doc = "`read()` method returns [comp6_csr::R](comp6_csr::R) reader structure"]
impl crate::Readable for COMP6_CSR {}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](comp6_csr::W) writer structure"]
impl crate::Writable for COMP6_CSR {}
#[doc = "control and status register"]
pub mod comp6_csr;
