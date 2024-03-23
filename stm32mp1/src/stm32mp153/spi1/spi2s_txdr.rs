#[doc = "Register `SPI2S_TXDR` writer"]
pub type W = crate::W<SPI2S_TXDRrs>;
#[doc = "Field `TXDR` writer - TXDR"]
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - TXDR"]
    #[inline(always)]
    #[must_use]
    pub fn txdr(&mut self) -> TXDR_W<SPI2S_TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
#[doc = "SPI/I2S transmit data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2s_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI2S_TXDRrs;
impl crate::RegisterSpec for SPI2S_TXDRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi2s_txdr::W`](W) writer structure"]
impl crate::Writable for SPI2S_TXDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI2S_TXDR to value 0"]
impl crate::Resettable for SPI2S_TXDRrs {
    const RESET_VALUE: u32 = 0;
}
