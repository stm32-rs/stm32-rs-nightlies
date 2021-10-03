#[doc = "Register `DMACCARxDR` reader"]
pub struct R(crate::R<DMACCARXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub struct CURRDESAPTR_R(crate::FieldReader<u32, u32>);
impl CURRDESAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRDESAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRDESAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_dr](index.html) module"]
pub struct DMACCARXDR_SPEC;
impl crate::RegisterSpec for DMACCARXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_dr::R](R) reader structure"]
impl crate::Readable for DMACCARXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
