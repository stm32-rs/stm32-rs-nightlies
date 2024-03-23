#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<ISRrs>;
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1R {
    #[doc = "0: No fault interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Fault interrupt occurred"]
    Event = 1,
}
impl From<FLT1R> for bool {
    #[inline(always)]
    fn from(variant: FLT1R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub type FLT1_R = crate::BitReader<FLT1R>;
impl FLT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1R {
        match self.bits {
            false => FLT1R::NoEvent,
            true => FLT1R::Event,
        }
    }
    #[doc = "No fault interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == FLT1R::NoEvent
    }
    #[doc = "Fault interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == FLT1R::Event
    }
}
#[doc = "Fault 1 Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1W {
    #[doc = "1: Clear fault interrupt"]
    Clear = 1,
}
impl From<FLT1W> for bool {
    #[inline(always)]
    fn from(variant: FLT1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1` writer - Fault 1 Interrupt Flag"]
pub type FLT1_W<'a, REG> = crate::BitWriter<'a, REG, FLT1W>;
impl<'a, REG> FLT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear fault interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1W::Clear)
    }
}
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub use FLT1_R as FLT2_R;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub use FLT1_R as FLT3_R;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub use FLT1_R as FLT4_R;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub use FLT1_R as FLT5_R;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub use FLT1_R as SYSFLT_R;
#[doc = "Field `FLT2` writer - Fault 2 Interrupt Flag"]
pub use FLT1_W as FLT2_W;
#[doc = "Field `FLT3` writer - Fault 3 Interrupt Flag"]
pub use FLT1_W as FLT3_W;
#[doc = "Field `FLT4` writer - Fault 4 Interrupt Flag"]
pub use FLT1_W as FLT4_W;
#[doc = "Field `FLT5` writer - Fault 5 Interrupt Flag"]
pub use FLT1_W as FLT5_W;
#[doc = "Field `SYSFLT` writer - System Fault Interrupt Flag"]
pub use FLT1_W as SYSFLT_W;
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYR {
    #[doc = "0: No DLL calibration ready interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: DLL calibration ready interrupt occurred"]
    Event = 1,
}
impl From<DLLRDYR> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub type DLLRDY_R = crate::BitReader<DLLRDYR>;
impl DLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLLRDYR {
        match self.bits {
            false => DLLRDYR::NoEvent,
            true => DLLRDYR::Event,
        }
    }
    #[doc = "No DLL calibration ready interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == DLLRDYR::NoEvent
    }
    #[doc = "DLL calibration ready interrupt occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == DLLRDYR::Event
    }
}
#[doc = "DLL Ready Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLLRDYW {
    #[doc = "1: Clear DLL calibration interrupt"]
    Clear = 1,
}
impl From<DLLRDYW> for bool {
    #[inline(always)]
    fn from(variant: DLLRDYW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLLRDY` writer - DLL Ready Interrupt Flag"]
pub type DLLRDY_W<'a, REG> = crate::BitWriter<'a, REG, DLLRDYW>;
impl<'a, REG> DLLRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear DLL calibration interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DLLRDYW::Clear)
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERR {
    #[doc = "0: No burst mode period interrupt occurred"]
    NoEvent = 0,
    #[doc = "1: Burst mode period interrupt occured"]
    Event = 1,
}
impl From<BMPERR> for bool {
    #[inline(always)]
    fn from(variant: BMPERR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub type BMPER_R = crate::BitReader<BMPERR>;
impl BMPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BMPERR {
        match self.bits {
            false => BMPERR::NoEvent,
            true => BMPERR::Event,
        }
    }
    #[doc = "No burst mode period interrupt occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == BMPERR::NoEvent
    }
    #[doc = "Burst mode period interrupt occured"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == BMPERR::Event
    }
}
#[doc = "Burst mode Period Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPERW {
    #[doc = "1: Clear burst mode period interrupt"]
    Clear = 1,
}
impl From<BMPERW> for bool {
    #[inline(always)]
    fn from(variant: BMPERW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMPER` writer - Burst mode Period Interrupt Flag"]
pub type BMPER_W<'a, REG> = crate::BitWriter<'a, REG, BMPERW>;
impl<'a, REG> BMPER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear burst mode period interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BMPERW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> FLT1_R {
        FLT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> FLT2_R {
        FLT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> FLT3_R {
        FLT3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> FLT4_R {
        FLT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> FLT5_R {
        FLT5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SYSFLT_R {
        SYSFLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DLLRDY_R {
        DLLRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt1(&mut self) -> FLT1_W<ISRrs> {
        FLT1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt2(&mut self) -> FLT2_W<ISRrs> {
        FLT2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt3(&mut self) -> FLT3_W<ISRrs> {
        FLT3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt4(&mut self) -> FLT4_W<ISRrs> {
        FLT4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flt5(&mut self) -> FLT5_W<ISRrs> {
        FLT5_W::new(self, 4)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sysflt(&mut self) -> SYSFLT_W<ISRrs> {
        SYSFLT_W::new(self, 5)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dllrdy(&mut self) -> DLLRDY_W<ISRrs> {
        DLLRDY_W::new(self, 16)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BMPER_W<ISRrs> {
        BMPER_W::new(self, 17)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
