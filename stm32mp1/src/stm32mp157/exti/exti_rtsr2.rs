///Register `EXTI_RTSR2` reader
pub type R = crate::R<EXTI_RTSR2rs>;
///Register `EXTI_RTSR2` writer
pub type W = crate::W<EXTI_RTSR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`exti_rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:EXTI_RTSR2)*/
pub struct EXTI_RTSR2rs;
impl crate::RegisterSpec for EXTI_RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_rtsr2::R`](R) reader structure
impl crate::Readable for EXTI_RTSR2rs {}
///`write(|w| ..)` method takes [`exti_rtsr2::W`](W) writer structure
impl crate::Writable for EXTI_RTSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_RTSR2 to value 0
impl crate::Resettable for EXTI_RTSR2rs {
    const RESET_VALUE: u32 = 0;
}
