///Register `SDMMC_RESPCMDR` reader
pub type R = crate::R<SDMMC_RESPCMDRrs>;
///Field `RESPCMD` reader - Response command index
pub type RESPCMD_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - Response command index
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_RESPCMDR")
            .field("respcmd", &self.respcmd())
            .finish()
    }
}
/**SDMMC command response register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_respcmdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#SDMMC1:SDMMC_RESPCMDR)*/
pub struct SDMMC_RESPCMDRrs;
impl crate::RegisterSpec for SDMMC_RESPCMDRrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_respcmdr::R`](R) reader structure
impl crate::Readable for SDMMC_RESPCMDRrs {}
///`reset()` method sets SDMMC_RESPCMDR to value 0xa3c5_dd01
impl crate::Resettable for SDMMC_RESPCMDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
