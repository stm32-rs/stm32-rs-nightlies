#[doc = "Register `RDATA` reader"]
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - RES"]
pub struct RES_R(crate::FieldReader<u32, u32>);
impl RES_R {
    pub(crate) fn new(bits: u32) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "CORDIC result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](index.html) module"]
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdata::R](R) reader structure"]
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
