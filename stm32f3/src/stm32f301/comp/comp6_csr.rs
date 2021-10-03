#[doc = "Register `COMP6_CSR` reader"]
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_CSR` writer"]
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<COMP6EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub struct COMP6EN_R(crate::FieldReader<bool, COMP6EN_A>);
impl COMP6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6EN_A {
        match self.bits {
            false => COMP6EN_A::DISABLED,
            true => COMP6EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMP6EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMP6EN_A::ENABLED
    }
}
impl core::ops::Deref for COMP6EN_R {
    type Target = crate::FieldReader<bool, COMP6EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub struct COMP6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::ENABLED)
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
#[doc = "Comparator 6 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6INMSEL_A {
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
    #[doc = "7: PB15"]
    PB15 = 7,
}
impl From<COMP6INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6INMSEL` reader - Comparator 6 inverting input selection"]
pub struct COMP6INMSEL_R(crate::FieldReader<u8, COMP6INMSEL_A>);
impl COMP6INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6INMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6INMSEL_A> {
        match self.bits {
            0 => Some(COMP6INMSEL_A::ONEQUARTERVREF),
            1 => Some(COMP6INMSEL_A::ONEHALFVREF),
            2 => Some(COMP6INMSEL_A::THREEQUARTERVREF),
            3 => Some(COMP6INMSEL_A::VREF),
            4 => Some(COMP6INMSEL_A::PA4_DAC1_CH1),
            5 => Some(COMP6INMSEL_A::DAC1_CH2),
            7 => Some(COMP6INMSEL_A::PB15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        **self == COMP6INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        **self == COMP6INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        **self == COMP6INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == COMP6INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        **self == COMP6INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        **self == COMP6INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        **self == COMP6INMSEL_A::PB15
    }
}
impl core::ops::Deref for COMP6INMSEL_R {
    type Target = crate::FieldReader<u8, COMP6INMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6INMSEL` writer - Comparator 6 inverting input selection"]
pub struct COMP6INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6INMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6INMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::VREF)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::PA4_DAC1_CH1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::DAC1_CH2)
    }
    #[doc = "PB15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::PB15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 6 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6OUTSEL_A {
    #[doc = "0: No selection"]
    NOSELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAKINPUT = 1,
    #[doc = "2: Timer 1 break input 2"]
    TIMER1BREAKINPUT2 = 2,
    #[doc = "6: Timer 2 input capture 2"]
    TIMER2INPUTCAPTURE2 = 6,
    #[doc = "8: Timer 2 OCREF_CLR input"]
    TIMER2OCREFCLEARINPUT = 8,
    #[doc = "9: Timer 16 OCREF_CLR input"]
    TIMER16OCREFCLEARINPUT = 9,
    #[doc = "10: Timer 16 input capture 1"]
    TIMER16INPUTCAPTURE1 = 10,
}
impl From<COMP6OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6OUTSEL` reader - Comparator 6 output selection"]
pub struct COMP6OUTSEL_R(crate::FieldReader<u8, COMP6OUTSEL_A>);
impl COMP6OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6OUTSEL_A> {
        match self.bits {
            0 => Some(COMP6OUTSEL_A::NOSELECTION),
            1 => Some(COMP6OUTSEL_A::TIMER1BREAKINPUT),
            2 => Some(COMP6OUTSEL_A::TIMER1BREAKINPUT2),
            6 => Some(COMP6OUTSEL_A::TIMER2INPUTCAPTURE2),
            8 => Some(COMP6OUTSEL_A::TIMER2OCREFCLEARINPUT),
            9 => Some(COMP6OUTSEL_A::TIMER16OCREFCLEARINPUT),
            10 => Some(COMP6OUTSEL_A::TIMER16INPUTCAPTURE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        **self == COMP6OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER1BREAKINPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE2`"]
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER2INPUTCAPTURE2
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER2OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER16OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        **self == COMP6OUTSEL_A::TIMER16INPUTCAPTURE1
    }
}
impl core::ops::Deref for COMP6OUTSEL_R {
    type Target = crate::FieldReader<u8, COMP6OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6OUTSEL` writer - Comparator 6 output selection"]
pub struct COMP6OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6OUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::NOSELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER1BREAKINPUT)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER1BREAKINPUT2)
    }
    #[doc = "Timer 2 input capture 2"]
    #[inline(always)]
    pub fn timer2input_capture2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER2INPUTCAPTURE2)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER2OCREFCLEARINPUT)
    }
    #[doc = "Timer 16 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer16ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER16OCREFCLEARINPUT)
    }
    #[doc = "Timer 16 input capture 1"]
    #[inline(always)]
    pub fn timer16input_capture1(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER16INPUTCAPTURE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Comparator 6 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6POL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP6POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub struct COMP6POL_R(crate::FieldReader<bool, COMP6POL_A>);
impl COMP6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6POL_A {
        match self.bits {
            false => COMP6POL_A::NOTINVERTED,
            true => COMP6POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == COMP6POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == COMP6POL_A::INVERTED
    }
}
impl core::ops::Deref for COMP6POL_R {
    type Target = crate::FieldReader<bool, COMP6POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub struct COMP6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::INVERTED)
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
#[doc = "Comparator 6 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6_BLANKING_A {
    #[doc = "0: No blanking"]
    NOBLANKING = 0,
    #[doc = "3: TIM2 OC4 selected as blanking source"]
    TIM2OC4 = 3,
    #[doc = "4: TIM15 OC2 selected as blanking source"]
    TIM15OC2 = 4,
}
impl From<COMP6_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub struct COMP6_BLANKING_R(crate::FieldReader<u8, COMP6_BLANKING_A>);
impl COMP6_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP6_BLANKING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6_BLANKING_A> {
        match self.bits {
            0 => Some(COMP6_BLANKING_A::NOBLANKING),
            3 => Some(COMP6_BLANKING_A::TIM2OC4),
            4 => Some(COMP6_BLANKING_A::TIM15OC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        **self == COMP6_BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM2OC4`"]
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        **self == COMP6_BLANKING_A::TIM2OC4
    }
    #[doc = "Checks if the value of the field is `TIM15OC2`"]
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        **self == COMP6_BLANKING_A::TIM15OC2
    }
}
impl core::ops::Deref for COMP6_BLANKING_R {
    type Target = crate::FieldReader<u8, COMP6_BLANKING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub struct COMP6_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6_BLANKING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6_BLANKING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::NOBLANKING)
    }
    #[doc = "TIM2 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc4(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::TIM2OC4)
    }
    #[doc = "TIM15 OC2 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc2(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::TIM15OC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Comparator 6 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP6OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub struct COMP6OUT_R(crate::FieldReader<bool, COMP6OUT_A>);
impl COMP6OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6OUT_A {
        match self.bits {
            false => COMP6OUT_A::LOW,
            true => COMP6OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == COMP6OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == COMP6OUT_A::HIGH
    }
}
impl core::ops::Deref for COMP6OUT_R {
    type Target = crate::FieldReader<bool, COMP6OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 6 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<COMP6LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub struct COMP6LOCK_R(crate::FieldReader<bool, COMP6LOCK_A>);
impl COMP6LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP6LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6LOCK_A {
        match self.bits {
            false => COMP6LOCK_A::UNLOCKED,
            true => COMP6LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == COMP6LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == COMP6LOCK_A::LOCKED
    }
}
impl core::ops::Deref for COMP6LOCK_R {
    type Target = crate::FieldReader<bool, COMP6LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub struct COMP6LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::LOCKED)
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
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel(&self) -> COMP6INMSEL_R {
        COMP6INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6outsel(&self) -> COMP6OUTSEL_R {
        COMP6OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&mut self) -> COMP6EN_W {
        COMP6EN_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel(&mut self) -> COMP6INMSEL_W {
        COMP6INMSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6outsel(&mut self) -> COMP6OUTSEL_W {
        COMP6OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&mut self) -> COMP6POL_W {
        COMP6POL_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W {
        COMP6_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W {
        COMP6LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](index.html) module"]
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_csr::R](R) reader structure"]
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](W) writer structure"]
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for COMP6_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
