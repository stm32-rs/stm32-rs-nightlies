#[doc = "Register `VLCCR` reader"]
pub struct R(crate::R<VLCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HLINE` reader - HLINE"]
pub struct HLINE_R(crate::FieldReader<u16, u16>);
impl HLINE_R {
    pub(crate) fn new(bits: u16) -> Self {
        HLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLINE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host video line current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlccr](index.html) module"]
pub struct VLCCR_SPEC;
impl crate::RegisterSpec for VLCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlccr::R](R) reader structure"]
impl crate::Readable for VLCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VLCCR to value 0"]
impl crate::Resettable for VLCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
