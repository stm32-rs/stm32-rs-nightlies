#[doc = "Register `VVACCR` reader"]
pub struct R(crate::R<VVACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VA` reader - Vertical Active duration"]
pub struct VA_R(crate::FieldReader<u16, u16>);
impl VA_R {
    pub(crate) fn new(bits: u16) -> Self {
        VA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host Video VA Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvaccr](index.html) module"]
pub struct VVACCR_SPEC;
impl crate::RegisterSpec for VVACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvaccr::R](R) reader structure"]
impl crate::Readable for VVACCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VVACCR to value 0"]
impl crate::Resettable for VVACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
