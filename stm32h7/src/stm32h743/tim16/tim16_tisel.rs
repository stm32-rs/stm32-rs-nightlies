///Register `TIM16_TISEL` reader
pub type R = crate::R<TIM16_TISELrs>;
///Register `TIM16_TISEL` writer
pub type W = crate::W<TIM16_TISELrs>;
///Field `TI1SEL` reader - selects TI1\[0\] to TI1\[15\] input
pub type TI1SEL_R = crate::FieldReader;
///Field `TI1SEL` writer - selects TI1\[0\] to TI1\[15\] input
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_TISEL")
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<'_, TIM16_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
}
/**TIM16 input selection register

You can [`read`](crate::Reg::read) this register and get [`tim16_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#TIM16:TIM16_TISEL)*/
pub struct TIM16_TISELrs;
impl crate::RegisterSpec for TIM16_TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tim16_tisel::R`](R) reader structure
impl crate::Readable for TIM16_TISELrs {}
///`write(|w| ..)` method takes [`tim16_tisel::W`](W) writer structure
impl crate::Writable for TIM16_TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM16_TISEL to value 0
impl crate::Resettable for TIM16_TISELrs {}
