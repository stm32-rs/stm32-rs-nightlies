#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1rs>;
#[doc = "Wakeup flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1 {
    #[doc = "0: No wakeup event detected on WKUP1"]
    Clear = 0,
    #[doc = "1: Wakeup event detected on WKUP1"]
    Wakeup = 1,
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
            false => WUF1::Clear,
            true => WUF1::Wakeup,
        }
    }
    #[doc = "No wakeup event detected on WKUP1"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF1::Clear
    }
    #[doc = "Wakeup event detected on WKUP1"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF1::Wakeup
    }
}
#[doc = "Wakeup flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF2 {
    #[doc = "0: No wakeup event detected on WKUP2"]
    Clear = 0,
    #[doc = "1: Wakeup event detected on WKUP2"]
    Wakeup = 1,
}
impl From<WUF2> for bool {
    #[inline(always)]
    fn from(variant: WUF2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF2` reader - Wakeup flag 2"]
pub type WUF2_R = crate::BitReader<WUF2>;
impl WUF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF2 {
        match self.bits {
            false => WUF2::Clear,
            true => WUF2::Wakeup,
        }
    }
    #[doc = "No wakeup event detected on WKUP2"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF2::Clear
    }
    #[doc = "Wakeup event detected on WKUP2"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF2::Wakeup
    }
}
#[doc = "Wakeup flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF3 {
    #[doc = "0: No wakeup event detected on WKUP3"]
    Clear = 0,
    #[doc = "1: Wakeup event detected on WKUP3"]
    Wakeup = 1,
}
impl From<WUF3> for bool {
    #[inline(always)]
    fn from(variant: WUF3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUF3` reader - Wakeup flag 3"]
pub type WUF3_R = crate::BitReader<WUF3>;
impl WUF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUF3 {
        match self.bits {
            false => WUF3::Clear,
            true => WUF3::Wakeup,
        }
    }
    #[doc = "No wakeup event detected on WKUP3"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF3::Clear
    }
    #[doc = "Wakeup event detected on WKUP3"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF3::Wakeup
    }
}
#[doc = "Wakeup PVD flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPVDF {
    #[doc = "0: No wakeup event detected on PVD"]
    Clear = 0,
    #[doc = "1: Wakeup event detected on PVD"]
    Wakeup = 1,
}
impl From<WPVDF> for bool {
    #[inline(always)]
    fn from(variant: WPVDF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPVDF` reader - Wakeup PVD flag"]
pub type WPVDF_R = crate::BitReader<WPVDF>;
impl WPVDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WPVDF {
        match self.bits {
            false => WPVDF::Clear,
            true => WPVDF::Wakeup,
        }
    }
    #[doc = "No wakeup event detected on PVD"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WPVDF::Clear
    }
    #[doc = "Wakeup event detected on PVD"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WPVDF::Wakeup
    }
}
#[doc = "Radio BUSY wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYF {
    #[doc = "0: No wakeup event detected on radio busy"]
    Clear = 0,
    #[doc = "1: Wakeup event detected on radio busy"]
    Wakeup = 1,
}
impl From<WRFBUSYF> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRFBUSYF` reader - Radio BUSY wakeup flag"]
pub type WRFBUSYF_R = crate::BitReader<WRFBUSYF>;
impl WRFBUSYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRFBUSYF {
        match self.bits {
            false => WRFBUSYF::Clear,
            true => WRFBUSYF::Wakeup,
        }
    }
    #[doc = "No wakeup event detected on radio busy"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WRFBUSYF::Clear
    }
    #[doc = "Wakeup event detected on radio busy"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WRFBUSYF::Wakeup
    }
}
#[doc = "Internal wakeup interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI {
    #[doc = "0: All internal wakeup sources are cleared"]
    Clear = 0,
    #[doc = "1: wakeup is detected on the internal wakeup line"]
    Wakeup = 1,
}
impl From<WUFI> for bool {
    #[inline(always)]
    fn from(variant: WUFI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFI` reader - Internal wakeup interrupt flag"]
pub type WUFI_R = crate::BitReader<WUFI>;
impl WUFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUFI {
        match self.bits {
            false => WUFI::Clear,
            true => WUFI::Wakeup,
        }
    }
    #[doc = "All internal wakeup sources are cleared"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUFI::Clear
    }
    #[doc = "wakeup is detected on the internal wakeup line"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUFI::Wakeup
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
    #[doc = "Bit 8 - Wakeup PVD flag"]
    #[inline(always)]
    pub fn wpvdf(&self) -> WPVDF_R {
        WPVDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Radio BUSY wakeup flag"]
    #[inline(always)]
    pub fn wrfbusyf(&self) -> WRFBUSYF_R {
        WRFBUSYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Internal wakeup interrupt flag"]
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
