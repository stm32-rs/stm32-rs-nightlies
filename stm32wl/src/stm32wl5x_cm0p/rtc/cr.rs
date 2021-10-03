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
#[doc = "RTC_OUT2 output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT2EN_A {
    #[doc = "0: RTC output 2 disable"]
    DISABLED = 0,
    #[doc = "1: RTC output 2 enable"]
    ENABLED = 1,
}
impl From<OUT2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OUT2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable"]
pub struct OUT2EN_R(crate::FieldReader<bool, OUT2EN_A>);
impl OUT2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT2EN_A {
        match self.bits {
            false => OUT2EN_A::DISABLED,
            true => OUT2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OUT2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OUT2EN_A::ENABLED
    }
}
impl core::ops::Deref for OUT2EN_R {
    type Target = crate::FieldReader<bool, OUT2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable"]
pub struct OUT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC output 2 disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUT2EN_A::DISABLED)
    }
    #[doc = "RTC output 2 enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OUT2EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "TAMPALRM output type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPALRM_TYPE_A {
    #[doc = "0: TAMPALRM is push-pull output"]
    PUSHPULL = 0,
    #[doc = "1: TAMPALRM is open-drain output"]
    OPENDRAIN = 1,
}
impl From<TAMPALRM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub struct TAMPALRM_TYPE_R(crate::FieldReader<bool, TAMPALRM_TYPE_A>);
impl TAMPALRM_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPALRM_TYPE_A {
        match self.bits {
            false => TAMPALRM_TYPE_A::PUSHPULL,
            true => TAMPALRM_TYPE_A::OPENDRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        **self == TAMPALRM_TYPE_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OPENDRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == TAMPALRM_TYPE_A::OPENDRAIN
    }
}
impl core::ops::Deref for TAMPALRM_TYPE_R {
    type Target = crate::FieldReader<bool, TAMPALRM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub struct TAMPALRM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPALRM_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(TAMPALRM_TYPE_A::PUSHPULL)
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(TAMPALRM_TYPE_A::OPENDRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "TAMPALRM pull-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPALRM_PU_A {
    #[doc = "0: No pull-up is applied on TAMPALRM output"]
    NOPULLUP = 0,
    #[doc = "1: A pull-up is applied on TAMPALRM output"]
    PULLUP = 1,
}
impl From<TAMPALRM_PU_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub struct TAMPALRM_PU_R(crate::FieldReader<bool, TAMPALRM_PU_A>);
impl TAMPALRM_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPALRM_PU_A {
        match self.bits {
            false => TAMPALRM_PU_A::NOPULLUP,
            true => TAMPALRM_PU_A::PULLUP,
        }
    }
    #[doc = "Checks if the value of the field is `NOPULLUP`"]
    #[inline(always)]
    pub fn is_no_pull_up(&self) -> bool {
        **self == TAMPALRM_PU_A::NOPULLUP
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == TAMPALRM_PU_A::PULLUP
    }
}
impl core::ops::Deref for TAMPALRM_PU_R {
    type Target = crate::FieldReader<bool, TAMPALRM_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub struct TAMPALRM_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPALRM_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn no_pull_up(self) -> &'a mut W {
        self.variant(TAMPALRM_PU_A::NOPULLUP)
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(TAMPALRM_PU_A::PULLUP)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Tamper detection output enable on TAMPALRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPOE_A {
    #[doc = "0: The tamper flag is not routed on TAMPALRM"]
    DISABLED = 0,
    #[doc = "1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL"]
    ENABLED = 1,
}
impl From<TAMPOE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub struct TAMPOE_R(crate::FieldReader<bool, TAMPOE_A>);
impl TAMPOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPOE_A {
        match self.bits {
            false => TAMPOE_A::DISABLED,
            true => TAMPOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMPOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMPOE_A::ENABLED
    }
}
impl core::ops::Deref for TAMPOE_R {
    type Target = crate::FieldReader<bool, TAMPOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub struct TAMPOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPOE_A::DISABLED)
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Activate timestamp on tamper detection event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPTS_A {
    #[doc = "0: Tamper detection event does not cause a RTC timestamp to be saved"]
    DISABLED = 0,
    #[doc = "1: Save RTC timestamp on tamper detection event"]
    ENABLED = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub struct TAMPTS_R(crate::FieldReader<bool, TAMPTS_A>);
impl TAMPTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::DISABLED,
            true => TAMPTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TAMPTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TAMPTS_A::ENABLED
    }
}
impl core::ops::Deref for TAMPTS_R {
    type Target = crate::FieldReader<bool, TAMPTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub struct TAMPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAMPTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPTS_A::DISABLED)
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPTS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "timestamp on internal event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITSE_A {
    #[doc = "0: Internal event timestamp disabled"]
    DISABLED = 0,
    #[doc = "1: Internal event timestamp enabled"]
    ENABLED = 1,
}
impl From<ITSE_A> for bool {
    #[inline(always)]
    fn from(variant: ITSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub struct ITSE_R(crate::FieldReader<bool, ITSE_A>);
impl ITSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITSE_A {
        match self.bits {
            false => ITSE_A::DISABLED,
            true => ITSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ITSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ITSE_A::ENABLED
    }
}
impl core::ops::Deref for ITSE_R {
    type Target = crate::FieldReader<bool, ITSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub struct ITSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal event timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITSE_A::DISABLED)
    }
    #[doc = "Internal event timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITSE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COE_A {
    #[doc = "0: Calibration output disabled"]
    DISABLED = 0,
    #[doc = "1: Calibration output enabled"]
    ENABLED = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COE` reader - Calibration output enable"]
pub struct COE_R(crate::FieldReader<bool, COE_A>);
impl COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::DISABLED,
            true => COE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COE_A::ENABLED
    }
}
impl core::ops::Deref for COE_R {
    type Target = crate::FieldReader<bool, COE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COE` writer - Calibration output enable"]
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COE_A::DISABLED)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSEL_A {
    #[doc = "0: Output disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A output enabled"]
    ALARMA = 1,
    #[doc = "2: Alarm B output enabled"]
    ALARMB = 2,
    #[doc = "3: Wakeup output enabled"]
    WAKEUP = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSEL` reader - Output selection"]
pub struct OSEL_R(crate::FieldReader<u8, OSEL_A>);
impl OSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::DISABLED,
            1 => OSEL_A::ALARMA,
            2 => OSEL_A::ALARMB,
            3 => OSEL_A::WAKEUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ALARMA`"]
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        **self == OSEL_A::ALARMA
    }
    #[doc = "Checks if the value of the field is `ALARMB`"]
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        **self == OSEL_A::ALARMB
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == OSEL_A::WAKEUP
    }
}
impl core::ops::Deref for OSEL_R {
    type Target = crate::FieldReader<u8, OSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSEL` writer - Output selection"]
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSEL_A::DISABLED)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMA)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMB)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(OSEL_A::WAKEUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_A {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    HIGH = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    LOW = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL` reader - Output polarity"]
pub struct POL_R(crate::FieldReader<bool, POL_A>);
impl POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::HIGH,
            true => POL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == POL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == POL_A::LOW
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL` writer - Output polarity"]
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_A::HIGH)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Calibration output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSEL_A {
    #[doc = "0: Calibration output is 512 Hz (with default prescaler setting)"]
    CALFREQ_512HZ = 0,
    #[doc = "1: Calibration output is 1 Hz (with default prescaler setting)"]
    CALFREQ_1HZ = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEL` reader - Calibration output selection"]
pub struct COSEL_R(crate::FieldReader<bool, COSEL_A>);
impl COSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::CALFREQ_512HZ,
            true => COSEL_A::CALFREQ_1HZ,
        }
    }
    #[doc = "Checks if the value of the field is `CALFREQ_512HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        **self == COSEL_A::CALFREQ_512HZ
    }
    #[doc = "Checks if the value of the field is `CALFREQ_1HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        **self == COSEL_A::CALFREQ_1HZ
    }
}
impl core::ops::Deref for COSEL_R {
    type Target = crate::FieldReader<bool, COSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COSEL` writer - Calibration output selection"]
pub struct COSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_512HZ)
    }
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_1HZ)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKP_A {
    #[doc = "0: Daylight Saving Time change has not been performed"]
    DSTNOTCHANGED = 0,
    #[doc = "1: Daylight Saving Time change has been performed"]
    DSTCHANGED = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Backup"]
pub struct BKP_R(crate::FieldReader<bool, BKP_A>);
impl BKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::DSTNOTCHANGED,
            true => BKP_A::DSTCHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `DSTNOTCHANGED`"]
    #[inline(always)]
    pub fn is_dstnot_changed(&self) -> bool {
        **self == BKP_A::DSTNOTCHANGED
    }
    #[doc = "Checks if the value of the field is `DSTCHANGED`"]
    #[inline(always)]
    pub fn is_dstchanged(&self) -> bool {
        **self == BKP_A::DSTCHANGED
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<bool, BKP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP` writer - Backup"]
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Daylight Saving Time change has not been performed"]
    #[inline(always)]
    pub fn dstnot_changed(self) -> &'a mut W {
        self.variant(BKP_A::DSTNOTCHANGED)
    }
    #[doc = "Daylight Saving Time change has been performed"]
    #[inline(always)]
    pub fn dstchanged(self) -> &'a mut W {
        self.variant(BKP_A::DSTCHANGED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Subtract 1 hour (winter time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB1H_AW {
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    SUB1 = 1,
}
impl From<SUB1H_AW> for bool {
    #[inline(always)]
    fn from(variant: SUB1H_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change)"]
pub struct SUB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB1H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUB1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    #[inline(always)]
    pub fn sub1(self) -> &'a mut W {
        self.variant(SUB1H_AW::SUB1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Add 1 hour (summer time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD1H_AW {
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    ADD1 = 1,
}
impl From<ADD1H_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1H_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub struct ADD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADD1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1H_AW::ADD1)
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
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
    #[doc = "0: Time-stamp Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Time-stamp Interrupt enabled"]
    ENABLED = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub struct TSIE_R(crate::FieldReader<bool, TSIE_A>);
impl TSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::DISABLED,
            true => TSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSIE_A::ENABLED
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, TSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Time-stamp Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSIE_A::DISABLED)
    }
    #[doc = "Time-stamp Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTIE_A {
    #[doc = "0: Wakeup timer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    ENABLED = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub struct WUTIE_R(crate::FieldReader<bool, WUTIE_A>);
impl WUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::DISABLED,
            true => WUTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WUTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WUTIE_A::ENABLED
    }
}
impl core::ops::Deref for WUTIE_R {
    type Target = crate::FieldReader<bool, WUTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub struct WUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTIE_A::DISABLED)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTIE_A::ENABLED)
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
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBIE_A {
    #[doc = "0: Alarm B Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B Interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub struct ALRBIE_R(crate::FieldReader<bool, ALRBIE_A>);
impl ALRBIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::DISABLED,
            true => ALRBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRBIE_A::ENABLED
    }
}
impl core::ops::Deref for ALRBIE_R {
    type Target = crate::FieldReader<bool, ALRBIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub struct ALRBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRBIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm B Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::DISABLED)
    }
    #[doc = "Alarm B Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::ENABLED)
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
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAIE_A {
    #[doc = "0: Alarm A interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub struct ALRAIE_R(crate::FieldReader<bool, ALRAIE_A>);
impl ALRAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::DISABLED,
            true => ALRAIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRAIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRAIE_A::ENABLED
    }
}
impl core::ops::Deref for ALRAIE_R {
    type Target = crate::FieldReader<bool, ALRAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub struct ALRAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::DISABLED)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSE_A {
    #[doc = "0: Timestamp disabled"]
    DISABLED = 0,
    #[doc = "1: Timestamp enabled"]
    ENABLED = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSE` reader - timestamp enable"]
pub struct TSE_R(crate::FieldReader<bool, TSE_A>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::DISABLED,
            true => TSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSE_A::ENABLED
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, TSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSE` writer - timestamp enable"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSE_A::DISABLED)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Wakeup timer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTE_A {
    #[doc = "0: Wakeup timer disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer enabled"]
    ENABLED = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub struct WUTE_R(crate::FieldReader<bool, WUTE_A>);
impl WUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::DISABLED,
            true => WUTE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WUTE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WUTE_A::ENABLED
    }
}
impl core::ops::Deref for WUTE_R {
    type Target = crate::FieldReader<bool, WUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub struct WUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTE_A::DISABLED)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTE_A::ENABLED)
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
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBE_A {
    #[doc = "0: Alarm B disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B enabled"]
    ENABLED = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub struct ALRBE_R(crate::FieldReader<bool, ALRBE_A>);
impl ALRBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::DISABLED,
            true => ALRBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRBE_A::ENABLED
    }
}
impl core::ops::Deref for ALRBE_R {
    type Target = crate::FieldReader<bool, ALRBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub struct ALRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBE_A::DISABLED)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBE_A::ENABLED)
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
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAE_A {
    #[doc = "0: Alarm A disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A enabled"]
    ENABLED = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub struct ALRAE_R(crate::FieldReader<bool, ALRAE_A>);
impl ALRAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::DISABLED,
            true => ALRAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRAE_A::ENABLED
    }
}
impl core::ops::Deref for ALRAE_R {
    type Target = crate::FieldReader<bool, ALRAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub struct ALRAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALRAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAE_A::DISABLED)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAE_A::ENABLED)
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
#[doc = "SSR underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRUIE_A {
    #[doc = "0: SSR underflow interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: SSR underflow interrupt enabled"]
    ENABLED = 1,
}
impl From<SSRUIE_A> for bool {
    #[inline(always)]
    fn from(variant: SSRUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSRUIE` reader - SSR underflow interrupt enable"]
pub struct SSRUIE_R(crate::FieldReader<bool, SSRUIE_A>);
impl SSRUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSRUIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSRUIE_A {
        match self.bits {
            false => SSRUIE_A::DISABLED,
            true => SSRUIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSRUIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSRUIE_A::ENABLED
    }
}
impl core::ops::Deref for SSRUIE_R {
    type Target = crate::FieldReader<bool, SSRUIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSRUIE` writer - SSR underflow interrupt enable"]
pub struct SSRUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSRUIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SSR underflow interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSRUIE_A::DISABLED)
    }
    #[doc = "SSR underflow interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSRUIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMT_A {
    #[doc = "0: 24 hour/day format"]
    TWENTYFOURHOUR = 0,
    #[doc = "1: AM/PM hour format"]
    AMPM = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub struct FMT_R(crate::FieldReader<bool, FMT_A>);
