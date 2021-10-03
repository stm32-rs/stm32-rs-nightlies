#[doc = "Register `CDR2` reader"]
pub struct R(crate::R<CDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs"]
pub struct RDATA_ALT_R(crate::FieldReader<u32, u32>);
impl RDATA_ALT_R {
    pub(crate) fn new(bits: u32) -> Self {
        RDATA_ALT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDATA_ALT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Regular data of the master/slave alternated ADCs"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ADC x common regular data register for 32-bit dual mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr2](index.html) module"]
pub struct CDR2_SPEC;
impl crate::RegisterSpec for CDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr2::R](R) reader structure"]
impl crate::Readable for CDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR2 to value 0"]
impl crate::Resettable for CDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
