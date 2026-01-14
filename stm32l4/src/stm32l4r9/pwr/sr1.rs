///Register `SR1` reader
pub type R = crate::R<SR1rs>;
/**Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1 {
    ///0: This bit is set when a wakeup event is detected on wakeup pin, WKUPx
    Set = 0,
    ///1: No wakeup event detected on WKUPx
    Cleared = 1,
}
impl From<WUF1> for bool {
    #[inline(always)]
    fn from(variant: WUF1) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF1` reader - Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register.
pub type WUF1_R = crate::BitReader<WUF1>;
impl WUF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUF1 {
        match self.bits {
            false => WUF1::Set,
            true => WUF1::Cleared,
        }
    }
    ///This bit is set when a wakeup event is detected on wakeup pin, WKUPx
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUF1::Set
    }
    ///No wakeup event detected on WKUPx
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUF1::Cleared
    }
}
///Field `WUF2` reader - Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register.
pub use WUF1_R as WUF2_R;
///Field `WUF3` reader - Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register.
pub use WUF1_R as WUF3_R;
///Field `WUF4` reader - Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register.
pub use WUF1_R as WUF4_R;
///Field `WUF5` reader - Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register.
pub use WUF1_R as WUF5_R;
/**Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBF {
    ///0: The device did not enter the Standby mode
    Set = 0,
    ///1: The device entered the Standby mode
    Cleared = 1,
}
impl From<SBF> for bool {
    #[inline(always)]
    fn from(variant: SBF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.
pub type SBF_R = crate::BitReader<SBF>;
impl SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBF {
        match self.bits {
            false => SBF::Set,
            true => SBF::Cleared,
        }
    }
    ///The device did not enter the Standby mode
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SBF::Set
    }
    ///The device entered the Standby mode
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == SBF::Cleared
    }
}
/**External SMPS ready This bit informs the state of regulator transition from Range 1 to Range 2 Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_SMPS_RDY {
    ///0: Internal regulator not ready in Range 2, the external SMPS cannot be connected
    NotReady = 0,
    ///1: Internal regulator ready in Range 2, the external SMPS can be connected
    Ready = 1,
}
impl From<EXT_SMPS_RDY> for bool {
    #[inline(always)]
    fn from(variant: EXT_SMPS_RDY) -> Self {
        variant as u8 != 0
    }
}
///Field `EXT_SMPS_RDY` reader - External SMPS ready This bit informs the state of regulator transition from Range 1 to Range 2 Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
pub type EXT_SMPS_RDY_R = crate::BitReader<EXT_SMPS_RDY>;
impl EXT_SMPS_RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EXT_SMPS_RDY {
        match self.bits {
            false => EXT_SMPS_RDY::NotReady,
            true => EXT_SMPS_RDY::Ready,
        }
    }
    ///Internal regulator not ready in Range 2, the external SMPS cannot be connected
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == EXT_SMPS_RDY::NotReady
    }
    ///Internal regulator ready in Range 2, the external SMPS can be connected
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXT_SMPS_RDY::Ready
    }
}
/**Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFI {
    ///0: This bit is set when a wakeup is detected on the internal wakeup line
    Set = 0,
    ///1: It is cleared when all internal wakeup sources are cleared
    Cleared = 1,
}
impl From<WUFI> for bool {
    #[inline(always)]
    fn from(variant: WUFI) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFI` reader - Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared.
pub type WUFI_R = crate::BitReader<WUFI>;
impl WUFI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUFI {
        match self.bits {
            false => WUFI::Set,
            true => WUFI::Cleared,
        }
    }
    ///This bit is set when a wakeup is detected on the internal wakeup line
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUFI::Set
    }
    ///It is cleared when all internal wakeup sources are cleared
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        *self == WUFI::Cleared
    }
}
impl R {
    ///Bit 0 - Wakeup flag 1 This bit is set when a wakeup event is detected on wakeup pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup flag 2 This bit is set when a wakeup event is detected on wakeup pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup flag 3 This bit is set when a wakeup event is detected on wakeup pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup flag 4 This bit is set when a wakeup event is detected on wakeup pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup flag 5 This bit is set when a wakeup event is detected on wakeup pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register.
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset.
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 13 - External SMPS ready This bit informs the state of regulator transition from Range 1 to Range 2 Note: This bit is only available on STM32L4P5xx and STM32L4Q5xx devices.
    #[inline(always)]
    pub fn ext_smps_rdy(&self) -> EXT_SMPS_RDY_R {
        EXT_SMPS_RDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Wakeup flag internal This bit is set when a wakeup is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared.
    #[inline(always)]
    pub fn wufi(&self) -> WUFI_R {
        WUFI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("wuf1", &self.wuf1())
            .field("wuf2", &self.wuf2())
            .field("wuf3", &self.wuf3())
            .field("wuf4", &self.wuf4())
            .field("wuf5", &self.wuf5())
            .field("sbf", &self.sbf())
            .field("ext_smps_rdy", &self.ext_smps_rdy())
            .field("wufi", &self.wufi())
            .finish()
    }
}
/**Power status register 1

You can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#PWR:SR1)*/
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
///`read()` method returns [`sr1::R`](R) reader structure
impl crate::Readable for SR1rs {}
///`reset()` method sets SR1 to value 0
impl crate::Resettable for SR1rs {}
