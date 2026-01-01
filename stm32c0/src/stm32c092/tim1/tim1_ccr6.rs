///Register `TIM1_CCR6` reader
pub type R = crate::R<TIM1_CCR6rs>;
///Register `TIM1_CCR6` writer
pub type W = crate::W<TIM1_CCR6rs>;
///Field `CCR6` reader - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output.
pub type CCR6_R = crate::FieldReader<u16>;
///Field `CCR6` writer - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output.
pub type CCR6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output.
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCR6")
            .field("ccr6", &self.ccr6())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 6 value CCR6 is the value to be loaded in the actual capture/compare 6 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC6PE). Else the preload value is copied in the active capture/compare 6 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC6 output.
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W<'_, TIM1_CCR6rs> {
        CCR6_W::new(self, 0)
    }
}
/**TIM1 capture/compare register 6

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR6)*/
pub struct TIM1_CCR6rs;
impl crate::RegisterSpec for TIM1_CCR6rs {
    type Ux = u16;
}
///`read()` method returns [`tim1_ccr6::R`](R) reader structure
impl crate::Readable for TIM1_CCR6rs {}
///`write(|w| ..)` method takes [`tim1_ccr6::W`](W) writer structure
impl crate::Writable for TIM1_CCR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCR6 to value 0
impl crate::Resettable for TIM1_CCR6rs {}
