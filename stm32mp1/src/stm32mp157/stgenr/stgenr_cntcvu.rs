#[doc = "Register `STGENR_CNTCVU` reader"]
pub struct R(crate::R<STGENR_CNTCVU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_CNTCVU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_CNTCVU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_CNTCVU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNTCVU_U_32` reader - CNTCVU_U_32"]
pub struct CNTCVU_U_32_R(crate::FieldReader<u32, u32>);
impl CNTCVU_U_32_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNTCVU_U_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTCVU_U_32_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cntcvu](index.html) module"]
pub struct STGENR_CNTCVU_SPEC;
impl crate::RegisterSpec for STGENR_CNTCVU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_cntcvu::R](R) reader structure"]
impl crate::Readable for STGENR_CNTCVU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_CNTCVU to value 0"]
impl crate::Resettable for STGENR_CNTCVU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
