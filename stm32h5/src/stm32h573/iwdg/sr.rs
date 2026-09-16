///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVU {
    ///0: No update on-going
    Idle = 0,
    ///1: Update on-going
    Busy = 1,
}
impl From<PVU> for bool {
    #[inline(always)]
    fn from(variant: PVU) -> Self {
        variant as u8 != 0
    }
}
///Field `PVU` reader - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.
pub type PVU_R = crate::BitReader<PVU>;
impl PVU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVU {
        match self.bits {
            false => PVU::Idle,
            true => PVU::Busy,
        }
    }
    ///No update on-going
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PVU::Idle
    }
    ///Update on-going
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PVU::Busy
    }
}
///Field `RVU` reader - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset.
pub use PVU_R as RVU_R;
///Field `WVU` reader - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic 'window' = 1
pub use PVU_R as WVU_R;
///Field `EWU` reader - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\[11:0\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\[11:0\] and EWIE fields can be updated only when EWU bit is reset.
pub use PVU_R as EWU_R;
/**Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONFR {
    ///0: IWDG is not activated
    NotActivated = 0,
    ///1: IWDG is activated
    Activated = 1,
}
impl From<ONFR> for bool {
    #[inline(always)]
    fn from(variant: ONFR) -> Self {
        variant as u8 != 0
    }
}
///Field `ONF` reader - Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.
pub type ONF_R = crate::BitReader<ONFR>;
impl ONF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ONFR {
        match self.bits {
            false => ONFR::NotActivated,
            true => ONFR::Activated,
        }
    }
    ///IWDG is not activated
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == ONFR::NotActivated
    }
    ///IWDG is activated
    #[inline(always)]
    pub fn is_activated(&self) -> bool {
        *self == ONFR::Activated
    }
}
/**Watchdog early interrupt flag This bit is set to '1' by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to '1'.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR {
    ///0: No pending interrupt
    NotPending = 0,
    ///1: Interrupt pending
    Pending = 1,
}
impl From<EWIFR> for bool {
    #[inline(always)]
    fn from(variant: EWIFR) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIF` reader - Watchdog early interrupt flag This bit is set to '1' by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to '1'.
pub type EWIF_R = crate::BitReader<EWIFR>;
impl EWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWIFR {
        match self.bits {
            false => EWIFR::NotPending,
            true => EWIFR::Pending,
        }
    }
    ///No pending interrupt
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == EWIFR::NotPending
    }
    ///Interrupt pending
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR::Pending
    }
}
impl R {
    ///Bit 0 - Watchdog prescaler value update This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is reset by hardware when the prescaler update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The prescaler value can be updated only when PVU bit is reset.
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The reload value can be updated only when RVU bit is reset.
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update This bit is set by hardware to indicate that an update of the window value is ongoing. It is reset by hardware when the reload value update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The window value can be updated only when WVU bit is reset. This bit is generated only if generic 'window' = 1
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Watchdog interrupt comparator value update This bit is set by hardware to indicate that an update of the interrupt comparator value (EWIT\[11:0\]) or an update of the EWIE is ongoing. It is reset by hardware when the update operation is completed in the VDD voltage domain (takes up to three periods of the IWDG kernel clock iwdg_ker_ck). The EWIT\[11:0\] and EWIE fields can be updated only when EWU bit is reset.
    #[inline(always)]
    pub fn ewu(&self) -> EWU_R {
        EWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Watchdog enable status bit. Set to ‘1’ by hardware as soon as the IWDG is started. In software mode, it remains to '1' until the IWDG is reset. In hardware mode, this bit is always set to '1'.
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - Watchdog early interrupt flag This bit is set to '1' by hardware in order to indicate that an early interrupt is pending. This bit must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to '1'.
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("pvu", &self.pvu())
            .field("rvu", &self.rvu())
            .field("wvu", &self.wvu())
            .field("ewu", &self.ewu())
            .field("ewif", &self.ewif())
            .field("onf", &self.onf())
            .finish()
    }
}
/**IWDG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#IWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
