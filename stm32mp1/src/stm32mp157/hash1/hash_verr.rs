#[doc = "Register `HASH_VERR` reader"]
pub struct R(crate::R<HASH_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_VERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VER` reader - VER"]
pub struct VER_R(crate::FieldReader<u8, u8>);
impl VER_R {
    pub(crate) fn new(bits: u8) -> Self {
        VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - VER"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HASH Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_verr](index.html) module"]
pub struct HASH_VERR_SPEC;
impl crate::RegisterSpec for HASH_VERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_verr::R](R) reader structure"]
impl crate::Readable for HASH_VERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_VERR to value 0x23"]
impl crate::Resettable for HASH_VERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x23
    }
}
