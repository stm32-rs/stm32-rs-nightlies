///Register `AHB1ENCR` writer
pub type W = crate::W<AHB1ENCRrs>;
///Field `GPDMA1ENC` writer - GPDMA1 enable
pub type GPDMA1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12ENC` writer - ADC12 enable
pub type ADC12ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 enable
    #[inline(always)]
    pub fn gpdma1enc(&mut self) -> GPDMA1ENC_W<'_, AHB1ENCRrs> {
        GPDMA1ENC_W::new(self, 4)
    }
    ///Bit 5 - ADC12 enable
    #[inline(always)]
    pub fn adc12enc(&mut self) -> ADC12ENC_W<'_, AHB1ENCRrs> {
        ADC12ENC_W::new(self, 5)
    }
}
/**RCC AHB1 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENCR)*/
pub struct AHB1ENCRrs;
impl crate::RegisterSpec for AHB1ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1encr::W`](W) writer structure
impl crate::Writable for AHB1ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENCR to value 0
impl crate::Resettable for AHB1ENCRrs {}
