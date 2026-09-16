///Register `TZENR3` reader
pub type R = crate::R<TZENR3rs>;
///Register `TZENR3` writer
pub type W = crate::W<TZENR3rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.

You can [`read`](crate::Reg::read) this register and get [`tzenr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzenr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:TZENR3)*/
pub struct TZENR3rs;
impl crate::RegisterSpec for TZENR3rs {
    type Ux = u32;
}
///`read()` method returns [`tzenr3::R`](R) reader structure
impl crate::Readable for TZENR3rs {}
///`write(|w| ..)` method takes [`tzenr3::W`](W) writer structure
impl crate::Writable for TZENR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TZENR3 to value 0
impl crate::Resettable for TZENR3rs {}
