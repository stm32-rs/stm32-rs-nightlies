///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {}
