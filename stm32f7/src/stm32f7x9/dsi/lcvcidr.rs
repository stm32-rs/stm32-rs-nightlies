#[doc = "Register `LCVCIDR` reader"]
pub struct R(crate::R<LCVCIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCVCIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCVCIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCID` reader - Virtual Channel ID"]
pub struct VCID_R(crate::FieldReader<u8, u8>);
impl VCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "DSI Host LTDC Current VCID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcvcidr](index.html) module"]
pub struct LCVCIDR_SPEC;
impl crate::RegisterSpec for LCVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcvcidr::R](R) reader structure"]
impl crate::Readable for LCVCIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCVCIDR to value 0"]
impl crate::Resettable for LCVCIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
