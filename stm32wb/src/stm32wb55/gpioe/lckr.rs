///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
/**Port x lock pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCK0 {
    ///0: Port configuration not locked
    Unlocked = 0,
    ///1: Port configuration locked
    Locked = 1,
}
impl From<LCK0> for bool {
    #[inline(always)]
    fn from(variant: LCK0) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK(0-4)` reader - Port x lock pin %s
pub type LCK_R = crate::BitReader<LCK0>;
impl LCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCK0 {
        match self.bits {
            false => LCK0::Unlocked,
            true => LCK0::Locked,
        }
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LCK0::Unlocked
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK0::Locked
    }
}
///Field `LCK(0-4)` writer - Port x lock pin %s
pub type LCK_W<'a, REG> = crate::BitWriter<'a, REG, LCK0>;
impl<'a, REG> LCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::Unlocked)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LCK0::Locked)
    }
}
/**Port x lock bit y (y= 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCKK {
    ///0: Port configuration lock key not active
    NotActive = 0,
    ///1: Port configuration lock key active
    Active = 1,
}
impl From<LCKK> for bool {
    #[inline(always)]
    fn from(variant: LCKK) -> Self {
        variant as u8 != 0
    }
}
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub type LCKK_R = crate::BitReader<LCKK>;
impl LCKK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LCKK {
        match self.bits {
            false => LCKK::NotActive,
            true => LCKK::Active,
        }
    }
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == LCKK::NotActive
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == LCKK::Active
    }
}
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub type LCKK_W<'a, REG> = crate::BitWriter<'a, REG, LCKK>;
impl<'a, REG> LCKK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::NotActive)
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(LCKK::Active)
    }
}
impl R {
    ///Port x lock pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LCK0` field.</div>
    #[inline(always)]
    pub fn lck(&self, n: u8) -> LCK_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        LCK_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Port x lock pin (0-4)
    #[inline(always)]
    pub fn lck_iter(&self) -> impl Iterator<Item = LCK_R> + '_ {
        (0..5).map(move |n| LCK_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Port x lock pin 0
    #[inline(always)]
    pub fn lck0(&self) -> LCK_R {
        LCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock pin 1
    #[inline(always)]
    pub fn lck1(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock pin 2
    #[inline(always)]
    pub fn lck2(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock pin 3
    #[inline(always)]
    pub fn lck3(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock pin 4
    #[inline(always)]
    pub fn lck4(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 4) & 1) != 0)
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
            .field("lck1", &self.lck1())
            .field("lck2", &self.lck2())
            .field("lck3", &self.lck3())
            .field("lck4", &self.lck4())
            .finish()
    }
}
impl W {
    ///Port x lock pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `LCK0` field.</div>
    #[inline(always)]
    pub fn lck(&mut self, n: u8) -> LCK_W<LCKRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        LCK_W::new(self, n)
    }
    ///Bit 0 - Port x lock pin 0
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 0)
    }
    ///Bit 1 - Port x lock pin 1
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 1)
    }
    ///Bit 2 - Port x lock pin 2
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 2)
    }
    ///Bit 3 - Port x lock pin 3
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 3)
    }
    ///Bit 4 - Port x lock pin 4
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK_W<LCKRrs> {
        LCK_W::new(self, 4)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:LCKR)*/
pub struct LCKRrs;
impl crate::RegisterSpec for LCKRrs {
    type Ux = u32;
}
///`read()` method returns [`lckr::R`](R) reader structure
impl crate::Readable for LCKRrs {}
///`write(|w| ..)` method takes [`lckr::W`](W) writer structure
impl crate::Writable for LCKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKRrs {
    const RESET_VALUE: u32 = 0;
}
