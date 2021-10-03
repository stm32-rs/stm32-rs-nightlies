#[doc = "Register `FMC_CSQEMSR` reader"]
pub struct R(crate::R<FMC_CSQEMSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CSQEMSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CSQEMSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CSQEMSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEM` reader - SEM"]
pub struct SEM_R(crate::FieldReader<u16, u16>);
impl SEM_R {
    pub(crate) fn new(bits: u16) -> Self {
        SEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - SEM"]
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This register holds a sector error mapping status when the whole transfer is complete.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_csqemsr](index.html) module"]
pub struct FMC_CSQEMSR_SPEC;
impl crate::RegisterSpec for FMC_CSQEMSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_csqemsr::R](R) reader structure"]
impl crate::Readable for FMC_CSQEMSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_CSQEMSR to value 0"]
impl crate::Resettable for FMC_CSQEMSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
