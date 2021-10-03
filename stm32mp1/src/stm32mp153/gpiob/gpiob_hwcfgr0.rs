#[doc = "Register `GPIOB_HWCFGR0` reader"]
pub struct R(crate::R<GPIOB_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OR_RES` reader - OR_RES"]
pub struct OR_RES_R(crate::FieldReader<u16, u16>);
impl OR_RES_R {
    pub(crate) fn new(bits: u16) -> Self {
        OR_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OR_RES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - OR_RES"]
    #[inline(always)]
    pub fn or_res(&self) -> OR_RES_R {
        OR_RES_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPIO hardware configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr0](index.html) module"]
pub struct GPIOB_HWCFGR0_SPEC;
impl crate::RegisterSpec for GPIOB_HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_hwcfgr0::R](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOB_HWCFGR0 to value 0"]
impl crate::Resettable for GPIOB_HWCFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
