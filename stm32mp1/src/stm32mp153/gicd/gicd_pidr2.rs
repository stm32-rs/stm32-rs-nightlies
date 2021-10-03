#[doc = "Register `GICD_PIDR2` reader"]
pub struct R(crate::R<GICD_PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR2` reader - PIDR2"]
pub struct PIDR2_R(crate::FieldReader<u32, u32>);
impl PIDR2_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR2"]
    #[inline(always)]
    pub fn pidr2(&self) -> PIDR2_R {
        PIDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr2](index.html) module"]
pub struct GICD_PIDR2_SPEC;
impl crate::RegisterSpec for GICD_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr2::R](R) reader structure"]
impl crate::Readable for GICD_PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR2 to value 0x2b"]
impl crate::Resettable for GICD_PIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2b
    }
}
