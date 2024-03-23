#[doc = "Register `SPI2S_RXDR` reader"]
pub type R = crate::R<SPI2S_RXDRrs>;
#[doc = "Field `RXDR` reader - RXDR"]
pub type RXDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
#[doc = "SPI/I2S receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi2s_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_RXDRrs;
impl crate::RegisterSpec for SPI2S_RXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi2s_rxdr::R`](R) reader structure"]
impl crate::Readable for SPI2S_RXDRrs {}
#[doc = "`reset()` method sets SPI2S_RXDR to value 0"]
impl crate::Resettable for SPI2S_RXDRrs {
    const RESET_VALUE: u32 = 0;
}
