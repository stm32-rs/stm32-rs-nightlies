#[doc = "Register `GICD_SPISR7` reader"]
pub struct R(crate::R<GICD_SPISR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPISR7` reader - SPISR7"]
pub struct SPISR7_R(crate::FieldReader<u32, u32>);
impl SPISR7_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPISR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPISR7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - SPISR7"]
    #[inline(always)]
    pub fn spisr7(&self) -> SPISR7_R {
        SPISR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_spisr7](index.html) module"]
pub struct GICD_SPISR7_SPEC;
impl crate::RegisterSpec for GICD_SPISR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_spisr7::R](R) reader structure"]
impl crate::Readable for GICD_SPISR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_SPISR7 to value 0"]
impl crate::Resettable for GICD_SPISR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