impl FMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::TWENTYFOURHOUR,
            true => FMT_A::AMPM,
        }
    }
    #[doc = "Checks if the value of the field is `TWENTYFOURHOUR`"]
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        **self == FMT_A::TWENTYFOURHOUR
    }
    #[doc = "Checks if the value of the field is `AMPM`"]
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        **self == FMT_A::AMPM
    }
}
impl core::ops::Deref for FMT_R {
    type Target = crate::FieldReader<bool, FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub struct FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut W {
        self.variant(FMT_A::TWENTYFOURHOUR)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut W {
        self.variant(FMT_A::AMPM)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Bypass the shadow registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPSHAD_A {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    SHADOWREG = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    BYPASSSHADOWREG = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers"]
pub struct BYPSHAD_R(crate::FieldReader<bool, BYPSHAD_A>);
impl BYPSHAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPSHAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::SHADOWREG,
            true => BYPSHAD_A::BYPASSSHADOWREG,
        }
    }
    #[doc = "Checks if the value of the field is `SHADOWREG`"]
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        **self == BYPSHAD_A::SHADOWREG
    }
    #[doc = "Checks if the value of the field is `BYPASSSHADOWREG`"]
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        **self == BYPSHAD_A::BYPASSSHADOWREG
    }
}
impl core::ops::Deref for BYPSHAD_R {
    type Target = crate::FieldReader<bool, BYPSHAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers"]
pub struct BYPSHAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPSHAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPSHAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::SHADOWREG)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::BYPASSSHADOWREG)
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
#[doc = "RTC_REFIN reference clock detection enable (50 or 60 Hz)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCKON_A {
    #[doc = "0: RTC_REFIN detection disabled"]
    DISABLED = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    ENABLED = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub struct REFCKON_R(crate::FieldReader<bool, REFCKON_A>);
