#[doc = "Register `SDMMC_RESPCMDR` reader"]
pub type R = crate::R<SDMMC_RESPCMDRrs>;
#[doc = "Field `RESPCMD` reader - RESPCMD"]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - RESPCMD"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_respcmdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_RESPCMDRrs;
impl crate::RegisterSpec for SDMMC_RESPCMDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_respcmdr::R`](R) reader structure"]
impl crate::Readable for SDMMC_RESPCMDRrs {}
#[doc = "`reset()` method sets SDMMC_RESPCMDR to value 0"]
impl crate::Resettable for SDMMC_RESPCMDRrs {
    const RESET_VALUE: u32 = 0;
}
