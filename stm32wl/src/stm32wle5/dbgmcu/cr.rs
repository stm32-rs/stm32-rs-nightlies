///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Allow debug in SLEEP mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_SLEEP {
    ///0: Debug Sleep Mode Disabled
    Disabled = 0,
    ///1: Debug Sleep Mode Enabled
    Enabled = 1,
}
impl From<DBG_SLEEP> for bool {
    #[inline(always)]
    fn from(variant: DBG_SLEEP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_SLEEP` reader - Allow debug in SLEEP mode
pub type DBG_SLEEP_R = crate::BitReader<DBG_SLEEP>;
impl DBG_SLEEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_SLEEP {
        match self.bits {
            false => DBG_SLEEP::Disabled,
            true => DBG_SLEEP::Enabled,
        }
    }
    ///Debug Sleep Mode Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_SLEEP::Disabled
    }
    ///Debug Sleep Mode Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_SLEEP::Enabled
    }
}
///Field `DBG_SLEEP` writer - Allow debug in SLEEP mode
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_SLEEP>;
impl<'a, REG> DBG_SLEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Debug Sleep Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SLEEP::Disabled)
    }
    ///Debug Sleep Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SLEEP::Enabled)
    }
}
/**Allow debug in STOP mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP {
    ///0: Debug Stop Mode Disabled
    Disabled = 0,
    ///1: Debug Stop Mode Enabled
    Enabled = 1,
}
impl From<DBG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_STOP` reader - Allow debug in STOP mode
pub type DBG_STOP_R = crate::BitReader<DBG_STOP>;
impl DBG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STOP {
        match self.bits {
            false => DBG_STOP::Disabled,
            true => DBG_STOP::Enabled,
        }
    }
    ///Debug Stop Mode Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STOP::Disabled
    }
    ///Debug Stop Mode Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STOP::Enabled
    }
}
///Field `DBG_STOP` writer - Allow debug in STOP mode
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STOP>;
impl<'a, REG> DBG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Debug Stop Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP::Disabled)
    }
    ///Debug Stop Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP::Enabled)
    }
}
/**Allow debug in STANDBY mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY {
    ///0: Debug Standby Mode Disabled
    Disabled = 0,
    ///1: Debug Standby Mode Enabled
    Enabled = 1,
}
impl From<DBG_STANDBY> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_STANDBY` reader - Allow debug in STANDBY mode
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY>;
impl DBG_STANDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STANDBY {
        match self.bits {
            false => DBG_STANDBY::Disabled,
            true => DBG_STANDBY::Enabled,
        }
    }
    ///Debug Standby Mode Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STANDBY::Disabled
    }
    ///Debug Standby Mode Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STANDBY::Enabled
    }
}
///Field `DBG_STANDBY` writer - Allow debug in STANDBY mode
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STANDBY>;
impl<'a, REG> DBG_STANDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Debug Standby Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY::Disabled)
    }
    ///Debug Standby Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY::Enabled)
    }
}
impl R {
    ///Bit 0 - Allow debug in SLEEP mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow debug in STOP mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allow debug in STANDBY mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .finish()
    }
}
impl W {
    ///Bit 0 - Allow debug in SLEEP mode
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<'_, CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    ///Bit 1 - Allow debug in STOP mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<'_, CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - Allow debug in STANDBY mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<'_, CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
}
/**DBGMCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#DBGMCU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
