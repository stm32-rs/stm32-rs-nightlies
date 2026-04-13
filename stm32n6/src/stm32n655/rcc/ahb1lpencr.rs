///Register `AHB1LPENCR` writer
pub type W = crate::W<AHB1LPENCRrs>;
///Field `GPDMA1LPENC` writer - GPDMA1 sleep enable
pub type GPDMA1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12LPENC` writer - ADC12 sleep enable
pub type ADC12LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB1LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 4 - GPDMA1 sleep enable
    #[inline(always)]
    pub fn gpdma1lpenc(&mut self) -> GPDMA1LPENC_W<'_, AHB1LPENCRrs> {
        GPDMA1LPENC_W::new(self, 4)
    }
    ///Bit 5 - ADC12 sleep enable
    #[inline(always)]
    pub fn adc12lpenc(&mut self) -> ADC12LPENC_W<'_, AHB1LPENCRrs> {
        ADC12LPENC_W::new(self, 5)
    }
}
/**RCC AHB1 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:AHB1LPENCR)*/
pub struct AHB1LPENCRrs;
impl crate::RegisterSpec for AHB1LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb1lpencr::W`](W) writer structure
impl crate::Writable for AHB1LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENCR to value 0
impl crate::Resettable for AHB1LPENCRrs {}
