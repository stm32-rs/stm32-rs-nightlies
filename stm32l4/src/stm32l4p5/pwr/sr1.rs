#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1rs>;
#[doc = "Wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1 {
    #[doc = "0: This bit is set when a wakeup event is detected on wakeup pin, WKUPx"]
    Set = 0,
    #[doc = "1: No wakeup event detected on WKUPx"]
    Cleared = 1,
}
impl From<WUF1> for bool {
    #[inline(always)]
    fn from(variant: WUF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF1` reader - Wakeup flag 1"]
pub type WUF1_R = crate::BitReader<WUF1>;
impl WUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF1 {
        match self.bits {
            false => WUF1::Set,
            true => WUF1::Cleared,
        }
    }
    #[doc = "This bit is set when a wakeup event is detected on wakeup pin, WKUPx"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUF1::Set
    }
    #[doc = "No wakeup event detected on WKUPx"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUF1::Cleared
    }
}
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub use WUF1_R as WUF2_R;
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub use WUF1_R as WUF3_R;
#[doc = "Field `WUF4` reader - Wakeup flag 4"]
pub use WUF1_R as WUF4_R;
#[doc = "Field `WUF5` reader - Wakeup flag 5"]
pub use WUF1_R as WUF5_R;
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF {
    #[doc = "0: The device did not enter the Standby mode"]
    Set = 0,
    #[doc = "1: The device entered the Standby mode"]
    Cleared = 1,
}
impl From<SBF> for bool {
    #[inline(always)]
    fn from(variant: SBF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<SBF>;
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBF {
        match self.bits {
            false => SBF::Set,
            true => SBF::Cleared,
        }
    }
    #[doc = "The device did not enter the Standby mode"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SBF::Set
    }
    #[doc = "The device entered the Standby mode"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SBF::Cleared
    }
}
#[doc = "External SMPS ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_SMPS_RDY {
    #[doc = "0: Internal regulator not ready in Range 2, the external SMPS cannot be connected"]
    NotReady = 0,
    #[doc = "1: Internal regulator ready in Range 2, the external SMPS can be connected"]
    Ready = 1,
}
impl From<EXT_SMPS_RDY> for bool {
    #[inline(always)]
    fn from(variant: EXT_SMPS_RDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_SMPS_RDY` reader - External SMPS ready"]
pub type EXT_SMPS_RDY_R = crate::BitReader<EXT_SMPS_RDY>;
impl EXT_SMPS_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXT_SMPS_RDY {
        match self.bits {
            false => EXT_SMPS_RDY::NotReady,
            true => EXT_SMPS_RDY::Ready,
        }
    }
    #[doc = "Internal regulator not ready in Range 2, the external SMPS cannot be connected"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == EXT_SMPS_RDY::NotReady
    }
    #[doc = "Internal regulator ready in Range 2, the external SMPS can be connected"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXT_SMPS_RDY::Ready
    }
}
#[doc = "Wakeup flag internal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI {
    #[doc = "0: This bit is set when a wakeup is detected on the internal wakeup line"]
    Set = 0,
    #[doc = "1: It is cleared when all internal wakeup sources are cleared"]
    Cleared = 1,
}
impl From<WUFI> for bool {
    #[inline(always)]
    fn from(variant: WUFI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFI` reader - Wakeup flag internal"]
pub type WUFI_R = crate::BitReader<WUFI>;
impl WUFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUFI {
        match self.bits {
            false => WUFI::Set,
            true => WUFI::Cleared,
        }
    }
    #[doc = "This bit is set when a wakeup is detected on the internal wakeup line"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUFI::Set
    }
    #[doc = "It is cleared when all internal wakeup sources are cleared"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUFI::Cleared
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag 1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag 2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag 3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag 4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag 5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - External SMPS ready"]
    #[inline(always)]
    pub fn ext_smps_rdy(&self) -> EXT_SMPS_RDY_R {
        EXT_SMPS_RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup flag internal"]
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Power status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for SR1rs {}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1rs {
    const RESET_VALUE: u32 = 0;
}
