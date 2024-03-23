#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Field `C2BOOTS` reader - PU2 boot/wakeup request source information"]
pub type C2BOOTS_R = crate::BitReader;
#[doc = "Radio BUSY signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYS {
    #[doc = "0: radio busy signal low (not busy)"]
    NotBusy = 0,
    #[doc = "1: radio busy signal high (busy)"]
    Busy = 1,
}
impl From<RFBUSYS> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFBUSYS` reader - Radio BUSY signal status"]
pub type RFBUSYS_R = crate::BitReader<RFBUSYS>;
impl RFBUSYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFBUSYS {
        match self.bits {
            false => RFBUSYS::NotBusy,
            true => RFBUSYS::Busy,
        }
    }
    #[doc = "radio busy signal low (not busy)"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYS::NotBusy
    }
    #[doc = "radio busy signal high (busy)"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYS::Busy
    }
}
#[doc = "Radio BUSY masked signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYMS {
    #[doc = "0: radio busy masked signal low (not busy)"]
    NotBusy = 0,
    #[doc = "1: radio busy masked signal high (busy)"]
    Busy = 1,
}
impl From<RFBUSYMS> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYMS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFBUSYMS` reader - Radio BUSY masked signal status"]
pub type RFBUSYMS_R = crate::BitReader<RFBUSYMS>;
impl RFBUSYMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFBUSYMS {
        match self.bits {
            false => RFBUSYMS::NotBusy,
            true => RFBUSYMS::Busy,
        }
    }
    #[doc = "radio busy masked signal low (not busy)"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYMS::NotBusy
    }
    #[doc = "radio busy masked signal high (busy)"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYMS::Busy
    }
}
#[doc = "SMPS ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSRDY {
    #[doc = "0: SMPS step-down converter not ready or off"]
    NotReady = 0,
    #[doc = "1: SMPS step-down converter ready"]
    Ready = 1,
}
impl From<SMPSRDY> for bool {
    #[inline(always)]
    fn from(variant: SMPSRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSRDY` reader - SMPS ready flag"]
pub type SMPSRDY_R = crate::BitReader<SMPSRDY>;
impl SMPSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMPSRDY {
        match self.bits {
            false => SMPSRDY::NotReady,
            true => SMPSRDY::Ready,
        }
    }
    #[doc = "SMPS step-down converter not ready or off"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == SMPSRDY::NotReady
    }
    #[doc = "SMPS step-down converter ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SMPSRDY::Ready
    }
}
#[doc = "LDO ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDORDY {
    #[doc = "0: LDO not ready or off"]
    NotReady = 0,
    #[doc = "1: LDO ready"]
    Ready = 1,
}
impl From<LDORDY> for bool {
    #[inline(always)]
    fn from(variant: LDORDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDORDY` reader - LDO ready flag"]
pub type LDORDY_R = crate::BitReader<LDORDY>;
impl LDORDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDORDY {
        match self.bits {
            false => LDORDY::NotReady,
            true => LDORDY::Ready,
        }
    }
    #[doc = "LDO not ready or off"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LDORDY::NotReady
    }
    #[doc = "LDO ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LDORDY::Ready
    }
}
#[doc = "Radio end of life flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLF {
    #[doc = "0: Supply voltage above radio end-of-life operating low level"]
    Above = 0,
    #[doc = "1: Supply voltage below radio end-of-life operating low level"]
    Below = 1,
}
impl From<RFEOLF> for bool {
    #[inline(always)]
    fn from(variant: RFEOLF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEOLF` reader - Radio end of life flag"]
pub type RFEOLF_R = crate::BitReader<RFEOLF>;
impl RFEOLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFEOLF {
        match self.bits {
            false => RFEOLF::Above,
            true => RFEOLF::Below,
        }
    }
    #[doc = "Supply voltage above radio end-of-life operating low level"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RFEOLF::Above
    }
    #[doc = "Supply voltage below radio end-of-life operating low level"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RFEOLF::Below
    }
}
#[doc = "regulator2 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMRS {
    #[doc = "0: Main regulator supplied directly from VDD"]
    VDd = 0,
    #[doc = "1: Main regulator supplied through LDO or SMPS"]
    LdoSmps = 1,
}
impl From<REGMRS> for bool {
    #[inline(always)]
    fn from(variant: REGMRS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGMRS` reader - regulator2 low power flag"]
pub type REGMRS_R = crate::BitReader<REGMRS>;
impl REGMRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGMRS {
        match self.bits {
            false => REGMRS::VDd,
            true => REGMRS::LdoSmps,
        }
    }
    #[doc = "Main regulator supplied directly from VDD"]
    #[inline(always)]
    pub fn is_v_dd(&self) -> bool {
        *self == REGMRS::VDd
    }
    #[doc = "Main regulator supplied through LDO or SMPS"]
    #[inline(always)]
    pub fn is_ldo_smps(&self) -> bool {
        *self == REGMRS::LdoSmps
    }
}
#[doc = "Flash ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRDY {
    #[doc = "0: Flash memory not ready to be accessed"]
    NotReady = 0,
    #[doc = "1: Flash memory ready to be accessed"]
    Ready = 1,
}
impl From<FLASHRDY> for bool {
    #[inline(always)]
    fn from(variant: FLASHRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHRDY` reader - Flash ready"]
pub type FLASHRDY_R = crate::BitReader<FLASHRDY>;
impl FLASHRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHRDY {
        match self.bits {
            false => FLASHRDY::NotReady,
            true => FLASHRDY::Ready,
        }
    }
    #[doc = "Flash memory not ready to be accessed"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == FLASHRDY::NotReady
    }
    #[doc = "Flash memory ready to be accessed"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == FLASHRDY::Ready
    }
}
#[doc = "regulator1 started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS {
    #[doc = "0: LPR not ready"]
    NotReady = 0,
    #[doc = "1: LPR ready"]
    Ready = 1,
}
impl From<REGLPS> for bool {
    #[inline(always)]
    fn from(variant: REGLPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPS` reader - regulator1 started"]
pub type REGLPS_R = crate::BitReader<REGLPS>;
impl REGLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGLPS {
        match self.bits {
            false => REGLPS::NotReady,
            true => REGLPS::Ready,
        }
    }
    #[doc = "LPR not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS::NotReady
    }
    #[doc = "LPR ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS::Ready
    }
}
#[doc = "regulator1 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF {
    #[doc = "0: Main regulator (MR) ready and used"]
    Main = 0,
    #[doc = "1: Low-power regulator (LPR) used"]
    LowPower = 1,
}
impl From<REGLPF> for bool {
    #[inline(always)]
    fn from(variant: REGLPF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPF` reader - regulator1 low power flag"]
pub type REGLPF_R = crate::BitReader<REGLPF>;
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REGLPF {
        match self.bits {
            false => REGLPF::Main,
            true => REGLPF::LowPower,
        }
    }
    #[doc = "Main regulator (MR) ready and used"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == REGLPF::Main
    }
    #[doc = "Low-power regulator (LPR) used"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == REGLPF::LowPower
    }
}
#[doc = "Voltage scaling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF {
    #[doc = "0: Regulator ready in the selected voltage range"]
    Ready = 0,
    #[doc = "1: Regulator output voltage changed to the required voltage level"]
    Change = 1,
}
impl From<VOSF> for bool {
    #[inline(always)]
    fn from(variant: VOSF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VOSF_R = crate::BitReader<VOSF>;
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOSF {
        match self.bits {
            false => VOSF::Ready,
            true => VOSF::Change,
        }
    }
    #[doc = "Regulator ready in the selected voltage range"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF::Ready
    }
    #[doc = "Regulator output voltage changed to the required voltage level"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == VOSF::Change
    }
}
#[doc = "Power voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO {
    #[doc = "0: VDD or voltage level on PVD_IN above the selected PVD threshold"]
    Above = 0,
    #[doc = "1: VDD or voltage level on PVD_IN below the selected PVD threshold"]
    Below = 1,
}
impl From<PVDO> for bool {
    #[inline(always)]
    fn from(variant: PVDO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub type PVDO_R = crate::BitReader<PVDO>;
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDO {
        match self.bits {
            false => PVDO::Above,
            true => PVDO::Below,
        }
    }
    #[doc = "VDD or voltage level on PVD_IN above the selected PVD threshold"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO::Above
    }
    #[doc = "VDD or voltage level on PVD_IN below the selected PVD threshold"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO::Below
    }
}
#[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3 {
    #[doc = "0: VDDA voltage above PVM3 threshold (around 1.62 V)"]
    Above = 0,
    #[doc = "1: VDDA voltage below PVM3 threshold (around 1.62 V)"]
    Below = 1,
}
impl From<PVMO3> for bool {
    #[inline(always)]
    fn from(variant: PVMO3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub type PVMO3_R = crate::BitReader<PVMO3>;
impl PVMO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMO3 {
        match self.bits {
            false => PVMO3::Above,
            true => PVMO3::Below,
        }
    }
    #[doc = "VDDA voltage above PVM3 threshold (around 1.62 V)"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3::Above
    }
    #[doc = "VDDA voltage below PVM3 threshold (around 1.62 V)"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3::Below
    }
}
impl R {
    #[doc = "Bit 0 - PU2 boot/wakeup request source information"]
    #[inline(always)]
    pub fn c2boots(&self) -> C2BOOTS_R {
        C2BOOTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Radio BUSY signal status"]
    #[inline(always)]
    pub fn rfbusys(&self) -> RFBUSYS_R {
        RFBUSYS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Radio BUSY masked signal status"]
    #[inline(always)]
    pub fn rfbusyms(&self) -> RFBUSYMS_R {
        RFBUSYMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMPS ready flag"]
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LDO ready flag"]
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Radio end of life flag"]
    #[inline(always)]
    pub fn rfeolf(&self) -> RFEOLF_R {
        RFEOLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - regulator2 low power flag"]
    #[inline(always)]
    pub fn regmrs(&self) -> REGMRS_R {
        REGMRS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash ready"]
    #[inline(always)]
    pub fn flashrdy(&self) -> FLASHRDY_R {
        FLASHRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - regulator1 started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - regulator1 low power flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
