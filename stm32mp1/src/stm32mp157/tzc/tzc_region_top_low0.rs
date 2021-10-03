#[doc = "Register `TZC_REGION_TOP_LOW0` reader"]
pub struct R(crate::R<TZC_REGION_TOP_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_TOP_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_TOP_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_TOP_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW"]
pub struct TOP_ADDRESS_LOW_R(crate::FieldReader<u32, u32>);
impl TOP_ADDRESS_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        TOP_ADDRESS_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_ADDRESS_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
#[doc = "Top address bits \\[31:12\\]
for region 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low0](index.html) module"]
pub struct TZC_REGION_TOP_LOW0_SPEC;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_top_low0::R](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_REGION_TOP_LOW0 to value 0xffff_ffff"]
impl crate::Resettable for TZC_REGION_TOP_LOW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
