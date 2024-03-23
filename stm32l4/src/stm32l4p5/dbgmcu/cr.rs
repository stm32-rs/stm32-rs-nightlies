#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Debug Sleep Mode\n\nValue on reset: 0"]
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
#[doc = "Field `DBG_SLEEP` reader - Debug Sleep Mode"]
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
#[doc = "Field `DBG_SLEEP` writer - Debug Sleep Mode"]
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
#[doc = "Debug Stop Mode\n\nValue on reset: 0"]
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
#[doc = "Field `DBG_STOP` reader - Debug Stop Mode"]
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
#[doc = "Field `DBG_STOP` writer - Debug Stop Mode"]
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
#[doc = "Debug Standby Mode\n\nValue on reset: 0"]
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
#[doc = "Field `DBG_STANDBY` reader - Debug Standby Mode"]
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
#[doc = "Field `DBG_STANDBY` writer - Debug Standby Mode"]
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
#[doc = "Trace pin assignment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACE_IOEN {
    #[doc = "0: Trace pins not assigned (default state)"]
    Disabled = 0,
    #[doc = "1: Trace pins assigned"]
    Enabled = 1,
}
impl From<TRACE_IOEN> for bool {
    #[inline(always)]
    fn from(variant: TRACE_IOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACE_IOEN` reader - Trace pin assignment control"]
pub type TRACE_IOEN_R = crate::BitReader<TRACE_IOEN>;
impl TRACE_IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_IOEN {
        match self.bits {
            false => TRACE_IOEN::Disabled,
            true => TRACE_IOEN::Enabled,
        }
    }
    #[doc = "Trace pins not assigned (default state)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRACE_IOEN::Disabled
    }
    #[doc = "Trace pins assigned"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRACE_IOEN::Enabled
    }
}
#[doc = "Field `TRACE_IOEN` writer - Trace pin assignment control"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG, TRACE_IOEN>;
impl<'a, REG> TRACE_IOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trace pins not assigned (default state)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN::Disabled)
    }
    #[doc = "Trace pins assigned"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN::Enabled)
    }
}
#[doc = "Trace pin assignment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACE_MODE {
    #[doc = "0: TRACE pin assignment for Asynchronous Mode"]
    Asynchronous = 0,
    #[doc = "1: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1"]
    Size1 = 1,
    #[doc = "2: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2"]
    Size2 = 2,
    #[doc = "3: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4"]
    Size4 = 3,
}
impl From<TRACE_MODE> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRACE_MODE {
    type Ux = u8;
}
#[doc = "Field `TRACE_MODE` reader - Trace pin assignment control"]
pub type TRACE_MODE_R = crate::FieldReader<TRACE_MODE>;
impl TRACE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_MODE {
        match self.bits {
            0 => TRACE_MODE::Asynchronous,
            1 => TRACE_MODE::Size1,
            2 => TRACE_MODE::Size2,
            3 => TRACE_MODE::Size4,
            _ => unreachable!(),
        }
    }
    #[doc = "TRACE pin assignment for Asynchronous Mode"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == TRACE_MODE::Asynchronous
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1"]
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == TRACE_MODE::Size1
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2"]
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == TRACE_MODE::Size2
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4"]
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == TRACE_MODE::Size4
    }
}
#[doc = "Field `TRACE_MODE` writer - Trace pin assignment control"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TRACE_MODE>;
impl<'a, REG> TRACE_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRACE pin assignment for Asynchronous Mode"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Asynchronous)
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1"]
    #[inline(always)]
    pub fn size1(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size1)
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2"]
    #[inline(always)]
    pub fn size2(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size2)
    }
    #[doc = "TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4"]
    #[inline(always)]
    pub fn size4(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size4)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Sleep Mode"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Sleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Standby Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 5 - Trace pin assignment control"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Trace pin assignment control"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
}
#[doc = "Debug MCU Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
