#[doc = "Register `DLYB_SIDR` reader"]
pub struct R(crate::R<DLYB_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYB_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYB_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYB_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SID` reader - SID"]
pub struct SID_R(crate::FieldReader<u32, u32>);
impl SID_R {
    pub(crate) fn new(bits: u32) -> Self {
        SID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DLYB size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyb_sidr](index.html) module"]
pub struct DLYB_SIDR_SPEC;
impl crate::RegisterSpec for DLYB_SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyb_sidr::R](R) reader structure"]
impl crate::Readable for DLYB_SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLYB_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for DLYB_SIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}