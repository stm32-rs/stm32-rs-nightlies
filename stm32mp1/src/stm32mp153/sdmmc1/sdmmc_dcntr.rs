#[doc = "Register `SDMMC_DCNTR` reader"]
pub type R = crate::R<SDMMC_DCNTRrs>;
#[doc = "Field `DATACOUNT` reader - DATACOUNT"]
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - DATACOUNT"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dcntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_DCNTRrs;
impl crate::RegisterSpec for SDMMC_DCNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dcntr::R`](R) reader structure"]
impl crate::Readable for SDMMC_DCNTRrs {}
#[doc = "`reset()` method sets SDMMC_DCNTR to value 0"]
impl crate::Resettable for SDMMC_DCNTRrs {
    const RESET_VALUE: u32 = 0;
}
