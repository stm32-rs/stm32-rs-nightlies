///Register `AHB1RSTSR` writer
pub type W = crate::W<AHB1RSTSRrs>;
///Field `GPDMA1RSTS` writer - GPDMA1 reset
pub type GPDMA1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12RSTS` writer - ADC12 reset
pub type ADC12RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 reset
    #[inline(always)]
    pub fn gpdma1rsts(&mut self) -> GPDMA1RSTS_W<'_, AHB1RSTSRrs> {
        GPDMA1RSTS_W::new(self, 4)
    }
    ///Bit 5 - ADC12 reset
    #[inline(always)]
    pub fn adc12rsts(&mut self) -> ADC12RSTS_W<'_, AHB1RSTSRrs> {
        ADC12RSTS_W::new(self, 5)
    }
}
/**RCC AHB1 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1RSTSR)*/
pub struct AHB1RSTSRrs;
impl crate::RegisterSpec for AHB1RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1rstsr::W`](W) writer structure
impl crate::Writable for AHB1RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTSR to value 0
impl crate::Resettable for AHB1RSTSRrs {}
