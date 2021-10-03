#[doc = "Register `TZC_REGION_BASE_HIGH3` reader"]
pub struct R(crate::R<TZC_REGION_BASE_HIGH3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_BASE_HIGH3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_BASE_HIGH3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_BASE_HIGH3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Base address high are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_base_high3](index.html) module"]
pub struct TZC_REGION_BASE_HIGH3_SPEC;
impl crate::RegisterSpec for TZC_REGION_BASE_HIGH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_base_high3::R](R) reader structure"]
impl crate::Readable for TZC_REGION_BASE_HIGH3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_REGION_BASE_HIGH3 to value 0"]
impl crate::Resettable for TZC_REGION_BASE_HIGH3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
