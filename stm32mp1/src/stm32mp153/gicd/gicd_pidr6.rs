#[doc = "Register `GICD_PIDR6` reader"]
pub struct R(crate::R<GICD_PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR6` reader - PIDR6"]
pub struct PIDR6_R(crate::FieldReader<u32, u32>);
impl PIDR6_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD peripheral ID5 to ID7 register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr6](index.html) module"]
pub struct GICD_PIDR6_SPEC;
impl crate::RegisterSpec for GICD_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr6::R](R) reader structure"]
impl crate::Readable for GICD_PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR6 to value 0"]
impl crate::Resettable for GICD_PIDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
