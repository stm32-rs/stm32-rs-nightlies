#[doc = "Register `GICH_EISR0` reader"]
pub struct R(crate::R<GICH_EISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_EISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_EISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_EISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EISR0` reader - EISR0"]
pub struct EISR0_R(crate::FieldReader<u32, u32>);
impl EISR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        EISR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EISR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - EISR0"]
    #[inline(always)]
    pub fn eisr0(&self) -> EISR0_R {
        EISR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICH end of interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_eisr0](index.html) module"]
pub struct GICH_EISR0_SPEC;
impl crate::RegisterSpec for GICH_EISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_eisr0::R](R) reader structure"]
impl crate::Readable for GICH_EISR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICH_EISR0 to value 0"]
impl crate::Resettable for GICH_EISR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
