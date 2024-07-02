///Register `EXTI_LOCKR` reader
pub type R = crate::R<EXTI_LOCKRrs>;
///Register `EXTI_LOCKR` writer
pub type W = crate::W<EXTI_LOCKRrs>;
///Field `LOCK` reader - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_LOCKR")
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Global security and privilege configuration registers (EXTI_SECCFGR and EXTI_PRIVCFGR) lock This bit is written once after reset.
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<EXTI_LOCKRrs> {
        LOCK_W::new(self, 0)
    }
}
/**EXTI lock register

You can [`read`](crate::Reg::read) this register and get [`exti_lockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_lockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#EXTI:EXTI_LOCKR)*/
pub struct EXTI_LOCKRrs;
impl crate::RegisterSpec for EXTI_LOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`exti_lockr::R`](R) reader structure
impl crate::Readable for EXTI_LOCKRrs {}
///`write(|w| ..)` method takes [`exti_lockr::W`](W) writer structure
impl crate::Writable for EXTI_LOCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_LOCKR to value 0
impl crate::Resettable for EXTI_LOCKRrs {
    const RESET_VALUE: u32 = 0;
}
