///Register `CCR1` reader
pub type R = crate::R<CCR1rs>;
///Register `CCR1` writer
pub type W = crate::W<CCR1rs>;
///Field `CCR1` reader - CCR1\[15:0\]: Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
pub type CCR1_R = crate::FieldReader<u16>;
///Field `CCR1` writer - CCR1\[15:0\]: Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CCR1\[15:0\]: Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR1").field("ccr1", &self.ccr1()).finish()
    }
}
impl W {
    ///Bits 0:15 - CCR1\[15:0\]: Capture/Compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR1 register (bit OC1PE). Else the preload value is copied in the active capture/compare 1 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 is the counter value transferred by the last input capture 1 event (IC1).
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W<'_, CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
/**CCR1 register

You can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#TIM2:CCR1)*/
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
///`read()` method returns [`ccr1::R`](R) reader structure
impl crate::Readable for CCR1rs {}
///`write(|w| ..)` method takes [`ccr1::W`](W) writer structure
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR1 to value 0
impl crate::Resettable for CCR1rs {}
