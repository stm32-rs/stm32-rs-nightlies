///Register `IDMABSIZER` reader
pub type R = crate::R<IDMABSIZERrs>;
///Register `IDMABSIZER` writer
pub type W = crate::W<IDMABSIZERrs>;
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
        f.debug_struct("IDMABSIZER")
            .field("idmabndt", &self.idmabndt())
            .finish()
    }
}
impl W {
    ///Bits 5:12 - Number of transfers per buffer. This 8-bit value shall be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x01: buffer size = 8 words = 32 bytes. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<'_, IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
/**The SDMMC_IDMABSIZER register contains the buffers size when in double buffer configuration.

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#SDMMC1:IDMABSIZER)*/
pub struct IDMABSIZERrs;
impl crate::RegisterSpec for IDMABSIZERrs {
    type Ux = u32;
}
///`read()` method returns [`idmabsizer::R`](R) reader structure
impl crate::Readable for IDMABSIZERrs {}
///`write(|w| ..)` method takes [`idmabsizer::W`](W) writer structure
impl crate::Writable for IDMABSIZERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDMABSIZER to value 0
impl crate::Resettable for IDMABSIZERrs {}
