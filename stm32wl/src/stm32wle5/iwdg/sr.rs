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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("pvu", &self.pvu())
            .field("wvu", &self.wvu())
            .field("rvu", &self.rvu())
            .finish()
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#IWDG:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
