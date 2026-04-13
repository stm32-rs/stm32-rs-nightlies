///Register `AHB1LPENR` reader
pub type R = crate::R<AHB1LPENRrs>;
///Register `AHB1LPENR` writer
pub type W = crate::W<AHB1LPENRrs>;
///Field `GPDMA1LPEN` reader - GPDMA1 sleep enable
pub type GPDMA1LPEN_R = crate::BitReader;
///Field `GPDMA1LPEN` writer - GPDMA1 sleep enable
pub type GPDMA1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC12LPEN` reader - ADC12 sleep enable
pub type ADC12LPEN_R = crate::BitReader;
///Field `ADC12LPEN` writer - ADC12 sleep enable
pub type ADC12LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - GPDMA1 sleep enable
    #[inline(always)]
    pub fn gpdma1lpen(&self) -> GPDMA1LPEN_R {
        GPDMA1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC12 sleep enable
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1LPENR")
            .field("gpdma1lpen", &self.gpdma1lpen())
            .field("adc12lpen", &self.adc12lpen())
            .finish()
    }
}
impl W {
    ///Bit 4 - GPDMA1 sleep enable
    #[inline(always)]
    pub fn gpdma1lpen(&mut self) -> GPDMA1LPEN_W<'_, AHB1LPENRrs> {
        GPDMA1LPEN_W::new(self, 4)
    }
    ///Bit 5 - ADC12 sleep enable
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<'_, AHB1LPENRrs> {
        ADC12LPEN_W::new(self, 5)
    }
}
/**RCC AHB1 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB1LPENR)*/
pub struct AHB1LPENRrs;
impl crate::RegisterSpec for AHB1LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1lpenr::R`](R) reader structure
impl crate::Readable for AHB1LPENRrs {}
///`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure
impl crate::Writable for AHB1LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1LPENR to value 0
impl crate::Resettable for AHB1LPENRrs {}
