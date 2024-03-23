#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Allow debug in SLEEP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_SLEEP {
    #[doc = "0: Debug Sleep Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Sleep Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_SLEEP> for bool {
    #[inline(always)]
    fn from(variant: DBG_SLEEP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_SLEEP` reader - Allow debug in SLEEP mode"]
pub type DBG_SLEEP_R = crate::BitReader<DBG_SLEEP>;
impl DBG_SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_SLEEP {
        match self.bits {
            false => DBG_SLEEP::Disabled,
            true => DBG_SLEEP::Enabled,
        }
    }
    #[doc = "Debug Sleep Mode Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_SLEEP::Disabled
    }
    #[doc = "Debug Sleep Mode Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_SLEEP::Enabled
    }
}
#[doc = "Field `DBG_SLEEP` writer - Allow debug in SLEEP mode"]
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_SLEEP>;
impl<'a, REG> DBG_SLEEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug Sleep Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SLEEP::Disabled)
    }
    #[doc = "Debug Sleep Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_SLEEP::Enabled)
    }
}
#[doc = "Allow debug in STOP mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP {
    #[doc = "0: Debug Stop Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Stop Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STOP` reader - Allow debug in STOP mode"]
pub type DBG_STOP_R = crate::BitReader<DBG_STOP>;
impl DBG_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STOP {
        match self.bits {
            false => DBG_STOP::Disabled,
            true => DBG_STOP::Enabled,
        }
    }
    #[doc = "Debug Stop Mode Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STOP::Disabled
    }
    #[doc = "Debug Stop Mode Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STOP::Enabled
    }
}
#[doc = "Field `DBG_STOP` writer - Allow debug in STOP mode"]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STOP>;
impl<'a, REG> DBG_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug Stop Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP::Disabled)
    }
    #[doc = "Debug Stop Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STOP::Enabled)
    }
}
#[doc = "Allow debug in STANDBY mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY {
    #[doc = "0: Debug Standby Mode Disabled"]
    Disabled = 0,
    #[doc = "1: Debug Standby Mode Enabled"]
    Enabled = 1,
}
impl From<DBG_STANDBY> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG_STANDBY` reader - Allow debug in STANDBY mode"]
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY>;
impl DBG_STANDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_STANDBY {
        match self.bits {
            false => DBG_STANDBY::Disabled,
            true => DBG_STANDBY::Enabled,
        }
    }
    #[doc = "Debug Standby Mode Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STANDBY::Disabled
    }
    #[doc = "Debug Standby Mode Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STANDBY::Enabled
    }
}
#[doc = "Field `DBG_STANDBY` writer - Allow debug in STANDBY mode"]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG, DBG_STANDBY>;
impl<'a, REG> DBG_STANDBY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Debug Standby Mode Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY::Disabled)
    }
    #[doc = "Debug Standby Mode Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_STANDBY::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Allow debug in SLEEP mode"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow debug in STOP mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow debug in STANDBY mode"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow debug in SLEEP mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Allow debug in STOP mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow debug in STANDBY mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
}
#[doc = "DBGMCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
