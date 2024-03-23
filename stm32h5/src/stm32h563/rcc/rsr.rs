#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSRrs>;
#[doc = "Register `RSR` writer"]
pub type W = crate::W<RSRrs>;
#[doc = "remove reset flag Set and reset by software to reset the value of the reset flags.\n\nValue on reset: 0"]
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
#[doc = "Field `RMVF` reader - remove reset flag Set and reset by software to reset the value of the reset flags."]
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
#[doc = "Field `RMVF` writer - remove reset flag Set and reset by software to reset the value of the reset flags."]
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
#[doc = "pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINRSTFR {
    #[doc = "0: No reset occurred for block"]
    NoResetOccurred = 0,
    #[doc = "1: Reset occurred for block"]
    ResetOccurred = 1,
}
impl From<PINRSTFR> for bool {
    #[inline(always)]
    fn from(variant: PINRSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_R = crate::BitReader<PINRSTFR>;
impl PINRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINRSTFR {
        match self.bits {
            false => PINRSTFR::NoResetOccurred,
            true => PINRSTFR::ResetOccurred,
        }
    }
    #[doc = "No reset occurred for block"]
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == PINRSTFR::NoResetOccurred
    }
    #[doc = "Reset occurred for block"]
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == PINRSTFR::ResetOccurred
    }
}
#[doc = "Field `PINRSTF` writer - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub type PINRSTF_W<'a, REG> = crate::BitWriter<'a, REG, PINRSTFR>;
impl<'a, REG> PINRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset occurred for block"]
    #[inline(always)]
    pub fn no_reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTFR::NoResetOccurred)
    }
    #[doc = "Reset occurred for block"]
    #[inline(always)]
    pub fn reset_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(PINRSTFR::ResetOccurred)
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub use PINRSTF_R as BORRSTF_R;
#[doc = "Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub use PINRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub use PINRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub use PINRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub use PINRSTF_R as LPWRRSTF_R;
#[doc = "Field `BORRSTF` writer - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub use PINRSTF_W as BORRSTF_W;
#[doc = "Field `SFTRSTF` writer - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
pub use PINRSTF_W as SFTRSTF_W;
#[doc = "Field `IWDGRSTF` writer - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub use PINRSTF_W as IWDGRSTF_W;
#[doc = "Field `WWDGRSTF` writer - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub use PINRSTF_W as WWDGRSTF_W;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
pub use PINRSTF_W as LPWRRSTF_W;
impl R {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<RSRrs> {
        RMVF_W::new(self, 23)
    }
    #[doc = "Bit 26 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<RSRrs> {
        PINRSTF_W::new(self, 26)
    }
    #[doc = "Bit 27 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<RSRrs> {
        BORRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<RSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    #[doc = "Bit 29 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<RSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    #[doc = "Bit 30 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<RSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<RSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
#[doc = "RCC reset status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSRrs;
impl crate::RegisterSpec for RSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSRrs {}
#[doc = "`write(|w| ..)` method takes [`rsr::W`](W) writer structure"]
impl crate::Writable for RSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSR to value 0x0c00_0000"]
impl crate::Resettable for RSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
