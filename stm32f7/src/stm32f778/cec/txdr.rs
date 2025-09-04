///Register `TXDR` writer
pub type W = crate::W<TXDRrs>;
///Field `TXD` writer - Tx Data register
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<TXDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<TXDRrs> {
        TXD_W::new(self, 0)
    }
}
/**Tx data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#CEC:TXDR)*/
pub struct TXDRrs;
impl crate::RegisterSpec for TXDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`txdr::W`](W) writer structure
impl crate::Writable for TXDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TXDR to value 0
impl crate::Resettable for TXDRrs {}
