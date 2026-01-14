///Register `IDMABSIZER` reader
pub type R = crate::R<IDMABSIZERrs>;
///Register `IDMABSIZER` writer
pub type W = crate::W<IDMABSIZERrs>;
///Field `IDMABNDT` reader - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_R = crate::FieldReader<u16>;
///Field `IDMABNDT` writer - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 5:16 - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabndt(&self) -> IDMABNDT_R {
        IDMABNDT_R::new(((self.bits >> 5) & 0x0fff) as u16)
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
    ///Bits 5:16 - Number of bytes per buffer This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by 32 to get the size of the buffer in bytes. Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes. Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmabndt(&mut self) -> IDMABNDT_W<'_, IDMABSIZERrs> {
        IDMABNDT_W::new(self, 5)
    }
}
/**SDMMC IDMA buffer size register

You can [`read`](crate::Reg::read) this register and get [`idmabsizer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmabsizer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#SDMMC1:IDMABSIZER)*/
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
