#[doc = "Register `ITLINE31` reader"]
pub struct R(crate::R<ITLINE31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RNG` reader - RNG"]
pub struct RNG_R(crate::FieldReader<bool, bool>);
impl RNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES` reader - AES"]
pub struct AES_R(crate::FieldReader<bool, bool>);
impl AES_R {
    pub(crate) fn new(bits: bool) -> Self {
        AES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "interrupt line 31 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline31](index.html) module"]
pub struct ITLINE31_SPEC;
impl crate::RegisterSpec for ITLINE31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline31::R](R) reader structure"]
impl crate::Readable for ITLINE31_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE31 to value 0"]
impl crate::Resettable for ITLINE31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
