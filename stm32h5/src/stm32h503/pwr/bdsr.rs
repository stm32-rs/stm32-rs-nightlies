///Register `BDSR` reader
pub type R = crate::R<BDSRrs>;
/**backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRDYR {
    ///0: Backup regulator not ready
    NotReady = 0,
    ///1: Backup regulator ready
    Ready = 1,
}
impl From<BRRDYR> for bool {
    #[inline(always)]
    fn from(variant: BRRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `BRRDY` reader - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.
pub type BRRDY_R = crate::BitReader<BRRDYR>;
impl BRRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRRDYR {
        match self.bits {
            false => BRRDYR::NotReady,
            true => BRRDYR::Ready,
        }
    }
    ///Backup regulator not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == BRRDYR::NotReady
    }
    ///Backup regulator ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BRRDYR::Ready
    }
}
/**V BAT level monitoring versus low threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATLR {
    ///0: Above low threshold level
    AboveThreshold = 0,
    ///1: Equal to or below low threshold level
    BelowThreshold = 1,
}
impl From<VBATLR> for bool {
    #[inline(always)]
    fn from(variant: VBATLR) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATL` reader - V BAT level monitoring versus low threshold
pub type VBATL_R = crate::BitReader<VBATLR>;
impl VBATL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATLR {
        match self.bits {
            false => VBATLR::AboveThreshold,
            true => VBATLR::BelowThreshold,
        }
    }
    ///Above low threshold level
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VBATLR::AboveThreshold
    }
    ///Equal to or below low threshold level
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VBATLR::BelowThreshold
    }
}
/**V BAT level monitoring versus high threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATHR {
    ///0: Below high threshold level
    BelowThreshold = 0,
    ///1: Equal to or Above high threshold level
    AboveThreshold = 1,
}
impl From<VBATHR> for bool {
    #[inline(always)]
    fn from(variant: VBATHR) -> Self {
        variant as u8 != 0
    }
}
///Field `VBATH` reader - V BAT level monitoring versus high threshold
pub type VBATH_R = crate::BitReader<VBATHR>;
impl VBATH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VBATHR {
        match self.bits {
            false => VBATHR::BelowThreshold,
            true => VBATHR::AboveThreshold,
        }
    }
    ///Below high threshold level
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VBATHR::BelowThreshold
    }
    ///Equal to or Above high threshold level
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VBATHR::AboveThreshold
    }
}
///Field `TEMPH` reader - temperature level monitoring versus high threshold
pub use VBATH_R as TEMPH_R;
///Field `TEMPL` reader - temperature level monitoring versus low threshold
pub use VBATL_R as TEMPL_R;
impl R {
    ///Bit 16 - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - V BAT level monitoring versus low threshold
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - V BAT level monitoring versus high threshold
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - temperature level monitoring versus low threshold
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - temperature level monitoring versus high threshold
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDSR")
            .field("brrdy", &self.brrdy())
            .field("vbatl", &self.vbatl())
            .field("vbath", &self.vbath())
            .field("templ", &self.templ())
            .field("temph", &self.temph())
            .finish()
    }
}
/**PWR Backup domain status register

You can [`read`](crate::Reg::read) this register and get [`bdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#PWR:BDSR)*/
pub struct BDSRrs;
impl crate::RegisterSpec for BDSRrs {
    type Ux = u32;
}
///`read()` method returns [`bdsr::R`](R) reader structure
impl crate::Readable for BDSRrs {}
///`reset()` method sets BDSR to value 0
impl crate::Resettable for BDSRrs {}
