#[doc = "Register `RFL` reader"]
pub struct R(crate::R<RFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFL` reader - Receive frame length"]
pub struct RFL_R(crate::FieldReader<u8, u8>);
impl RFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive frame length"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "SWPMI Receive Frame Length register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfl](index.html) module"]
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfl::R](R) reader structure"]
impl crate::Readable for RFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFL to value 0"]
impl crate::Resettable for RFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
