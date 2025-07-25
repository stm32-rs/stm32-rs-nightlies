///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_SLEEP` reader - DBG_SLEEP
pub type DBG_SLEEP_R = crate::BitReader;
///Field `DBG_SLEEP` writer - DBG_SLEEP
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STOP` reader - DBG_STOP
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - DBG_STOP
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - DBG_STANDBY
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - DBG_STANDBY
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
/**TRACE_IOEN

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACE_IOEN {
    ///0: Trace pins not assigned (default state)
    Disabled = 0,
    ///1: Trace pins assigned
    Enabled = 1,
}
impl From<TRACE_IOEN> for bool {
    #[inline(always)]
    fn from(variant: TRACE_IOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TRACE_IOEN` reader - TRACE_IOEN
pub type TRACE_IOEN_R = crate::BitReader<TRACE_IOEN>;
impl TRACE_IOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRACE_IOEN {
        match self.bits {
            false => TRACE_IOEN::Disabled,
            true => TRACE_IOEN::Enabled,
        }
    }
    ///Trace pins not assigned (default state)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRACE_IOEN::Disabled
    }
    ///Trace pins assigned
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRACE_IOEN::Enabled
    }
}
///Field `TRACE_IOEN` writer - TRACE_IOEN
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG, TRACE_IOEN>;
impl<'a, REG> TRACE_IOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trace pins not assigned (default state)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN::Disabled)
    }
    ///Trace pins assigned
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_IOEN::Enabled)
    }
}
/**TRACE_MODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACE_MODE {
    ///0: TRACE pin assignment for Asynchronous Mode
    Asynchronous = 0,
    ///1: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
    Size1 = 1,
    ///2: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
    Size2 = 2,
    ///3: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
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
impl crate::IsEnum for TRACE_MODE {}
///Field `TRACE_MODE` reader - TRACE_MODE
pub type TRACE_MODE_R = crate::FieldReader<TRACE_MODE>;
impl TRACE_MODE_R {
    ///Get enumerated values variant
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
    ///TRACE pin assignment for Asynchronous Mode
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == TRACE_MODE::Asynchronous
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == TRACE_MODE::Size1
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == TRACE_MODE::Size2
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == TRACE_MODE::Size4
    }
}
///Field `TRACE_MODE` writer - TRACE_MODE
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRACE_MODE, crate::Safe>;
impl<'a, REG> TRACE_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TRACE pin assignment for Asynchronous Mode
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Asynchronous)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
    #[inline(always)]
    pub fn size1(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size1)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
    #[inline(always)]
    pub fn size2(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size2)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
    #[inline(always)]
    pub fn size4(self) -> &'a mut crate::W<REG> {
        self.variant(TRACE_MODE::Size4)
    }
}
impl R {
    ///Bit 0 - DBG_SLEEP
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DBG_STOP
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DBG_STANDBY
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - TRACE_IOEN
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - TRACE_MODE
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_mode", &self.trace_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG_SLEEP
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    ///Bit 1 - DBG_STOP
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - DBG_STANDBY
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    ///Bit 5 - TRACE_IOEN
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    ///Bits 6:7 - TRACE_MODE
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F756.html#DBGMCU:CR)*/
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
