#[doc = "Register `C1_RSR` reader"]
pub type R = crate::R<C1_RSRrs>;
#[doc = "Register `C1_RSR` writer"]
pub type W = crate::W<C1_RSRrs>;
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVF {
    #[doc = "0: Reset not activated"]
    NotActivated = 0,
    #[doc = "1: Reset the reset status flags"]
    Reset = 1,
}
impl From<RMVF> for bool {
    #[inline(always)]
    fn from(variant: RMVF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RMVF {
        match self.bits {
            false => RMVF::NotActivated,
            true => RMVF::Reset,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == RMVF::NotActivated
    }
    #[doc = "Reset the reset status flags"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RMVF::Reset
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVF>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn not_activated(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::NotActivated)
    }
    #[doc = "Reset the reset status flags"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RMVF::Reset)
    }
}
#[doc = "CPU reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPURSTFR {
    #[doc = "0: No reset occurred for block"]
    NoResetOccurred = 0,
    #[doc = "1: Reset occurred for block"]
    ResetOccurred = 1,
}
impl From<CPURSTFR> for bool {
    #[inline(always)]
    fn from(variant: CPURSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPURSTF` reader - CPU reset flag"]
pub type CPURSTF_R = crate::BitReader<CPURSTFR>;
impl CPURSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPURSTFR {
        match self.bits {
            false => CPURSTFR::NoResetOccurred,
            true => CPURSTFR::ResetOccurred,
        }
    }
    #[doc = "No reset occurred for block"]
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == CPURSTFR::NoResetOccurred
    }
    #[doc = "Reset occurred for block"]
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == CPURSTFR::ResetOccurred
    }
}
#[doc = "Field `CPURSTF` writer - CPU reset flag"]
pub type CPURSTF_W<'a, REG> = crate::BitWriter<'a, REG, CPURSTFR>;
impl<'a, REG> CPURSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset occurred for block"]
    #[inline(always)]
    pub fn no_reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(CPURSTFR::NoResetOccurred)
    }
    #[doc = "Reset occurred for block"]
    #[inline(always)]
    pub fn reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(CPURSTFR::ResetOccurred)
    }
}
#[doc = "Field `D1RSTF` reader - D1 domain power switch reset flag"]
pub use CPURSTF_R as D1RSTF_R;
#[doc = "Field `D2RSTF` reader - D2 domain power switch reset flag"]
pub use CPURSTF_R as D2RSTF_R;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub use CPURSTF_R as BORRSTF_R;
#[doc = "Field `PINRSTF` reader - Pin reset flag (NRST)"]
pub use CPURSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use CPURSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - System reset from CPU reset flag"]
pub use CPURSTF_R as SFTRSTF_R;
#[doc = "Field `IWDG1RSTF` reader - Independent Watchdog reset flag"]
pub use CPURSTF_R as IWDG1RSTF_R;
#[doc = "Field `WWDG1RSTF` reader - Window Watchdog reset flag"]
pub use CPURSTF_R as WWDG1RSTF_R;
#[doc = "Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub use CPURSTF_R as LPWRRSTF_R;
#[doc = "Field `D1RSTF` writer - D1 domain power switch reset flag"]
pub use CPURSTF_W as D1RSTF_W;
#[doc = "Field `D2RSTF` writer - D2 domain power switch reset flag"]
pub use CPURSTF_W as D2RSTF_W;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub use CPURSTF_W as BORRSTF_W;
#[doc = "Field `PINRSTF` writer - Pin reset flag (NRST)"]
pub use CPURSTF_W as PINRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use CPURSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - System reset from CPU reset flag"]
pub use CPURSTF_W as SFTRSTF_W;
#[doc = "Field `IWDG1RSTF` writer - Independent Watchdog reset flag"]
pub use CPURSTF_W as IWDG1RSTF_W;
#[doc = "Field `WWDG1RSTF` writer - Window Watchdog reset flag"]
pub use CPURSTF_W as WWDG1RSTF_W;
#[doc = "Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub use CPURSTF_W as LPWRRSTF_W;
impl R {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&self) -> CPURSTF_R {
        CPURSTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&self) -> D1RSTF_R {
        D1RSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&self) -> D2RSTF_R {
        D2RSTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<C1_RSRrs> {
        RMVF_W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn cpurstf(&mut self) -> CPURSTF_W<C1_RSRrs> {
        CPURSTF_W::new(self, 17)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1rstf(&mut self) -> D1RSTF_W<C1_RSRrs> {
        D1RSTF_W::new(self, 19)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2rstf(&mut self) -> D2RSTF_W<C1_RSRrs> {
        D2RSTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<C1_RSRrs> {
        BORRSTF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<C1_RSRrs> {
        PINRSTF_W::new(self, 22)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<C1_RSRrs> {
        PORRSTF_W::new(self, 23)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<C1_RSRrs> {
        SFTRSTF_W::new(self, 24)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<C1_RSRrs> {
        IWDG1RSTF_W::new(self, 26)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<C1_RSRrs> {
        WWDG1RSTF_W::new(self, 28)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<C1_RSRrs> {
        LPWRRSTF_W::new(self, 30)
    }
}
#[doc = "RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_RSRrs;
impl crate::RegisterSpec for C1_RSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_rsr::R`](R) reader structure"]
impl crate::Readable for C1_RSRrs {}
#[doc = "`write(|w| ..)` method takes [`c1_rsr::W`](W) writer structure"]
impl crate::Writable for C1_RSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1_RSR to value 0"]
impl crate::Resettable for C1_RSRrs {
    const RESET_VALUE: u32 = 0;
}
