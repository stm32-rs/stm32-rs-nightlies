#[doc = "This register contains the timing parameters of each SDRAM bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](sdtr) module"]
pub type SDTR = crate::Reg<u32, _SDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDTR;
#[doc = "`read()` method returns [sdtr::R](sdtr::R) reader structure"]
impl crate::Readable for SDTR {}
#[doc = "`write(|w| ..)` method takes [sdtr::W](sdtr::W) writer structure"]
impl crate::Writable for SDTR {}
#[doc = "This register contains the timing parameters of each SDRAM bank"]
pub mod sdtr;
#[doc = "This register contains the control parameters for each SDRAM memory bank\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](sdcr) module"]
pub type SDCR = crate::Reg<u32, _SDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR;
#[doc = "`read()` method returns [sdcr::R](sdcr::R) reader structure"]
impl crate::Readable for SDCR {}
#[doc = "`write(|w| ..)` method takes [sdcr::W](sdcr::W) writer structure"]
impl crate::Writable for SDCR {}
#[doc = "This register contains the control parameters for each SDRAM memory bank"]
pub mod sdcr;
