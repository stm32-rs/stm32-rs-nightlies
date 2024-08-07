///Register `SPI_RXCRC` reader
pub type R = crate::R<SPI_RXCRCrs>;
///Field `RXCRC` reader - RXCRC
pub type RXCRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RXCRC
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_RXCRC")
            .field("rxcrc", &self.rxcrc())
            .finish()
    }
}
/**SPI receiver CRC register

You can [`read`](crate::Reg::read) this register and get [`spi_rxcrc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPI1:SPI_RXCRC)*/
pub struct SPI_RXCRCrs;
impl crate::RegisterSpec for SPI_RXCRCrs {
    type Ux = u32;
}
///`read()` method returns [`spi_rxcrc::R`](R) reader structure
impl crate::Readable for SPI_RXCRCrs {}
///`reset()` method sets SPI_RXCRC to value 0
impl crate::Resettable for SPI_RXCRCrs {
    const RESET_VALUE: u32 = 0;
}
