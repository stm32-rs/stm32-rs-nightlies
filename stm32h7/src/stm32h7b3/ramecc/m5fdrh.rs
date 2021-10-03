#[doc = "Register `M5FDRH` reader"]
pub struct R(crate::R<M5FDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5FDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5FDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5FDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEC` reader - Failing error code"]
pub struct FEC_R(crate::FieldReader<u32, u32>);
impl FEC_R {
    pub(crate) fn new(bits: u32) -> Self {
        FEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5fdrh](index.html) module"]
pub struct M5FDRH_SPEC;
impl crate::RegisterSpec for M5FDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5fdrh::R](R) reader structure"]
impl crate::Readable for M5FDRH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M5FDRH to value 0"]
impl crate::Resettable for M5FDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
