#[doc = "Register `RNG_DR` reader"]
pub struct R(crate::R<RNG_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RNDATA` reader - Random data 32-bit random data which are valid when DRDY=1."]
pub struct RNDATA_R(crate::FieldReader<u32, u32>);
impl RNDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RNDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Random data 32-bit random data which are valid when DRDY=1."]
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_dr](index.html) module"]
pub struct RNG_DR_SPEC;
impl crate::RegisterSpec for RNG_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_dr::R](R) reader structure"]
impl crate::Readable for RNG_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RNG_DR to value 0"]
impl crate::Resettable for RNG_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
