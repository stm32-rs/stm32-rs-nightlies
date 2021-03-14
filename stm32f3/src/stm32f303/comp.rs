#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 28usize],
    #[doc = "0x1c - control and status register"]
    pub comp1_csr: COMP1_CSR,
    #[doc = "0x20 - control and status register"]
    pub comp2_csr: COMP2_CSR,
    #[doc = "0x24 - control and status register"]
    pub comp3_csr: COMP3_CSR,
    #[doc = "0x28 - control and status register"]
    pub comp4_csr: COMP4_CSR,
    #[doc = "0x2c - control and status register"]
    pub comp5_csr: COMP5_CSR,
    #[doc = "0x30 - control and status register"]
    pub comp6_csr: COMP6_CSR,
    #[doc = "0x34 - control and status register"]
    pub comp7_csr: COMP7_CSR,
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
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp3_csr](comp3_csr) module"]
pub type COMP3_CSR = crate::Reg<u32, _COMP3_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP3_CSR;
#[doc = "`read()` method returns [comp3_csr::R](comp3_csr::R) reader structure"]
impl crate::Readable for COMP3_CSR {}
#[doc = "`write(|w| ..)` method takes [comp3_csr::W](comp3_csr::W) writer structure"]
impl crate::Writable for COMP3_CSR {}
#[doc = "control and status register"]
pub mod comp3_csr;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp5_csr](comp5_csr) module"]
pub type COMP5_CSR = crate::Reg<u32, _COMP5_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP5_CSR;
#[doc = "`read()` method returns [comp5_csr::R](comp5_csr::R) reader structure"]
impl crate::Readable for COMP5_CSR {}
#[doc = "`write(|w| ..)` method takes [comp5_csr::W](comp5_csr::W) writer structure"]
impl crate::Writable for COMP5_CSR {}
#[doc = "control and status register"]
pub mod comp5_csr;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp7_csr](comp7_csr) module"]
pub type COMP7_CSR = crate::Reg<u32, _COMP7_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP7_CSR;
#[doc = "`read()` method returns [comp7_csr::R](comp7_csr::R) reader structure"]
impl crate::Readable for COMP7_CSR {}
#[doc = "`write(|w| ..)` method takes [comp7_csr::W](comp7_csr::W) writer structure"]
impl crate::Writable for COMP7_CSR {}
#[doc = "control and status register"]
pub mod comp7_csr;
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](comp1_csr) module"]
pub type COMP1_CSR = crate::Reg<u32, _COMP1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP1_CSR;
#[doc = "`read()` method returns [comp1_csr::R](comp1_csr::R) reader structure"]
impl crate::Readable for COMP1_CSR {}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](comp1_csr::W) writer structure"]
impl crate::Writable for COMP1_CSR {}
#[doc = "control and status register"]
pub mod comp1_csr;
