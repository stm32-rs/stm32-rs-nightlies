///Register `AHB1RSTCR` writer
pub type W = crate::W<AHB1RSTCRrs>;
///Field `GPDMA1RSTC` writer - GPDMA1 reset
pub type GPDMA1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12RSTC` writer - ADC12 reset
pub type ADC12RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 reset
    #[inline(always)]
    pub fn gpdma1rstc(&mut self) -> GPDMA1RSTC_W<'_, AHB1RSTCRrs> {
        GPDMA1RSTC_W::new(self, 4)
    }
    ///Bit 5 - ADC12 reset
    #[inline(always)]
    pub fn adc12rstc(&mut self) -> ADC12RSTC_W<'_, AHB1RSTCRrs> {
        ADC12RSTC_W::new(self, 5)
    }
}
/**RCC AHB1 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB1RSTCR)*/
pub struct AHB1RSTCRrs;
impl crate::RegisterSpec for AHB1RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1rstcr::W`](W) writer structure
impl crate::Writable for AHB1RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1RSTCR to value 0
impl crate::Resettable for AHB1RSTCRrs {}
