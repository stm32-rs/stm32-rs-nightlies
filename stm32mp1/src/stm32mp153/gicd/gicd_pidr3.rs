#[doc = "Register `GICD_PIDR3` reader"]
pub struct R(crate::R<GICD_PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR3` reader - PIDR3"]
pub struct PIDR3_R(crate::FieldReader<u32, u32>);
impl PIDR3_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR3"]
    #[inline(always)]
    pub fn pidr3(&self) -> PIDR3_R {
        PIDR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr3](index.html) module"]
pub struct GICD_PIDR3_SPEC;
impl crate::RegisterSpec for GICD_PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr3::R](R) reader structure"]
impl crate::Readable for GICD_PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR3 to value 0"]
impl crate::Resettable for GICD_PIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
