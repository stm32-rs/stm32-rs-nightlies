///Register `SPI2S_RXDR` reader
pub type R = crate::R<SPI2S_RXDRrs>;
///Field `RXDR` reader - RXDR
pub type RXDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - RXDR
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2S_RXDR")
            .field("rxdr", &self.rxdr())
            .finish()
    }
}
/**SPI/I2S receive data register

You can [`read`](crate::Reg::read) this register and get [`spi2s_rxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPI1:SPI2S_RXDR)*/
pub struct SPI2S_RXDRrs;
impl crate::RegisterSpec for SPI2S_RXDRrs {
    type Ux = u32;
}
///`read()` method returns [`spi2s_rxdr::R`](R) reader structure
impl crate::Readable for SPI2S_RXDRrs {}
///`reset()` method sets SPI2S_RXDR to value 0
impl crate::Resettable for SPI2S_RXDRrs {}
