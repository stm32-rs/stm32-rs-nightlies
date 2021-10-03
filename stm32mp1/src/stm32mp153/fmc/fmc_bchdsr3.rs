#[doc = "Register `FMC_BCHDSR3` reader"]
pub struct R(crate::R<FMC_BCHDSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EBP5` reader - EBP5"]
pub struct EBP5_R(crate::FieldReader<u16, u16>);
impl EBP5_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBP6` reader - EBP6"]
pub struct EBP6_R(crate::FieldReader<u16, u16>);
impl EBP6_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - EBP5"]
    #[inline(always)]
    pub fn ebp5(&self) -> EBP5_R {
        EBP5_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP6"]
    #[inline(always)]
    pub fn ebp6(&self) -> EBP6_R {
        EBP6_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr3](index.html) module"]
pub struct FMC_BCHDSR3_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchdsr3::R](R) reader structure"]
impl crate::Readable for FMC_BCHDSR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHDSR3 to value 0"]
impl crate::Resettable for FMC_BCHDSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
