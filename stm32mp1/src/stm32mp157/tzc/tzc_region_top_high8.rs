#[doc = "Register `TZC_REGION_TOP_HIGH8` reader"]
pub struct R(crate::R<TZC_REGION_TOP_HIGH8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_TOP_HIGH8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_TOP_HIGH8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_TOP_HIGH8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Top address high of region are not used with 32-bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_high8](index.html) module"]
pub struct TZC_REGION_TOP_HIGH8_SPEC;
impl crate::RegisterSpec for TZC_REGION_TOP_HIGH8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_top_high8::R](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH8_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_REGION_TOP_HIGH8 to value 0"]
impl crate::Resettable for TZC_REGION_TOP_HIGH8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
