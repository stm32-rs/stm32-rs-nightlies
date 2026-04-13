///Register `SPI_CRCPR` reader
pub type R = crate::R<SPI_CRCPRrs>;
///Register `SPI_CRCPR` writer
pub type W = crate::W<SPI_CRCPRrs>;
///Field `CRCPOLY` reader - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required.
pub type CRCPOLY_R = crate::FieldReader<u16>;
///Field `CRCPOLY` writer - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required.
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required.
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CRCPR")
            .field("crcpoly", &self.crcpoly())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CRC polynomial register This register contains the polynomial for the CRC calculation. The CRC polynomial (0x0007) is the reset value of this register. Another polynomial can be configured as required.
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<'_, SPI_CRCPRrs> {
        CRCPOLY_W::new(self, 0)
    }
}
/**SPI CRC polynomial register

You can [`read`](crate::Reg::read) this register and get [`spi_crcpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_crcpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SPI1:SPI_CRCPR)*/
pub struct SPI_CRCPRrs;
impl crate::RegisterSpec for SPI_CRCPRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_crcpr::R`](R) reader structure
impl crate::Readable for SPI_CRCPRrs {}
///`write(|w| ..)` method takes [`spi_crcpr::W`](W) writer structure
impl crate::Writable for SPI_CRCPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_CRCPR to value 0x07
impl crate::Resettable for SPI_CRCPRrs {
    const RESET_VALUE: u16 = 0x07;
}
