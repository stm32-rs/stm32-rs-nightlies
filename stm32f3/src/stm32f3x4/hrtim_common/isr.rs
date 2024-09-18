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
/**Fault 1 Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1W {
    ///1: Clear fault interrupt
    Clear = 1,
}
impl From<FLT1W> for bool {
    #[inline(always)]
    fn from(variant: FLT1W) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1` writer - Fault 1 Interrupt Flag
pub type FLT1_W<'a, REG> = crate::BitWriter<'a, REG, FLT1W>;
impl<'a, REG> FLT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear fault interrupt
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1W::Clear)
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
///Field `SYSFLT` reader - System Fault Interrupt Flag
pub use FLT1_R as SYSFLT_R;
///Field `FLT2` writer - Fault 2 Interrupt Flag
pub use FLT1_W as FLT2_W;
///Field `FLT3` writer - Fault 3 Interrupt Flag
pub use FLT1_W as FLT3_W;
///Field `FLT4` writer - Fault 4 Interrupt Flag
pub use FLT1_W as FLT4_W;
///Field `FLT5` writer - Fault 5 Interrupt Flag
pub use FLT1_W as FLT5_W;
///Field `SYSFLT` writer - System Fault Interrupt Flag
pub use FLT1_W as SYSFLT_W;
/**DLL Ready Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYR {
    ///0: No DLL calibration ready interrupt occurred
    NoEvent = 0,
    ///1: DLL calibration ready interrupt occurred
    Event = 1,
}
impl From<DLLRDYR> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `DLLRDY` reader - DLL Ready Interrupt Flag
pub type DLLRDY_R = crate::BitReader<DLLRDYR>;
impl DLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLLRDYR {
        match self.bits {
            false => DLLRDYR::NoEvent,
            true => DLLRDYR::Event,
        }
    }
    ///No DLL calibration ready interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == DLLRDYR::NoEvent
    }
    ///DLL calibration ready interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLLRDYR::Event
    }
}
/**DLL Ready Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYW {
    ///1: Clear DLL calibration interrupt
    Clear = 1,
}
impl From<DLLRDYW> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `DLLRDY` writer - DLL Ready Interrupt Flag
pub type DLLRDY_W<'a, REG> = crate::BitWriter<'a, REG, DLLRDYW>;
impl<'a, REG> DLLRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear DLL calibration interrupt
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DLLRDYW::Clear)
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
/**Burst mode Period Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERW {
    ///1: Clear burst mode period interrupt
    Clear = 1,
}
impl From<BMPERW> for bool {
    #[inline(always)]
    fn from(variant: BMPERW) -> Self {
        variant as u8 != 0
    }
}
///Field `BMPER` writer - Burst mode Period Interrupt Flag
pub type BMPER_W<'a, REG> = crate::BitWriter<'a, REG, BMPERW>;
impl<'a, REG> BMPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear burst mode period interrupt
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BMPERW::Clear)
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
    ///Bit 16 - DLL Ready Interrupt Flag
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 1) != 0)
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
            .field("dllrdy", &self.dllrdy())
            .field("flt1", &self.flt1())
            .field("sysflt", &self.sysflt())
            .field("flt5", &self.flt5())
            .field("flt4", &self.flt4())
            .field("flt3", &self.flt3())
            .field("flt2", &self.flt2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault 1 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn flt1(&mut self) -> FLT1_W<ISRrs> {
        FLT1_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn flt2(&mut self) -> FLT2_W<ISRrs> {
        FLT2_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn flt3(&mut self) -> FLT3_W<ISRrs> {
        FLT3_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn flt4(&mut self) -> FLT4_W<ISRrs> {
        FLT4_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn flt5(&mut self) -> FLT5_W<ISRrs> {
        FLT5_W::new(self, 4)
    }
    ///Bit 5 - System Fault Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn sysflt(&mut self) -> SYSFLT_W<ISRrs> {
        SYSFLT_W::new(self, 5)
    }
    ///Bit 16 - DLL Ready Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn dllrdy(&mut self) -> DLLRDY_W<ISRrs> {
        DLLRDY_W::new(self, 16)
    }
    ///Bit 17 - Burst mode Period Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BMPER_W<ISRrs> {
        BMPER_W::new(self, 17)
    }
}
/**Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_Common:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
