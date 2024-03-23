#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Page Erase"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Page Erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Mass Erase"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Mass Erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTPG` reader - Option byte programming"]
pub type OPTPG_R = crate::BitReader;
#[doc = "Field `OPTPG` writer - Option byte programming"]
pub type OPTPG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTER` reader - Option byte erase"]
pub type OPTER_R = crate::BitReader;
#[doc = "Field `OPTER` writer - Option byte erase"]
pub type OPTER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTWRE` reader - Option bytes write enable"]
pub type OPTWRE_R = crate::BitReader;
#[doc = "Field `OPTWRE` writer - Option bytes write enable"]
pub type OPTWRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn optwre(&self) -> OPTWRE_R {
        OPTWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    #[must_use]
    pub fn optpg(&mut self) -> OPTPG_W<CRrs> {
        OPTPG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    #[must_use]
    pub fn opter(&mut self) -> OPTER_W<CRrs> {
        OPTER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 7)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    #[must_use]
    pub fn optwre(&mut self) -> OPTWRE_W<CRrs> {
        OPTWRE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 12)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x80"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x80;
}
