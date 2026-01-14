///Register `CCR4` reader
pub type R = crate::R<CCR4rs>;
///Register `CCR4` writer
pub type W = crate::W<CCR4rs>;
///Field `CCR4` reader - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM2_CCR4 register is read-only and cannot be programmed.
pub type CCR4_R = crate::FieldReader<u16>;
///Field `CCR4` writer - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM2_CCR4 register is read-only and cannot be programmed.
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM2_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR4").field("ccr4", &self.ccr4()).finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM2_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<'_, CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPTIM2:CCR4)*/
pub struct CCR4rs;
impl crate::RegisterSpec for CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`ccr4::R`](R) reader structure
impl crate::Readable for CCR4rs {}
///`write(|w| ..)` method takes [`ccr4::W`](W) writer structure
impl crate::Writable for CCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4rs {}
