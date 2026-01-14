///Register `APB1_FZ` reader
pub type R = crate::R<APB1_FZrs>;
///Register `APB1_FZ` writer
pub type W = crate::W<APB1_FZrs>;
/**Debug Timer 2 stopped when Core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIMER2_STOP {
    ///0: The counter clock of TIMx is fed even if the core is halted
    Continue = 0,
    ///1: The counter clock of TIMx is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_TIMER2_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER2_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted
pub type DBG_TIMER2_STOP_R = crate::BitReader<DBG_TIMER2_STOP>;
impl DBG_TIMER2_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIMER2_STOP {
        match self.bits {
            false => DBG_TIMER2_STOP::Continue,
            true => DBG_TIMER2_STOP::Stop,
        }
    }
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIMER2_STOP::Continue
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIMER2_STOP::Stop
    }
}
///Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted
pub type DBG_TIMER2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIMER2_STOP>;
impl<'a, REG> DBG_TIMER2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIMER2_STOP::Continue)
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIMER2_STOP::Stop)
    }
}
///Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIMER6_STOP_R;
///Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIMER6_STOP_W;
/**Debug RTC stopped when Core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP {
    ///0: The clock of the RTC counter is fed even if the core is halted
    Continue = 0,
    ///1: The clock of the RTC counter is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_RTC_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP>;
impl DBG_RTC_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RTC_STOP {
        match self.bits {
            false => DBG_RTC_STOP::Continue,
            true => DBG_RTC_STOP::Stop,
        }
    }
    ///The clock of the RTC counter is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_RTC_STOP::Continue
    }
    ///The clock of the RTC counter is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_RTC_STOP::Stop
    }
}
///Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RTC_STOP>;
impl<'a, REG> DBG_RTC_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The clock of the RTC counter is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::Continue)
    }
    ///The clock of the RTC counter is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::Stop)
    }
}
/**Debug Window Wachdog stopped when Core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP {
    ///0: The window watchdog counter clock continues even if the core is halted
    Continue = 0,
    ///1: The window watchdog counter clock is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_WWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP>;
impl DBG_WWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_WWDG_STOP {
        match self.bits {
            false => DBG_WWDG_STOP::Continue,
            true => DBG_WWDG_STOP::Stop,
        }
    }
    ///The window watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_WWDG_STOP::Continue
    }
    ///The window watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_WWDG_STOP::Stop
    }
}
///Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_WWDG_STOP>;
impl<'a, REG> DBG_WWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The window watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::Continue)
    }
    ///The window watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::Stop)
    }
}
/**Debug Independent Wachdog stopped when Core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP {
    ///0: The independent watchdog counter clock continues even if the core is halted
    Continue = 0,
    ///1: The independent watchdog counter clock is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP>;
impl DBG_IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_IWDG_STOP {
        match self.bits {
            false => DBG_IWDG_STOP::Continue,
            true => DBG_IWDG_STOP::Stop,
        }
    }
    ///The independent watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_IWDG_STOP::Continue
    }
    ///The independent watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_IWDG_STOP::Stop
    }
}
///Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_IWDG_STOP>;
impl<'a, REG> DBG_IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The independent watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::Continue)
    }
    ///The independent watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::Stop)
    }
}
/**I2C1 SMBUS timeout mode stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_STOP {
    ///0: Same behavior as in normal mode
    NormalMode = 0,
    ///1: I2C3 SMBUS timeout is frozen
    SmbusTimeoutFrozen = 1,
}
impl From<DBG_I2C1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_STOP_R = crate::BitReader<DBG_I2C1_STOP>;
impl DBG_I2C1_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C1_STOP {
        match self.bits {
            false => DBG_I2C1_STOP::NormalMode,
            true => DBG_I2C1_STOP::SmbusTimeoutFrozen,
        }
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_I2C1_STOP::NormalMode
    }
    ///I2C3 SMBUS timeout is frozen
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        *self == DBG_I2C1_STOP::SmbusTimeoutFrozen
    }
}
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C1_STOP>;
impl<'a, REG> DBG_I2C1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP::NormalMode)
    }
    ///I2C3 SMBUS timeout is frozen
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP::SmbusTimeoutFrozen)
    }
}
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted
pub use DBG_I2C1_STOP_R as DBG_I2C2_STOP_R;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted
pub use DBG_I2C1_STOP_W as DBG_I2C2_STOP_W;
/**LPTIM1 counter stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIMER_STOP {
    ///0: LPTIM1 counter clock is fed even if the core is halted
    Continue = 0,
    ///1: LPTIM1 counter clock is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_LPTIMER_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIMER_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIMER_STOP` reader - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIMER_STOP_R = crate::BitReader<DBG_LPTIMER_STOP>;
impl DBG_LPTIMER_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIMER_STOP {
        match self.bits {
            false => DBG_LPTIMER_STOP::Continue,
            true => DBG_LPTIMER_STOP::Stop,
        }
    }
    ///LPTIM1 counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIMER_STOP::Continue
    }
    ///LPTIM1 counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIMER_STOP::Stop
    }
}
///Field `DBG_LPTIMER_STOP` writer - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIMER_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIMER_STOP>;
impl<'a, REG> DBG_LPTIMER_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM1 counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIMER_STOP::Continue)
    }
    ///LPTIM1 counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIMER_STOP::Stop)
    }
}
impl R {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DBG_LPTIMER_STOP_R {
        DBG_LPTIMER_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1_FZ")
            .field("dbg_timer2_stop", &self.dbg_timer2_stop())
            .field("dbg_timer6_stop", &self.dbg_timer6_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .field("dbg_lptimer_stop", &self.dbg_lptimer_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<'_, APB1_FZrs> {
        DBG_TIMER2_STOP_W::new(self, 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<'_, APB1_FZrs> {
        DBG_TIMER6_STOP_W::new(self, 4)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB1_FZrs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB1_FZrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB1_FZrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, APB1_FZrs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<'_, APB1_FZrs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DBG_LPTIMER_STOP_W<'_, APB1_FZrs> {
        DBG_LPTIMER_STOP_W::new(self, 31)
    }
}
/**APB Low Freeze Register

You can [`read`](crate::Reg::read) this register and get [`apb1_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#DBG:APB1_FZ)*/
pub struct APB1_FZrs;
impl crate::RegisterSpec for APB1_FZrs {
    type Ux = u32;
}
///`read()` method returns [`apb1_fz::R`](R) reader structure
impl crate::Readable for APB1_FZrs {}
///`write(|w| ..)` method takes [`apb1_fz::W`](W) writer structure
impl crate::Writable for APB1_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1_FZ to value 0
impl crate::Resettable for APB1_FZrs {}
