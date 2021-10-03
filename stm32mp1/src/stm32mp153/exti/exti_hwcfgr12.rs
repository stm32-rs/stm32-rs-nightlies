#[doc = "Register `EXTI_HWCFGR12` reader"]
pub struct R(crate::R<EXTI_HWCFGR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TZ` reader - TZ"]
pub struct TZ_R(crate::FieldReader<u32, u32>);
impl TZ_R {
    pub(crate) fn new(bits: u32) -> Self {
        TZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TZ"]
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "EXTI hardware configuration register 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr12](index.html) module"]
pub struct EXTI_HWCFGR12_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr12::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR12 to value 0x050e_ffff"]
impl crate::Resettable for EXTI_HWCFGR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x050e_ffff
    }
}