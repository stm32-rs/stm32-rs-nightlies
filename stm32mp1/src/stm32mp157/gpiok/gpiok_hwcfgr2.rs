#[doc = "Register `GPIOK_HWCFGR2` reader"]
pub struct R(crate::R<GPIOK_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOK_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOK_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOK_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AFRL_RES` reader - AFRL_RES"]
pub struct AFRL_RES_R(crate::FieldReader<u32, u32>);
impl AFRL_RES_R {
    pub(crate) fn new(bits: u32) -> Self {
        AFRL_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRL_RES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AFRL_RES"]
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_hwcfgr2](index.html) module"]
pub struct GPIOK_HWCFGR2_SPEC;
impl crate::RegisterSpec for GPIOK_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiok_hwcfgr2::R](R) reader structure"]
impl crate::Readable for GPIOK_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOK_HWCFGR2 to value 0"]
impl crate::Resettable for GPIOK_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
