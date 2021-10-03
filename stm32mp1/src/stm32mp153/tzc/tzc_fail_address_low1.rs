#[doc = "Register `TZC_FAIL_ADDRESS_LOW1` reader"]
pub struct R(crate::R<TZC_FAIL_ADDRESS_LOW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_ADDRESS_LOW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_ADDRESS_LOW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_ADDRESS_LOW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR_STATUS_LOW` reader - ADDR_STATUS_LOW"]
pub struct ADDR_STATUS_LOW_R(crate::FieldReader<u32, u32>);
impl ADDR_STATUS_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_STATUS_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_STATUS_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ADDR_STATUS_LOW"]
    #[inline(always)]
    pub fn addr_status_low(&self) -> ADDR_STATUS_LOW_R {
        ADDR_STATUS_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Address low bits of the first failed access in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_address_low1](index.html) module"]
pub struct TZC_FAIL_ADDRESS_LOW1_SPEC;
impl crate::RegisterSpec for TZC_FAIL_ADDRESS_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_fail_address_low1::R](R) reader structure"]
impl crate::Readable for TZC_FAIL_ADDRESS_LOW1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_FAIL_ADDRESS_LOW1 to value 0"]
impl crate::Resettable for TZC_FAIL_ADDRESS_LOW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
