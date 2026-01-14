///Register `SR2` reader
pub type R = crate::R<SR2rs>;
/**Radio BUSY signal status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYS {
    ///0: radio busy signal low (not busy)
    NotBusy = 0,
    ///1: radio busy signal high (busy)
    Busy = 1,
}
impl From<RFBUSYS> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYS) -> Self {
        variant as u8 != 0
    }
}
///Field `RFBUSYS` reader - Radio BUSY signal status
pub type RFBUSYS_R = crate::BitReader<RFBUSYS>;
impl RFBUSYS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFBUSYS {
        match self.bits {
            false => RFBUSYS::NotBusy,
            true => RFBUSYS::Busy,
        }
    }
    ///radio busy signal low (not busy)
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYS::NotBusy
    }
    ///radio busy signal high (busy)
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYS::Busy
    }
}
/**Radio BUSY masked signal status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYMS {
    ///0: radio busy masked signal low (not busy)
    NotBusy = 0,
    ///1: radio busy masked signal high (busy)
    Busy = 1,
}
impl From<RFBUSYMS> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYMS) -> Self {
        variant as u8 != 0
    }
}
///Field `RFBUSYMS` reader - Radio BUSY masked signal status
pub type RFBUSYMS_R = crate::BitReader<RFBUSYMS>;
impl RFBUSYMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFBUSYMS {
        match self.bits {
            false => RFBUSYMS::NotBusy,
            true => RFBUSYMS::Busy,
        }
    }
    ///radio busy masked signal low (not busy)
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYMS::NotBusy
    }
    ///radio busy masked signal high (busy)
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYMS::Busy
    }
}
/**SMPS ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSRDY {
    ///0: SMPS step-down converter not ready or off
    NotReady = 0,
    ///1: SMPS step-down converter ready
    Ready = 1,
}
impl From<SMPSRDY> for bool {
    #[inline(always)]
    fn from(variant: SMPSRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `SMPSRDY` reader - SMPS ready flag
pub type SMPSRDY_R = crate::BitReader<SMPSRDY>;
impl SMPSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMPSRDY {
        match self.bits {
            false => SMPSRDY::NotReady,
            true => SMPSRDY::Ready,
        }
    }
    ///SMPS step-down converter not ready or off
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == SMPSRDY::NotReady
    }
    ///SMPS step-down converter ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SMPSRDY::Ready
    }
}
/**LDO ready flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDORDY {
    ///0: LDO not ready or off
    NotReady = 0,
    ///1: LDO ready
    Ready = 1,
}
impl From<LDORDY> for bool {
    #[inline(always)]
    fn from(variant: LDORDY) -> Self {
        variant as u8 != 0
    }
}
///Field `LDORDY` reader - LDO ready flag
pub type LDORDY_R = crate::BitReader<LDORDY>;
impl LDORDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDORDY {
        match self.bits {
            false => LDORDY::NotReady,
            true => LDORDY::Ready,
        }
    }
    ///LDO not ready or off
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LDORDY::NotReady
    }
    ///LDO ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LDORDY::Ready
    }
}
/**Radio end of life flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLF {
    ///0: Supply voltage above radio end-of-life operating low level
    Above = 0,
    ///1: Supply voltage below radio end-of-life operating low level
    Below = 1,
}
impl From<RFEOLF> for bool {
    #[inline(always)]
    fn from(variant: RFEOLF) -> Self {
        variant as u8 != 0
    }
}
///Field `RFEOLF` reader - Radio end of life flag
pub type RFEOLF_R = crate::BitReader<RFEOLF>;
impl RFEOLF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RFEOLF {
        match self.bits {
            false => RFEOLF::Above,
            true => RFEOLF::Below,
        }
    }
    ///Supply voltage above radio end-of-life operating low level
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RFEOLF::Above
    }
    ///Supply voltage below radio end-of-life operating low level
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RFEOLF::Below
    }
}
/**regulator2 low power flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMRS {
    ///0: Main regulator supplied directly from VDD
    VDd = 0,
    ///1: Main regulator supplied through LDO or SMPS
    LdoSmps = 1,
}
impl From<REGMRS> for bool {
    #[inline(always)]
    fn from(variant: REGMRS) -> Self {
        variant as u8 != 0
    }
}
///Field `REGMRS` reader - regulator2 low power flag
pub type REGMRS_R = crate::BitReader<REGMRS>;
impl REGMRS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGMRS {
        match self.bits {
            false => REGMRS::VDd,
            true => REGMRS::LdoSmps,
        }
    }
    ///Main regulator supplied directly from VDD
    #[inline(always)]
    pub fn is_v_dd(&self) -> bool {
        *self == REGMRS::VDd
    }
    ///Main regulator supplied through LDO or SMPS
    #[inline(always)]
    pub fn is_ldo_smps(&self) -> bool {
        *self == REGMRS::LdoSmps
    }
}
/**Flash ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRDY {
    ///0: Flash memory not ready to be accessed
    NotReady = 0,
    ///1: Flash memory ready to be accessed
    Ready = 1,
}
impl From<FLASHRDY> for bool {
    #[inline(always)]
    fn from(variant: FLASHRDY) -> Self {
        variant as u8 != 0
    }
}
///Field `FLASHRDY` reader - Flash ready
pub type FLASHRDY_R = crate::BitReader<FLASHRDY>;
impl FLASHRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FLASHRDY {
        match self.bits {
            false => FLASHRDY::NotReady,
            true => FLASHRDY::Ready,
        }
    }
    ///Flash memory not ready to be accessed
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == FLASHRDY::NotReady
    }
    ///Flash memory ready to be accessed
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == FLASHRDY::Ready
    }
}
/**regulator1 started

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS {
    ///0: LPR not ready
    NotReady = 0,
    ///1: LPR ready
    Ready = 1,
}
impl From<REGLPS> for bool {
    #[inline(always)]
    fn from(variant: REGLPS) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPS` reader - regulator1 started
pub type REGLPS_R = crate::BitReader<REGLPS>;
impl REGLPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGLPS {
        match self.bits {
            false => REGLPS::NotReady,
            true => REGLPS::Ready,
        }
    }
    ///LPR not ready
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS::NotReady
    }
    ///LPR ready
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS::Ready
    }
}
/**regulator1 low power flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF {
    ///0: Main regulator (MR) ready and used
    Main = 0,
    ///1: Low-power regulator (LPR) used
    LowPower = 1,
}
impl From<REGLPF> for bool {
    #[inline(always)]
    fn from(variant: REGLPF) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPF` reader - regulator1 low power flag
pub type REGLPF_R = crate::BitReader<REGLPF>;
impl REGLPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGLPF {
        match self.bits {
            false => REGLPF::Main,
            true => REGLPF::LowPower,
        }
    }
    ///Main regulator (MR) ready and used
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == REGLPF::Main
    }
    ///Low-power regulator (LPR) used
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == REGLPF::LowPower
    }
}
/**Voltage scaling flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF {
    ///0: Regulator ready in the selected voltage range
    Ready = 0,
    ///1: Regulator output voltage changed to the required voltage level
    Change = 1,
}
impl From<VOSF> for bool {
    #[inline(always)]
    fn from(variant: VOSF) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSF` reader - Voltage scaling flag
pub type VOSF_R = crate::BitReader<VOSF>;
impl VOSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSF {
        match self.bits {
            false => VOSF::Ready,
            true => VOSF::Change,
        }
    }
    ///Regulator ready in the selected voltage range
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF::Ready
    }
    ///Regulator output voltage changed to the required voltage level
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == VOSF::Change
    }
}
/**Power voltage detector output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    ///0: VDD or voltage level on PVD_IN above the selected PVD threshold
    Above = 0,
    ///1: VDD or voltage level on PVD_IN below the selected PVD threshold
    Below = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::Above,
            true => PVDO::Below,
        }
    }
    ///VDD or voltage level on PVD_IN above the selected PVD threshold
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO::Above
    }
    ///VDD or voltage level on PVD_IN below the selected PVD threshold
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO::Below
    }
}
/**Peripheral voltage monitoring output: VDDA vs. 1.62 V

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3 {
    ///0: VDDA voltage above PVM3 threshold (around 1.62 V)
    Above = 0,
    ///1: VDDA voltage below PVM3 threshold (around 1.62 V)
    Below = 1,
}
impl From<PVMO3> for bool {
    #[inline(always)]
    fn from(variant: PVMO3) -> Self {
        variant as u8 != 0
    }
}
///Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V
pub type PVMO3_R = crate::BitReader<PVMO3>;
impl PVMO3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVMO3 {
        match self.bits {
            false => PVMO3::Above,
            true => PVMO3::Below,
        }
    }
    ///VDDA voltage above PVM3 threshold (around 1.62 V)
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3::Above
    }
    ///VDDA voltage below PVM3 threshold (around 1.62 V)
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3::Below
    }
}
impl R {
    ///Bit 1 - Radio BUSY signal status
    #[inline(always)]
    pub fn rfbusys(&self) -> RFBUSYS_R {
        RFBUSYS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Radio BUSY masked signal status
    #[inline(always)]
    pub fn rfbusyms(&self) -> RFBUSYMS_R {
        RFBUSYMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SMPS ready flag
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LDO ready flag
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Radio end of life flag
    #[inline(always)]
    pub fn rfeolf(&self) -> RFEOLF_R {
        RFEOLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - regulator2 low power flag
    #[inline(always)]
    pub fn regmrs(&self) -> REGMRS_R {
        REGMRS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Flash ready
    #[inline(always)]
    pub fn flashrdy(&self) -> FLASHRDY_R {
        FLASHRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - regulator1 started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - regulator1 low power flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("pvmo3", &self.pvmo3())
            .field("pvdo", &self.pvdo())
            .field("vosf", &self.vosf())
            .field("reglpf", &self.reglpf())
            .field("reglps", &self.reglps())
            .field("flashrdy", &self.flashrdy())
            .field("regmrs", &self.regmrs())
            .field("rfeolf", &self.rfeolf())
            .field("ldordy", &self.ldordy())
            .field("smpsrdy", &self.smpsrdy())
            .field("rfbusyms", &self.rfbusyms())
            .field("rfbusys", &self.rfbusys())
            .finish()
    }
}
/**Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
