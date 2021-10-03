#[doc = "Register `GPIOE_HWCFGR1` reader"]
pub struct R(crate::R<GPIOE_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AFRH_RES` reader - AFRH_RES"]
pub struct AFRH_RES_R(crate::FieldReader<u32, u32>);
impl AFRH_RES_R {
    pub(crate) fn new(bits: u32) -> Self {
        AFRH_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFRH_RES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AFRH_RES"]
    #[inline(always)]
    pub fn afrh_res(&self) -> AFRH_RES_R {
        AFRH_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO hardware configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_hwcfgr1](index.html) module"]
pub struct GPIOE_HWCFGR1_SPEC;
impl crate::RegisterSpec for GPIOE_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_hwcfgr1::R](R) reader structure"]
impl crate::Readable for GPIOE_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOE_HWCFGR1 to value 0"]
impl crate::Resettable for GPIOE_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
