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
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCAL_A {
    #[doc = "0: Calibration complete"]
    COMPLETE = 0,
    #[doc = "1: Start the calibration of the ADC"]
    CALIBRATION = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub struct ADCAL_R(crate::FieldReader<bool, ADCAL_A>);
impl ADCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::COMPLETE,
            true => ADCAL_A::CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == ADCAL_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `CALIBRATION`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        **self == ADCAL_A::CALIBRATION
    }
}
impl core::ops::Deref for ADCAL_R {
    type Target = crate::FieldReader<bool, ADCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub struct ADCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(ADCAL_A::COMPLETE)
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(ADCAL_A::CALIBRATION)
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
#[doc = "ADC differential mode for calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCALDIF_A {
    #[doc = "0: Calibration for single-ended mode"]
    SINGLEENDED = 0,
    #[doc = "1: Calibration for differential mode"]
    DIFFERENTIAL = 1,
}
impl From<ADCALDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALDIF` reader - ADC differential mode for calibration"]
pub struct ADCALDIF_R(crate::FieldReader<bool, ADCALDIF_A>);
impl ADCALDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCALDIF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALDIF_A {
        match self.bits {
            false => ADCALDIF_A::SINGLEENDED,
            true => ADCALDIF_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLEENDED`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        **self == ADCALDIF_A::SINGLEENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        **self == ADCALDIF_A::DIFFERENTIAL
    }
}
impl core::ops::Deref for ADCALDIF_R {
    type Target = crate::FieldReader<bool, ADCALDIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCALDIF` writer - ADC differential mode for calibration"]
pub struct ADCALDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCALDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCALDIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(ADCALDIF_A::SINGLEENDED)
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(ADCALDIF_A::DIFFERENTIAL)
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
#[doc = "ADC deep power down enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEPPWD_A {
    #[doc = "0: ADC not in deep power down"]
    POWERUP = 0,
    #[doc = "1: ADC in deep power down"]
    POWERDOWN = 1,
}
impl From<DEEPPWD_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPPWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPPWD` reader - ADC deep power down enable"]
pub struct DEEPPWD_R(crate::FieldReader<bool, DEEPPWD_A>);
impl DEEPPWD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPPWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPPWD_A {
        match self.bits {
            false => DEEPPWD_A::POWERUP,
            true => DEEPPWD_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERUP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        **self == DEEPPWD_A::POWERUP
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == DEEPPWD_A::POWERDOWN
    }
}
impl core::ops::Deref for DEEPPWD_R {
    type Target = crate::FieldReader<bool, DEEPPWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPPWD` writer - ADC deep power down enable"]
pub struct DEEPPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPPWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEEPPWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC not in deep power down"]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(DEEPPWD_A::POWERUP)
    }
    #[doc = "ADC in deep power down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(DEEPPWD_A::POWERDOWN)
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
#[doc = "ADC voltage regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADVREGEN_A {
    #[doc = "0: ADC voltage regulator disabled"]
    DISABLED = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    ENABLED = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable"]
pub struct ADVREGEN_R(crate::FieldReader<bool, ADVREGEN_A>);
impl ADVREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADVREGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::DISABLED,
            true => ADVREGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADVREGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADVREGEN_A::ENABLED
    }
}
impl core::ops::Deref for ADVREGEN_R {
    type Target = crate::FieldReader<bool, ADVREGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable"]
pub struct ADVREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVREGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADVREGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::DISABLED)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Linearity calibration ready Word 6"]
pub type LINCALRDYW6_A = LINCALRDYW1_A;
#[doc = "Field `LINCALRDYW6` reader - Linearity calibration ready Word 6"]
pub type LINCALRDYW6_R = LINCALRDYW1_R;
#[doc = "Field `LINCALRDYW6` writer - Linearity calibration ready Word 6"]
pub struct LINCALRDYW6_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW6_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW6_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Linearity calibration ready Word 5"]
pub type LINCALRDYW5_A = LINCALRDYW1_A;
#[doc = "Field `LINCALRDYW5` reader - Linearity calibration ready Word 5"]
pub type LINCALRDYW5_R = LINCALRDYW1_R;
#[doc = "Field `LINCALRDYW5` writer - Linearity calibration ready Word 5"]
pub struct LINCALRDYW5_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW5_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW5_A::SET)
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
#[doc = "Linearity calibration ready Word 4"]
pub type LINCALRDYW4_A = LINCALRDYW1_A;
#[doc = "Field `LINCALRDYW4` reader - Linearity calibration ready Word 4"]
pub type LINCALRDYW4_R = LINCALRDYW1_R;
#[doc = "Field `LINCALRDYW4` writer - Linearity calibration ready Word 4"]
pub struct LINCALRDYW4_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW4_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW4_A::SET)
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
#[doc = "Linearity calibration ready Word 3"]
pub type LINCALRDYW3_A = LINCALRDYW1_A;
#[doc = "Field `LINCALRDYW3` reader - Linearity calibration ready Word 3"]
pub type LINCALRDYW3_R = LINCALRDYW1_R;
#[doc = "Field `LINCALRDYW3` writer - Linearity calibration ready Word 3"]
pub struct LINCALRDYW3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW3_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW3_A::SET)
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
#[doc = "Linearity calibration ready Word 2"]
pub type LINCALRDYW2_A = LINCALRDYW1_A;
#[doc = "Field `LINCALRDYW2` reader - Linearity calibration ready Word 2"]
pub type LINCALRDYW2_R = LINCALRDYW1_R;
#[doc = "Field `LINCALRDYW2` writer - Linearity calibration ready Word 2"]
pub struct LINCALRDYW2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW2_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW2_A::SET)
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
#[doc = "Linearity calibration ready Word 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINCALRDYW1_A {
    #[doc = "0: LINCALFACT Word Read"]
    RESET = 0,
    #[doc = "1: LINCALFACT Word Write"]
    SET = 1,
}
impl From<LINCALRDYW1_A> for bool {
    #[inline(always)]
    fn from(variant: LINCALRDYW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCALRDYW1` reader - Linearity calibration ready Word 1"]
pub struct LINCALRDYW1_R(crate::FieldReader<bool, LINCALRDYW1_A>);
impl LINCALRDYW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCALRDYW1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINCALRDYW1_A {
        match self.bits {
            false => LINCALRDYW1_A::RESET,
            true => LINCALRDYW1_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == LINCALRDYW1_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == LINCALRDYW1_A::SET
    }
}
impl core::ops::Deref for LINCALRDYW1_R {
    type Target = crate::FieldReader<bool, LINCALRDYW1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALRDYW1` writer - Linearity calibration ready Word 1"]
pub struct LINCALRDYW1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALRDYW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINCALRDYW1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LINCALFACT Word Read"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LINCALRDYW1_A::RESET)
    }
    #[doc = "LINCALFACT Word Write"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LINCALRDYW1_A::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Linearity calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCALLIN_A {
    #[doc = "0: ADC calibration without linearaity calibration"]
    NOLINEARITY = 0,
    #[doc = "1: ADC calibration with linearaity calibration"]
    LINEARITY = 1,
}
impl From<ADCALLIN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALLIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALLIN` reader - Linearity calibration"]
pub struct ADCALLIN_R(crate::FieldReader<bool, ADCALLIN_A>);
impl ADCALLIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCALLIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALLIN_A {
        match self.bits {
            false => ADCALLIN_A::NOLINEARITY,
            true => ADCALLIN_A::LINEARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NOLINEARITY`"]
    #[inline(always)]
    pub fn is_no_linearity(&self) -> bool {
        **self == ADCALLIN_A::NOLINEARITY
    }
    #[doc = "Checks if the value of the field is `LINEARITY`"]
    #[inline(always)]
    pub fn is_linearity(&self) -> bool {
        **self == ADCALLIN_A::LINEARITY
    }
}
impl core::ops::Deref for ADCALLIN_R {
    type Target = crate::FieldReader<bool, ADCALLIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCALLIN` writer - Linearity calibration"]
pub struct ADCALLIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCALLIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCALLIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC calibration without linearaity calibration"]
    #[inline(always)]
    pub fn no_linearity(self) -> &'a mut W {
        self.variant(ADCALLIN_A::NOLINEARITY)
    }
    #[doc = "ADC calibration with linearaity calibration"]
    #[inline(always)]
    pub fn linearity(self) -> &'a mut W {
        self.variant(ADCALLIN_A::LINEARITY)
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
#[doc = "Boost mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOST_A {
    #[doc = "0: Boost mode used when ADC clock ≤ 6.25 MHz"]
    LT6_25 = 0,
    #[doc = "1: Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz"]
    LT12_5 = 1,
    #[doc = "2: Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz"]
    LT25 = 2,
    #[doc = "3: Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz"]
    LT50 = 3,
}
impl From<BOOST_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BOOST` reader - Boost mode control"]
pub struct BOOST_R(crate::FieldReader<u8, BOOST_A>);
impl BOOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        BOOST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOST_A {
        match self.bits {
            0 => BOOST_A::LT6_25,
            1 => BOOST_A::LT12_5,
            2 => BOOST_A::LT25,
            3 => BOOST_A::LT50,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LT6_25`"]
    #[inline(always)]
    pub fn is_lt6_25(&self) -> bool {
        **self == BOOST_A::LT6_25
    }
    #[doc = "Checks if the value of the field is `LT12_5`"]
    #[inline(always)]
    pub fn is_lt12_5(&self) -> bool {
        **self == BOOST_A::LT12_5
    }
    #[doc = "Checks if the value of the field is `LT25`"]
    #[inline(always)]
    pub fn is_lt25(&self) -> bool {
        **self == BOOST_A::LT25
    }
    #[doc = "Checks if the value of the field is `LT50`"]
    #[inline(always)]
    pub fn is_lt50(&self) -> bool {
        **self == BOOST_A::LT50
    }
}
impl core::ops::Deref for BOOST_R {
    type Target = crate::FieldReader<u8, BOOST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOST` writer - Boost mode control"]
pub struct BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Boost mode used when ADC clock ≤ 6.25 MHz"]
    #[inline(always)]
    pub fn lt6_25(self) -> &'a mut W {
        self.variant(BOOST_A::LT6_25)
    }
    #[doc = "Boost mode used when 6.25 MHz < ADC clock ≤ 12.5 MHz"]
    #[inline(always)]
    pub fn lt12_5(self) -> &'a mut W {
        self.variant(BOOST_A::LT12_5)
    }
    #[doc = "Boost mode used when 12.5 MHz < ADC clock ≤ 25.0 MHz"]
    #[inline(always)]
    pub fn lt25(self) -> &'a mut W {
        self.variant(BOOST_A::LT25)
    }
    #[doc = "Boost mode used when 25.0 MHz < ADC clock ≤ 50.0 MHz"]
    #[inline(always)]
    pub fn lt50(self) -> &'a mut W {
        self.variant(BOOST_A::LT50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "ADC group injected conversion stop"]
pub type JADSTP_A = ADSTP_A;
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub type JADSTP_R = ADSTP_R;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub struct JADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JADSTP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop conversion of channel"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(JADSTP_A::STOP)
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
#[doc = "ADC group regular conversion stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTP_A {
    #[doc = "1: Stop conversion of channel"]
    STOP = 1,
}
impl From<ADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub struct ADSTP_R(crate::FieldReader<bool, ADSTP_A>);
impl ADSTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADSTP_A> {
        match self.bits {
            true => Some(ADSTP_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == ADSTP_A::STOP
    }
}
impl core::ops::Deref for ADSTP_R {
    type Target = crate::FieldReader<bool, ADSTP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub struct ADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop conversion of channel"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(ADSTP_A::STOP)
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
#[doc = "ADC group injected conversion start"]
pub type JADSTART_A = ADSTART_A;
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub type JADSTART_R = ADSTART_R;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub struct JADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JADSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JADSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Starts conversion of channel"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JADSTART_A::START)
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
#[doc = "ADC group regular conversion start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTART_A {
    #[doc = "1: Starts conversion of channel"]
    START = 1,
}
impl From<ADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub struct ADSTART_R(crate::FieldReader<bool, ADSTART_A>);
impl ADSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADSTART_A> {
        match self.bits {
            true => Some(ADSTART_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == ADSTART_A::START
    }
}
impl core::ops::Deref for ADSTART_R {
    type Target = crate::FieldReader<bool, ADSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub struct ADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Starts conversion of channel"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ADSTART_A::START)
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
#[doc = "ADC disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIS_A {
    #[doc = "0: Disable ADC conversion and go to power down mode"]
    DISABLE = 0,
}
impl From<ADDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable"]
pub struct ADDIS_R(crate::FieldReader<bool, ADDIS_A>);
impl ADDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDIS_A> {
        match self.bits {
            false => Some(ADDIS_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADDIS_A::DISABLE
    }
}
impl core::ops::Deref for ADDIS_R {
    type Target = crate::FieldReader<bool, ADDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDIS` writer - ADC disable"]
pub struct ADDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable ADC conversion and go to power down mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDIS_A::DISABLE)
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
#[doc = "ADC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_A {
    #[doc = "1: Enable ADC"]
    ENABLE = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable"]
pub struct ADEN_R(crate::FieldReader<bool, ADEN_A>);
impl ADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADEN_A> {
        match self.bits {
            true => Some(ADEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADEN_A::ENABLE
    }
}
impl core::ops::Deref for ADEN_R {
    type Target = crate::FieldReader<bool, ADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADEN` writer - ADC enable"]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable ADC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADEN_A::ENABLE)
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
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> LINCALRDYW6_R {
        LINCALRDYW6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> LINCALRDYW5_R {
        LINCALRDYW5_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> LINCALRDYW4_R {
        LINCALRDYW4_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> LINCALRDYW3_R {
        LINCALRDYW3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> LINCALRDYW2_R {
        LINCALRDYW2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> LINCALRDYW1_R {
        LINCALRDYW1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&self) -> ADCALLIN_R {
        ADCALLIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&self) -> BOOST_R {
        BOOST_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W {
        ADCAL_W { w: self }
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&mut self) -> ADCALDIF_W {
        ADCALDIF_W { w: self }
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DEEPPWD_W {
        DEEPPWD_W { w: self }
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&mut self) -> ADVREGEN_W {
        ADVREGEN_W { w: self }
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&mut self) -> LINCALRDYW6_W {
        LINCALRDYW6_W { w: self }
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&mut self) -> LINCALRDYW5_W {
        LINCALRDYW5_W { w: self }
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&mut self) -> LINCALRDYW4_W {
        LINCALRDYW4_W { w: self }
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&mut self) -> LINCALRDYW3_W {
        LINCALRDYW3_W { w: self }
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&mut self) -> LINCALRDYW2_W {
        LINCALRDYW2_W { w: self }
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&mut self) -> LINCALRDYW1_W {
        LINCALRDYW1_W { w: self }
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&mut self) -> ADCALLIN_W {
        ADCALLIN_W { w: self }
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&mut self) -> BOOST_W {
        BOOST_W { w: self }
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&mut self) -> JADSTP_W {
        JADSTP_W { w: self }
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W {
        ADSTP_W { w: self }
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JADSTART_W {
        JADSTART_W { w: self }
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W {
        ADSTART_W { w: self }
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W {
        ADDIS_W { w: self }
    }
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0x2000_0000"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
