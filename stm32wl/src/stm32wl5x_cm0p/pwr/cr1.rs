#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPR_A {
    #[doc = "0: Voltage regulator in Main mode in Low-power run mode"]
    MAINMODE = 0,
    #[doc = "1: Voltage regulator in low-power mode in Low-power run mode"]
    LOWPOWERMODE = 1,
}
impl From<LPR_A> for bool {
    #[inline(always)]
    fn from(variant: LPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPR` reader - Low-power run"]
pub struct LPR_R(crate::FieldReader<bool, LPR_A>);
impl LPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPR_A {
        match self.bits {
            false => LPR_A::MAINMODE,
            true => LPR_A::LOWPOWERMODE,
        }
    }
    #[doc = "Checks if the value of the field is `MAINMODE`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPR_A::MAINMODE
    }
    #[doc = "Checks if the value of the field is `LOWPOWERMODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPR_A::LOWPOWERMODE
    }
}
impl core::ops::Deref for LPR_R {
    type Target = crate::FieldReader<bool, LPR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPR` writer - Low-power run"]
pub struct LPR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPR_A::MAINMODE)
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPR_A::LOWPOWERMODE)
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
#[doc = "Voltage scaling range selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "1: 1.2 V (range 1)"]
    V1_2 = 1,
    #[doc = "2: 1.0 V (range 2)"]
    V1_0 = 2,
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
            1 => Some(VOS_A::V1_2),
            2 => Some(VOS_A::V1_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V1_2`"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        **self == VOS_A::V1_2
    }
    #[doc = "Checks if the value of the field is `V1_0`"]
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        **self == VOS_A::V1_0
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
    #[doc = "1.2 V (range 1)"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(VOS_A::V1_2)
    }
    #[doc = "1.0 V (range 2)"]
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut W {
        self.variant(VOS_A::V1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBP_A {
    #[doc = "0: Access to RTC and backup registers disabled"]
    DISABLED = 0,
    #[doc = "1: Access to RTC and backup registers enabled"]
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
    #[doc = "Access to RTC and backup registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::DISABLED)
    }
    #[doc = "Access to RTC and backup registers enabled"]
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
#[doc = "Flash memory power down mode during LPSleep for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDS_A {
    #[doc = "0: Flash memory in Idle mode when system is in LPSleep mode"]
    IDLE = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPSleep mode"]
    POWERDOWN = 1,
}
impl From<FPDS_A> for bool {
    #[inline(always)]
    fn from(variant: FPDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU1"]
pub struct FPDS_R(crate::FieldReader<bool, FPDS_A>);
impl FPDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDS_A {
        match self.bits {
            false => FPDS_A::IDLE,
            true => FPDS_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == FPDS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == FPDS_A::POWERDOWN
    }
}
impl core::ops::Deref for FPDS_R {
    type Target = crate::FieldReader<bool, FPDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU1"]
pub struct FPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDS_A::IDLE)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDS_A::POWERDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Flash memory power down mode during LPRun for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDR_A {
    #[doc = "0: Flash memory in Idle mode when system is in LPRun mode"]
    IDLE = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPRun mode"]
    POWERDOWN = 1,
}
impl From<FPDR_A> for bool {
    #[inline(always)]
    fn from(variant: FPDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDR` reader - Flash memory power down mode during LPRun for CPU1"]
pub struct FPDR_R(crate::FieldReader<bool, FPDR_A>);
impl FPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPDR_A {
        match self.bits {
            false => FPDR_A::IDLE,
            true => FPDR_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == FPDR_A::IDLE
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == FPDR_A::POWERDOWN
    }
}
impl core::ops::Deref for FPDR_R {
    type Target = crate::FieldReader<bool, FPDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPDR` writer - Flash memory power down mode during LPRun for CPU1"]
pub struct FPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPDR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDR_A::IDLE)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDR_A::POWERDOWN)
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
#[doc = "sub-GHz SPI NSS source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBGHZSPINSSSEL_A {
    #[doc = "0: sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
    SUBGHZSPICR = 0,
    #[doc = "1: sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
    LPTIM3 = 1,
}
impl From<SUBGHZSPINSSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPINSSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUBGHZSPINSSSEL` reader - sub-GHz SPI NSS source select"]
pub struct SUBGHZSPINSSSEL_R(crate::FieldReader<bool, SUBGHZSPINSSSEL_A>);
impl SUBGHZSPINSSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPINSSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUBGHZSPINSSSEL_A {
        match self.bits {
            false => SUBGHZSPINSSSEL_A::SUBGHZSPICR,
            true => SUBGHZSPINSSSEL_A::LPTIM3,
        }
    }
    #[doc = "Checks if the value of the field is `SUBGHZSPICR`"]
    #[inline(always)]
    pub fn is_subghzspicr(&self) -> bool {
        **self == SUBGHZSPINSSSEL_A::SUBGHZSPICR
    }
    #[doc = "Checks if the value of the field is `LPTIM3`"]
    #[inline(always)]
    pub fn is_lptim3(&self) -> bool {
        **self == SUBGHZSPINSSSEL_A::LPTIM3
    }
}
impl core::ops::Deref for SUBGHZSPINSSSEL_R {
    type Target = crate::FieldReader<bool, SUBGHZSPINSSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPINSSSEL` writer - sub-GHz SPI NSS source select"]
pub struct SUBGHZSPINSSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPINSSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUBGHZSPINSSSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
    #[inline(always)]
    pub fn subghzspicr(self) -> &'a mut W {
        self.variant(SUBGHZSPINSSSEL_A::SUBGHZSPICR)
    }
    #[doc = "sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
    #[inline(always)]
    pub fn lptim3(self) -> &'a mut W {
        self.variant(SUBGHZSPINSSSEL_A::LPTIM3)
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
#[doc = "Low-power mode selection for CPU1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMS_A {
    #[doc = "0: Stop 0 mode"]
    STOP0 = 0,
    #[doc = "1: Stop 1 mode"]
    STOP1 = 1,
    #[doc = "2: Stop 2 mode"]
    STOP2 = 2,
    #[doc = "3: Standby mode"]
    STANDBY = 3,
    #[doc = "4: Shutdown mode"]
    SHUTDOWN = 4,
}
impl From<LPMS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU1"]
pub struct LPMS_R(crate::FieldReader<u8, LPMS_A>);
impl LPMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMS_A> {
        match self.bits {
            0 => Some(LPMS_A::STOP0),
            1 => Some(LPMS_A::STOP1),
            2 => Some(LPMS_A::STOP2),
            3 => Some(LPMS_A::STANDBY),
            4 => Some(LPMS_A::SHUTDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP0`"]
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        **self == LPMS_A::STOP0
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        **self == LPMS_A::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        **self == LPMS_A::STOP2
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        **self == LPMS_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `SHUTDOWN`"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        **self == LPMS_A::SHUTDOWN
    }
}
impl core::ops::Deref for LPMS_R {
    type Target = crate::FieldReader<u8, LPMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU1"]
pub struct LPMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn stop0(self) -> &'a mut W {
        self.variant(LPMS_A::STOP0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(LPMS_A::STOP1)
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(LPMS_A::STOP2)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMS_A::STANDBY)
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(LPMS_A::SHUTDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU1"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU1"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - sub-GHz SPI NSS source select"]
    #[inline(always)]
    pub fn subghzspinsssel(&self) -> SUBGHZSPINSSSEL_R {
        SUBGHZSPINSSSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU1"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&mut self) -> LPR_W {
        LPR_W { w: self }
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W {
        VOS_W { w: self }
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU1"]
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W {
        FPDS_W { w: self }
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU1"]
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W {
        FPDR_W { w: self }
    }
    #[doc = "Bit 3 - sub-GHz SPI NSS source select"]
    #[inline(always)]
    pub fn subghzspinsssel(&mut self) -> SUBGHZSPINSSSEL_W {
        SUBGHZSPINSSSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU1"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W {
        LPMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0x0200"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
