///Register `SPI_SSPCRCPR` reader
pub type R = crate::R<SPI_SSPCRCPRrs>;
///Register `SPI_SSPCRCPR` writer
pub type W = crate::W<SPI_SSPCRCPRrs>;
///Field `CRCPOLY` reader - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required.
pub type CRCPOLY_R = crate::FieldReader<u16>;
///Field `CRCPOLY` writer - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required.
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required.
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SSPCRCPR")
            .field("crcpoly", &self.crcpoly())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0007h) is the reset value of this register. Another polynomial can be configured as required.
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<'_, SPI_SSPCRCPRrs> {
        CRCPOLY_W::new(self, 0)
    }
}
/**SPI_SSPCRCPR register

You can [`read`](crate::Reg::read) this register and get [`spi_sspcrcpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sspcrcpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#SPI3:SPI_SSPCRCPR)*/
pub struct SPI_SSPCRCPRrs;
impl crate::RegisterSpec for SPI_SSPCRCPRrs {
    type Ux = u32;
}
///`read()` method returns [`spi_sspcrcpr::R`](R) reader structure
impl crate::Readable for SPI_SSPCRCPRrs {}
///`write(|w| ..)` method takes [`spi_sspcrcpr::W`](W) writer structure
impl crate::Writable for SPI_SSPCRCPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_SSPCRCPR to value 0x07
impl crate::Resettable for SPI_SSPCRCPRrs {
    const RESET_VALUE: u32 = 0x07;
}
