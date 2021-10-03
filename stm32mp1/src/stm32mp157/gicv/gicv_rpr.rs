#[doc = "Register `GICV_RPR` reader"]
pub struct R(crate::R<GICV_RPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_RPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_RPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_RPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub struct PRIORITY_R(crate::FieldReader<u8, u8>);
impl PRIORITY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[doc = "GICV VM running priority register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_rpr](index.html) module"]
pub struct GICV_RPR_SPEC;
impl crate::RegisterSpec for GICV_RPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_rpr::R](R) reader structure"]
impl crate::Readable for GICV_RPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICV_RPR to value 0xff"]
impl crate::Resettable for GICV_RPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
