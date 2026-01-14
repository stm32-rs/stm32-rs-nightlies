///Register `CDSR` reader
pub type R = crate::R<CDSRrs>;
/**Vertical Data Enable display Status

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDES {
    ///0: Currently not in vertical Data Enable phase
    NotActive = 0,
    ///1: Currently in vertical Data Enable phase
    Active = 1,
}
impl From<VDES> for bool {
    #[inline(always)]
    fn from(variant: VDES) -> Self {
        variant as u8 != 0
    }
}
///Field `VDES` reader - Vertical Data Enable display Status
pub type VDES_R = crate::BitReader<VDES>;
impl VDES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VDES {
        match self.bits {
            false => VDES::NotActive,
            true => VDES::Active,
        }
    }
    ///Currently not in vertical Data Enable phase
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VDES::NotActive
    }
    ///Currently in vertical Data Enable phase
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VDES::Active
    }
}
/**Horizontal Data Enable display Status

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDES {
    ///0: Currently not in horizontal Data Enable phase
    NotActive = 0,
    ///1: Currently in horizontal Data Enable phase
    Active = 1,
}
impl From<HDES> for bool {
    #[inline(always)]
    fn from(variant: HDES) -> Self {
        variant as u8 != 0
    }
}
///Field `HDES` reader - Horizontal Data Enable display Status
pub type HDES_R = crate::BitReader<HDES>;
impl HDES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDES {
        match self.bits {
            false => HDES::NotActive,
            true => HDES::Active,
        }
    }
    ///Currently not in horizontal Data Enable phase
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HDES::NotActive
    }
    ///Currently in horizontal Data Enable phase
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HDES::Active
    }
}
/**Vertical Synchronization display Status

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNCS {
    ///0: Currently not in VSYNC phase
    NotActive = 0,
    ///1: Currently in VSYNC phase
    Active = 1,
}
impl From<VSYNCS> for bool {
    #[inline(always)]
    fn from(variant: VSYNCS) -> Self {
        variant as u8 != 0
    }
}
///Field `VSYNCS` reader - Vertical Synchronization display Status
pub type VSYNCS_R = crate::BitReader<VSYNCS>;
impl VSYNCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSYNCS {
        match self.bits {
            false => VSYNCS::NotActive,
            true => VSYNCS::Active,
        }
    }
    ///Currently not in VSYNC phase
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VSYNCS::NotActive
    }
    ///Currently in VSYNC phase
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VSYNCS::Active
    }
}
/**Horizontal Synchronization display Status

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSYNCS {
    ///0: Currently not in HSYNC phase
    NotActive = 0,
    ///1: Currently in HSYNC phase
    Active = 1,
}
impl From<HSYNCS> for bool {
    #[inline(always)]
    fn from(variant: HSYNCS) -> Self {
        variant as u8 != 0
    }
}
///Field `HSYNCS` reader - Horizontal Synchronization display Status
pub type HSYNCS_R = crate::BitReader<HSYNCS>;
impl HSYNCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSYNCS {
        match self.bits {
            false => HSYNCS::NotActive,
            true => HSYNCS::Active,
        }
    }
    ///Currently not in HSYNC phase
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HSYNCS::NotActive
    }
    ///Currently in HSYNC phase
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HSYNCS::Active
    }
}
impl R {
    ///Bit 0 - Vertical Data Enable display Status
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Horizontal Data Enable display Status
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Vertical Synchronization display Status
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Horizontal Synchronization display Status
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDSR")
            .field("hsyncs", &self.hsyncs())
            .field("vsyncs", &self.vsyncs())
            .field("hdes", &self.hdes())
            .field("vdes", &self.vdes())
            .finish()
    }
}
/**Current Display Status Register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#LTDC:CDSR)*/
pub struct CDSRrs;
impl crate::RegisterSpec for CDSRrs {
    type Ux = u32;
}
///`read()` method returns [`cdsr::R`](R) reader structure
impl crate::Readable for CDSRrs {}
///`reset()` method sets CDSR to value 0x0f
impl crate::Resettable for CDSRrs {
    const RESET_VALUE: u32 = 0x0f;
}
