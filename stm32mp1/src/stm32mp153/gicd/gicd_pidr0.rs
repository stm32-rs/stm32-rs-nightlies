#[doc = "Register `GICD_PIDR0` reader"]
pub struct R(crate::R<GICD_PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR0` reader - PIDR0"]
pub struct PIDR0_R(crate::FieldReader<u32, u32>);
impl PIDR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR0"]
    #[inline(always)]
    pub fn pidr0(&self) -> PIDR0_R {
        PIDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr0](index.html) module"]
pub struct GICD_PIDR0_SPEC;
impl crate::RegisterSpec for GICD_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr0::R](R) reader structure"]
impl crate::Readable for GICD_PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR0 to value 0x90"]
impl crate::Resettable for GICD_PIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x90
    }
}
