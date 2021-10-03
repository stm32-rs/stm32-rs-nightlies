#[doc = "Register `VHSACCR` reader"]
pub struct R(crate::R<VHSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHSACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSA` reader - HSA"]
pub struct HSA_R(crate::FieldReader<u16, u16>);
impl HSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsaccr](index.html) module"]
pub struct VHSACCR_SPEC;
impl crate::RegisterSpec for VHSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhsaccr::R](R) reader structure"]
impl crate::Readable for VHSACCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VHSACCR to value 0"]
impl crate::Resettable for VHSACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
