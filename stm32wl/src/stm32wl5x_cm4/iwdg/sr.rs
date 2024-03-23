#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Watchdog prescaler value update\n\nValue on reset: 0"]
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
#[doc = "Field `PVU` reader - Watchdog prescaler value update"]
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
#[doc = "Field `RVU` reader - Watchdog counter reload value update"]
pub use PVU_R as RVU_R;
#[doc = "Field `WVU` reader - Watchdog counter window value update"]
pub use PVU_R as WVU_R;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update"]
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
