///Register `SDMMC_IDMABSIZER` reader
pub type R = crate::R<SDMMC_IDMABSIZERrs>;
///Register `SDMMC_IDMABSIZER` writer
pub type W = crate::W<SDMMC_IDMABSIZERrs>;
///Field `IDMABNDT` reader - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_R = crate::FieldReader;
///Field `IDMABNDT` writer - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_IDMABSIZER")
            .field("idmabndt", &self.idmabndt())
            .finish()
    }
}
impl W {
    ///Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    #[must_use]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<SDMMC_IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
/**The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabsizer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabsizer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#SDMMC1:SDMMC_IDMABSIZER)*/
pub struct SDMMC_IDMABSIZERrs;
impl crate::RegisterSpec for SDMMC_IDMABSIZERrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_idmabsizer::R`](R) reader structure
impl crate::Readable for SDMMC_IDMABSIZERrs {}
///`write(|w| ..)` method takes [`sdmmc_idmabsizer::W`](W) writer structure
impl crate::Writable for SDMMC_IDMABSIZERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_IDMABSIZER to value 0
impl crate::Resettable for SDMMC_IDMABSIZERrs {
    const RESET_VALUE: u32 = 0;
}
