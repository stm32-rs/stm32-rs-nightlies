#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RXCRCRrs>;
#[doc = "Field `RXCRC` reader - Rx CRC register When CRC calculation is enabled, the RXCRC\\[15:0\\]
bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used in I2S mode."]
pub type RXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register When CRC calculation is enabled, the RXCRC\\[15:0\\]
bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
#[doc = "SPI Rx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCRrs;
impl crate::RegisterSpec for RXCRCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RXCRCRrs {}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RXCRCRrs {
    const RESET_VALUE: u16 = 0;
}
