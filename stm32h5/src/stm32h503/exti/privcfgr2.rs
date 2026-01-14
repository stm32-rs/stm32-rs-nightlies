///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
/**Privilege enable on event input x (x = 42 to 37)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENT_PRIVILEGE {
    ///0: Event privilege disabled
    Unprivileged = 0,
    ///1: Event privilege enabled
    Privileged = 1,
}
impl From<EVENT_PRIVILEGE> for bool {
    #[inline(always)]
    fn from(variant: EVENT_PRIVILEGE) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV37` reader - Privilege enable on event input x (x = 42 to 37)
pub type PRIV37_R = crate::BitReader<EVENT_PRIVILEGE>;
impl PRIV37_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EVENT_PRIVILEGE {
        match self.bits {
            false => EVENT_PRIVILEGE::Unprivileged,
            true => EVENT_PRIVILEGE::Privileged,
        }
    }
    ///Event privilege disabled
    #[inline(always)]
    pub fn is_unprivileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Unprivileged
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn is_privileged(&self) -> bool {
        *self == EVENT_PRIVILEGE::Privileged
    }
}
///Field `PRIV37` writer - Privilege enable on event input x (x = 42 to 37)
pub type PRIV37_W<'a, REG> = crate::BitWriter<'a, REG, EVENT_PRIVILEGE>;
impl<'a, REG> PRIV37_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event privilege disabled
    #[inline(always)]
    pub fn unprivileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Unprivileged)
    }
    ///Event privilege enabled
    #[inline(always)]
    pub fn privileged(self) -> &'a mut crate::W<REG> {
        self.variant(EVENT_PRIVILEGE::Privileged)
    }
}
///Field `PRIV38` reader - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_R as PRIV38_R;
///Field `PRIV39` reader - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_R as PRIV39_R;
///Field `PRIV40` reader - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_R as PRIV40_R;
///Field `PRIV41` reader - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_R as PRIV41_R;
///Field `PRIV42` reader - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_R as PRIV42_R;
///Field `PRIV47` reader - Privilege enable on event input x
pub use PRIV37_R as PRIV47_R;
///Field `PRIV49` reader - Privilege enable on event input x (x = 50 to 49)
pub use PRIV37_R as PRIV49_R;
///Field `PRIV50` reader - Privilege enable on event input x (x = 50 to 49)
pub use PRIV37_R as PRIV50_R;
///Field `PRIV53` reader - Privilege enable on event input x
pub use PRIV37_R as PRIV53_R;
///Field `PRIV38` writer - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_W as PRIV38_W;
///Field `PRIV39` writer - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_W as PRIV39_W;
///Field `PRIV40` writer - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_W as PRIV40_W;
///Field `PRIV41` writer - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_W as PRIV41_W;
///Field `PRIV42` writer - Privilege enable on event input x (x = 42 to 37)
pub use PRIV37_W as PRIV42_W;
///Field `PRIV47` writer - Privilege enable on event input x
pub use PRIV37_W as PRIV47_W;
///Field `PRIV49` writer - Privilege enable on event input x (x = 50 to 49)
pub use PRIV37_W as PRIV49_W;
///Field `PRIV50` writer - Privilege enable on event input x (x = 50 to 49)
pub use PRIV37_W as PRIV50_W;
///Field `PRIV53` writer - Privilege enable on event input x
pub use PRIV37_W as PRIV53_W;
impl R {
    ///Bit 5 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv47(&self) -> PRIV47_R {
        PRIV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Privilege enable on event input x (x = 50 to 49)
    #[inline(always)]
    pub fn priv49(&self) -> PRIV49_R {
        PRIV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Privilege enable on event input x (x = 50 to 49)
    #[inline(always)]
    pub fn priv50(&self) -> PRIV50_R {
        PRIV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv53(&self) -> PRIV53_R {
        PRIV53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("priv37", &self.priv37())
            .field("priv38", &self.priv38())
            .field("priv39", &self.priv39())
            .field("priv40", &self.priv40())
            .field("priv41", &self.priv41())
            .field("priv42", &self.priv42())
            .field("priv47", &self.priv47())
            .field("priv49", &self.priv49())
            .field("priv50", &self.priv50())
            .field("priv53", &self.priv53())
            .finish()
    }
}
impl W {
    ///Bit 5 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv37(&mut self) -> PRIV37_W<'_, PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    ///Bit 6 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv38(&mut self) -> PRIV38_W<'_, PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    ///Bit 7 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv39(&mut self) -> PRIV39_W<'_, PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    ///Bit 8 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv40(&mut self) -> PRIV40_W<'_, PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    ///Bit 9 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv41(&mut self) -> PRIV41_W<'_, PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    ///Bit 10 - Privilege enable on event input x (x = 42 to 37)
    #[inline(always)]
    pub fn priv42(&mut self) -> PRIV42_W<'_, PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
    ///Bit 15 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv47(&mut self) -> PRIV47_W<'_, PRIVCFGR2rs> {
        PRIV47_W::new(self, 15)
    }
    ///Bit 17 - Privilege enable on event input x (x = 50 to 49)
    #[inline(always)]
    pub fn priv49(&mut self) -> PRIV49_W<'_, PRIVCFGR2rs> {
        PRIV49_W::new(self, 17)
    }
    ///Bit 18 - Privilege enable on event input x (x = 50 to 49)
    #[inline(always)]
    pub fn priv50(&mut self) -> PRIV50_W<'_, PRIVCFGR2rs> {
        PRIV50_W::new(self, 18)
    }
    ///Bit 21 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv53(&mut self) -> PRIV53_W<'_, PRIVCFGR2rs> {
        PRIV53_W::new(self, 21)
    }
}
/**EXTI privilege configuration register 2

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#EXTI:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
