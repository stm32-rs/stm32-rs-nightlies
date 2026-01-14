///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {}
