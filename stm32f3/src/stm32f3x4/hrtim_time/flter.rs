#[doc = "Register `FLTER` reader"]
pub type R = crate::R<FLTERrs>;
#[doc = "Register `FLTER` writer"]
pub type W = crate::W<FLTERrs>;
#[doc = "Fault 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1EN {
    #[doc = "0: Fault input ignored"]
    Ignored = 0,
    #[doc = "1: Fault input is active and can disable HRTIM outputs"]
    Active = 1,
}
impl From<FLT1EN> for bool {
    #[inline(always)]
    fn from(variant: FLT1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader<FLT1EN>;
impl FLT1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLT1EN {
        match self.bits {
            false => FLT1EN::Ignored,
            true => FLT1EN::Active,
        }
    }
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == FLT1EN::Ignored
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FLT1EN::Active
    }
}
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, REG> = crate::BitWriter<'a, REG, FLT1EN>;
impl<'a, REG> FLT1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault input ignored"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1EN::Ignored)
    }
    #[doc = "Fault input is active and can disable HRTIM outputs"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(FLT1EN::Active)
    }
}
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub use FLT1EN_R as FLT2EN_R;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub use FLT1EN_R as FLT3EN_R;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub use FLT1EN_R as FLT4EN_R;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub use FLT1EN_R as FLT5EN_R;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub use FLT1EN_W as FLT2EN_W;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub use FLT1EN_W as FLT3EN_W;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub use FLT1EN_W as FLT4EN_W;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub use FLT1EN_W as FLT5EN_W;
#[doc = "Fault sources Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTLCK {
    #[doc = "0: FLT1EN..FLT5EN bits are read/write"]
    Unlocked = 0,
    #[doc = "1: FLT1EN..FLT5EN bits are read only"]
    Locked = 1,
}
impl From<FLTLCK> for bool {
    #[inline(always)]
    fn from(variant: FLTLCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FLTLCK_R = crate::BitReader<FLTLCK>;
impl FLTLCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLTLCK {
        match self.bits {
            false => FLTLCK::Unlocked,
            true => FLTLCK::Locked,
        }
    }
    #[doc = "FLT1EN..FLT5EN bits are read/write"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLTLCK::Unlocked
    }
    #[doc = "FLT1EN..FLT5EN bits are read only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLTLCK::Locked
    }
}
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FLTLCK_W<'a, REG> = crate::BitWriter<'a, REG, FLTLCK>;
impl<'a, REG> FLTLCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLT1EN..FLT5EN bits are read/write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(FLTLCK::Unlocked)
    }
    #[doc = "FLT1EN..FLT5EN bits are read only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(FLTLCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<FLTERrs> {
        FLT1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<FLTERrs> {
        FLT2EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<FLTERrs> {
        FLT3EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<FLTERrs> {
        FLT4EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<FLTERrs> {
        FLT5EN_W::new(self, 4)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<FLTERrs> {
        FLTLCK_W::new(self, 31)
    }
}
#[doc = "Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTERrs;
impl crate::RegisterSpec for FLTERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flter::R`](R) reader structure"]
impl crate::Readable for FLTERrs {}
#[doc = "`write(|w| ..)` method takes [`flter::W`](W) writer structure"]
impl crate::Writable for FLTERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLTER to value 0"]
impl crate::Resettable for FLTERrs {
    const RESET_VALUE: u32 = 0;
}
