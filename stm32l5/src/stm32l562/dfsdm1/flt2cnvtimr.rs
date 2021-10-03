#[doc = "Register `FLT2CNVTIMR` reader"]
pub struct R(crate::R<FLT2CNVTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT2CNVTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT2CNVTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT2CNVTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN"]
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
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDM_CKIN"]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
#[doc = "conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt2cnvtimr](index.html) module"]
pub struct FLT2CNVTIMR_SPEC;
impl crate::RegisterSpec for FLT2CNVTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt2cnvtimr::R](R) reader structure"]
impl crate::Readable for FLT2CNVTIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLT2CNVTIMR to value 0"]
impl crate::Resettable for FLT2CNVTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
