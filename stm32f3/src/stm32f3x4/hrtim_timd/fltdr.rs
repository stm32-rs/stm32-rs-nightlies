///Register `FLTDR` reader
pub type R = crate::R<FLTDRrs>;
///Register `FLTDR` writer
pub type W = crate::W<FLTDRrs>;
/**Fault 1 enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1EN {
    ///0: Fault input ignored
    Ignored = 0,
    ///1: Fault input is active and can disable HRTIM outputs
    Active = 1,
}
impl From<FLT1EN> for bool {
    #[inline(always)]
    fn from(variant: FLT1EN) -> Self {
        variant as u8 != 0
    }
}
///Field `FLT1EN` reader - Fault 1 enable
pub type FLT1EN_R = crate::BitReader<FLT1EN>;
impl FLT1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLT1EN {
        match self.bits {
            false => FLT1EN::Ignored,
            true => FLT1EN::Active,
        }
    }
    ///Fault input ignored
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == FLT1EN::Ignored
    }
    ///Fault input is active and can disable HRTIM outputs
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FLT1EN::Active
    }
}
///Field `FLT1EN` writer - Fault 1 enable
pub type FLT1EN_W<'a, REG> = crate::BitWriter<'a, REG, FLT1EN>;
impl<'a, REG> FLT1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fault input ignored
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1EN::Ignored)
    }
    ///Fault input is active and can disable HRTIM outputs
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1EN::Active)
    }
}
///Field `FLT2EN` reader - Fault 2 enable
pub use FLT1EN_R as FLT2EN_R;
///Field `FLT3EN` reader - Fault 3 enable
pub use FLT1EN_R as FLT3EN_R;
///Field `FLT4EN` reader - Fault 4 enable
pub use FLT1EN_R as FLT4EN_R;
///Field `FLT5EN` reader - Fault 5 enable
pub use FLT1EN_R as FLT5EN_R;
///Field `FLT2EN` writer - Fault 2 enable
pub use FLT1EN_W as FLT2EN_W;
///Field `FLT3EN` writer - Fault 3 enable
pub use FLT1EN_W as FLT3EN_W;
///Field `FLT4EN` writer - Fault 4 enable
pub use FLT1EN_W as FLT4EN_W;
///Field `FLT5EN` writer - Fault 5 enable
pub use FLT1EN_W as FLT5EN_W;
/**Fault sources Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTLCK {
    ///0: FLT1EN..FLT5EN bits are read/write
    Unlocked = 0,
    ///1: FLT1EN..FLT5EN bits are read only
    Locked = 1,
}
impl From<FLTLCK> for bool {
    #[inline(always)]
    fn from(variant: FLTLCK) -> Self {
        variant as u8 != 0
    }
}
///Field `FLTLCK` reader - Fault sources Lock
pub type FLTLCK_R = crate::BitReader<FLTLCK>;
impl FLTLCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLTLCK {
        match self.bits {
            false => FLTLCK::Unlocked,
            true => FLTLCK::Locked,
        }
    }
    ///FLT1EN..FLT5EN bits are read/write
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLTLCK::Unlocked
    }
    ///FLT1EN..FLT5EN bits are read only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLTLCK::Locked
    }
}
///Field `FLTLCK` writer - Fault sources Lock
pub type FLTLCK_W<'a, REG> = crate::BitWriter<'a, REG, FLTLCK>;
impl<'a, REG> FLTLCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FLT1EN..FLT5EN bits are read/write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(FLTLCK::Unlocked)
    }
    ///FLT1EN..FLT5EN bits are read only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(FLTLCK::Locked)
    }
}
impl R {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTDR")
            .field("fltlck", &self.fltlck())
            .field("flt1en", &self.flt1en())
            .field("flt5en", &self.flt5en())
            .field("flt4en", &self.flt4en())
            .field("flt3en", &self.flt3en())
            .field("flt2en", &self.flt2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<FLTDRrs> {
        FLT1EN_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<FLTDRrs> {
        FLT2EN_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<FLTDRrs> {
        FLT3EN_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<FLTDRrs> {
        FLT4EN_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<FLTDRrs> {
        FLT5EN_W::new(self, 4)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<FLTDRrs> {
        FLTLCK_W::new(self, 31)
    }
}
/**Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMD:FLTDR)*/
pub struct FLTDRrs;
impl crate::RegisterSpec for FLTDRrs {
    type Ux = u32;
}
///`read()` method returns [`fltdr::R`](R) reader structure
impl crate::Readable for FLTDRrs {}
///`write(|w| ..)` method takes [`fltdr::W`](W) writer structure
impl crate::Writable for FLTDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLTDR to value 0
impl crate::Resettable for FLTDRrs {
    const RESET_VALUE: u32 = 0;
}
