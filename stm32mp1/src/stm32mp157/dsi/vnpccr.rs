#[doc = "Register `VNPCCR` reader"]
pub struct R(crate::R<VNPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VNPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VNPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VNPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub struct NPSIZE_R(crate::FieldReader<u16, u16>);
impl NPSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video null packet current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpccr](index.html) module"]
pub struct VNPCCR_SPEC;
impl crate::RegisterSpec for VNPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vnpccr::R](R) reader structure"]
impl crate::Readable for VNPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VNPCCR to value 0"]
impl crate::Resettable for VNPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
