///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**Fault 1 Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1R {
    ///0: No fault interrupt occurred
    NoEvent = 0,
    ///1: Fault interrupt occurred
    Event = 1,
}
impl From<FLT1R> for bool {
    #[inline(always)]
    fn from(variant: FLT1R) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1` reader - Fault 1 Interrupt Flag
pub type FLT1_R = crate::BitReader<FLT1R>;
impl FLT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1R {
        match self.bits {
            false => FLT1R::NoEvent,
            true => FLT1R::Event,
        }
    }
    ///No fault interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == FLT1R::NoEvent
    }
    ///Fault interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == FLT1R::Event
    }
}
///Field `FLT2` reader - Fault 2 Interrupt Flag
pub use FLT1_R as FLT2_R;
///Field `FLT3` reader - Fault 3 Interrupt Flag
pub use FLT1_R as FLT3_R;
///Field `FLT4` reader - Fault 4 Interrupt Flag
pub use FLT1_R as FLT4_R;
///Field `FLT5` reader - Fault 5 Interrupt Flag
pub use FLT1_R as FLT5_R;
/**System Fault Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSFLTR {
    ///0: No fault interrupt occurred
    NoEvent = 0,
    ///1: Fault interrupt occurred
    Event = 1,
}
impl From<SYSFLTR> for bool {
    #[inline(always)]
    fn from(variant: SYSFLTR) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSFLT` reader - System Fault Interrupt Flag
pub type SYSFLT_R = crate::BitReader<SYSFLTR>;
impl SYSFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSFLTR {
        match self.bits {
            false => SYSFLTR::NoEvent,
            true => SYSFLTR::Event,
        }
    }
    ///No fault interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYSFLTR::NoEvent
    }
    ///Fault interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYSFLTR::Event
    }
}
///Field `SYSFLT` writer - System Fault Interrupt Flag
pub type SYSFLT_W<'a, REG> = crate::BitWriter<'a, REG, SYSFLTR>;
impl<'a, REG> SYSFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No fault interrupt occurred
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(SYSFLTR::NoEvent)
    }
    ///Fault interrupt occurred
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(SYSFLTR::Event)
    }
}
/**Burst mode Period Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERR {
    ///0: No burst mode period interrupt occurred
    NoEvent = 0,
    ///1: Burst mode period interrupt occured
    Event = 1,
}
impl From<BMPERR> for bool {
    #[inline(always)]
    fn from(variant: BMPERR) -> Self {
        variant as u8 != 0
    }
}
///Field `BMPER` reader - Burst mode Period Interrupt Flag
pub type BMPER_R = crate::BitReader<BMPERR>;
impl BMPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BMPERR {
        match self.bits {
            false => BMPERR::NoEvent,
            true => BMPERR::Event,
        }
    }
    ///No burst mode period interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BMPERR::NoEvent
    }
    ///Burst mode period interrupt occured
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == BMPERR::Event
    }
}
impl R {
    ///Bit 0 - Fault 1 Interrupt Flag
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 Interrupt Flag
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 Interrupt Flag
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 Interrupt Flag
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 Interrupt Flag
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - System Fault Interrupt Flag
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 17 - Burst mode Period Interrupt Flag
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("bmper", &self.bmper())
            .field("sysflt", &self.sysflt())
            .field("flt1", &self.flt1())
            .field("flt5", &self.flt5())
            .field("flt4", &self.flt4())
            .field("flt3", &self.flt3())
            .field("flt2", &self.flt2())
            .finish()
    }
}
impl W {
    ///Bit 5 - System Fault Interrupt Flag
    #[inline(always)]
    pub fn sysflt(&mut self) -> SYSFLT_W<'_, ISRrs> {
        SYSFLT_W::new(self, 5)
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#HRTIM_Common:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
