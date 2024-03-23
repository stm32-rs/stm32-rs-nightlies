#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKRrs>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKRrs>;
#[doc = "Port x lock bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK0 {
    #[doc = "0: Port configuration not locked"]
    Unlocked = 0,
    #[doc = "1: Port configuration locked"]
    Locked = 1,
}
impl From<LCK0> for bool {
    #[inline(always)]
    fn from(variant: LCK0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK0` reader - Port x lock bit y (y= 0..15)"]
pub type LCK0_R = crate::BitReader<LCK0>;
impl LCK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK0 {
        match self.bits {
            false => LCK0::Unlocked,
            true => LCK0::Locked,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LCK0::Unlocked
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK0::Locked
    }
}
#[doc = "Field `LCK0` writer - Port x lock bit y (y= 0..15)"]
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG, LCK0>;
impl<'a, REG> LCK0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::Unlocked)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::Locked)
    }
}
#[doc = "Field `LCK1` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK1_R;
#[doc = "Field `LCK2` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK2_R;
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK3_R;
#[doc = "Field `LCK4` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK4_R;
#[doc = "Field `LCK5` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK5_R;
#[doc = "Field `LCK6` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK6_R;
#[doc = "Field `LCK13` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK13_R;
#[doc = "Field `LCK14` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK14_R;
#[doc = "Field `LCK15` reader - Port x lock bit y (y= 0..15)"]
pub use LCK0_R as LCK15_R;
#[doc = "Field `LCK1` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK1_W;
#[doc = "Field `LCK2` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK2_W;
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK3_W;
#[doc = "Field `LCK4` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK4_W;
#[doc = "Field `LCK5` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK5_W;
#[doc = "Field `LCK6` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK6_W;
#[doc = "Field `LCK13` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK13_W;
#[doc = "Field `LCK14` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK14_W;
#[doc = "Field `LCK15` writer - Port x lock bit y (y= 0..15)"]
pub use LCK0_W as LCK15_W;
#[doc = "Port x lock bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK {
    #[doc = "0: Port configuration lock key not active"]
    NotActive = 0,
    #[doc = "1: Port configuration lock key active"]
    Active = 1,
}
impl From<LCKK> for bool {
    #[inline(always)]
    fn from(variant: LCKK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKK` reader - Port x lock bit y (y= 0..15)"]
pub type LCKK_R = crate::BitReader<LCKK>;
impl LCKK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCKK {
        match self.bits {
            false => LCKK::NotActive,
            true => LCKK::Active,
        }
    }
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LCKK::NotActive
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LCKK::Active
    }
}
#[doc = "Field `LCKK` writer - Port x lock bit y (y= 0..15)"]
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LCKK>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration lock key not active"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::NotActive)
    }
    #[doc = "Port configuration lock key active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::Active)
    }
}
impl R {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<LCKRrs> {
        LCK0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<LCKRrs> {
        LCK1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<LCKRrs> {
        LCK2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<LCKRrs> {
        LCK3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<LCKRrs> {
        LCK4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK5_W<LCKRrs> {
        LCK5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK6_W<LCKRrs> {
        LCK6_W::new(self, 6)
    }
    #[doc = "Bit 13 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck13(&mut self) -> LCK13_W<LCKRrs> {
        LCK13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck14(&mut self) -> LCK14_W<LCKRrs> {
        LCK14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck15(&mut self) -> LCK15_W<LCKRrs> {
        LCK15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
#[doc = "GPIO port configuration lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LCKRrs {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKRrs {
    const RESET_VALUE: u32 = 0;
}
