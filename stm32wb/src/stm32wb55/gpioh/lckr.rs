///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
/**Port x lock bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: Port configuration not locked
    Unlocked = 0,
    ///1: Port configuration locked
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK0` reader - Port x lock bit y (y= 0..15)
pub type LCK0_R = crate::BitReader<LOCK>;
impl LCK0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::Unlocked,
            true => LOCK::Locked,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK::Unlocked
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK::Locked
    }
}
///Field `LCK0` writer - Port x lock bit y (y= 0..15)
pub type LCK0_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LCK0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
///Field `LCK1` reader - Port x lock bit y (y= 0..15)
pub use LCK0_R as LCK1_R;
///Field `LCK3` reader - Port x lock bit y (y= 0..15)
pub use LCK0_R as LCK3_R;
///Field `LCK1` writer - Port x lock bit y (y= 0..15)
pub use LCK0_W as LCK1_W;
///Field `LCK3` writer - Port x lock bit y (y= 0..15)
pub use LCK0_W as LCK3_W;
/**Port x lock bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_KEY {
    ///0: Port configuration lock key not active
    NotActive = 0,
    ///1: Port configuration lock key active
    Active = 1,
}
impl From<LOCK_KEY> for bool {
    #[inline(always)]
    fn from(variant: LOCK_KEY) -> Self {
        variant as u8 != 0
    }
}
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub type LCKK_R = crate::BitReader<LOCK_KEY>;
impl LCKK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_KEY {
        match self.bits {
            false => LOCK_KEY::NotActive,
            true => LOCK_KEY::Active,
        }
    }
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LOCK_KEY::NotActive
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LOCK_KEY::Active
    }
}
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_KEY>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_KEY::NotActive)
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_KEY::Active)
    }
}
impl R {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("lckk", &self.lckk())
            .field("lck0", &self.lck0())
            .field("lck3", &self.lck3())
            .field("lck1", &self.lck1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W<'_, LCKRrs> {
        LCK0_W::new(self, 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W<'_, LCKRrs> {
        LCK1_W::new(self, 1)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W<'_, LCKRrs> {
        LCK3_W::new(self, 3)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<'_, LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH:LCKR)*/
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`lckr::R`](R) reader structure
impl crate::Readable for LCKRrs {}
///`write(|w| ..)` method takes [`lckr::W`](W) writer structure
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKRrs {}
