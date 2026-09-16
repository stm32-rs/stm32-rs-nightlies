///Register `TIM14_CCR1` reader
pub type R = crate::R<TIM14_CCR1rs>;
///Register `TIM14_CCR1` writer
pub type W = crate::W<TIM14_CCR1rs>;
///Field `CCR1` reader - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
pub type CCR1_R = crate::FieldReader<u16>;
///Field `CCR1` writer - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14_CCR1")
            .field("ccr1", &self.ccr1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W<'_, TIM14_CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
/**TIM14 capture/compare register 1

You can [`read`](crate::Reg::read) this register and get [`tim14_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM14:TIM14_CCR1)*/
pub struct TIM14_CCR1rs;
impl crate::RegisterSpec for TIM14_CCR1rs {
    type Ux = u16;
}
///`read()` method returns [`tim14_ccr1::R`](R) reader structure
impl crate::Readable for TIM14_CCR1rs {}
///`write(|w| ..)` method takes [`tim14_ccr1::W`](W) writer structure
impl crate::Writable for TIM14_CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM14_CCR1 to value 0
impl crate::Resettable for TIM14_CCR1rs {}
