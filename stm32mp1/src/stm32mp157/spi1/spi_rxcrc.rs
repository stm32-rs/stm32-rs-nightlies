#[doc = "Register `SPI_RXCRC` reader"]
pub type R = crate::R<SPI_RXCRCrs>;
#[doc = "Field `RXCRC` reader - RXCRC"]
pub type RXCRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXCRC"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
#[doc = "SPI receiver CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rxcrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_RXCRCrs;
impl crate::RegisterSpec for SPI_RXCRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rxcrc::R`](R) reader structure"]
impl crate::Readable for SPI_RXCRCrs {}
#[doc = "`reset()` method sets SPI_RXCRC to value 0"]
impl crate::Resettable for SPI_RXCRCrs {
    const RESET_VALUE: u32 = 0;
}
