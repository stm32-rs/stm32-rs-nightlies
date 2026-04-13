///Register `TXDR` writer
pub type W = crate::W<TXDRrs>;
///Field `TXDR` writer - transmit data register
pub type TXDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<TXDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - transmit data register
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W<'_, TXDRrs> {
        TXDR_W::new(self, 0)
    }
}
#[doc = "/I2SSPI/I2S transmit data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nSee register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#SPI1:TXDR)"]
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`txdr::W`](W) writer structure
impl crate::Writable for TXDRrs {
    type Safety = crate::Safe;
}
///`reset()` method sets TXDR to value 0
impl crate::Resettable for TXDRrs {}
