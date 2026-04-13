///Register `TIM1_CCR4` reader
pub type R = crate::R<TIM1_CCR4rs>;
///Register `TIM1_CCR4` writer
pub type W = crate::W<TIM1_CCR4rs>;
///Field `CCR4` reader - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
pub type CCR4_R = crate::FieldReader<u16>;
///Field `CCR4` writer - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_CCR4")
            .field("ccr4", &self.ccr4())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value If channel CC4 is configured as output: CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. If channel CC4 is configured as input: CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<'_, TIM1_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**TIM1 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim1_ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM1:TIM1_CCR4)*/
pub struct TIM1_CCR4rs;
impl crate::RegisterSpec for TIM1_CCR4rs {
    type Ux = u16;
}
///`read()` method returns [`tim1_ccr4::R`](R) reader structure
impl crate::Readable for TIM1_CCR4rs {}
///`write(|w| ..)` method takes [`tim1_ccr4::W`](W) writer structure
impl crate::Writable for TIM1_CCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_CCR4 to value 0
impl crate::Resettable for TIM1_CCR4rs {}
