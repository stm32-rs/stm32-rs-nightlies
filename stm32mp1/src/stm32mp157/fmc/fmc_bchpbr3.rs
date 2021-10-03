#[doc = "Register `FMC_BCHPBR3` reader"]
pub struct R(crate::R<FMC_BCHPBR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHPBR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHPBR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHPBR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCHPB` reader - BCHPB"]
pub struct BCHPB_R(crate::FieldReader<u32, u32>);
impl BCHPB_R {
    pub(crate) fn new(bits: u32) -> Self {
        BCHPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCHPB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "FMC BCH Parity Bits Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr3](index.html) module"]
pub struct FMC_BCHPBR3_SPEC;
impl crate::RegisterSpec for FMC_BCHPBR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchpbr3::R](R) reader structure"]
impl crate::Readable for FMC_BCHPBR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHPBR3 to value 0"]
impl crate::Resettable for FMC_BCHPBR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
