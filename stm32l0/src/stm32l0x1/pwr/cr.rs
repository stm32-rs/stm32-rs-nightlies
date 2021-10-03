#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Regulator in Low-power deepsleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDS_A {
    #[doc = "0: Voltage regulator in Main mode during Deepsleep mode (Stop mode)"]
    MAIN_MODE = 0,
    #[doc = "1: Voltage regulator switches to low-power mode when the CPU enters Deepsleep mode (Stop mode)"]
    LOW_POWER_MODE = 1,
}
impl From<LPDS_A> for bool {
    #[inline(always)]
    fn from(variant: LPDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDS` reader - Regulator in Low-power deepsleep mode"]
pub struct LPDS_R(crate::FieldReader<bool, LPDS_A>);
impl LPDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDS_A {
        match self.bits {
            false => LPDS_A::MAIN_MODE,
            true => LPDS_A::LOW_POWER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_MODE`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPDS_A::MAIN_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPDS_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for LPDS_R {
    type Target = crate::FieldReader<bool, LPDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPDS` writer - Regulator in Low-power deepsleep mode"]
pub struct LPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage regulator in Main mode during Deepsleep mode (Stop mode)"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPDS_A::MAIN_MODE)
    }
    #[doc = "Voltage regulator switches to low-power mode when the CPU enters Deepsleep mode (Stop mode)"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPDS_A::LOW_POWER_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Power down deepsleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDS_A {
    #[doc = "0: Enter Stop mode when the CPU enters deepsleep"]
    STOP_MODE = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    STANDBY_MODE = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub struct PDDS_R(crate::FieldReader<bool, PDDS_A>);
impl PDDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::STOP_MODE,
            true => PDDS_A::STANDBY_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE`"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        **self == PDDS_A::STOP_MODE
    }
    #[doc = "Checks if the value of the field is `STANDBY_MODE`"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        **self == PDDS_A::STANDBY_MODE
    }
}
impl core::ops::Deref for PDDS_R {
    type Target = crate::FieldReader<bool, PDDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub struct PDDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STOP_MODE)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STANDBY_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWUF_A {
    #[doc = "1: Clear the WUF Wakeup flag after 2 system clock cycles"]
    CLEAR = 1,
}
impl From<CWUF_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub struct CWUF_R(crate::FieldReader<bool, CWUF_A>);
impl CWUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CWUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CWUF_A> {
        match self.bits {
            true => Some(CWUF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CWUF_A::CLEAR
    }
}
impl core::ops::Deref for CWUF_R {
    type Target = crate::FieldReader<bool, CWUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub struct CWUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the WUF Wakeup flag after 2 system clock cycles"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF_A::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSBF_A {
    #[doc = "1: Clear the SBF Standby flag"]
    CLEAR = 1,
}
impl From<CSBF_A> for bool {
    #[inline(always)]
    fn from(variant: CSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub struct CSBF_R(crate::FieldReader<bool, CSBF_A>);
impl CSBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSBF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSBF_A> {
        match self.bits {
            true => Some(CSBF_A::CLEAR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CSBF_A::CLEAR
    }
}
impl core::ops::Deref for CSBF_R {
    type Target = crate::FieldReader<bool, CSBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub struct CSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSBF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear the SBF Standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSBF_A::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDE_A {
    #[doc = "0: PVD Disabled"]
    DISABLED = 0,
    #[doc = "1: PVD Enabled"]
    ENABLED = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub struct PVDE_R(crate::FieldReader<bool, PVDE_A>);
impl PVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::DISABLED,
            true => PVDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PVDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PVDE_A::ENABLED
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, PVDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PVDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::DISABLED)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "PVD level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: 1.9 V"]
    V1_9 = 0,
    #[doc = "1: 2.1 V"]
    V2_1 = 1,
    #[doc = "2: 2.3 V"]
    V2_3 = 2,
    #[doc = "3: 2.5 V"]
    V2_5 = 3,
    #[doc = "4: 2.7 V"]
    V2_7 = 4,
    #[doc = "5: 2.9 V"]
    V2_9 = 5,
    #[doc = "6: 3.1 V"]
    V3_1 = 6,
    #[doc = "7: External input analog voltage (Compare internally to VREFINT)"]
    EXTERNAL = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLS` reader - PVD level selection"]
pub struct PLS_R(crate::FieldReader<u8, PLS_A>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::V1_9,
            1 => PLS_A::V2_1,
            2 => PLS_A::V2_3,
            3 => PLS_A::V2_5,
            4 => PLS_A::V2_7,
            5 => PLS_A::V2_9,
            6 => PLS_A::V3_1,
            7 => PLS_A::EXTERNAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V1_9`"]
    #[inline(always)]
    pub fn is_v1_9(&self) -> bool {
        **self == PLS_A::V1_9
    }
    #[doc = "Checks if the value of the field is `V2_1`"]
    #[inline(always)]
    pub fn is_v2_1(&self) -> bool {
        **self == PLS_A::V2_1
    }
    #[doc = "Checks if the value of the field is `V2_3`"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        **self == PLS_A::V2_3
    }
    #[doc = "Checks if the value of the field is `V2_5`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        **self == PLS_A::V2_5
    }
    #[doc = "Checks if the value of the field is `V2_7`"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        **self == PLS_A::V2_7
    }
    #[doc = "Checks if the value of the field is `V2_9`"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        **self == PLS_A::V2_9
    }
    #[doc = "Checks if the value of the field is `V3_1`"]
    #[inline(always)]
    pub fn is_v3_1(&self) -> bool {
        **self == PLS_A::V3_1
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == PLS_A::EXTERNAL
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, PLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - PVD level selection"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn v1_9(self) -> &'a mut W {
        self.variant(PLS_A::V1_9)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn v2_1(self) -> &'a mut W {
        self.variant(PLS_A::V2_1)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut W {
        self.variant(PLS_A::V2_3)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(PLS_A::V2_5)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut W {
        self.variant(PLS_A::V2_7)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(PLS_A::V2_9)
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn v3_1(self) -> &'a mut W {
        self.variant(PLS_A::V3_1)
    }
    #[doc = "External input analog voltage (Compare internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLS_A::EXTERNAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBP_A {
    #[doc = "0: Access to RTC, RTC Backup and RCC CSR registers disabled"]
    DISABLED = 0,
    #[doc = "1: Access to RTC, RTC Backup and RCC CSR registers enabled"]
    ENABLED = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub struct DBP_R(crate::FieldReader<bool, DBP_A>);
impl DBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::DISABLED,
            true => DBP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBP_A::ENABLED
    }
}
impl core::ops::Deref for DBP_R {
    type Target = crate::FieldReader<bool, DBP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub struct DBP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::DISABLED)
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBP_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Ultra-low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULP_A {
    #[doc = "0: VREFINT is on in low-power mode"]
    ENABLED = 0,
    #[doc = "1: VREFINT is off in low-power mode"]
    DISABLED = 1,
}
impl From<ULP_A> for bool {
    #[inline(always)]
    fn from(variant: ULP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULP` reader - Ultra-low-power mode"]
pub struct ULP_R(crate::FieldReader<bool, ULP_A>);
impl ULP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULP_A {
        match self.bits {
            false => ULP_A::ENABLED,
            true => ULP_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ULP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ULP_A::DISABLED
    }
}
impl core::ops::Deref for ULP_R {
    type Target = crate::FieldReader<bool, ULP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULP` writer - Ultra-low-power mode"]
pub struct ULP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREFINT is on in low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ULP_A::ENABLED)
    }
    #[doc = "VREFINT is off in low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ULP_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Fast wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWU_A {
    #[doc = "0: Low-power modes exit occurs only when VREFINT is ready"]
    DISABLED = 0,
    #[doc = "1: VREFINT start up time is ignored when exiting low-power modes"]
    ENABLED = 1,
}
impl From<FWU_A> for bool {
    #[inline(always)]
    fn from(variant: FWU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWU` reader - Fast wakeup"]
pub struct FWU_R(crate::FieldReader<bool, FWU_A>);
impl FWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWU_A {
        match self.bits {
            false => FWU_A::DISABLED,
            true => FWU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FWU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FWU_A::ENABLED
    }
}
impl core::ops::Deref for FWU_R {
    type Target = crate::FieldReader<bool, FWU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWU` writer - Fast wakeup"]
pub struct FWU_W<'a> {
    w: &'a mut W,
}
impl<'a> FWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low-power modes exit occurs only when VREFINT is ready"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWU_A::DISABLED)
    }
    #[doc = "VREFINT start up time is ignored when exiting low-power modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWU_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Voltage scaling range selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "1: 1.8 V (range 1)"]
    V1_8 = 1,
    #[doc = "2: 1.5 V (range 2)"]
    V1_5 = 2,
    #[doc = "3: 1.2 V (range 3)"]
    V1_2 = 3,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub struct VOS_R(crate::FieldReader<u8, VOS_A>);
