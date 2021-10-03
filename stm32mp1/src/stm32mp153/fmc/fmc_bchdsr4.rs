#[doc = "Register `FMC_BCHDSR4` reader"]
pub struct R(crate::R<FMC_BCHDSR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHDSR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHDSR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHDSR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EBP7` reader - EBP7"]
pub struct EBP7_R(crate::FieldReader<u16, u16>);
impl EBP7_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBP8` reader - EBP8"]
pub struct EBP8_R(crate::FieldReader<u16, u16>);
impl EBP8_R {
    pub(crate) fn new(bits: u16) -> Self {
        EBP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBP8_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - EBP7"]
    #[inline(always)]
    pub fn ebp7(&self) -> EBP7_R {
        EBP7_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP8"]
    #[inline(always)]
    pub fn ebp8(&self) -> EBP8_R {
        EBP8_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "The maximum error correction capability of the BCH block embedded in the FMC is 8 errors. This register contains the positions of the 7th and 8th error bits in EBP7 and EPB8 fields, respectively. .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchdsr4](index.html) module"]
pub struct FMC_BCHDSR4_SPEC;
impl crate::RegisterSpec for FMC_BCHDSR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchdsr4::R](R) reader structure"]
impl crate::Readable for FMC_BCHDSR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHDSR4 to value 0"]
impl crate::Resettable for FMC_BCHDSR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
