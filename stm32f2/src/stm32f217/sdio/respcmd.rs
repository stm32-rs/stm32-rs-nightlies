///Register `RESPCMD` reader
pub type R = crate::R<RESPCMDrs>;
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
        f.debug_struct("RESPCMD")
            .field("respcmd", &self.respcmd())
            .finish()
    }
}
/**command response register

You can [`read`](crate::Reg::read) this register and get [`respcmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#SDIO:RESPCMD)*/
pub struct RESPCMDrs;
impl crate::RegisterSpec for RESPCMDrs {
    type Ux = u32;
}
///`read()` method returns [`respcmd::R`](R) reader structure
impl crate::Readable for RESPCMDrs {}
///`reset()` method sets RESPCMD to value 0
impl crate::Resettable for RESPCMDrs {}
