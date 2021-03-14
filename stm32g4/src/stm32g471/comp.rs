#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator control/status register"]
    pub comp_c1csr: COMP_C1CSR,
    #[doc = "0x04 - Comparator control/status register"]
    pub comp_c2csr: COMP_C2CSR,
    #[doc = "0x08 - Comparator control/status register"]
    pub comp_c3csr: COMP_C3CSR,
    #[doc = "0x0c - Comparator control/status register"]
    pub comp_c4csr: COMP_C4CSR,
}
#[doc = "Comparator control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_c1csr](comp_c1csr) module"]
pub type COMP_C1CSR = crate::Reg<u32, _COMP_C1CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C1CSR;
#[doc = "`read()` method returns [comp_c1csr::R](comp_c1csr::R) reader structure"]
impl crate::Readable for COMP_C1CSR {}
#[doc = "`write(|w| ..)` method takes [comp_c1csr::W](comp_c1csr::W) writer structure"]
impl crate::Writable for COMP_C1CSR {}
#[doc = "Comparator control/status register"]
pub mod comp_c1csr;
#[doc = "Comparator control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_c2csr](comp_c2csr) module"]
pub type COMP_C2CSR = crate::Reg<u32, _COMP_C2CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C2CSR;
#[doc = "`read()` method returns [comp_c2csr::R](comp_c2csr::R) reader structure"]
impl crate::Readable for COMP_C2CSR {}
#[doc = "`write(|w| ..)` method takes [comp_c2csr::W](comp_c2csr::W) writer structure"]
impl crate::Writable for COMP_C2CSR {}
#[doc = "Comparator control/status register"]
pub mod comp_c2csr;
#[doc = "Comparator control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_c3csr](comp_c3csr) module"]
pub type COMP_C3CSR = crate::Reg<u32, _COMP_C3CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C3CSR;
#[doc = "`read()` method returns [comp_c3csr::R](comp_c3csr::R) reader structure"]
impl crate::Readable for COMP_C3CSR {}
#[doc = "`write(|w| ..)` method takes [comp_c3csr::W](comp_c3csr::W) writer structure"]
impl crate::Writable for COMP_C3CSR {}
#[doc = "Comparator control/status register"]
pub mod comp_c3csr;
#[doc = "Comparator control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_c4csr](comp_c4csr) module"]
pub type COMP_C4CSR = crate::Reg<u32, _COMP_C4CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_C4CSR;
#[doc = "`read()` method returns [comp_c4csr::R](comp_c4csr::R) reader structure"]
impl crate::Readable for COMP_C4CSR {}
#[doc = "`write(|w| ..)` method takes [comp_c4csr::W](comp_c4csr::W) writer structure"]
impl crate::Writable for COMP_C4CSR {}
#[doc = "Comparator control/status register"]
pub mod comp_c4csr;
