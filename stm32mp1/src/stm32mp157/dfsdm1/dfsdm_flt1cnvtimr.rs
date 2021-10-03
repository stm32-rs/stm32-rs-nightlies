#[doc = "Register `DFSDM_FLT1CNVTIMR` reader"]
pub struct R(crate::R<DFSDM_FLT1CNVTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1CNVTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1CNVTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1CNVTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNVCNT` reader - CNVCNT"]
pub struct CNVCNT_R(crate::FieldReader<u32, u32>);
impl CNVCNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNVCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNVCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:31 - CNVCNT"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
#[doc = "DFSDM filter 1 conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1cnvtimr](index.html) module"]
pub struct DFSDM_FLT1CNVTIMR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1CNVTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1cnvtimr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1CNVTIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT1CNVTIMR to value 0"]
impl crate::Resettable for DFSDM_FLT1CNVTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
