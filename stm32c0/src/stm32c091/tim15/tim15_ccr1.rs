///Register `TIM15_CCR1` reader
pub type R = crate::R<TIM15_CCR1rs>;
///Register `TIM15_CCR1` writer
pub type W = crate::W<TIM15_CCR1rs>;
///Field `CCR1` reader - Capture/Compare 1 value
pub type CCR1_R = crate::FieldReader<u16>;
///Field `CCR1` writer - Capture/Compare 1 value
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CCR1")
            .field("ccr1", &self.ccr1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W<'_, TIM15_CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
/**TIM15 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim15_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_CCR1)*/
pub struct TIM15_CCR1rs;
impl crate::RegisterSpec for TIM15_CCR1rs {
    type Ux = u16;
}
///`read()` method returns [`tim15_ccr1::R`](R) reader structure
impl crate::Readable for TIM15_CCR1rs {}
///`write(|w| ..)` method takes [`tim15_ccr1::W`](W) writer structure
impl crate::Writable for TIM15_CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_CCR1 to value 0
impl crate::Resettable for TIM15_CCR1rs {}
