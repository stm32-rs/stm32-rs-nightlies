#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - CSR register lock bit"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - CSR register lock bit"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
#[doc = "Comparator 2 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALUE_A {
    #[doc = "0: Comparator output is low"]
    LOW = 0,
    #[doc = "1: Comparator output is high"]
    HIGH = 1,
}
impl From<VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALUE` reader - Comparator 2 output status bit"]
pub struct VALUE_R(crate::FieldReader<bool, VALUE_A>);
impl VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALUE_A {
        match self.bits {
            false => VALUE_A::LOW,
            true => VALUE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == VALUE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == VALUE_A::HIGH
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<bool, VALUE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "comparator 2 input minus extended selection bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INMESEL_A {
    #[doc = "0: PB2 connected to input minus"]
    PB2 = 0,
    #[doc = "1: PA10 connected to input minus"]
    PA10 = 1,
    #[doc = "2: PA11 connected to input minus"]
    PA11 = 2,
}
impl From<INMESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INMESEL` reader - comparator 2 input minus extended selection bits."]
pub struct INMESEL_R(crate::FieldReader<u8, INMESEL_A>);
impl INMESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INMESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INMESEL_A> {
        match self.bits {
            0 => Some(INMESEL_A::PB2),
            1 => Some(INMESEL_A::PA10),
            2 => Some(INMESEL_A::PA11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        **self == INMESEL_A::PB2
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        **self == INMESEL_A::PA10
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        **self == INMESEL_A::PA11
    }
}
impl core::ops::Deref for INMESEL_R {
    type Target = crate::FieldReader<u8, INMESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INMESEL` writer - comparator 2 input minus extended selection bits."]
pub struct INMESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INMESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INMESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PB2 connected to input minus"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(INMESEL_A::PB2)
    }
    #[doc = "PA10 connected to input minus"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(INMESEL_A::PA10)
    }
    #[doc = "PA11 connected to input minus"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(INMESEL_A::PA11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Voltage scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCALEN_A {
    #[doc = "0: Voltage scaler disabled"]
    DISABLED = 0,
    #[doc = "1: Voltage scaler enabled"]
    ENABLED = 1,
}
impl From<SCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub struct SCALEN_R(crate::FieldReader<bool, SCALEN_A>);
impl SCALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCALEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALEN_A {
        match self.bits {
            false => SCALEN_A::DISABLED,
            true => SCALEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SCALEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SCALEN_A::ENABLED
    }
}
impl core::ops::Deref for SCALEN_R {
    type Target = crate::FieldReader<bool, SCALEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub struct SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCALEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Voltage scaler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCALEN_A::DISABLED)
    }
    #[doc = "Voltage scaler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCALEN_A::ENABLED)
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
#[doc = "Scaler bridge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGEN_A {
    #[doc = "0: Scaler resistor bridge disabled"]
    DISABLED = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    ENABLED = 1,
}
impl From<BRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub struct BRGEN_R(crate::FieldReader<bool, BRGEN_A>);
impl BRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGEN_A {
        match self.bits {
            false => BRGEN_A::DISABLED,
            true => BRGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BRGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BRGEN_A::ENABLED
    }
}
impl core::ops::Deref for BRGEN_R {
    type Target = crate::FieldReader<bool, BRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub struct BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRGEN_A::DISABLED)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRGEN_A::ENABLED)
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
#[doc = "Comparator 2 blanking source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLANKING_A {
    #[doc = "0: No blanking"]
    NOBLANKING = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    TIM1OC5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
    TIM2OC3 = 2,
}
impl From<BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BLANKING` reader - Comparator 2 blanking source selection bits"]
pub struct BLANKING_R(crate::FieldReader<u8, BLANKING_A>);
impl BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLANKING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLANKING_A> {
        match self.bits {
            0 => Some(BLANKING_A::NOBLANKING),
            1 => Some(BLANKING_A::TIM1OC5),
            2 => Some(BLANKING_A::TIM2OC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        **self == BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM1OC5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        **self == BLANKING_A::TIM1OC5
    }
    #[doc = "Checks if the value of the field is `TIM2OC3`"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        **self == BLANKING_A::TIM2OC3
    }
}
impl core::ops::Deref for BLANKING_R {
    type Target = crate::FieldReader<u8, BLANKING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLANKING` writer - Comparator 2 blanking source selection bits"]
pub struct BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLANKING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(BLANKING_A::NOBLANKING)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(BLANKING_A::TIM1OC5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut W {
        self.variant(BLANKING_A::TIM2OC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Comparator 2 hysteresis selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    NOHYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOWHYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUMHYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGHHYSTERESIS = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selection bits"]
pub struct HYST_R(crate::FieldReader<u8, HYST_A>);
impl HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::NOHYSTERESIS,
            1 => HYST_A::LOWHYSTERESIS,
            2 => HYST_A::MEDIUMHYSTERESIS,
            3 => HYST_A::HIGHHYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOHYSTERESIS`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        **self == HYST_A::NOHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `LOWHYSTERESIS`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        **self == HYST_A::LOWHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `MEDIUMHYSTERESIS`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        **self == HYST_A::MEDIUMHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `HIGHHYSTERESIS`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        **self == HYST_A::HIGHHYSTERESIS
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, HYST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selection bits"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::NOHYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::LOWHYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::MEDIUMHYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::HIGHHYSTERESIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selection bit"]
pub struct POLARITY_R(crate::FieldReader<bool, POLARITY_A>);
impl POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLARITY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::NOTINVERTED,
            true => POLARITY_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == POLARITY_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == POLARITY_A::INVERTED
    }
}
impl core::ops::Deref for POLARITY_R {
    type Target = crate::FieldReader<bool, POLARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selection bit"]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::INVERTED)
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
#[doc = "Windows mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODE_A {
    #[doc = "0: COMP2 input plus is not connected to COMP1"]
    DISABLED = 0,
    #[doc = "1: COMP2 input plus is connected to COMP1"]
    ENABLED = 1,
}
impl From<WINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINMODE` reader - Windows mode selection bit"]
pub struct WINMODE_R(crate::FieldReader<bool, WINMODE_A>);
impl WINMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WINMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINMODE_A {
        match self.bits {
            false => WINMODE_A::DISABLED,
            true => WINMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WINMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WINMODE_A::ENABLED
    }
}
impl core::ops::Deref for WINMODE_R {
    type Target = crate::FieldReader<bool, WINMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WINMODE` writer - Windows mode selection bit"]
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COMP2 input plus is not connected to COMP1"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WINMODE_A::DISABLED)
    }
    #[doc = "COMP2 input plus is connected to COMP1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WINMODE_A::ENABLED)
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
#[doc = "Comparator 1 input plus selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: PB4 connected to input plus"]
    PB4 = 0,
    #[doc = "1: PB1 connected to input plus"]
    PB1 = 1,
    #[doc = "2: PA15 connected to input plus"]
    PA15 = 2,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPSEL` reader - Comparator 1 input plus selection bit"]
pub struct INPSEL_R(crate::FieldReader<u8, INPSEL_A>);
impl INPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPSEL_A> {
        match self.bits {
            0 => Some(INPSEL_A::PB4),
            1 => Some(INPSEL_A::PB1),
            2 => Some(INPSEL_A::PA15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PB4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        **self == INPSEL_A::PB4
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        **self == INPSEL_A::PB1
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        **self == INPSEL_A::PA15
    }
}
impl core::ops::Deref for INPSEL_R {
    type Target = crate::FieldReader<u8, INPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPSEL` writer - Comparator 1 input plus selection bit"]
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PB4 connected to input plus"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(INPSEL_A::PB4)
    }
    #[doc = "PB1 connected to input plus"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(INPSEL_A::PB1)
    }
    #[doc = "PA15 connected to input plus"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(INPSEL_A::PA15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Comparator 2 input minus selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONEQUARTERVREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONEHALFVREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREEQUARTERVREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: DAC Channel 1"]
    DAC_CH1 = 4,
    #[doc = "6: PB3"]
    PB3 = 6,
    #[doc = "7: GPIO pin selected by INMESEL"]
    GPIO = 7,
}
impl From<INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INMSEL` reader - Comparator 2 input minus selection bits"]
pub struct INMSEL_R(crate::FieldReader<u8, INMSEL_A>);
impl INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INMSEL_A> {
        match self.bits {
            0 => Some(INMSEL_A::ONEQUARTERVREF),
            1 => Some(INMSEL_A::ONEHALFVREF),
            2 => Some(INMSEL_A::THREEQUARTERVREF),
            3 => Some(INMSEL_A::VREF),
            4 => Some(INMSEL_A::DAC_CH1),
            6 => Some(INMSEL_A::PB3),
            7 => Some(INMSEL_A::GPIO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        **self == INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        **self == INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        **self == INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `DAC_CH1`"]
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        **self == INMSEL_A::DAC_CH1
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        **self == INMSEL_A::PB3
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        **self == INMSEL_A::GPIO
    }
}
impl core::ops::Deref for INMSEL_R {
    type Target = crate::FieldReader<u8, INMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INMSEL` writer - Comparator 2 input minus selection bits"]
