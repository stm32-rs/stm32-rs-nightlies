#[doc = "Register `TZC_REGION_TOP_HIGH7` reader"]
pub type R = crate::R<TZC_REGION_TOP_HIGH7rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TZC_REGION_TOP_HIGH7rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Top address high of region are not used with 32-bit address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_high7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_REGION_TOP_HIGH7rs;
impl crate::RegisterSpec for TZC_REGION_TOP_HIGH7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_region_top_high7::R`](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_HIGH7rs {}
#[doc = "`reset()` method sets TZC_REGION_TOP_HIGH7 to value 0"]
impl crate::Resettable for TZC_REGION_TOP_HIGH7rs {
    const RESET_VALUE: u32 = 0;
}
