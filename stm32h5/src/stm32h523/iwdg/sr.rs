///Register `SR` reader
pub type R = crate::R<SRrs>;
/**Watchdog prescaler value update

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
///Field `PVU` reader - Watchdog prescaler value update
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
///Field `RVU` reader - Watchdog counter reload value update
pub use PVU_R as RVU_R;
///Field `WVU` reader - Watchdog counter window value update
pub use PVU_R as WVU_R;
///Field `EWU` reader - Watchdog interrupt comparator value update
pub use PVU_R as EWU_R;
/**Watchdog enable status bit

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
///Field `ONF` reader - Watchdog enable status bit
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
/**Watchdog early interrupt flag

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
///Field `EWIF` reader - Watchdog early interrupt flag
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
    ///Bit 0 - Watchdog prescaler value update
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Watchdog counter window value update
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Watchdog interrupt comparator value update
    #[inline(always)]
    pub fn ewu(&self) -> EWU_R {
        EWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Watchdog enable status bit
    #[inline(always)]
    pub fn onf(&self) -> ONF_R {
        ONF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 14 - Watchdog early interrupt flag
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
            .field("onf", &self.onf())
            .field("ewif", &self.ewif())
            .finish()
    }
}
/**IWDG status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#IWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
