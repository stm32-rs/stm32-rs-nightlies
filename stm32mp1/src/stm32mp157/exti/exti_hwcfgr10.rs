#[doc = "Register `EXTI_HWCFGR10` reader"]
pub struct R(crate::R<EXTI_HWCFGR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EXTI hardware configuration register 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr10](index.html) module"]
pub struct EXTI_HWCFGR10_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr10::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR10 to value 0"]
impl crate::Resettable for EXTI_HWCFGR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
