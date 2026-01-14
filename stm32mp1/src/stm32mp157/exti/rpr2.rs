///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Contains only register bits for configurable events.

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
