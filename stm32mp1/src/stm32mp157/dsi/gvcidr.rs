#[doc = "Register `GVCIDR` reader"]
pub struct R(crate::R<GVCIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GVCIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GVCIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCID` reader - VCID"]
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
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "DSI Host generic VCID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gvcidr](index.html) module"]
pub struct GVCIDR_SPEC;
impl crate::RegisterSpec for GVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gvcidr::R](R) reader structure"]
impl crate::Readable for GVCIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GVCIDR to value 0"]
impl crate::Resettable for GVCIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
