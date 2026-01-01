///Register `C2EMR2` reader
pub type R = crate::R<C2EMR2rs>;
///Register `C2EMR2` writer
pub type W = crate::W<C2EMR2rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**EXTI CPU2 wakeup with event mask register

You can [`read`](crate::Reg::read) this register and get [`c2emr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2emr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:C2EMR2)*/
pub struct C2EMR2rs;
impl crate::RegisterSpec for C2EMR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2emr2::R`](R) reader structure
impl crate::Readable for C2EMR2rs {}
///`write(|w| ..)` method takes [`c2emr2::W`](W) writer structure
impl crate::Writable for C2EMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2EMR2 to value 0
impl crate::Resettable for C2EMR2rs {}
