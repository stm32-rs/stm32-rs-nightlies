#[doc = "Register `COUNTR` reader"]
pub struct R(crate::R<COUNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - COUNT"]
pub struct COUNT_R(crate::FieldReader<u32, u32>);
impl COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - COUNT"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "TAMP monotonic counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [countr](index.html) module"]
pub struct COUNTR_SPEC;
impl crate::RegisterSpec for COUNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [countr::R](R) reader structure"]
impl crate::Readable for COUNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNTR to value 0"]
impl crate::Resettable for COUNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
