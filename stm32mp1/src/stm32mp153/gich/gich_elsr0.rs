#[doc = "Register `GICH_ELSR0` reader"]
pub struct R(crate::R<GICH_ELSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_ELSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_ELSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_ELSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ELSR0` reader - ELSR0"]
pub struct ELSR0_R(crate::FieldReader<u32, u32>);
impl ELSR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        ELSR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELSR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ELSR0"]
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICH empty list status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gich_elsr0](index.html) module"]
pub struct GICH_ELSR0_SPEC;
impl crate::RegisterSpec for GICH_ELSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gich_elsr0::R](R) reader structure"]
impl crate::Readable for GICH_ELSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICH_ELSR0 to value 0x0f"]
impl crate::Resettable for GICH_ELSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
