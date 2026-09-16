///Register `PVTREG_LOCKR` reader
pub type R = crate::R<PVTREG_LOCKRrs>;
///Register `PVTREG_LOCKR` writer
pub type W = crate::W<PVTREG_LOCKRrs>;
///Field `LOCK` reader - PVT software lock register
pub type LOCK_R = crate::FieldReader<u32>;
///Field `LOCK` writer - PVT software lock register
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PVT software lock register
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTREG_LOCKR")
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PVT software lock register
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, PVTREG_LOCKRrs> {
        LOCK_W::new(self, 0)
    }
}
/**DTS PVT register lock register

You can [`read`](crate::Reg::read) this register and get [`pvtreg_lockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvtreg_lockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:PVTREG_LOCKR)*/
pub struct PVTREG_LOCKRrs;
impl crate::RegisterSpec for PVTREG_LOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`pvtreg_lockr::R`](R) reader structure
impl crate::Readable for PVTREG_LOCKRrs {}
///`write(|w| ..)` method takes [`pvtreg_lockr::W`](W) writer structure
impl crate::Writable for PVTREG_LOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PVTREG_LOCKR to value 0
impl crate::Resettable for PVTREG_LOCKRrs {}
