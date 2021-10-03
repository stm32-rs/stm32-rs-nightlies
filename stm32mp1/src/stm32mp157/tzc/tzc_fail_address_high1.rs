#[doc = "Register `TZC_FAIL_ADDRESS_HIGH1` reader"]
pub struct R(crate::R<TZC_FAIL_ADDRESS_HIGH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_ADDRESS_HIGH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_ADDRESS_HIGH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_ADDRESS_HIGH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Address high bit of the first failed access in the associated filter (0 to 1). Not used with 32bit address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_high1](index.html) module"]
pub struct TZC_FAIL_ADDRESS_HIGH1_SPEC;
impl crate::RegisterSpec for TZC_FAIL_ADDRESS_HIGH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_fail_address_high1::R](R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_HIGH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_FAIL_ADDRESS_HIGH1 to value 0"]
impl crate::Resettable for TZC_FAIL_ADDRESS_HIGH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
