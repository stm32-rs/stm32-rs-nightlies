///Register `SPI2S_TXDR` writer
pub type W = crate::W<SPI2S_TXDRrs>;
///Field `TXDR` writer - TXDR
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<SPI2S_TXDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - TXDR
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<'_, SPI2S_TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
/**SPI/I2S transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi2s_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPI1:SPI2S_TXDR)*/
pub struct SPI2S_TXDRrs;
impl crate::RegisterSpec for SPI2S_TXDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`spi2s_txdr::W`](W) writer structure
impl crate::Writable for SPI2S_TXDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI2S_TXDR to value 0
impl crate::Resettable for SPI2S_TXDRrs {}
