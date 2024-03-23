#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKRrs>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKRrs>;
#[doc = "Port x lock pin %s\n\nValue on reset: 0"]
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
#[doc = "Field `LCK(0-15)` reader - Port x lock pin %s"]
pub type LCK_R = crate::BitReader<LCK0>;
impl LCK_R {
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
#[doc = "Field `LCK(0-15)` writer - Port x lock pin %s"]
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG, LCK0>;
impl<'a, REG> LCK_W<'a, REG>
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
    #[doc = "Port x lock pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LCK0` field"]
    #[inline(always)]
    pub fn lck(&self, n: u8) -> LCK_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LCK_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port x lock pin (0-15)"]
    #[inline(always)]
    pub fn lck_iter(&self) -> impl Iterator<Item = LCK_R> + '_ {
        (0..16).map(move |n| LCK_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Port x lock pin 0"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x lock pin 1"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x lock pin 2"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x lock pin 3"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x lock pin 4"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x lock pin 5"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x lock pin 6"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x lock pin 7"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x lock pin 8"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x lock pin 9"]
    #[inline(always)]
    pub fn lck9(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x lock pin 10"]
    #[inline(always)]
    pub fn lck10(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x lock pin 11"]
    #[inline(always)]
    pub fn lck11(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x lock pin 12"]
    #[inline(always)]
    pub fn lck12(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x lock pin 13"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x lock pin 14"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x lock pin 15"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Port x lock pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `LCK0` field"]
    #[inline(always)]
    #[must_use]
    pub fn lck(&mut self, n: u8) -> LCK_W<LCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        LCK_W::new(self, n)
    }
    #[doc = "Bit 0 - Port x lock pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x lock pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x lock pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x lock pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x lock pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x lock pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn lck5(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x lock pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn lck6(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x lock pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn lck7(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x lock pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn lck8(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x lock pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn lck9(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x lock pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn lck10(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x lock pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn lck11(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x lock pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn lck12(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x lock pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn lck13(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x lock pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn lck14(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x lock pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn lck15(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 15)
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
