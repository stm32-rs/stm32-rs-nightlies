///Register `APB1FZR1` reader
pub type R = crate::R<APB1FZR1rs>;
///Register `APB1FZR1` writer
pub type W = crate::W<APB1FZR1rs>;
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
///Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIM3_STOP_R;
///Field `DBG_TIM4_STOP` reader - TIM4 counter stopped when core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIM4_STOP_R;
///Field `DBG_TIM5_STOP` reader - TIM5 counter stopped when core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIM5_STOP_R;
///Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIMER6_STOP_R;
///Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted
pub use DBG_TIMER2_STOP_R as DBG_TIM7_STOP_R;
///Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIM3_STOP_W;
///Field `DBG_TIM4_STOP` writer - TIM4 counter stopped when core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIM4_STOP_W;
///Field `DBG_TIM5_STOP` writer - TIM5 counter stopped when core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIM5_STOP_W;
///Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIMER6_STOP_W;
///Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted
pub use DBG_TIMER2_STOP_W as DBG_TIM7_STOP_W;
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
    ///1: I2Cx SMBUS timeout is frozen
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
    ///I2Cx SMBUS timeout is frozen
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
    ///I2Cx SMBUS timeout is frozen
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_STOP::SmbusTimeoutFrozen)
    }
}
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted
pub use DBG_I2C1_STOP_R as DBG_I2C2_STOP_R;
///Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout counter stopped when core is halted
pub use DBG_I2C1_STOP_R as DBG_I2C3_STOP_R;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted
pub use DBG_I2C1_STOP_W as DBG_I2C2_STOP_W;
///Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout counter stopped when core is halted
pub use DBG_I2C1_STOP_W as DBG_I2C3_STOP_W;
/**bxCAN stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_CAN1_STOP {
    ///0: Same behavior as in normal mode
    NormalMode = 0,
    ///1: The bxCAN1 receive registers are frozen
    ReceiveRegistersFrozen = 1,
}
impl From<DBG_CAN1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_CAN1_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_CAN1_STOP` reader - bxCAN stopped when core is halted
pub type DBG_CAN1_STOP_R = crate::BitReader<DBG_CAN1_STOP>;
impl DBG_CAN1_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_CAN1_STOP {
        match self.bits {
            false => DBG_CAN1_STOP::NormalMode,
            true => DBG_CAN1_STOP::ReceiveRegistersFrozen,
        }
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_CAN1_STOP::NormalMode
    }
    ///The bxCAN1 receive registers are frozen
    #[inline(always)]
    pub fn is_receive_registers_frozen(&self) -> bool {
        *self == DBG_CAN1_STOP::ReceiveRegistersFrozen
    }
}
///Field `DBG_CAN1_STOP` writer - bxCAN stopped when core is halted
pub type DBG_CAN1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_CAN1_STOP>;
impl<'a, REG> DBG_CAN1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_CAN1_STOP::NormalMode)
    }
    ///The bxCAN1 receive registers are frozen
    #[inline(always)]
    pub fn receive_registers_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_CAN1_STOP::ReceiveRegistersFrozen)
    }
}
/**LPTIM1 counter stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM1_STOP {
    ///0: LPTIMx counter clock is fed even if the core is halted
    Continue = 0,
    ///1: LPTIMx counter clock is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_LPTIM1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIM1_STOP_R = crate::BitReader<DBG_LPTIM1_STOP>;
impl DBG_LPTIM1_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM1_STOP {
        match self.bits {
            false => DBG_LPTIM1_STOP::Continue,
            true => DBG_LPTIM1_STOP::Stop,
        }
    }
    ///LPTIMx counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM1_STOP::Continue
    }
    ///LPTIMx counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM1_STOP::Stop
    }
}
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM1_STOP>;
impl<'a, REG> DBG_LPTIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIMx counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP::Continue)
    }
    ///LPTIMx counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM1_STOP::Stop)
    }
}
impl R {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
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
    ///Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - bxCAN stopped when core is halted
    #[inline(always)]
    pub fn dbg_can1_stop(&self) -> DBG_CAN1_STOP_R {
        DBG_CAN1_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1FZR1")
            .field("dbg_timer2_stop", &self.dbg_timer2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_timer6_stop", &self.dbg_timer6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_can1_stop", &self.dbg_can1_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<'_, APB1FZR1rs> {
        DBG_TIMER2_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB1FZR1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 2 - TIM4 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<'_, APB1FZR1rs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    ///Bit 3 - TIM5 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<'_, APB1FZR1rs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<'_, APB1FZR1rs> {
        DBG_TIMER6_STOP_W::new(self, 4)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<'_, APB1FZR1rs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB1FZR1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB1FZR1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB1FZR1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, APB1FZR1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<'_, APB1FZR1rs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    ///Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<'_, APB1FZR1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    ///Bit 25 - bxCAN stopped when core is halted
    #[inline(always)]
    pub fn dbg_can1_stop(&mut self) -> DBG_CAN1_STOP_W<'_, APB1FZR1rs> {
        DBG_CAN1_STOP_W::new(self, 25)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<'_, APB1FZR1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
/**APB Low Freeze Register 1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#DBGMCU:APB1FZR1)*/
pub struct APB1FZR1rs;
impl crate::RegisterSpec for APB1FZR1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1fzr1::R`](R) reader structure
impl crate::Readable for APB1FZR1rs {}
///`write(|w| ..)` method takes [`apb1fzr1::W`](W) writer structure
impl crate::Writable for APB1FZR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1FZR1 to value 0
impl crate::Resettable for APB1FZR1rs {}
