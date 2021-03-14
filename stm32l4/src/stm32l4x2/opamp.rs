#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    pub opamp1_otr: OPAMP1_OTR,
    #[doc = "0x08 - OPAMP1 offset trimming register in low-power mode"]
    pub opamp1_lpotr: OPAMP1_LPOTR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - OPAMP2 control/status register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    pub opamp2_otr: OPAMP2_OTR,
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    pub opamp2_lpotr: OPAMP2_LPOTR,
}
#[doc = "OPAMP1 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_csr](opamp1_csr) module"]
pub type OPAMP1_CSR = crate::Reg<u32, _OPAMP1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_CSR;
#[doc = "`read()` method returns [opamp1_csr::R](opamp1_csr::R) reader structure"]
impl crate::Readable for OPAMP1_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp1_csr::W](opamp1_csr::W) writer structure"]
impl crate::Writable for OPAMP1_CSR {}
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1 offset trimming register in normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_otr](opamp1_otr) module"]
pub type OPAMP1_OTR = crate::Reg<u32, _OPAMP1_OTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_OTR;
#[doc = "`read()` method returns [opamp1_otr::R](opamp1_otr::R) reader structure"]
impl crate::Readable for OPAMP1_OTR {}
#[doc = "`write(|w| ..)` method takes [opamp1_otr::W](opamp1_otr::W) writer structure"]
impl crate::Writable for OPAMP1_OTR {}
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1 offset trimming register in low-power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_lpotr](opamp1_lpotr) module"]
pub type OPAMP1_LPOTR = crate::Reg<u32, _OPAMP1_LPOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP1_LPOTR;
#[doc = "`read()` method returns [opamp1_lpotr::R](opamp1_lpotr::R) reader structure"]
impl crate::Readable for OPAMP1_LPOTR {}
#[doc = "`write(|w| ..)` method takes [opamp1_lpotr::W](opamp1_lpotr::W) writer structure"]
impl crate::Writable for OPAMP1_LPOTR {}
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub mod opamp1_lpotr;
#[doc = "OPAMP2 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](opamp2_csr) module"]
pub type OPAMP2_CSR = crate::Reg<u32, _OPAMP2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_CSR;
#[doc = "`read()` method returns [opamp2_csr::R](opamp2_csr::R) reader structure"]
impl crate::Readable for OPAMP2_CSR {}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](opamp2_csr::W) writer structure"]
impl crate::Writable for OPAMP2_CSR {}
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP2 offset trimming register in normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_otr](opamp2_otr) module"]
pub type OPAMP2_OTR = crate::Reg<u32, _OPAMP2_OTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_OTR;
#[doc = "`read()` method returns [opamp2_otr::R](opamp2_otr::R) reader structure"]
impl crate::Readable for OPAMP2_OTR {}
#[doc = "`write(|w| ..)` method takes [opamp2_otr::W](opamp2_otr::W) writer structure"]
impl crate::Writable for OPAMP2_OTR {}
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2 offset trimming register in low-power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_lpotr](opamp2_lpotr) module"]
pub type OPAMP2_LPOTR = crate::Reg<u32, _OPAMP2_LPOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPAMP2_LPOTR;
#[doc = "`read()` method returns [opamp2_lpotr::R](opamp2_lpotr::R) reader structure"]
impl crate::Readable for OPAMP2_LPOTR {}
#[doc = "`write(|w| ..)` method takes [opamp2_lpotr::W](opamp2_lpotr::W) writer structure"]
impl crate::Writable for OPAMP2_LPOTR {}
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_lpotr;
