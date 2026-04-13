///Register `FLTR` reader
pub type R = crate::R<FLTRrs>;
///Register `FLTR` writer
pub type W = crate::W<FLTRrs>;
/**Fault %s enable

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
///Field `FLTEN(1-5)` reader - Fault %s enable
pub type FLTEN_R = crate::BitReader<FLT1EN>;
impl FLTEN_R {
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
///Field `FLTEN(1-5)` writer - Fault %s enable
pub type FLTEN_W<'a, REG> = crate::BitWriter<'a, REG, FLT1EN>;
impl<'a, REG> FLTEN_W<'a, REG>
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
    ///Fault (1-5) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1EN` field.</div>
    #[inline(always)]
    pub fn flten(&self, n: u8) -> FLTEN_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FLTEN_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Fault (1-5) enable
    #[inline(always)]
    pub fn flten_iter(&self) -> impl Iterator<Item = FLTEN_R> + '_ {
        (0..5).map(move |n| FLTEN_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    pub fn flt1en(&self) -> FLTEN_R {
        FLTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    pub fn flt2en(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    pub fn flt3en(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    pub fn flt4en(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    pub fn flt5en(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTR")
            .field("fltlck", &self.fltlck())
            .field("flt1en", &self.flt1en())
            .field("flt2en", &self.flt2en())
            .field("flt3en", &self.flt3en())
            .field("flt4en", &self.flt4en())
            .field("flt5en", &self.flt5en())
            .finish()
    }
}
impl W {
    ///Fault (1-5) enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `FLT1EN` field.</div>
    #[inline(always)]
    pub fn flten(&mut self, n: u8) -> FLTEN_W<'_, FLTRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        FLTEN_W::new(self, n)
    }
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    pub fn flt1en(&mut self) -> FLTEN_W<'_, FLTRrs> {
        FLTEN_W::new(self, 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    pub fn flt2en(&mut self) -> FLTEN_W<'_, FLTRrs> {
        FLTEN_W::new(self, 1)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    pub fn flt3en(&mut self) -> FLTEN_W<'_, FLTRrs> {
        FLTEN_W::new(self, 2)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    pub fn flt4en(&mut self) -> FLTEN_W<'_, FLTRrs> {
        FLTEN_W::new(self, 3)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    pub fn flt5en(&mut self) -> FLTEN_W<'_, FLTRrs> {
        FLTEN_W::new(self, 4)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    pub fn fltlck(&mut self) -> FLTLCK_W<'_, FLTRrs> {
        FLTLCK_W::new(self, 31)
    }
}
/**Timerx Fault Register

You can [`read`](crate::Reg::read) this register and get [`fltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_TIMA:FLTR)*/
pub struct FLTRrs;
impl crate::RegisterSpec for FLTRrs {
    type Ux = u32;
}
///`read()` method returns [`fltr::R`](R) reader structure
impl crate::Readable for FLTRrs {}
///`write(|w| ..)` method takes [`fltr::W`](W) writer structure
impl crate::Writable for FLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLTR to value 0
impl crate::Resettable for FLTRrs {}
