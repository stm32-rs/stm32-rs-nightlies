#[doc = "Register `GPIOI_HWCFGR5` reader"]
pub struct R(crate::R<GPIOI_HWCFGR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOI_HWCFGR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOI_HWCFGR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOI_HWCFGR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PUPDR_RES` reader - PUPDR_RES"]
pub struct PUPDR_RES_R(crate::FieldReader<u32, u32>);
impl PUPDR_RES_R {
    pub(crate) fn new(bits: u32) -> Self {
        PUPDR_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUPDR_RES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PUPDR_RES"]
    #[inline(always)]
    pub fn pupdr_res(&self) -> PUPDR_RES_R {
        PUPDR_RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO hardware configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioi_hwcfgr5](index.html) module"]
pub struct GPIOI_HWCFGR5_SPEC;
impl crate::RegisterSpec for GPIOI_HWCFGR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioi_hwcfgr5::R](R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOI_HWCFGR5 to value 0"]
impl crate::Resettable for GPIOI_HWCFGR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
