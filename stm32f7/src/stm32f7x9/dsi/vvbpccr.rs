#[doc = "Register `VVBPCCR` reader"]
pub struct R(crate::R<VVBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VBP` reader - Vertical Back-Porch duration"]
pub struct VBP_R(crate::FieldReader<u16, u16>);
impl VBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        VBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host Video VBP Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvbpccr](index.html) module"]
pub struct VVBPCCR_SPEC;
impl crate::RegisterSpec for VVBPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvbpccr::R](R) reader structure"]
impl crate::Readable for VVBPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VVBPCCR to value 0"]
impl crate::Resettable for VVBPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
