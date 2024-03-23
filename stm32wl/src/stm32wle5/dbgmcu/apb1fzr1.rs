#[doc = "Register `APB1FZR1` reader"]
pub type R = crate::R<APB1FZR1rs>;
#[doc = "Register `APB1FZR1` writer"]
pub type W = crate::W<APB1FZR1rs>;
#[doc = "TIM2 stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM2_STOP {
    #[doc = "0: The counter clock of TIMx is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The counter clock of TIMx is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_TIM2_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM2_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - TIM2 stop in CPU1 debug"]
pub type DBG_TIM2_STOP_R = crate::BitReader<DBG_TIM2_STOP>;
impl DBG_TIM2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM2_STOP {
        match self.bits {
            false => DBG_TIM2_STOP::Continue,
            true => DBG_TIM2_STOP::Stop,
        }
    }
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIM2_STOP::Continue
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIM2_STOP::Stop
    }
}
#[doc = "Field `DBG_TIM2_STOP` writer - TIM2 stop in CPU1 debug"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM2_STOP>;
impl<'a, REG> DBG_TIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter clock of TIMx is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP::Continue)
    }
    #[doc = "The counter clock of TIMx is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP::Stop)
    }
}
#[doc = "RTC stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP {
    #[doc = "0: The clock of the RTC counter is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: The clock of the RTC counter is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_RTC_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in CPU1 debug"]
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP>;
impl DBG_RTC_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RTC_STOP {
        match self.bits {
            false => DBG_RTC_STOP::Continue,
            true => DBG_RTC_STOP::Stop,
        }
    }
    #[doc = "The clock of the RTC counter is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_RTC_STOP::Continue
    }
    #[doc = "The clock of the RTC counter is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_RTC_STOP::Stop
    }
}
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in CPU1 debug"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RTC_STOP>;
impl<'a, REG> DBG_RTC_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock of the RTC counter is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::Continue)
    }
    #[doc = "The clock of the RTC counter is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::Stop)
    }
}
#[doc = "WWDG stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP {
    #[doc = "0: The window watchdog counter clock continues even if the core is halted"]
    Continue = 0,
    #[doc = "1: The window watchdog counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_WWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_WWDG_STOP` reader - WWDG stop in CPU1 debug"]
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP>;
impl DBG_WWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_WWDG_STOP {
        match self.bits {
            false => DBG_WWDG_STOP::Continue,
            true => DBG_WWDG_STOP::Stop,
        }
    }
    #[doc = "The window watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_WWDG_STOP::Continue
    }
    #[doc = "The window watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_WWDG_STOP::Stop
    }
}
#[doc = "Field `DBG_WWDG_STOP` writer - WWDG stop in CPU1 debug"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_WWDG_STOP>;
impl<'a, REG> DBG_WWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The window watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::Continue)
    }
    #[doc = "The window watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::Stop)
    }
}
#[doc = "IWDG stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP {
    #[doc = "0: The independent watchdog counter clock continues even if the core is halted"]
    Continue = 0,
    #[doc = "1: The independent watchdog counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG stop in CPU1 debug"]
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP>;
impl DBG_IWDG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_IWDG_STOP {
        match self.bits {
            false => DBG_IWDG_STOP::Continue,
            true => DBG_IWDG_STOP::Stop,
        }
    }
    #[doc = "The independent watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_IWDG_STOP::Continue
    }
    #[doc = "The independent watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_IWDG_STOP::Stop
    }
}
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG stop in CPU1 debug"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_IWDG_STOP>;
impl<'a, REG> DBG_IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The independent watchdog counter clock continues even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::Continue)
    }
    #[doc = "The independent watchdog counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::Stop)
    }
}
#[doc = "I2C1 SMBUS timeout stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_STOP {
    #[doc = "0: Same behavior as in normal mode"]
    NormalMode = 0,
    #[doc = "1: I2Cx SMBUS timeout is frozen"]
    SmbusTimeoutFrozen = 1,
}
impl From<DBG_I2C1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in CPU1 debug"]
pub type DBG_I2C1_STOP_R = crate::BitReader<DBG_I2C1_STOP>;
impl DBG_I2C1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C1_STOP {
        match self.bits {
            false => DBG_I2C1_STOP::NormalMode,
            true => DBG_I2C1_STOP::SmbusTimeoutFrozen,
        }
    }
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_I2C1_STOP::NormalMode
    }
    #[doc = "I2Cx SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        *self == DBG_I2C1_STOP::SmbusTimeoutFrozen
    }
}
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in CPU1 debug"]
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C1_STOP>;
impl<'a, REG> DBG_I2C1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Same behavior as in normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP::NormalMode)
    }
    #[doc = "I2Cx SMBUS timeout is frozen"]
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP::SmbusTimeoutFrozen)
    }
}
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in CPU1 debug"]
pub use DBG_I2C1_STOP_R as DBG_I2C2_STOP_R;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stop in CPU1 debug"]
pub use DBG_I2C1_STOP_R as DBG_I2C3_STOP_R;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in CPU1 debug"]
pub use DBG_I2C1_STOP_W as DBG_I2C2_STOP_W;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stop in CPU1 debug"]
pub use DBG_I2C1_STOP_W as DBG_I2C3_STOP_W;
#[doc = "LPTIM1 stop in CPU1 debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM1_STOP {
    #[doc = "0: LPTIMx counter clock is fed even if the core is halted"]
    Continue = 0,
    #[doc = "1: LPTIMx counter clock is stopped when the core is halted"]
    Stop = 1,
}
impl From<DBG_LPTIM1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in CPU1 debug"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader<DBG_LPTIM1_STOP>;
impl DBG_LPTIM1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM1_STOP {
        match self.bits {
            false => DBG_LPTIM1_STOP::Continue,
            true => DBG_LPTIM1_STOP::Stop,
        }
    }
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM1_STOP::Continue
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM1_STOP::Stop
    }
}
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in CPU1 debug"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM1_STOP>;
impl<'a, REG> DBG_LPTIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIMx counter clock is fed even if the core is halted"]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP::Continue)
    }
    #[doc = "LPTIMx counter clock is stopped when the core is halted"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP::Stop)
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - RTC stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 stop in CPU1 debug"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<APB1FZR1rs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 10 - RTC stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB1FZR1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB1FZR1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - IWDG stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB1FZR1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<APB1FZR1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<APB1FZR1rs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<APB1FZR1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    #[doc = "Bit 31 - LPTIM1 stop in CPU1 debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<APB1FZR1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1FZR1rs;
impl crate::RegisterSpec for APB1FZR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1fzr1::R`](R) reader structure"]
impl crate::Readable for APB1FZR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1fzr1::W`](W) writer structure"]
impl crate::Writable for APB1FZR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1FZR1 to value 0"]
impl crate::Resettable for APB1FZR1rs {
    const RESET_VALUE: u32 = 0;
}
