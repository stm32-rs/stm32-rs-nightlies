///Register `AHB1LPENSR` writer
pub type W = crate::W<AHB1LPENSRrs>;
///Field `GPDMA1LPENS` writer - GPDMA1 sleep enable
pub type GPDMA1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12LPENS` writer - ADC12 sleep enable
pub type ADC12LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 sleep enable
    #[inline(always)]
    pub fn gpdma1lpens(&mut self) -> GPDMA1LPENS_W<'_, AHB1LPENSRrs> {
        GPDMA1LPENS_W::new(self, 4)
    }
    ///Bit 5 - ADC12 sleep enable
    #[inline(always)]
    pub fn adc12lpens(&mut self) -> ADC12LPENS_W<'_, AHB1LPENSRrs> {
        ADC12LPENS_W::new(self, 5)
    }
}
/**RCC AHB1 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB1LPENSR)*/
pub struct AHB1LPENSRrs;
impl crate::RegisterSpec for AHB1LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1lpensr::W`](W) writer structure
impl crate::Writable for AHB1LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENSR to value 0
impl crate::Resettable for AHB1LPENSRrs {}
