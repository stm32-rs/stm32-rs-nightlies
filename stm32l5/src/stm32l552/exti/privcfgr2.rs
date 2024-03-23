#[doc = "Register `PRIVCFGR2` reader"]
pub type R = crate::R<PRIVCFGR2rs>;
#[doc = "Register `PRIVCFGR2` writer"]
pub type W = crate::W<PRIVCFGR2rs>;
#[doc = "Field `PRIV32` reader - PRIV32"]
pub type PRIV32_R = crate::BitReader;
#[doc = "Field `PRIV32` writer - PRIV32"]
pub type PRIV32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV33` reader - PRIV33"]
pub type PRIV33_R = crate::BitReader;
#[doc = "Field `PRIV33` writer - PRIV33"]
pub type PRIV33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV34` reader - PRIV34"]
pub type PRIV34_R = crate::BitReader;
#[doc = "Field `PRIV34` writer - PRIV34"]
pub type PRIV34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV35` reader - PRIV35"]
pub type PRIV35_R = crate::BitReader;
#[doc = "Field `PRIV35` writer - PRIV35"]
pub type PRIV35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV36` reader - PRIV36"]
pub type PRIV36_R = crate::BitReader;
#[doc = "Field `PRIV36` writer - PRIV36"]
pub type PRIV36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV37` reader - PRIV37"]
pub type PRIV37_R = crate::BitReader;
#[doc = "Field `PRIV37` writer - PRIV37"]
pub type PRIV37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV38` reader - PRIV38"]
pub type PRIV38_R = crate::BitReader;
#[doc = "Field `PRIV38` writer - PRIV38"]
pub type PRIV38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV39` reader - PRIV39"]
pub type PRIV39_R = crate::BitReader;
#[doc = "Field `PRIV39` writer - PRIV39"]
pub type PRIV39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV40` reader - PRIV40"]
pub type PRIV40_R = crate::BitReader;
#[doc = "Field `PRIV40` writer - PRIV40"]
pub type PRIV40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV41` reader - PRIV41"]
pub type PRIV41_R = crate::BitReader;
#[doc = "Field `PRIV41` writer - PRIV41"]
pub type PRIV41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV42` reader - PRIV42"]
pub type PRIV42_R = crate::BitReader;
#[doc = "Field `PRIV42` writer - PRIV42"]
pub type PRIV42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    #[must_use]
    pub fn priv32(&mut self) -> PRIV32_W<PRIVCFGR2rs> {
        PRIV32_W::new(self, 0)
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    #[must_use]
    pub fn priv33(&mut self) -> PRIV33_W<PRIVCFGR2rs> {
        PRIV33_W::new(self, 1)
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    #[must_use]
    pub fn priv34(&mut self) -> PRIV34_W<PRIVCFGR2rs> {
        PRIV34_W::new(self, 2)
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    #[must_use]
    pub fn priv35(&mut self) -> PRIV35_W<PRIVCFGR2rs> {
        PRIV35_W::new(self, 3)
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    #[must_use]
    pub fn priv36(&mut self) -> PRIV36_W<PRIVCFGR2rs> {
        PRIV36_W::new(self, 4)
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    #[must_use]
    pub fn priv37(&mut self) -> PRIV37_W<PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    #[must_use]
    pub fn priv38(&mut self) -> PRIV38_W<PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    #[must_use]
    pub fn priv39(&mut self) -> PRIV39_W<PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    #[must_use]
    pub fn priv40(&mut self) -> PRIV40_W<PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    #[must_use]
    pub fn priv41(&mut self) -> PRIV41_W<PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    #[must_use]
    pub fn priv42(&mut self) -> PRIV42_W<PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
}
#[doc = "EXTI security enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr2::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIVCFGR2 to value 0"]
impl crate::Resettable for PRIVCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
