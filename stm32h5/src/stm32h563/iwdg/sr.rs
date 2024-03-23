#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVU {
    #[doc = "0: No update on-going"]
    Idle = 0,
    #[doc = "1: Update on-going"]
    Busy = 1,
}
impl From<PVU> for bool {
    #[inline(always)]
    fn from(variant: PVU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset."]
pub type PVU_R = crate::BitReader<PVU>;
impl PVU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVU {
        match self.bits {
            false => PVU::Idle,
            true => PVU::Busy,
        }
    }
    #[doc = "No update on-going"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PVU::Idle
    }
    #[doc = "Update on-going"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PVU::Busy
    }
}
#[doc = "Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset."]
pub use PVU_R as RVU_R;
#[doc = "Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic “window” = 1"]
pub use PVU_R as WVU_R;
#[doc = "Field `EWU` reader - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
pub use PVU_R as EWU_R;
#[doc = "Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONFR {
    #[doc = "0: IWDG is not activated"]
    NotActivated = 0,
    #[doc = "1: IWDG is activated"]
    Activated = 1,
}
impl From<ONFR> for bool {
    #[inline(always)]
    fn from(variant: ONFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONF` reader - Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'."]
pub type ONF_R = crate::BitReader<ONFR>;
impl ONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ONFR {
        match self.bits {
            false => ONFR::NotActivated,
            true => ONFR::Activated,
        }
    }
    #[doc = "IWDG is not activated"]
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == ONFR::NotActivated
    }
    #[doc = "IWDG is activated"]
    #[inline(always)]
    pub fn is_activated(&self) -> bool {
        *self == ONFR::Activated
    }
}
#[doc = "Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR {
    #[doc = "0: No pending interrupt"]
    NotPending = 0,
    #[doc = "1: Interrupt pending"]
    Pending = 1,
}
impl From<EWIFR> for bool {
    #[inline(always)]
    fn from(variant: EWIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` reader - Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’."]
pub type EWIF_R = crate::BitReader<EWIFR>;
impl EWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWIFR {
        match self.bits {
            false => EWIFR::NotPending,
            true => EWIFR::Pending,
        }
    }
    #[doc = "No pending interrupt"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == EWIFR::NotPending
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset."]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset."]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic “window” = 1"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\\[11:0\\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\\[11:0\\]
and EWIE fields can be updated only when EWU bit is reset."]
    #[inline(always)]
    pub fn ewu(&self) -> EWU_R {
        EWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'."]
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Watchdog early interrupt flag This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’."]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "IWDG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
