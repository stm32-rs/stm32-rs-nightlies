///Register `LPTIM2_CCR3` reader
pub type R = crate::R<LPTIM2_CCR3rs>;
///Register `LPTIM2_CCR3` writer
pub type W = crate::W<LPTIM2_CCR3rs>;
///Field `CCR3` reader - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed.
pub type CCR3_R = crate::FieldReader<u16>;
///Field `CCR3` writer - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed.
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM2_CCR3")
            .field("ccr3", &self.ccr3())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/compare 3 value If channel CC3 is configured as output: CCR3 is the value to be loaded in the capture/compare 3 register. Depending on the PRELOAD option, the CCR3 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 3 contains the value to be compared to the counter LPTIM2_CNT and signaled on OC3 output. If channel CC3 is configured as input: CCR3 becomes read-only, it contains the counter value transferred by the last input capture 3 event. The LPTIM2_CCR3 register is read-only and cannot be programmed.
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W<LPTIM2_CCR3rs> {
        CCR3_W::new(self, 0)
    }
}
/**LPTIM compare register 3

You can [`read`](crate::Reg::read) this register and get [`lptim2_ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptim2_ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPTIM2:LPTIM2_CCR3)*/
pub struct LPTIM2_CCR3rs;
impl crate::RegisterSpec for LPTIM2_CCR3rs {
    type Ux = u32;
}
///`read()` method returns [`lptim2_ccr3::R`](R) reader structure
impl crate::Readable for LPTIM2_CCR3rs {}
///`write(|w| ..)` method takes [`lptim2_ccr3::W`](W) writer structure
impl crate::Writable for LPTIM2_CCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPTIM2_CCR3 to value 0
impl crate::Resettable for LPTIM2_CCR3rs {
    const RESET_VALUE: u32 = 0;
}