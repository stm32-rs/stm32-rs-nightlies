#[doc = "Register `LPTIM_SIDR` reader"]
pub struct R(crate::R<LPTIM_SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_SIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `S_ID` reader - S_ID"]
pub struct S_ID_R(crate::FieldReader<u32, u32>);
impl S_ID_R {
    pub(crate) fn new(bits: u32) -> Self {
        S_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - S_ID"]
    #[inline(always)]
    pub fn s_id(&self) -> S_ID_R {
        S_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "LPTIM registers map size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_sidr](index.html) module"]
pub struct LPTIM_SIDR_SPEC;
impl crate::RegisterSpec for LPTIM_SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_sidr::R](R) reader structure"]
impl crate::Readable for LPTIM_SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for LPTIM_SIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
