#[doc = "Register `CRRCR` reader"]
pub struct R(crate::R<CRRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSI48CAL` reader - Internal RC 48 MHz clock calibration"]
pub struct HSI48CAL_R(crate::FieldReader<u16, u16>);
impl HSI48CAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSI48CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48CAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Internal RC 48 MHz clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "RCC Clock Recovery RC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crrcr](index.html) module"]
pub struct CRRCR_SPEC;
impl crate::RegisterSpec for CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crrcr::R](R) reader structure"]
impl crate::Readable for CRRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
