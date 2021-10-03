#[doc = "Register `RNG_HWCFGR` reader"]
pub struct R(crate::R<RNG_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RNG hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_hwcfgr](index.html) module"]
pub struct RNG_HWCFGR_SPEC;
impl crate::RegisterSpec for RNG_HWCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_hwcfgr::R](R) reader structure"]
impl crate::Readable for RNG_HWCFGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RNG_HWCFGR to value 0x06"]
impl crate::Resettable for RNG_HWCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
