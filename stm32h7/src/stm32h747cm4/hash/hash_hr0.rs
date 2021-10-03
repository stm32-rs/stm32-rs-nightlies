#[doc = "Register `HASH_HR0` reader"]
pub struct R(crate::R<HASH_HR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_HR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_HR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_HR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H0` reader - H0"]
pub struct H0_R(crate::FieldReader<u32, u32>);
impl H0_R {
    pub(crate) fn new(bits: u32) -> Self {
        H0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h0(&self) -> H0_R {
        H0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "HASH digest register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_hr0](index.html) module"]
pub struct HASH_HR0_SPEC;
impl crate::RegisterSpec for HASH_HR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_hr0::R](R) reader structure"]
impl crate::Readable for HASH_HR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_HR0 to value 0"]
impl crate::Resettable for HASH_HR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
