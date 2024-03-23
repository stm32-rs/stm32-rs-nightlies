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
#[doc = "CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDRSTFR {
    #[doc = "0: No reset occurred for block"]
    NoResetOccurred = 0,
    #[doc = "1: Reset occurred for block"]
    ResetOccurred = 1,
}
impl From<CDRSTFR> for bool {
    #[inline(always)]
    fn from(variant: CDRSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDRSTF` reader - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)"]
pub type CDRSTF_R = crate::BitReader<CDRSTFR>;
impl CDRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CDRSTFR {
        match self.bits {
            false => CDRSTFR::NoResetOccurred,
            true => CDRSTFR::ResetOccurred,
        }
    }
    #[doc = "No reset occurred for block"]
    #[inline(always)]
    pub fn is_no_reset_occurred(&self) -> bool {
        *self == CDRSTFR::NoResetOccurred
    }
    #[doc = "Reset occurred for block"]
    #[inline(always)]
    pub fn is_reset_occurred(&self) -> bool {
        *self == CDRSTFR::ResetOccurred
    }
}
#[doc = "Field `BORRSTF` reader - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
pub use CDRSTF_R as BORRSTF_R;
#[doc = "Field `PINRSTF` reader - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
pub use CDRSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
pub use CDRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
pub use CDRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
pub use CDRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
pub use CDRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop."]
pub use CDRSTF_R as LPWRRSTF_R;
impl R {
    #[doc = "Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU domain power-switch reset flag Reset by software by writing the RMVF bit. Set by hardware when a the CPU domain exits from DStop or after of power-on reset. Set also when the CPU domain exists DStop2 but only when a pad reset has occurred during DStop2 (PINRST bit also set by hardware)"]
    #[inline(always)]
    pub fn cdrstf(&self) -> CDRSTF_R {
        CDRSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag Reset by software by writing the RMVF bit. Set by hardware when a POR/PDR reset occurs."]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M7."]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - reset due to illegal CD DStop or CD DStop2 or CPU CStop flag Reset by software by writing the RMVF bit. Set by hardware when the CPU domain goes erroneously in DStop or DStop2, or when the CPU goes erroneously in CStop."]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - remove reset flag Set and reset by software to reset the value of the reset flags."]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<RSRrs> {
        RMVF_W::new(self, 16)
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
#[doc = "`reset()` method sets RSR to value 0x00e8_0000"]
impl crate::Resettable for RSRrs {
    const RESET_VALUE: u32 = 0x00e8_0000;
}
