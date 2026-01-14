///Register `EMR2` reader
pub type R = crate::R<EMR2rs>;
///Register `EMR2` writer
pub type W = crate::W<EMR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**EXTI CPU wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EMR2)*/
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`emr2::R`](R) reader structure
impl crate::Readable for EMR2rs {}
///`write(|w| ..)` method takes [`emr2::W`](W) writer structure
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EMR2 to value 0
impl crate::Resettable for EMR2rs {}
