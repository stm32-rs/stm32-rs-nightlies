#[doc = "Register `TXCRCR` reader"]
pub type R = crate::R<TXCRCRrs>;
#[doc = "Field `TXCRC` reader - Tx CRC register When CRC calculation is enabled, the TXCRC\\[7:0\\]
bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used in I2S mode."]
pub type TXCRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register When CRC calculation is enabled, the TXCRC\\[7:0\\]
bits contain the computed CRC value of the subsequently transmitted bytes. This register is reset when the CRCEN bit of SPI_CR1 is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY flag is set could return an incorrect value. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
#[doc = "SPI Tx CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRCRrs;
impl crate::RegisterSpec for TXCRCRrs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txcrcr::R`](R) reader structure"]
impl crate::Readable for TXCRCRrs {}
#[doc = "`reset()` method sets TXCRCR to value 0"]
impl crate::Resettable for TXCRCRrs {
    const RESET_VALUE: u16 = 0;
}
