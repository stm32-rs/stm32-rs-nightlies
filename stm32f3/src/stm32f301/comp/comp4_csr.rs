#[doc = "Register `COMP4_CSR` reader"]
pub struct R(crate::R<COMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP4_CSR` writer"]
pub struct W(crate::W<COMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP4_CSR_SPEC>;
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
impl From<crate::W<COMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<COMP4EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub struct COMP4EN_R(crate::FieldReader<bool, COMP4EN_A>);
impl COMP4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4EN_A {
        match self.bits {
            false => COMP4EN_A::DISABLED,
            true => COMP4EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMP4EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMP4EN_A::ENABLED
    }
}
impl core::ops::Deref for COMP4EN_R {
    type Target = crate::FieldReader<bool, COMP4EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub struct COMP4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP4EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP4EN_A::ENABLED)
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
#[doc = "Comparator 4 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONEQUARTERVREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONEHALFVREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREEQUARTERVREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: PA4 or DAC1_CH1 output if enabled"]
    PA4_DAC1_CH1 = 4,
    #[doc = "5: DAC1_CH2"]
    DAC1_CH2 = 5,
    #[doc = "7: PB2"]
    PB2 = 7,
}
impl From<COMP4INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4INMSEL` reader - Comparator 4 inverting input selection"]
pub struct COMP4INMSEL_R(crate::FieldReader<u8, COMP4INMSEL_A>);
impl COMP4INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4INMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4INMSEL_A> {
        match self.bits {
            0 => Some(COMP4INMSEL_A::ONEQUARTERVREF),
            1 => Some(COMP4INMSEL_A::ONEHALFVREF),
            2 => Some(COMP4INMSEL_A::THREEQUARTERVREF),
            3 => Some(COMP4INMSEL_A::VREF),
            4 => Some(COMP4INMSEL_A::PA4_DAC1_CH1),
            5 => Some(COMP4INMSEL_A::DAC1_CH2),
            7 => Some(COMP4INMSEL_A::PB2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        **self == COMP4INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        **self == COMP4INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        **self == COMP4INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == COMP4INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        **self == COMP4INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        **self == COMP4INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        **self == COMP4INMSEL_A::PB2
    }
}
impl core::ops::Deref for COMP4INMSEL_R {
    type Target = crate::FieldReader<u8, COMP4INMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4INMSEL` writer - Comparator 4 inverting input selection"]
pub struct COMP4INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4INMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4INMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::VREF)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::PA4_DAC1_CH1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::DAC1_CH2)
    }
    #[doc = "PB2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::PB2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 4 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4OUTSEL_A {
    #[doc = "0: No selection"]
    NOSELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAKINPUT = 1,
    #[doc = "2: Timer 1 break input 2"]
    TIMER1BREAKINPUT2 = 2,
    #[doc = "6: Timer 3 input capture 3"]
    TIMER3INPUTCAPTURE3 = 6,
    #[doc = "8: Timer 15 input capture 2"]
    TIMER15INPUTCAPTURE2 = 8,
    #[doc = "10: Timer 15 OCREF_CLR input"]
    TIMER15OCREFCLEARINPUT = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    TIMER3OCREFCLEARINPUT = 11,
}
impl From<COMP4OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4OUTSEL` reader - Comparator 4 output selection"]
pub struct COMP4OUTSEL_R(crate::FieldReader<u8, COMP4OUTSEL_A>);
impl COMP4OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4OUTSEL_A> {
        match self.bits {
            0 => Some(COMP4OUTSEL_A::NOSELECTION),
            1 => Some(COMP4OUTSEL_A::TIMER1BREAKINPUT),
            2 => Some(COMP4OUTSEL_A::TIMER1BREAKINPUT2),
            6 => Some(COMP4OUTSEL_A::TIMER3INPUTCAPTURE3),
            8 => Some(COMP4OUTSEL_A::TIMER15INPUTCAPTURE2),
            10 => Some(COMP4OUTSEL_A::TIMER15OCREFCLEARINPUT),
            11 => Some(COMP4OUTSEL_A::TIMER3OCREFCLEARINPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        **self == COMP4OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER1BREAKINPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUTCAPTURE3`"]
    #[inline(always)]
    pub fn is_timer3input_capture3(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER3INPUTCAPTURE3
    }
    #[doc = "Checks if the value of the field is `TIMER15INPUTCAPTURE2`"]
    #[inline(always)]
    pub fn is_timer15input_capture2(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER15INPUTCAPTURE2
    }
    #[doc = "Checks if the value of the field is `TIMER15OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer15ocref_clear_input(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER15OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        **self == COMP4OUTSEL_A::TIMER3OCREFCLEARINPUT
    }
}
impl core::ops::Deref for COMP4OUTSEL_R {
    type Target = crate::FieldReader<u8, COMP4OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4OUTSEL` writer - Comparator 4 output selection"]
pub struct COMP4OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4OUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::NOSELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER1BREAKINPUT)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER1BREAKINPUT2)
    }
    #[doc = "Timer 3 input capture 3"]
    #[inline(always)]
    pub fn timer3input_capture3(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER3INPUTCAPTURE3)
    }
    #[doc = "Timer 15 input capture 2"]
    #[inline(always)]
    pub fn timer15input_capture2(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER15INPUTCAPTURE2)
    }
    #[doc = "Timer 15 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer15ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER15OCREFCLEARINPUT)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::TIMER3OCREFCLEARINPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Comparator 4 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4POL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP4POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub struct COMP4POL_R(crate::FieldReader<bool, COMP4POL_A>);
impl COMP4POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4POL_A {
        match self.bits {
            false => COMP4POL_A::NOTINVERTED,
            true => COMP4POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == COMP4POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == COMP4POL_A::INVERTED
    }
}
impl core::ops::Deref for COMP4POL_R {
    type Target = crate::FieldReader<bool, COMP4POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub struct COMP4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP4POL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP4POL_A::INVERTED)
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
#[doc = "Comparator 4 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4_BLANKING_A {
    #[doc = "0: No blanking"]
    NOBLANKING = 0,
    #[doc = "1: TIM3 OC4 selected as blanking source"]
    TIM3OC4 = 1,
    #[doc = "3: TIM15 OC1 selected as blanking source"]
    TIM15OC1 = 3,
}
impl From<COMP4_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub struct COMP4_BLANKING_R(crate::FieldReader<u8, COMP4_BLANKING_A>);
impl COMP4_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP4_BLANKING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4_BLANKING_A> {
        match self.bits {
            0 => Some(COMP4_BLANKING_A::NOBLANKING),
            1 => Some(COMP4_BLANKING_A::TIM3OC4),
            3 => Some(COMP4_BLANKING_A::TIM15OC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        **self == COMP4_BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM3OC4`"]
    #[inline(always)]
    pub fn is_tim3oc4(&self) -> bool {
        **self == COMP4_BLANKING_A::TIM3OC4
    }
    #[doc = "Checks if the value of the field is `TIM15OC1`"]
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        **self == COMP4_BLANKING_A::TIM15OC1
    }
}
impl core::ops::Deref for COMP4_BLANKING_R {
    type Target = crate::FieldReader<u8, COMP4_BLANKING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub struct COMP4_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4_BLANKING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4_BLANKING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::NOBLANKING)
    }
    #[doc = "TIM3 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc4(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::TIM3OC4)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc1(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::TIM15OC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Comparator 4 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP4OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub struct COMP4OUT_R(crate::FieldReader<bool, COMP4OUT_A>);
impl COMP4OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4OUT_A {
        match self.bits {
            false => COMP4OUT_A::LOW,
            true => COMP4OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == COMP4OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == COMP4OUT_A::HIGH
    }
}
impl core::ops::Deref for COMP4OUT_R {
    type Target = crate::FieldReader<bool, COMP4OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 4 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<COMP4LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub struct COMP4LOCK_R(crate::FieldReader<bool, COMP4LOCK_A>);
impl COMP4LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP4LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4LOCK_A {
        match self.bits {
            false => COMP4LOCK_A::UNLOCKED,
            true => COMP4LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == COMP4LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == COMP4LOCK_A::LOCKED
    }
}
impl core::ops::Deref for COMP4LOCK_R {
    type Target = crate::FieldReader<bool, COMP4LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub struct COMP4LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP4LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP4LOCK_A::LOCKED)
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
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> COMP4EN_R {
        COMP4EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel(&self) -> COMP4INMSEL_R {
        COMP4INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4outsel(&self) -> COMP4OUTSEL_R {
        COMP4OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> COMP4POL_R {
        COMP4POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> COMP4_BLANKING_R {
        COMP4_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> COMP4OUT_R {
        COMP4OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> COMP4LOCK_R {
        COMP4LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&mut self) -> COMP4EN_W {
        COMP4EN_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel(&mut self) -> COMP4INMSEL_W {
        COMP4INMSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4outsel(&mut self) -> COMP4OUTSEL_W {
        COMP4OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&mut self) -> COMP4POL_W {
        COMP4POL_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&mut self) -> COMP4_BLANKING_W {
        COMP4_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&mut self) -> COMP4LOCK_W {
        COMP4LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4_csr](index.html) module"]
pub struct COMP4_CSR_SPEC;
impl crate::RegisterSpec for COMP4_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp4_csr::R](R) reader structure"]
impl crate::Readable for COMP4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp4_csr::W](W) writer structure"]
impl crate::Writable for COMP4_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for COMP4_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
