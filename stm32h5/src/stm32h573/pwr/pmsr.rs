#[doc = "Register `PMSR` reader"]
pub type R = crate::R<PMSRrs>;
#[doc = "Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPFR {
    #[doc = "0: System has not been in stop mode"]
    NoStopMode = 0,
    #[doc = "1: System has been in Stop mode"]
    StopModePreviouslyEntered = 1,
}
impl From<STOPFR> for bool {
    #[inline(always)]
    fn from(variant: STOPFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
pub type STOPF_R = crate::BitReader<STOPFR>;
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPFR {
        match self.bits {
            false => STOPFR::NoStopMode,
            true => STOPFR::StopModePreviouslyEntered,
        }
    }
    #[doc = "System has not been in stop mode"]
    #[inline(always)]
    pub fn is_no_stop_mode(&self) -> bool {
        *self == STOPFR::NoStopMode
    }
    #[doc = "System has been in Stop mode"]
    #[inline(always)]
    pub fn is_stop_mode_previously_entered(&self) -> bool {
        *self == STOPFR::StopModePreviouslyEntered
    }
}
#[doc = "System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR {
    #[doc = "0: System has not been in standby mode"]
    NoStandbyMode = 0,
    #[doc = "1: System has been in Standby mode"]
    StandbyModePreviouslyEntered = 1,
}
impl From<SBFR> for bool {
    #[inline(always)]
    fn from(variant: SBFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
pub type SBF_R = crate::BitReader<SBFR>;
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBFR {
        match self.bits {
            false => SBFR::NoStandbyMode,
            true => SBFR::StandbyModePreviouslyEntered,
        }
    }
    #[doc = "System has not been in standby mode"]
    #[inline(always)]
    pub fn is_no_standby_mode(&self) -> bool {
        *self == SBFR::NoStandbyMode
    }
    #[doc = "System has been in Standby mode"]
    #[inline(always)]
    pub fn is_standby_mode_previously_entered(&self) -> bool {
        *self == SBFR::StandbyModePreviouslyEntered
    }
}
impl R {
    #[doc = "Bit 5 - Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "PWR status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMSRrs;
impl crate::RegisterSpec for PMSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmsr::R`](R) reader structure"]
impl crate::Readable for PMSRrs {}
#[doc = "`reset()` method sets PMSR to value 0"]
impl crate::Resettable for PMSRrs {
    const RESET_VALUE: u32 = 0;
}
