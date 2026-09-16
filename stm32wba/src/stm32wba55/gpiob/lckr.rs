///Register `LCKR` reader
pub type R = crate::R<LCKRrs>;
///Register `LCKR` writer
pub type W = crate::W<LCKRrs>;
/**Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.

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
///Field `LCK0` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
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
///Field `LCK0` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
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
///Field `LCK1` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK1_R;
///Field `LCK2` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK2_R;
///Field `LCK3` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK3_R;
///Field `LCK4` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK4_R;
///Field `LCK5` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK5_R;
///Field `LCK6` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK6_R;
///Field `LCK7` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK7_R;
///Field `LCK8` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK8_R;
///Field `LCK9` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK9_R;
///Field `LCK10` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK10_R;
///Field `LCK11` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK11_R;
///Field `LCK12` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK12_R;
///Field `LCK13` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK13_R;
///Field `LCK14` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK14_R;
///Field `LCK15` reader - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_R as LCK15_R;
///Field `LCK1` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK1_W;
///Field `LCK2` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK2_W;
///Field `LCK3` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK3_W;
///Field `LCK4` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK4_W;
///Field `LCK5` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK5_W;
///Field `LCK6` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK6_W;
///Field `LCK7` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK7_W;
///Field `LCK8` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK8_W;
///Field `LCK9` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK9_W;
///Field `LCK10` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK10_W;
///Field `LCK11` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK11_W;
///Field `LCK12` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK12_W;
///Field `LCK13` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK13_W;
///Field `LCK14` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK14_W;
///Field `LCK15` writer - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
pub use LCK0_W as LCK15_W;
/**Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access can be protected by any GPIOB SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCKR\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.

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
///Field `LCKK` reader - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access can be protected by any GPIOB SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCKR\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
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
///Field `LCKK` writer - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access can be protected by any GPIOB SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCKR\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
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
    ///Bit 0 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access can be protected by any GPIOB SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCKR\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("lck0", &self.lck0())
            .field("lck1", &self.lck1())
            .field("lck2", &self.lck2())
            .field("lck3", &self.lck3())
            .field("lck4", &self.lck4())
            .field("lck5", &self.lck5())
            .field("lck6", &self.lck6())
            .field("lck7", &self.lck7())
            .field("lck8", &self.lck8())
            .field("lck9", &self.lck9())
            .field("lck10", &self.lck10())
            .field("lck11", &self.lck11())
            .field("lck12", &self.lck12())
            .field("lck13", &self.lck13())
            .field("lck14", &self.lck14())
            .field("lck15", &self.lck15())
            .field("lckk", &self.lckk())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W<'_, LCKRrs> {
        LCK0_W::new(self, 0)
    }
    ///Bit 1 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W<'_, LCKRrs> {
        LCK1_W::new(self, 1)
    }
    ///Bit 2 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W<'_, LCKRrs> {
        LCK2_W::new(self, 2)
    }
    ///Bit 3 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W<'_, LCKRrs> {
        LCK3_W::new(self, 3)
    }
    ///Bit 4 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W<'_, LCKRrs> {
        LCK4_W::new(self, 4)
    }
    ///Bit 5 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W<'_, LCKRrs> {
        LCK5_W::new(self, 5)
    }
    ///Bit 6 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W<'_, LCKRrs> {
        LCK6_W::new(self, 6)
    }
    ///Bit 7 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W<'_, LCKRrs> {
        LCK7_W::new(self, 7)
    }
    ///Bit 8 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W<'_, LCKRrs> {
        LCK8_W::new(self, 8)
    }
    ///Bit 9 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W<'_, LCKRrs> {
        LCK9_W::new(self, 9)
    }
    ///Bit 10 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W<'_, LCKRrs> {
        LCK10_W::new(self, 10)
    }
    ///Bit 11 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W<'_, LCKRrs> {
        LCK11_W::new(self, 11)
    }
    ///Bit 12 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W<'_, LCKRrs> {
        LCK12_W::new(self, 12)
    }
    ///Bit 13 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W<'_, LCKRrs> {
        LCK13_W::new(self, 13)
    }
    ///Bit 14 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W<'_, LCKRrs> {
        LCK14_W::new(self, 14)
    }
    ///Bit 15 - Port lock I/O pin y These bits are read/write but can only be written when the LCKK bit is 0 Access can be protected by GPIOB SECy. Note that bit 10 is reserved on STM32WBA55xx devices.
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W<'_, LCKRrs> {
        LCK15_W::new(self, 15)
    }
    ///Bit 16 - Lock key This bit can be read any time. It can only be modified using the lock key write sequence. Access can be protected by any GPIOB SECy. - LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] - LOCK key read RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCKR\[15:0\] must not change. Note: Any error in the lock sequence aborts the LOCK. Note: After the first LOCK sequence on any bit of the port, any read access on the LCKK bit returns 1 until the next MCU reset or peripheral reset.
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W<'_, LCKRrs> {
        LCKK_W::new(self, 16)
    }
}
/**GPIO port B configuration lock register

You can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOB:LCKR)*/
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