pub struct INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(INMSEL_A::VREF)
    }
    #[doc = "DAC Channel 1"]
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut W {
        self.variant(INMSEL_A::DAC_CH1)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(INMSEL_A::PB3)
    }
    #[doc = "GPIO pin selected by INMESEL"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(INMSEL_A::GPIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Power Mode of the comparator 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRMODE_A {
    #[doc = "0: High speed / full power"]
    HIGHSPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUMSPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOWSPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERYLOWSPEED = 3,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRMODE` reader - Power Mode of the comparator 2"]
pub struct PWRMODE_R(crate::FieldReader<u8, PWRMODE_A>);
impl PWRMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRMODE_A {
        match self.bits {
            0 => PWRMODE_A::HIGHSPEED,
            1 => PWRMODE_A::MEDIUMSPEED,
            2 => PWRMODE_A::LOWSPEED,
            3 => PWRMODE_A::VERYLOWSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == PWRMODE_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        **self == PWRMODE_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == PWRMODE_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `VERYLOWSPEED`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        **self == PWRMODE_A::VERYLOWSPEED
    }
}
impl core::ops::Deref for PWRMODE_R {
    type Target = crate::FieldReader<u8, PWRMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 2"]
pub struct PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::HIGHSPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::MEDIUMSPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::LOWSPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::VERYLOWSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Comparator 1 disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator 1 enabled"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator 2 enable bit"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN_A::ENABLED
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Comparator 2 enable bit"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
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
    #[doc = "Bit 31 - CSR register lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - comparator 2 input minus extended selection bits."]
    #[inline(always)]
    pub fn inmesel(&self) -> INMESEL_R {
        INMESEL_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - Comparator 1 input plus selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 input minus selection bits"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CSR register lock bit"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bits 25:26 - comparator 2 input minus extended selection bits."]
    #[inline(always)]
    pub fn inmesel(&mut self) -> INMESEL_W {
        INMESEL_W { w: self }
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W {
        SCALEN_W { w: self }
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W {
        BRGEN_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W {
        BLANKING_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    #[doc = "Bits 7:8 - Comparator 1 input plus selection bit"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 input minus selection bits"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W {
        INMSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W {
        PWRMODE_W { w: self }
    }
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP2_CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
