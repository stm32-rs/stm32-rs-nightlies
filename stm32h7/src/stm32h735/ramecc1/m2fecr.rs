#[doc = "Register `M2FECR` reader"]
pub struct R(crate::R<M2FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2FECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FADD` reader - ECC error failing address"]
pub struct FADD_R(crate::FieldReader<u32, u32>);
impl FADD_R {
    pub(crate) fn new(bits: u32) -> Self {
        FADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fecr](index.html) module"]
pub struct M2FECR_SPEC;
impl crate::RegisterSpec for M2FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m2fecr::R](R) reader structure"]
impl crate::Readable for M2FECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M2FECR to value 0"]
impl crate::Resettable for M2FECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
