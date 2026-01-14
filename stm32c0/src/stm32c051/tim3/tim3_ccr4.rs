///Register `TIM3_CCR4` reader
pub type R = crate::R<TIM3_CCR4rs>;
///Register `TIM3_CCR4` writer
pub type W = crate::W<TIM3_CCR4rs>;
///Field `CCR4` reader - Capture/Compare value if CC4 channel is configured as output (CC4S bits): CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. if CC4 channel is configured as input (CC4S bits in TIMx_CCMR4 register): CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
pub type CCR4_R = crate::FieldReader<u16>;
///Field `CCR4` writer - Capture/Compare value if CC4 channel is configured as output (CC4S bits): CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. if CC4 channel is configured as input (CC4S bits in TIMx_CCMR4 register): CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare value if CC4 channel is configured as output (CC4S bits): CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. if CC4 channel is configured as input (CC4S bits in TIMx_CCMR4 register): CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_CCR4")
            .field("ccr4", &self.ccr4())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value if CC4 channel is configured as output (CC4S bits): CCR4 is the value to be loaded in the actual capture/compare 4 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR2 register (bit OC4PE). Else the preload value is copied in the active capture/compare 4 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signalled on OC4 output. if CC4 channel is configured as input (CC4S bits in TIMx_CCMR4 register): CCR4 is the counter value transferred by the last input capture 4 event (IC4). The TIMx_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<'_, TIM3_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**TIM3 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim3_ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM3:TIM3_CCR4)*/
pub struct TIM3_CCR4rs;
impl crate::RegisterSpec for TIM3_CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`tim3_ccr4::R`](R) reader structure
impl crate::Readable for TIM3_CCR4rs {}
///`write(|w| ..)` method takes [`tim3_ccr4::W`](W) writer structure
impl crate::Writable for TIM3_CCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_CCR4 to value 0
impl crate::Resettable for TIM3_CCR4rs {}
