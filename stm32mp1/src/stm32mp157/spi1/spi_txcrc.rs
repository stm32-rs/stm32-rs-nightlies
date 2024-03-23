#[doc = "Register `SPI_TXCRC` reader"]
pub type R = crate::R<SPI_TXCRCrs>;
#[doc = "Field `TXCRC` reader - TXCRC"]
pub type TXCRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXCRC"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
#[doc = "SPI transmitter CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_txcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_TXCRCrs;
impl crate::RegisterSpec for SPI_TXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_txcrc::R`](R) reader structure"]
impl crate::Readable for SPI_TXCRCrs {}
#[doc = "`reset()` method sets SPI_TXCRC to value 0"]
impl crate::Resettable for SPI_TXCRCrs {
    const RESET_VALUE: u32 = 0;
}