impl REFCKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFCKON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::DISABLED,
            true => REFCKON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REFCKON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REFCKON_A::ENABLED
    }
}
impl core::ops::Deref for REFCKON_R {
    type Target = crate::FieldReader<bool, REFCKON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub struct REFCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCKON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REFCKON_A::DISABLED)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REFCKON_A::ENABLED)
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
#[doc = "Timestamp event active edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEDGE_A {
    #[doc = "0: RTC_TS input rising edge generates a time-stamp event"]
    RISINGEDGE = 0,
    #[doc = "1: RTC_TS input falling edge generates a time-stamp event"]
    FALLINGEDGE = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEDGE` reader - Timestamp event active edge"]
pub struct TSEDGE_R(crate::FieldReader<bool, TSEDGE_A>);
impl TSEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::RISINGEDGE,
            true => TSEDGE_A::FALLINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == TSEDGE_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == TSEDGE_A::FALLINGEDGE
    }
}
impl core::ops::Deref for TSEDGE_R {
    type Target = crate::FieldReader<bool, TSEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEDGE` writer - Timestamp event active edge"]
pub struct TSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::RISINGEDGE)
    }
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::FALLINGEDGE)
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
#[doc = "Wakeup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    #[doc = "0: RTC/16 clock is selected"]
    DIV16 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    DIV8 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    DIV4 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    DIV2 = 3,
    #[doc = "4: ck_spre (usually 1 Hz) clock is selected"]
    CLOCKSPARE = 4,
    #[doc = "6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    CLOCKSPAREWITHOFFSET = 6,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection"]
