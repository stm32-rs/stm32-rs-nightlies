///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Page Erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Page Erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - Mass Erase
pub type MER_R = crate::BitReader;
///Field `MER` writer - Mass Erase
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTPG` reader - Option byte programming
pub type OPTPG_R = crate::BitReader;
///Field `OPTPG` writer - Option byte programming
pub type OPTPG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTER` reader - Option byte erase
pub type OPTER_R = crate::BitReader;
///Field `OPTER` writer - Option byte erase
pub type OPTER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Lock
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Lock
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTWRE` reader - Option bytes write enable
pub type OPTWRE_R = crate::BitReader;
///Field `OPTWRE` writer - Option bytes write enable
pub type OPTWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - End of operation interrupt enable
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - End of operation interrupt enable
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page Erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass Erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Option byte programming
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    pub fn optwre(&self) -> OPTWRE_R {
        OPTWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("optpg", &self.optpg())
            .field("opter", &self.opter())
            .field("strt", &self.strt())
            .field("lock", &self.lock())
            .field("optwre", &self.optwre())
            .field("errie", &self.errie())
            .field("eopie", &self.eopie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page Erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Mass Erase
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, CRrs> {
        MER_W::new(self, 2)
    }
    ///Bit 4 - Option byte programming
    #[inline(always)]
    pub fn optpg(&mut self) -> OPTPG_W<'_, CRrs> {
        OPTPG_W::new(self, 4)
    }
    ///Bit 5 - Option byte erase
    #[inline(always)]
    pub fn opter(&mut self) -> OPTER_W<'_, CRrs> {
        OPTER_W::new(self, 5)
    }
    ///Bit 6 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, CRrs> {
        STRT_W::new(self, 6)
    }
    ///Bit 7 - Lock
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, CRrs> {
        LOCK_W::new(self, 7)
    }
    ///Bit 9 - Option bytes write enable
    #[inline(always)]
    pub fn optwre(&mut self) -> OPTWRE_W<'_, CRrs> {
        OPTWRE_W::new(self, 9)
    }
    ///Bit 10 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CRrs> {
        ERRIE_W::new(self, 10)
    }
    ///Bit 12 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, CRrs> {
        EOPIE_W::new(self, 12)
    }
}
/**Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#FLASH:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x80
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x80;
}
