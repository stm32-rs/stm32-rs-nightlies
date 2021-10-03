#[doc = "Register `VHBPCCR` reader"]
pub struct R(crate::R<VHBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HBP` reader - HBP"]
pub struct HBP_R(crate::FieldReader<u16, u16>);
impl HBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        HBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HBP current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhbpccr](index.html) module"]
pub struct VHBPCCR_SPEC;
impl crate::RegisterSpec for VHBPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhbpccr::R](R) reader structure"]
impl crate::Readable for VHBPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VHBPCCR to value 0"]
impl crate::Resettable for VHBPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
