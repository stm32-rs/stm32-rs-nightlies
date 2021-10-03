#[doc = "Register `HDP_VAL` reader"]
pub struct R(crate::R<HDP_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HDPVAL` reader - HDPVAL"]
pub struct HDPVAL_R(crate::FieldReader<u8, u8>);
impl HDPVAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDPVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDPVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - HDPVAL"]
    #[inline(always)]
    pub fn hdpval(&self) -> HDPVAL_R {
        HDPVAL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HDP value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_val](index.html) module"]
pub struct HDP_VAL_SPEC;
impl crate::RegisterSpec for HDP_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hdp_val::R](R) reader structure"]
impl crate::Readable for HDP_VAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HDP_VAL to value 0"]
impl crate::Resettable for HDP_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
