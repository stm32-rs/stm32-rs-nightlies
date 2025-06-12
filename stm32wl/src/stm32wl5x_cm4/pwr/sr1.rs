///Register `SR1` reader
pub type R = crate::R<SR1rs>;
/**Wakeup flag 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1 {
    ///0: No wakeup event detected on WKUP1
    Clear = 0,
    ///1: Wakeup event detected on WKUP1
    Wakeup = 1,
}
impl From<WUF1> for bool {
    #[inline(always)]
    fn from(variant: WUF1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF1` reader - Wakeup flag 1
pub type WUF1_R = crate::BitReader<WUF1>;
impl WUF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF1 {
        match self.bits {
            false => WUF1::Clear,
            true => WUF1::Wakeup,
        }
    }
    ///No wakeup event detected on WKUP1
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF1::Clear
    }
    ///Wakeup event detected on WKUP1
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF1::Wakeup
    }
}
/**Wakeup flag 2

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF2 {
    ///0: No wakeup event detected on WKUP2
    Clear = 0,
    ///1: Wakeup event detected on WKUP2
    Wakeup = 1,
}
impl From<WUF2> for bool {
    #[inline(always)]
    fn from(variant: WUF2) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF2` reader - Wakeup flag 2
pub type WUF2_R = crate::BitReader<WUF2>;
impl WUF2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF2 {
        match self.bits {
            false => WUF2::Clear,
            true => WUF2::Wakeup,
        }
    }
    ///No wakeup event detected on WKUP2
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF2::Clear
    }
    ///Wakeup event detected on WKUP2
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF2::Wakeup
    }
}
/**Wakeup flag 3

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF3 {
    ///0: No wakeup event detected on WKUP3
    Clear = 0,
    ///1: Wakeup event detected on WKUP3
    Wakeup = 1,
}
impl From<WUF3> for bool {
    #[inline(always)]
    fn from(variant: WUF3) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF3` reader - Wakeup flag 3
pub type WUF3_R = crate::BitReader<WUF3>;
impl WUF3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF3 {
        match self.bits {
            false => WUF3::Clear,
            true => WUF3::Wakeup,
        }
    }
    ///No wakeup event detected on WKUP3
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUF3::Clear
    }
    ///Wakeup event detected on WKUP3
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUF3::Wakeup
    }
}
/**Wakeup PVD flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPVDF {
    ///0: No wakeup event detected on PVD
    Clear = 0,
    ///1: Wakeup event detected on PVD
    Wakeup = 1,
}
impl From<WPVDF> for bool {
    #[inline(always)]
    fn from(variant: WPVDF) -> Self {
        variant as u8 != 0
    }
}
///Field `WPVDF` reader - Wakeup PVD flag
pub type WPVDF_R = crate::BitReader<WPVDF>;
impl WPVDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WPVDF {
        match self.bits {
            false => WPVDF::Clear,
            true => WPVDF::Wakeup,
        }
    }
    ///No wakeup event detected on PVD
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WPVDF::Clear
    }
    ///Wakeup event detected on PVD
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WPVDF::Wakeup
    }
}
/**Radio BUSY wakeup flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYF {
    ///0: No wakeup event detected on radio busy
    Clear = 0,
    ///1: Wakeup event detected on radio busy
    Wakeup = 1,
}
impl From<WRFBUSYF> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYF) -> Self {
        variant as u8 != 0
    }
}
///Field `WRFBUSYF` reader - Radio BUSY wakeup flag
pub type WRFBUSYF_R = crate::BitReader<WRFBUSYF>;
impl WRFBUSYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRFBUSYF {
        match self.bits {
            false => WRFBUSYF::Clear,
            true => WRFBUSYF::Wakeup,
        }
    }
    ///No wakeup event detected on radio busy
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WRFBUSYF::Clear
    }
    ///Wakeup event detected on radio busy
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WRFBUSYF::Wakeup
    }
}
///Field `C2HF` reader - PU2 Hold interrupt flag
pub type C2HF_R = crate::BitReader;
/**Internal wakeup interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI {
    ///0: All internal wakeup sources are cleared
    Clear = 0,
    ///1: wakeup is detected on the internal wakeup line
    Wakeup = 1,
}
impl From<WUFI> for bool {
    #[inline(always)]
    fn from(variant: WUFI) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFI` reader - Internal wakeup interrupt flag
pub type WUFI_R = crate::BitReader<WUFI>;
impl WUFI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUFI {
        match self.bits {
            false => WUFI::Clear,
            true => WUFI::Wakeup,
        }
    }
    ///All internal wakeup sources are cleared
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WUFI::Clear
    }
    ///wakeup is detected on the internal wakeup line
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WUFI::Wakeup
    }
}
impl R {
    ///Bit 0 - Wakeup flag 1
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Wakeup PVD flag
    #[inline(always)]
    pub fn wpvdf(&self) -> WPVDF_R {
        WPVDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Radio BUSY wakeup flag
    #[inline(always)]
    pub fn wrfbusyf(&self) -> WRFBUSYF_R {
        WRFBUSYF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - PU2 Hold interrupt flag
    #[inline(always)]
    pub fn c2hf(&self) -> C2HF_R {
        C2HF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Internal wakeup interrupt flag
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("wufi", &self.wufi())
            .field("c2hf", &self.c2hf())
            .field("wrfbusyf", &self.wrfbusyf())
            .field("wpvdf", &self.wpvdf())
            .field("wuf3", &self.wuf3())
            .field("wuf2", &self.wuf2())
            .field("wuf1", &self.wuf1())
            .finish()
    }
}
/**Power status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#PWR:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
