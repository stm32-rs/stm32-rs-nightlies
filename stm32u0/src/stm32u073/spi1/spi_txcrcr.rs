///Register `SPI_TXCRCR` reader
pub type R = crate::R<SPI_TXCRCRrs>;
/**Field `TXCRC` reader - Tx CRC register When CRC calculation is enabled, the TXCRC\[7:0\]
bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value.*/
pub type TXCRC_R = crate::FieldReader<u16>;
impl R {
    /**Bits 0:15 - Tx CRC register When CRC calculation is enabled, the TXCRC\[7:0\]
    bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value.*/
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_TXCRCR")
            .field("txcrc", &self.txcrc())
            .finish()
    }
}
/**SPI Tx CRC register

You can [`read`](crate::Reg::read) this register and get [`spi_txcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SPI1:SPI_TXCRCR)*/
pub struct SPI_TXCRCRrs;
impl crate::RegisterSpec for SPI_TXCRCRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_txcrcr::R`](R) reader structure
impl crate::Readable for SPI_TXCRCRrs {}
///`reset()` method sets SPI_TXCRCR to value 0
impl crate::Resettable for SPI_TXCRCRrs {
    const RESET_VALUE: u16 = 0;
}