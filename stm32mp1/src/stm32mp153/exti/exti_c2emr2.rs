///Register `EXTI_C2EMR2` reader
pub type R = crate::R<EXTI_C2EMR2rs>;
///Register `EXTI_C2EMR2` writer
pub type W = crate::W<EXTI_C2EMR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`exti_c2emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_c2emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTI_C2EMR2)*/
pub struct EXTI_C2EMR2rs;
impl crate::RegisterSpec for EXTI_C2EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_c2emr2::R`](R) reader structure
impl crate::Readable for EXTI_C2EMR2rs {}
///`write(|w| ..)` method takes [`exti_c2emr2::W`](W) writer structure
impl crate::Writable for EXTI_C2EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_C2EMR2 to value 0
impl crate::Resettable for EXTI_C2EMR2rs {
    const RESET_VALUE: u32 = 0;
}
