#[doc = "Register `SDMMC_DCNTR` reader"]
pub struct R(crate::R<SDMMC_DCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATACOUNT` reader - DATACOUNT"]
pub struct DATACOUNT_R(crate::FieldReader<u32, u32>);
impl DATACOUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATACOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATACOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:24 - DATACOUNT"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dcntr](index.html) module"]
pub struct SDMMC_DCNTR_SPEC;
impl crate::RegisterSpec for SDMMC_DCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dcntr::R](R) reader structure"]
impl crate::Readable for SDMMC_DCNTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDMMC_DCNTR to value 0"]
impl crate::Resettable for SDMMC_DCNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
