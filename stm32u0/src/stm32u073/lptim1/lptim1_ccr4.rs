///Register `LPTIM1_CCR4` reader
pub type R = crate::R<LPTIM1_CCR4rs>;
///Register `LPTIM1_CCR4` writer
pub type W = crate::W<LPTIM1_CCR4rs>;
///Field `CCR4` reader - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM1_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM1_CCR4 register is read-only and cannot be programmed.
pub type CCR4_R = crate::FieldReader<u16>;
///Field `CCR4` writer - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM1_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM1_CCR4 register is read-only and cannot be programmed.
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM1_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM1_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM1_CCR4")
            .field("ccr4", &self.ccr4())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/compare 4 value If channel CC4 is configured as output: CCR4 is the value to be loaded in the capture/compare 4 register. Depending on the PRELOAD option, the CCR4 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 4 contains the value to be compared to the counter LPTIM1_CNT and signaled on OC4 output. If channel CC4 is configured as input: CCR4 becomes read-only, it contains the counter value transferred by the last input capture 4 event. The LPTIM1_CCR4 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<LPTIM1_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**LPTIM compare register 4

You can [`read`](crate::Reg::read) this register and get [`lptim1_ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim1_ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPTIM1:LPTIM1_CCR4)*/
pub struct LPTIM1_CCR4rs;
impl crate::RegisterSpec for LPTIM1_CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`lptim1_ccr4::R`](R) reader structure
impl crate::Readable for LPTIM1_CCR4rs {}
///`write(|w| ..)` method takes [`lptim1_ccr4::W`](W) writer structure
impl crate::Writable for LPTIM1_CCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM1_CCR4 to value 0
impl crate::Resettable for LPTIM1_CCR4rs {
    const RESET_VALUE: u32 = 0;
}
