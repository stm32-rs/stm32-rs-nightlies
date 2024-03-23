#[doc = "Register `PRIVCFGR2` reader"]
pub type R = crate::R<PRIVCFGR2rs>;
#[doc = "Register `PRIVCFGR2` writer"]
pub type W = crate::W<PRIVCFGR2rs>;
#[doc = "Privilege enable on event input x (x = 42 to 37)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV37 {
    #[doc = "0: Event privilege disabled"]
    Unprivileged = 0,
    #[doc = "1: Event privilege enabled"]
    Privileged = 1,
}
impl From<PRIV37> for bool {
    #[inline(always)]
    fn from(variant: PRIV37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV37` reader - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_R = crate::BitReader<PRIV37>;
impl PRIV37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV37 {
        match self.bits {
            false => PRIV37::Unprivileged,
            true => PRIV37::Privileged,
        }
    }
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == PRIV37::Unprivileged
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == PRIV37::Privileged
    }
}
#[doc = "Field `PRIV37` writer - Privilege enable on event input x (x = 42 to 37)"]
pub type PRIV37_W<'a, REG> = crate::BitWriter<'a, REG, PRIV37>;
impl<'a, REG> PRIV37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event privilege disabled"]
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV37::Unprivileged)
    }
    #[doc = "Event privilege enabled"]
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV37::Privileged)
    }
}
#[doc = "Field `PRIV38` reader - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_R as PRIV38_R;
#[doc = "Field `PRIV39` reader - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_R as PRIV39_R;
#[doc = "Field `PRIV40` reader - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_R as PRIV40_R;
#[doc = "Field `PRIV41` reader - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_R as PRIV41_R;
#[doc = "Field `PRIV42` reader - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_R as PRIV42_R;
#[doc = "Field `PRIV47` reader - Privilege enable on event input x"]
pub use PRIV37_R as PRIV47_R;
#[doc = "Field `PRIV49` reader - Privilege enable on event input x (x = 50 to 49)"]
pub use PRIV37_R as PRIV49_R;
#[doc = "Field `PRIV50` reader - Privilege enable on event input x (x = 50 to 49)"]
pub use PRIV37_R as PRIV50_R;
#[doc = "Field `PRIV53` reader - Privilege enable on event input x"]
pub use PRIV37_R as PRIV53_R;
#[doc = "Field `PRIV38` writer - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_W as PRIV38_W;
#[doc = "Field `PRIV39` writer - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_W as PRIV39_W;
#[doc = "Field `PRIV40` writer - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_W as PRIV40_W;
#[doc = "Field `PRIV41` writer - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_W as PRIV41_W;
#[doc = "Field `PRIV42` writer - Privilege enable on event input x (x = 42 to 37)"]
pub use PRIV37_W as PRIV42_W;
#[doc = "Field `PRIV47` writer - Privilege enable on event input x"]
pub use PRIV37_W as PRIV47_W;
#[doc = "Field `PRIV49` writer - Privilege enable on event input x (x = 50 to 49)"]
pub use PRIV37_W as PRIV49_W;
#[doc = "Field `PRIV50` writer - Privilege enable on event input x (x = 50 to 49)"]
pub use PRIV37_W as PRIV50_W;
#[doc = "Field `PRIV53` writer - Privilege enable on event input x"]
pub use PRIV37_W as PRIV53_W;
impl R {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv37(&mut self) -> PRIV37_W<PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv38(&mut self) -> PRIV38_W<PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv39(&mut self) -> PRIV39_W<PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv40(&mut self) -> PRIV40_W<PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv41(&mut self) -> PRIV41_W<PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Privilege enable on event input x (x = 42 to 37)"]
    #[inline(always)]
    #[must_use]
    pub fn priv42(&mut self) -> PRIV42_W<PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
    #[doc = "Bit 15 - Privilege enable on event input x"]
    #[inline(always)]
    #[must_use]
    pub fn priv47(&mut self) -> PRIV47_W<PRIVCFGR2rs> {
        PRIV47_W::new(self, 15)
    }
    #[doc = "Bit 17 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    #[must_use]
    pub fn priv49(&mut self) -> PRIV49_W<PRIVCFGR2rs> {
        PRIV49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Privilege enable on event input x (x = 50 to 49)"]
    #[inline(always)]
    #[must_use]
    pub fn priv50(&mut self) -> PRIV50_W<PRIVCFGR2rs> {
        PRIV50_W::new(self, 18)
    }
    #[doc = "Bit 21 - Privilege enable on event input x"]
    #[inline(always)]
    #[must_use]
    pub fn priv53(&mut self) -> PRIV53_W<PRIVCFGR2rs> {
        PRIV53_W::new(self, 21)
    }
}
#[doc = "EXTI privilege configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`privcfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
