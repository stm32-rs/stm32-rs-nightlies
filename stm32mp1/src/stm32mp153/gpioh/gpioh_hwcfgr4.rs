#[doc = "Register `GPIOH_HWCFGR4` reader"]
pub struct R(crate::R<GPIOH_HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOH_HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOH_HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOH_HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSPEED_RES` reader - OSPEED_RES"]
pub struct OSPEED_RES_R(crate::FieldReader<u32, u32>);
impl OSPEED_RES_R {
    pub(crate) fn new(bits: u32) -> Self {
        OSPEED_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSPEED_RES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - OSPEED_RES"]
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioh_hwcfgr4](index.html) module"]
pub struct GPIOH_HWCFGR4_SPEC;
impl crate::RegisterSpec for GPIOH_HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioh_hwcfgr4::R](R) reader structure"]
impl crate::Readable for GPIOH_HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOH_HWCFGR4 to value 0"]
impl crate::Resettable for GPIOH_HWCFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
