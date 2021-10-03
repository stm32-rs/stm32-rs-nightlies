#[doc = "Register `VVFPCCR` reader"]
pub struct R(crate::R<VVFPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVFPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVFPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVFPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VFP` reader - VFP"]
pub struct VFP_R(crate::FieldReader<u16, u16>);
impl VFP_R {
    pub(crate) fn new(bits: u16) -> Self {
        VFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VFP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VFP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvfpccr](index.html) module"]
pub struct VVFPCCR_SPEC;
impl crate::RegisterSpec for VVFPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvfpccr::R](R) reader structure"]
impl crate::Readable for VVFPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VVFPCCR to value 0"]
impl crate::Resettable for VVFPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
