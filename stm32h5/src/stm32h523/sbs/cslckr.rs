///Register `CSLCKR` reader
pub type R = crate::R<CSLCKRrs>;
///Register `CSLCKR` writer
pub type W = crate::W<CSLCKRrs>;
///Field `LOCKSVTAIRCR` reader - VTOR_S and AIRCR register lock
pub type LOCKSVTAIRCR_R = crate::BitReader;
///Field `LOCKSVTAIRCR` writer - VTOR_S and AIRCR register lock
pub type LOCKSVTAIRCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSMPU` reader - secure MPU registers lock
pub type LOCKSMPU_R = crate::BitReader;
///Field `LOCKSMPU` writer - secure MPU registers lock
pub type LOCKSMPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCKSAU` reader - SAU registers lock
pub type LOCKSAU_R = crate::BitReader;
///Field `LOCKSAU` writer - SAU registers lock
pub type LOCKSAU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - VTOR_S and AIRCR register lock
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure MPU registers lock
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SAU registers lock
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSLCKR")
            .field("locksvtaircr", &self.locksvtaircr())
            .field("locksmpu", &self.locksmpu())
            .field("locksau", &self.locksau())
            .finish()
    }
}
impl W {
    ///Bit 0 - VTOR_S and AIRCR register lock
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<'_, CSLCKRrs> {
        LOCKSVTAIRCR_W::new(self, 0)
    }
    ///Bit 1 - secure MPU registers lock
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<'_, CSLCKRrs> {
        LOCKSMPU_W::new(self, 1)
    }
    ///Bit 2 - SAU registers lock
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W<'_, CSLCKRrs> {
        LOCKSAU_W::new(self, 2)
    }
}
/**SBS CPU secure lock register

You can [`read`](crate::Reg::read) this register and get [`cslckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cslckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#SBS:CSLCKR)*/
pub struct CSLCKRrs;
impl crate::RegisterSpec for CSLCKRrs {
    type Ux = u32;
}
///`read()` method returns [`cslckr::R`](R) reader structure
impl crate::Readable for CSLCKRrs {}
///`write(|w| ..)` method takes [`cslckr::W`](W) writer structure
impl crate::Writable for CSLCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSLCKR to value 0
impl crate::Resettable for CSLCKRrs {}
