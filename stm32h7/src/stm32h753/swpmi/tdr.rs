///Register `TDR` writer
pub type W = crate::W<TDRrs>;
///Field `TD` writer - Transmit data
pub type TD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<TDRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Transmit data
    #[inline(always)]
    pub fn td(&mut self) -> TD_W<'_, TDRrs> {
        TD_W::new(self, 0)
    }
}
/**SWPMI Transmit data register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#SWPMI:TDR)*/
pub struct TDRrs;
impl crate::RegisterSpec for TDRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tdr::W`](W) writer structure
impl crate::Writable for TDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TDR to value 0
impl crate::Resettable for TDRrs {}
