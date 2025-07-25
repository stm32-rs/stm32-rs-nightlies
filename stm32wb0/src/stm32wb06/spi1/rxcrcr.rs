///Register `RXCRCR` reader
pub type R = crate::R<RXCRCRrs>;
///Field `RXCRC` reader - Rx CRC register When CRC calculation is enabled, the RxCRC\[15:0\] bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPIx_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPIx_CRCPR register. Only the 8 LSB bits are considered when the data frame format is set to be 8-bit data (CRCL bit in the SPIx_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit data frame format is selected (CRCL bit in the SPIx_CR1 register is set). CRC calculation is done based on any CRC16 standard. A read to this register when the BSY Flag is set could return an incorrect value.
pub type RXCRC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Rx CRC register When CRC calculation is enabled, the RxCRC\[15:0\] bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPIx_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPIx_CRCPR register. Only the 8 LSB bits are considered when the data frame format is set to be 8-bit data (CRCL bit in the SPIx_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit data frame format is selected (CRCL bit in the SPIx_CR1 register is set). CRC calculation is done based on any CRC16 standard. A read to this register when the BSY Flag is set could return an incorrect value.
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXCRCR")
            .field("rxcrc", &self.rxcrc())
            .finish()
    }
}
/**SPI1_RXCRCR register

You can [`read`](crate::Reg::read) this register and get [`rxcrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SPI1:RXCRCR)*/
pub struct RXCRCRrs;
impl crate::RegisterSpec for RXCRCRrs {
    type Ux = u16;
}
///`read()` method returns [`rxcrcr::R`](R) reader structure
impl crate::Readable for RXCRCRrs {}
///`reset()` method sets RXCRCR to value 0
impl crate::Resettable for RXCRCRrs {}
