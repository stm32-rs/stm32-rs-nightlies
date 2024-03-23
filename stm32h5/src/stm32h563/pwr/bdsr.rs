#[doc = "Register `BDSR` reader"]
pub type R = crate::R<BDSRrs>;
#[doc = "backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRDYR {
    #[doc = "0: Backup regulator not ready"]
    NotReady = 0,
    #[doc = "1: Backup regulator ready"]
    Ready = 1,
}
impl From<BRRDYR> for bool {
    #[inline(always)]
    fn from(variant: BRRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRRDY` reader - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
pub type BRRDY_R = crate::BitReader<BRRDYR>;
impl BRRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRRDYR {
        match self.bits {
            false => BRRDYR::NotReady,
            true => BRRDYR::Ready,
        }
    }
    #[doc = "Backup regulator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == BRRDYR::NotReady
    }
    #[doc = "Backup regulator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == BRRDYR::Ready
    }
}
#[doc = "V&lt;sub>BAT&lt;/sub> level monitoring versus low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATLR {
    #[doc = "0: Above low threshold level"]
    AboveThreshold = 0,
    #[doc = "1: Equal to or below low threshold level"]
    BelowThreshold = 1,
}
impl From<VBATLR> for bool {
    #[inline(always)]
    fn from(variant: VBATLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATL` reader - V&lt;sub>BAT&lt;/sub> level monitoring versus low threshold"]
pub type VBATL_R = crate::BitReader<VBATLR>;
impl VBATL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATLR {
        match self.bits {
            false => VBATLR::AboveThreshold,
            true => VBATLR::BelowThreshold,
        }
    }
    #[doc = "Above low threshold level"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VBATLR::AboveThreshold
    }
    #[doc = "Equal to or below low threshold level"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VBATLR::BelowThreshold
    }
}
#[doc = "V&lt;sub>BAT&lt;/sub> level monitoring versus high threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATHR {
    #[doc = "0: Below high threshold level"]
    BelowThreshold = 0,
    #[doc = "1: Equal to or Above high threshold level"]
    AboveThreshold = 1,
}
impl From<VBATHR> for bool {
    #[inline(always)]
    fn from(variant: VBATHR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATH` reader - V&lt;sub>BAT&lt;/sub> level monitoring versus high threshold"]
pub type VBATH_R = crate::BitReader<VBATHR>;
impl VBATH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATHR {
        match self.bits {
            false => VBATHR::BelowThreshold,
            true => VBATHR::AboveThreshold,
        }
    }
    #[doc = "Below high threshold level"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == VBATHR::BelowThreshold
    }
    #[doc = "Equal to or Above high threshold level"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == VBATHR::AboveThreshold
    }
}
#[doc = "Field `TEMPH` reader - temperature level monitoring versus high threshold"]
pub use VBATH_R as TEMPH_R;
#[doc = "Field `TEMPL` reader - temperature level monitoring versus low threshold"]
pub use VBATL_R as TEMPL_R;
impl R {
    #[doc = "Bit 16 - backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - V&lt;sub>BAT&lt;/sub> level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - V&lt;sub>BAT&lt;/sub> level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "PWR Backup domain status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDSRrs;
impl crate::RegisterSpec for BDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdsr::R`](R) reader structure"]
impl crate::Readable for BDSRrs {}
#[doc = "`reset()` method sets BDSR to value 0"]
impl crate::Resettable for BDSRrs {
    const RESET_VALUE: u32 = 0;
}
