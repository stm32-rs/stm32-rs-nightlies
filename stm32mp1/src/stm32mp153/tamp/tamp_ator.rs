#[doc = "Register `TAMP_ATOR` reader"]
pub struct R(crate::R<TAMP_ATOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ATOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ATOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ATOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRNG` reader - PRNG"]
pub struct PRNG_R(crate::FieldReader<u8, u8>);
impl PRNG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRNG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEEDF` reader - SEEDF"]
pub struct SEEDF_R(crate::FieldReader<bool, bool>);
impl SEEDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEEDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEEDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITS` reader - INITS"]
pub struct INITS_R(crate::FieldReader<bool, bool>);
impl INITS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PRNG"]
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - SEEDF"]
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - INITS"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ator](index.html) module"]
pub struct TAMP_ATOR_SPEC;
impl crate::RegisterSpec for TAMP_ATOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_ator::R](R) reader structure"]
impl crate::Readable for TAMP_ATOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAMP_ATOR to value 0"]
impl crate::Resettable for TAMP_ATOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
