#[doc = "Register `FMC_BCHDSR1` reader"]
pub struct R(crate::R<FMC_BCHDSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EBP1` reader - EBP1"]
pub struct EBP1_R(crate::FieldReader<u16, u16>);
impl EBP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBP2` reader - EBP2"]
pub struct EBP2_R(crate::FieldReader<u16, u16>);
impl EBP2_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - EBP1"]
    #[inline(always)]
    pub fn ebp1(&self) -> EBP1_R {
        EBP1_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP2"]
    #[inline(always)]
    pub fn ebp2(&self) -> EBP2_R {
        EBP2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr1](index.html) module"]
pub struct FMC_BCHDSR1_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchdsr1::R](R) reader structure"]
impl crate::Readable for FMC_BCHDSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHDSR1 to value 0"]
impl crate::Resettable for FMC_BCHDSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
