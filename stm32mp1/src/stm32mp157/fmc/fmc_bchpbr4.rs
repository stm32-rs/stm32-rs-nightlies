#[doc = "Register `FMC_BCHPBR4` reader"]
pub struct R(crate::R<FMC_BCHPBR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHPBR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHPBR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHPBR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCHPB` reader - BCHPB"]
pub struct BCHPB_R(crate::FieldReader<u8, u8>);
impl BCHPB_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCHPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCHPB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FMC BCH Parity Bits Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr4](index.html) module"]
pub struct FMC_BCHPBR4_SPEC;
impl crate::RegisterSpec for FMC_BCHPBR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchpbr4::R](R) reader structure"]
impl crate::Readable for FMC_BCHPBR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHPBR4 to value 0"]
impl crate::Resettable for FMC_BCHPBR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
