#[doc = "Register `EXTI_HWCFGR9` reader"]
pub struct R(crate::R<EXTI_HWCFGR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EXTI hardware configuration register 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr9](index.html) module"]
pub struct EXTI_HWCFGR9_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr9::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR9_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR9 to value 0"]
impl crate::Resettable for EXTI_HWCFGR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