impl VOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOS_A> {
        match self.bits {
            1 => Some(VOS_A::V1_8),
            2 => Some(VOS_A::V1_5),
            3 => Some(VOS_A::V1_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V1_8`"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        **self == VOS_A::V1_8
    }
    #[doc = "Checks if the value of the field is `V1_5`"]
    #[inline(always)]
    pub fn is_v1_5(&self) -> bool {
        **self == VOS_A::V1_5
    }
    #[doc = "Checks if the value of the field is `V1_2`"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        **self == VOS_A::V1_2
    }
}
impl core::ops::Deref for VOS_R {
    type Target = crate::FieldReader<u8, VOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub struct VOS_W<'a> {
    w: &'a mut W,
}
impl<'a> VOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1.8 V (range 1)"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut W {
        self.variant(VOS_A::V1_8)
    }
    #[doc = "1.5 V (range 2)"]
    #[inline(always)]
    pub fn v1_5(self) -> &'a mut W {
        self.variant(VOS_A::V1_5)
    }
    #[doc = "1.2 V (range 3)"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(VOS_A::V1_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Deep sleep mode with Flash memory kept off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS_EE_KOFF_A {
    #[doc = "0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    NVMWAKEUP = 0,
    #[doc = "1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    NVMSLEEP = 1,
}
impl From<DS_EE_KOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DS_EE_KOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS_EE_KOFF` reader - Deep sleep mode with Flash memory kept off"]
pub struct DS_EE_KOFF_R(crate::FieldReader<bool, DS_EE_KOFF_A>);
impl DS_EE_KOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DS_EE_KOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS_EE_KOFF_A {
        match self.bits {
            false => DS_EE_KOFF_A::NVMWAKEUP,
            true => DS_EE_KOFF_A::NVMSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NVMWAKEUP`"]
    #[inline(always)]
    pub fn is_nvmwake_up(&self) -> bool {
        **self == DS_EE_KOFF_A::NVMWAKEUP
    }
    #[doc = "Checks if the value of the field is `NVMSLEEP`"]
    #[inline(always)]
    pub fn is_nvmsleep(&self) -> bool {
        **self == DS_EE_KOFF_A::NVMSLEEP
    }
}
impl core::ops::Deref for DS_EE_KOFF_R {
    type Target = crate::FieldReader<bool, DS_EE_KOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DS_EE_KOFF` writer - Deep sleep mode with Flash memory kept off"]
pub struct DS_EE_KOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_EE_KOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DS_EE_KOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    #[inline(always)]
    pub fn nvmwake_up(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::NVMWAKEUP)
    }
    #[doc = "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    #[inline(always)]
    pub fn nvmsleep(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::NVMSLEEP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Low power run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRUN_A {
    #[doc = "0: Voltage regulator in Main mode in Low-power run mode"]
    MAIN_MODE = 0,
    #[doc = "1: Voltage regulator in low-power mode in Low-power run mode"]
    LOW_POWER_MODE = 1,
}
impl From<LPRUN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRUN` reader - Low power run mode"]
pub struct LPRUN_R(crate::FieldReader<bool, LPRUN_A>);
impl LPRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPRUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRUN_A {
        match self.bits {
            false => LPRUN_A::MAIN_MODE,
            true => LPRUN_A::LOW_POWER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_MODE`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPRUN_A::MAIN_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPRUN_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for LPRUN_R {
    type Target = crate::FieldReader<bool, LPRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRUN` writer - Low power run mode"]
pub struct LPRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPRUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::MAIN_MODE)
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::LOW_POWER_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Low-power deepsleep/Sleep/Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSDSR_A {
    #[doc = "0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    MAIN_MODE = 0,
    #[doc = "1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    LOW_POWER_MODE = 1,
}
impl From<LPSDSR_A> for bool {
    #[inline(always)]
    fn from(variant: LPSDSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSDSR` reader - Low-power deepsleep/Sleep/Low-power run"]
pub struct LPSDSR_R(crate::FieldReader<bool, LPSDSR_A>);
impl LPSDSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPSDSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSDSR_A {
        match self.bits {
            false => LPSDSR_A::MAIN_MODE,
            true => LPSDSR_A::LOW_POWER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_MODE`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPSDSR_A::MAIN_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPSDSR_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for LPSDSR_R {
    type Target = crate::FieldReader<bool, LPSDSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPSDSR` writer - Low-power deepsleep/Sleep/Low-power run"]
pub struct LPSDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSDSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSDSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::MAIN_MODE)
    }
    #[doc = "Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::LOW_POWER_MODE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Regulator in Low-power deepsleep mode"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    pub fn ds_ee_koff(&self) -> DS_EE_KOFF_R {
        DS_EE_KOFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Regulator in Low-power deepsleep mode"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W { w: self }
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W { w: self }
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W {
        CWUF_W { w: self }
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W {
        CSBF_W { w: self }
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    pub fn ulp(&mut self) -> ULP_W {
        ULP_W { w: self }
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    pub fn fwu(&mut self) -> FWU_W {
        FWU_W { w: self }
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W {
        VOS_W { w: self }
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    pub fn ds_ee_koff(&mut self) -> DS_EE_KOFF_W {
        DS_EE_KOFF_W { w: self }
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    pub fn lprun(&mut self) -> LPRUN_W {
        LPRUN_W { w: self }
    }
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    pub fn lpsdsr(&mut self) -> LPSDSR_W {
        LPSDSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x1000"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
