///Register `CEC_TXDR` writer
pub type W = crate::W<CEC_TXDRrs>;
///Field `TXD` writer - Tx Data register
pub type TXD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<CEC_TXDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Tx Data register
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<'_, CEC_TXDRrs> {
        TXD_W::new(self, 0)
    }
}
/**CEC Tx data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_txdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_TXDR)*/
pub struct CEC_TXDRrs;
impl crate::RegisterSpec for CEC_TXDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cec_txdr::W`](W) writer structure
impl crate::Writable for CEC_TXDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEC_TXDR to value 0
impl crate::Resettable for CEC_TXDRrs {}
