#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LCKRrs>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LCKRrs>;
#[doc = "Port x lock bit y (y= 0..15)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK3 {
    #[doc = "0: Port configuration not locked"]
    Unlocked = 0,
    #[doc = "1: Port configuration locked"]
    Locked = 1,
}
impl From<LCK3> for bool {
    #[inline(always)]
    fn from(variant: LCK3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCK3` reader - Port x lock bit y (y= 0..15)"]
pub type LCK3_R = crate::BitReader<LCK3>;
impl LCK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LCK3 {
        match self.bits {
            false => LCK3::Unlocked,
            true => LCK3::Locked,
        }
    }
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LCK3::Unlocked
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK3::Locked
    }
}
#[doc = "Field `LCK3` writer - Port x lock bit y (y= 0..15)"]
pub type LCK3_W<'a, REG> = crate::BitWriter<'a, REG, LCK3>;
impl<'a, REG> LCK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port configuration not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3::Unlocked)
    }
    #[doc = "Port configuration locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK3::Locked)
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
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Port x lock bit y (y= 0..15)"]
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<LCKRrs> {
        LCK3_W::new(self, 3)
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
