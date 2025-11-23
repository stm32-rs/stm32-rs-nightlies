///Register `LOCKRG` reader
pub type R = crate::R<LOCKRGrs>;
///Register `LOCKRG` writer
pub type W = crate::W<LOCKRGrs>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCKRG")
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - LOCK
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, LOCKRGrs> {
        LOCK_W::new(self, 0)
    }
}
/**EXTI lock register

You can [`read`](crate::Reg::read) this register and get [`lockrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#EXTI:LOCKRG)*/
pub struct LOCKRGrs;
impl crate::RegisterSpec for LOCKRGrs {
    type Ux = u32;
}
///`read()` method returns [`lockrg::R`](R) reader structure
impl crate::Readable for LOCKRGrs {}
///`write(|w| ..)` method takes [`lockrg::W`](W) writer structure
impl crate::Writable for LOCKRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKRG to value 0
impl crate::Resettable for LOCKRGrs {}
