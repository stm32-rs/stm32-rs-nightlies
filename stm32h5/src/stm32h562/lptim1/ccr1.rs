#[doc = "Register `CCR1` reader"]
pub type R = crate::R<CCR1rs>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<CCR1rs>;
#[doc = "Field `CCR1` reader - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
pub type CCR1_R = crate::FieldReader<u16>;
#[doc = "Field `CCR1` writer - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
pub type CCR1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<CCR1rs> {
        CCR1_W::new(self, 0)
    }
}
#[doc = "LPTIM compare register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1rs;
impl crate::RegisterSpec for CCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for CCR1rs {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for CCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for CCR1rs {
    const RESET_VALUE: u32 = 0;
}
