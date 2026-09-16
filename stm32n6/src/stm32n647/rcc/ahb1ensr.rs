///Register `AHB1ENSR` writer
pub type W = crate::W<AHB1ENSRrs>;
///Field `GPDMA1ENS` writer - GPDMA1 enable
pub type GPDMA1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12ENS` writer - ADC12 enable
pub type ADC12ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 enable
    #[inline(always)]
    pub fn gpdma1ens(&mut self) -> GPDMA1ENS_W<'_, AHB1ENSRrs> {
        GPDMA1ENS_W::new(self, 4)
    }
    ///Bit 5 - ADC12 enable
    #[inline(always)]
    pub fn adc12ens(&mut self) -> ADC12ENS_W<'_, AHB1ENSRrs> {
        ADC12ENS_W::new(self, 5)
    }
}
/**RCC AHB1 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB1ENSR)*/
pub struct AHB1ENSRrs;
impl crate::RegisterSpec for AHB1ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1ensr::W`](W) writer structure
impl crate::Writable for AHB1ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENSR to value 0
impl crate::Resettable for AHB1ENSRrs {}
