///Register `RESPCMDR` reader
pub type R = crate::R<RESPCMDRrs>;
///Field `RESPCMD` reader - RESPCMD
pub type RESPCMD_R = crate::FieldReader;
impl R {
    ///Bits 0:5 - RESPCMD
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESPCMDR")
            .field("respcmd", &self.respcmd())
            .finish()
    }
}
/**The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).

You can [`read`](crate::Reg::read) this register and get [`respcmdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:RESPCMDR)*/
pub struct RESPCMDRrs;
impl crate::RegisterSpec for RESPCMDRrs {
    type Ux = u32;
}
///`read()` method returns [`respcmdr::R`](R) reader structure
impl crate::Readable for RESPCMDRrs {}
///`reset()` method sets RESPCMDR to value 0
impl crate::Resettable for RESPCMDRrs {}
