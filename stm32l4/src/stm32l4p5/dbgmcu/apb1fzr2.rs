///Register `APB1FZR2` reader
pub type R = crate::R<APB1FZR2rs>;
///Register `APB1FZR2` writer
pub type W = crate::W<APB1FZR2rs>;
/**I2C4 SMBUS timeout counter stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_I2C4_STOP {
    ///0: Same behavior as in normal mode
    NormalMode = 0,
    ///1: I2Cx SMBUS timeout is frozen
    SmbusTimeoutFrozen = 1,
}
impl From<DBG_I2C4_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C4_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_I2C4_STOP` reader - I2C4 SMBUS timeout counter stopped when core is halted
pub type DBG_I2C4_STOP_R = crate::BitReader<DBG_I2C4_STOP>;
impl DBG_I2C4_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_I2C4_STOP {
        match self.bits {
            false => DBG_I2C4_STOP::NormalMode,
            true => DBG_I2C4_STOP::SmbusTimeoutFrozen,
        }
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DBG_I2C4_STOP::NormalMode
    }
    ///I2Cx SMBUS timeout is frozen
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        *self == DBG_I2C4_STOP::SmbusTimeoutFrozen
    }
}
///Field `DBG_I2C4_STOP` writer - I2C4 SMBUS timeout counter stopped when core is halted
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_I2C4_STOP>;
impl<'a, REG> DBG_I2C4_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C4_STOP::NormalMode)
    }
    ///I2Cx SMBUS timeout is frozen
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_I2C4_STOP::SmbusTimeoutFrozen)
    }
}
/**LPTIM2 counter stopped when core is halted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_LPTIM2_STOP {
    ///0: LPTIMx counter clock is fed even if the core is halted
    Continue = 0,
    ///1: LPTIMx counter clock is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_LPTIM2_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_R = crate::BitReader<DBG_LPTIM2_STOP>;
impl DBG_LPTIM2_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_LPTIM2_STOP {
        match self.bits {
            false => DBG_LPTIM2_STOP::Continue,
            true => DBG_LPTIM2_STOP::Stop,
        }
    }
    ///LPTIMx counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_LPTIM2_STOP::Continue
    }
    ///LPTIMx counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_LPTIM2_STOP::Stop
    }
}
///Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_LPTIM2_STOP>;
impl<'a, REG> DBG_LPTIM2_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIMx counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP::Continue)
    }
    ///LPTIMx counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_LPTIM2_STOP::Stop)
    }
}
impl R {
    ///Bit 1 - I2C4 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1FZR2")
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
            .finish()
    }
}
impl W {
    ///Bit 1 - I2C4 SMBUS timeout counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<'_, APB1FZR2rs> {
        DBG_I2C4_STOP_W::new(self, 1)
    }
    ///Bit 5 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<'_, APB1FZR2rs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
/**APB Low Freeze Register 2

You can [`read`](crate::Reg::read) this register and get [`apb1fzr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#DBGMCU:APB1FZR2)*/
pub struct APB1FZR2rs;
impl crate::RegisterSpec for APB1FZR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1fzr2::R`](R) reader structure
impl crate::Readable for APB1FZR2rs {}
///`write(|w| ..)` method takes [`apb1fzr2::W`](W) writer structure
impl crate::Writable for APB1FZR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1FZR2 to value 0
impl crate::Resettable for APB1FZR2rs {}
