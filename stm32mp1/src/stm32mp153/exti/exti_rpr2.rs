///Register `EXTI_RPR2` reader
pub type R = crate::R<EXTI_RPR2rs>;
///Register `EXTI_RPR2` writer
pub type W = crate::W<EXTI_RPR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`exti_rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTI_RPR2)*/
pub struct EXTI_RPR2rs;
impl crate::RegisterSpec for EXTI_RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_rpr2::R`](R) reader structure
impl crate::Readable for EXTI_RPR2rs {}
///`write(|w| ..)` method takes [`exti_rpr2::W`](W) writer structure
impl crate::Writable for EXTI_RPR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_RPR2 to value 0
impl crate::Resettable for EXTI_RPR2rs {
    const RESET_VALUE: u32 = 0;
}
