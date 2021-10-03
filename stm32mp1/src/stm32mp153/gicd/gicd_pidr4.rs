#[doc = "Register `GICD_PIDR4` reader"]
pub struct R(crate::R<GICD_PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR4` reader - PIDR4"]
pub struct PIDR4_R(crate::FieldReader<u32, u32>);
impl PIDR4_R {
    pub(crate) fn new(bits: u32) -> Self {
        PIDR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIDR4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PIDR4"]
    #[inline(always)]
    pub fn pidr4(&self) -> PIDR4_R {
        PIDR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_pidr4](index.html) module"]
pub struct GICD_PIDR4_SPEC;
impl crate::RegisterSpec for GICD_PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_pidr4::R](R) reader structure"]
impl crate::Readable for GICD_PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PIDR4 to value 0x04"]
impl crate::Resettable for GICD_PIDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