pub struct WUCKSEL_R(crate::FieldReader<u8, WUCKSEL_A>);
impl WUCKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUCKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::DIV16),
            1 => Some(WUCKSEL_A::DIV8),
            2 => Some(WUCKSEL_A::DIV4),
            3 => Some(WUCKSEL_A::DIV2),
            4 => Some(WUCKSEL_A::CLOCKSPARE),
            6 => Some(WUCKSEL_A::CLOCKSPAREWITHOFFSET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == WUCKSEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == WUCKSEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == WUCKSEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == WUCKSEL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `CLOCKSPARE`"]
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        **self == WUCKSEL_A::CLOCKSPARE
    }
    #[doc = "Checks if the value of the field is `CLOCKSPAREWITHOFFSET`"]
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        **self == WUCKSEL_A::CLOCKSPAREWITHOFFSET
    }
}
impl core::ops::Deref for WUCKSEL_R {
    type Target = crate::FieldReader<u8, WUCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection"]
pub struct WUCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUCKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV16)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV8)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV4)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV2)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPARE)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPAREWITHOFFSET)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - RTC_OUT2 output enable"]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    pub fn ssruie(&self) -> SSRUIE_R {
        SSRUIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - RTC_OUT2 output enable"]
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W {
        OUT2EN_W { w: self }
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W {
        TAMPALRM_TYPE_W { w: self }
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W {
        TAMPALRM_PU_W { w: self }
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W {
        TAMPOE_W { w: self }
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W {
        ITSE_W { w: self }
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W {
        COSEL_W { w: self }
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W {
        SUB1H_W { w: self }
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W {
        ADD1H_W { w: self }
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W {
        WUTIE_W { w: self }
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W {
        ALRBIE_W { w: self }
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W {
        ALRAIE_W { w: self }
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W {
        WUTE_W { w: self }
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W {
        ALRBE_W { w: self }
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W {
        ALRAE_W { w: self }
    }
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    pub fn ssruie(&mut self) -> SSRUIE_W {
        SSRUIE_W { w: self }
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W {
        FMT_W { w: self }
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W {
        BYPSHAD_W { w: self }
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W {
        REFCKON_W { w: self }
    }
    #[doc = "Bit 3 - Timestamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W {
        TSEDGE_W { w: self }
    }
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W {
        WUCKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
