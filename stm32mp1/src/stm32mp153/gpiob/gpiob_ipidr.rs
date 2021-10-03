#[doc = "Register `GPIOB_IPIDR` reader"]
pub struct R(crate::R<GPIOB_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPIDR` reader - IPIDR"]
pub struct IPIDR_R(crate::FieldReader<u32, u32>);
impl IPIDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        IPIDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPIDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - IPIDR"]
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_ipidr](index.html) module"]
pub struct GPIOB_IPIDR_SPEC;
impl crate::RegisterSpec for GPIOB_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_ipidr::R](R) reader structure"]
impl crate::Readable for GPIOB_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOB_IPIDR to value 0x000f_0002"]
impl crate::Resettable for GPIOB_IPIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_0002
    }
}
