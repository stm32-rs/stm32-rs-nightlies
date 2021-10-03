#[doc = "Register `RDR` reader"]
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receive data value"]
pub struct RDR_R(crate::FieldReader<u16, u16>);
impl RDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        RDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - Receive data value"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](index.html) module"]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdr::R](R) reader structure"]
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
