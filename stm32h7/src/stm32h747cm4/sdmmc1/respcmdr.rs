#[doc = "Register `RESPCMDR` reader"]
pub struct R(crate::R<RESPCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESPCMD` reader - Response command index"]
pub struct RESPCMD_R(crate::FieldReader<u8, u8>);
impl RESPCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESPCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "SDMMC command response register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [respcmdr](index.html) module"]
pub struct RESPCMDR_SPEC;
impl crate::RegisterSpec for RESPCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [respcmdr::R](R) reader structure"]
impl crate::Readable for RESPCMDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESPCMDR to value 0xa3c5_dd01"]
impl crate::Resettable for RESPCMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
