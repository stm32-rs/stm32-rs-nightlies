///Register `APB_FZ1` reader
pub type R = crate::R<APB_FZ1rs>;
///Register `APB_FZ1` writer
pub type W = crate::W<APB_FZ1rs>;
/**Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM2_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM2_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM2_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM2_STOP` reader - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
pub type DBG_TIM2_STOP_R = crate::BitReader<DBG_TIM2_STOP>;
impl DBG_TIM2_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM2_STOP {
        match self.bits {
            false => DBG_TIM2_STOP::B0x0,
            true => DBG_TIM2_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM2_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM2_STOP::B0x1
    }
}
///Field `DBG_TIM2_STOP` writer - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM2_STOP>;
impl<'a, REG> DBG_TIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM2_STOP::B0x1)
    }
}
/**Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM3_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_TIM3_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM3_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub type DBG_TIM3_STOP_R = crate::BitReader<DBG_TIM3_STOP>;
impl DBG_TIM3_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM3_STOP {
        match self.bits {
            false => DBG_TIM3_STOP::B0x0,
            true => DBG_TIM3_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_TIM3_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_TIM3_STOP::B0x1
    }
}
///Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM3_STOP>;
impl<'a, REG> DBG_TIM3_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM3_STOP::B0x1)
    }
}
/**Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RTC_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_RTC_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub type DBG_RTC_STOP_R = crate::BitReader<DBG_RTC_STOP>;
impl DBG_RTC_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RTC_STOP {
        match self.bits {
            false => DBG_RTC_STOP::B0x0,
            true => DBG_RTC_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_RTC_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_RTC_STOP::B0x1
    }
}
///Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RTC_STOP>;
impl<'a, REG> DBG_RTC_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RTC_STOP::B0x1)
    }
}
/**Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_WWDG_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_WWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub type DBG_WWDG_STOP_R = crate::BitReader<DBG_WWDG_STOP>;
impl DBG_WWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_WWDG_STOP {
        match self.bits {
            false => DBG_WWDG_STOP::B0x0,
            true => DBG_WWDG_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_WWDG_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_WWDG_STOP::B0x1
    }
}
///Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_WWDG_STOP>;
impl<'a, REG> DBG_WWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_WWDG_STOP::B0x1)
    }
}
/**Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_IWDG_STOP {
    ///0: Enable
    B0x0 = 0,
    ///1: Disable
    B0x1 = 1,
}
impl From<DBG_IWDG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub type DBG_IWDG_STOP_R = crate::BitReader<DBG_IWDG_STOP>;
impl DBG_IWDG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_IWDG_STOP {
        match self.bits {
            false => DBG_IWDG_STOP::B0x0,
            true => DBG_IWDG_STOP::B0x1,
        }
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_IWDG_STOP::B0x0
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_IWDG_STOP::B0x1
    }
}
///Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_IWDG_STOP>;
impl<'a, REG> DBG_IWDG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::B0x0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_IWDG_STOP::B0x1)
    }
}
/**SMBUS timeout when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C1_SMBUS_TIMEOUT {
    ///0: Same behavior as in normal mode
    B0x0 = 0,
    ///1: The SMBUS timeout is frozen
    B0x1 = 1,
}
impl From<DBG_I2C1_SMBUS_TIMEOUT> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_SMBUS_TIMEOUT) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader<DBG_I2C1_SMBUS_TIMEOUT>;
impl DBG_I2C1_SMBUS_TIMEOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C1_SMBUS_TIMEOUT {
        match self.bits {
            false => DBG_I2C1_SMBUS_TIMEOUT::B0x0,
            true => DBG_I2C1_SMBUS_TIMEOUT::B0x1,
        }
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT::B0x0
    }
    ///The SMBUS timeout is frozen
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBG_I2C1_SMBUS_TIMEOUT::B0x1
    }
}
///Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C1_SMBUS_TIMEOUT>;
impl<'a, REG> DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT::B0x0)
    }
    ///The SMBUS timeout is frozen
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT::B0x1)
    }
}
impl R {
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ1")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, APB_FZ1rs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB_FZ1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB_FZ1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB_FZ1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB_FZ1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<'_, APB_FZ1rs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
}
/**DBG APB freeze register 1

You can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#DBG:APB_FZ1)*/
pub struct APB_FZ1rs;
impl crate::RegisterSpec for APB_FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz1::R`](R) reader structure
impl crate::Readable for APB_FZ1rs {}
///`write(|w| ..)` method takes [`apb_fz1::W`](W) writer structure
impl crate::Writable for APB_FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB_FZ1 to value 0
impl crate::Resettable for APB_FZ1rs {}
