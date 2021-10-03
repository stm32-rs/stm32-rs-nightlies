#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVMO3_A {
    #[doc = "0: VDDA voltage above PVM3 threshold (around 1.62 V)"]
    ABOVE = 0,
    #[doc = "1: VDDA voltage below PVM3 threshold (around 1.62 V)"]
    BELOW = 1,
}
impl From<PVMO3_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub struct PVMO3_R(crate::FieldReader<bool, PVMO3_A>);
impl PVMO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMO3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMO3_A {
        match self.bits {
            false => PVMO3_A::ABOVE,
            true => PVMO3_A::BELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == PVMO3_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == PVMO3_A::BELOW
    }
}
impl core::ops::Deref for PVMO3_R {
    type Target = crate::FieldReader<bool, PVMO3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Power voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDO_A {
    #[doc = "0: VDD or voltage level on PVD_IN above the selected PVD threshold"]
    ABOVE = 0,
    #[doc = "1: VDD or voltage level on PVD_IN below the selected PVD threshold"]
    BELOW = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub struct PVDO_R(crate::FieldReader<bool, PVDO_A>);
impl PVDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::ABOVE,
            true => PVDO_A::BELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == PVDO_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == PVDO_A::BELOW
    }
}
impl core::ops::Deref for PVDO_R {
    type Target = crate::FieldReader<bool, PVDO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Voltage scaling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOSF_A {
    #[doc = "0: Regulator ready in the selected voltage range"]
    READY = 0,
    #[doc = "1: Regulator output voltage changed to the required voltage level"]
    CHANGE = 1,
}
impl From<VOSF_A> for bool {
    #[inline(always)]
    fn from(variant: VOSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub struct VOSF_R(crate::FieldReader<bool, VOSF_A>);
impl VOSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOSF_A {
        match self.bits {
            false => VOSF_A::READY,
            true => VOSF_A::CHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == VOSF_A::READY
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        **self == VOSF_A::CHANGE
    }
}
impl core::ops::Deref for VOSF_R {
    type Target = crate::FieldReader<bool, VOSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "regulator1 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGLPF_A {
    #[doc = "0: Main regulator (MR) ready and used"]
    MAIN = 0,
    #[doc = "1: Low-power regulator (LPR) used"]
    LOWPOWER = 1,
}
impl From<REGLPF_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPF` reader - regulator1 low power flag"]
pub struct REGLPF_R(crate::FieldReader<bool, REGLPF_A>);
impl REGLPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGLPF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPF_A {
        match self.bits {
            false => REGLPF_A::MAIN,
            true => REGLPF_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        **self == REGLPF_A::MAIN
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == REGLPF_A::LOWPOWER
    }
}
impl core::ops::Deref for REGLPF_R {
    type Target = crate::FieldReader<bool, REGLPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "regulator1 started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGLPS_A {
    #[doc = "0: LPR not ready"]
    NOTREADY = 0,
    #[doc = "1: LPR ready"]
    READY = 1,
}
impl From<REGLPS_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGLPS` reader - regulator1 started"]
pub struct REGLPS_R(crate::FieldReader<bool, REGLPS_A>);
impl REGLPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGLPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPS_A {
        match self.bits {
            false => REGLPS_A::NOTREADY,
            true => REGLPS_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == REGLPS_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == REGLPS_A::READY
    }
}
impl core::ops::Deref for REGLPS_R {
    type Target = crate::FieldReader<bool, REGLPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHRDY_A {
    #[doc = "0: Flash memory not ready to be accessed"]
    NOTREADY = 0,
    #[doc = "1: Flash memory ready to be accessed"]
    READY = 1,
}
impl From<FLASHRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHRDY` reader - Flash ready"]
pub struct FLASHRDY_R(crate::FieldReader<bool, FLASHRDY_A>);
impl FLASHRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHRDY_A {
        match self.bits {
            false => FLASHRDY_A::NOTREADY,
            true => FLASHRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == FLASHRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == FLASHRDY_A::READY
    }
}
impl core::ops::Deref for FLASHRDY_R {
    type Target = crate::FieldReader<bool, FLASHRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "regulator2 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGMRS_A {
    #[doc = "0: Main regulator supplied directly from VDD"]
    V_DD = 0,
    #[doc = "1: Main regulator supplied through LDO or SMPS"]
    LDO_SMPS = 1,
}
impl From<REGMRS_A> for bool {
    #[inline(always)]
    fn from(variant: REGMRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGMRS` reader - regulator2 low power flag"]
pub struct REGMRS_R(crate::FieldReader<bool, REGMRS_A>);
impl REGMRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGMRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMRS_A {
        match self.bits {
            false => REGMRS_A::V_DD,
            true => REGMRS_A::LDO_SMPS,
        }
    }
    #[doc = "Checks if the value of the field is `V_DD`"]
    #[inline(always)]
    pub fn is_v_dd(&self) -> bool {
        **self == REGMRS_A::V_DD
    }
    #[doc = "Checks if the value of the field is `LDO_SMPS`"]
    #[inline(always)]
    pub fn is_ldo_smps(&self) -> bool {
        **self == REGMRS_A::LDO_SMPS
    }
}
impl core::ops::Deref for REGMRS_R {
    type Target = crate::FieldReader<bool, REGMRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Radio end of life flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEOLF_A {
    #[doc = "0: Supply voltage above radio end-of-life operating low level"]
    ABOVE = 0,
    #[doc = "1: Supply voltage below radio end-of-life operating low level"]
    BELOW = 1,
}
impl From<RFEOLF_A> for bool {
    #[inline(always)]
    fn from(variant: RFEOLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEOLF` reader - Radio end of life flag"]
pub struct RFEOLF_R(crate::FieldReader<bool, RFEOLF_A>);
impl RFEOLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFEOLF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEOLF_A {
        match self.bits {
            false => RFEOLF_A::ABOVE,
            true => RFEOLF_A::BELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        **self == RFEOLF_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        **self == RFEOLF_A::BELOW
    }
}
impl core::ops::Deref for RFEOLF_R {
    type Target = crate::FieldReader<bool, RFEOLF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LDO ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDORDY_A {
    #[doc = "0: LDO not ready or off"]
    NOTREADY = 0,
    #[doc = "1: LDO ready"]
    READY = 1,
}
impl From<LDORDY_A> for bool {
    #[inline(always)]
    fn from(variant: LDORDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDORDY` reader - LDO ready flag"]
pub struct LDORDY_R(crate::FieldReader<bool, LDORDY_A>);
impl LDORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDORDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDORDY_A {
        match self.bits {
            false => LDORDY_A::NOTREADY,
            true => LDORDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == LDORDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LDORDY_A::READY
    }
}
impl core::ops::Deref for LDORDY_R {
    type Target = crate::FieldReader<bool, LDORDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "SMPS ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPSRDY_A {
    #[doc = "0: SMPS step-down converter not ready or off"]
    NOTREADY = 0,
    #[doc = "1: SMPS step-down converter ready"]
    READY = 1,
}
impl From<SMPSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPSRDY` reader - SMPS ready flag"]
pub struct SMPSRDY_R(crate::FieldReader<bool, SMPSRDY_A>);
impl SMPSRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSRDY_A {
        match self.bits {
            false => SMPSRDY_A::NOTREADY,
            true => SMPSRDY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == SMPSRDY_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == SMPSRDY_A::READY
    }
}
impl core::ops::Deref for SMPSRDY_R {
    type Target = crate::FieldReader<bool, SMPSRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Radio BUSY masked signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFBUSYMS_A {
    #[doc = "0: radio busy masked signal low (not busy)"]
    NOTBUSY = 0,
    #[doc = "1: radio busy masked signal high (busy)"]
    BUSY = 1,
}
impl From<RFBUSYMS_A> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFBUSYMS` reader - Radio BUSY masked signal status"]
pub struct RFBUSYMS_R(crate::FieldReader<bool, RFBUSYMS_A>);
impl RFBUSYMS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFBUSYMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFBUSYMS_A {
        match self.bits {
            false => RFBUSYMS_A::NOTBUSY,
            true => RFBUSYMS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        **self == RFBUSYMS_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == RFBUSYMS_A::BUSY
    }
}
impl core::ops::Deref for RFBUSYMS_R {
    type Target = crate::FieldReader<bool, RFBUSYMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Radio BUSY signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFBUSYS_A {
    #[doc = "0: radio busy signal low (not busy)"]
    NOTBUSY = 0,
    #[doc = "1: radio busy signal high (busy)"]
    BUSY = 1,
}
impl From<RFBUSYS_A> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFBUSYS` reader - Radio BUSY signal status"]
pub struct RFBUSYS_R(crate::FieldReader<bool, RFBUSYS_A>);
impl RFBUSYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFBUSYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFBUSYS_A {
        match self.bits {
            false => RFBUSYS_A::NOTBUSY,
            true => RFBUSYS_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        **self == RFBUSYS_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == RFBUSYS_A::BUSY
    }
}
impl core::ops::Deref for RFBUSYS_R {
    type Target = crate::FieldReader<bool, RFBUSYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2BOOTS` reader - PU2 boot/wakeup request source information"]
pub struct C2BOOTS_R(crate::FieldReader<bool, bool>);
impl C2BOOTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2BOOTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2BOOTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - regulator1 low power flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - regulator1 started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash ready"]
    #[inline(always)]
    pub fn flashrdy(&self) -> FLASHRDY_R {
        FLASHRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - regulator2 low power flag"]
    #[inline(always)]
    pub fn regmrs(&self) -> REGMRS_R {
        REGMRS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Radio end of life flag"]
    #[inline(always)]
    pub fn rfeolf(&self) -> RFEOLF_R {
        RFEOLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LDO ready flag"]
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMPS ready flag"]
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Radio BUSY masked signal status"]
    #[inline(always)]
    pub fn rfbusyms(&self) -> RFBUSYMS_R {
        RFBUSYMS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Radio BUSY signal status"]
    #[inline(always)]
    pub fn rfbusys(&self) -> RFBUSYS_R {
        RFBUSYS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PU2 boot/wakeup request source information"]
    #[inline(always)]
    pub fn c2boots(&self) -> C2BOOTS_R {
        C2BOOTS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Power status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
