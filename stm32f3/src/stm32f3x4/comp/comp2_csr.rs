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
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub struct COMP2EN_R(crate::FieldReader<bool, COMP2EN_A>);
impl COMP2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::DISABLED,
            true => COMP2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMP2EN_A::ENABLED
    }
}
impl core::ops::Deref for COMP2EN_R {
    type Target = crate::FieldReader<bool, COMP2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::ENABLED)
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
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2INMSEL_A {
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
    #[doc = "6: PA2"]
    PA2 = 6,
}
impl From<COMP2INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2INMSEL` reader - Comparator 2 inverting input selection"]
pub struct COMP2INMSEL_R(crate::FieldReader<u8, COMP2INMSEL_A>);
impl COMP2INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2INMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INMSEL_A> {
        match self.bits {
            0 => Some(COMP2INMSEL_A::ONEQUARTERVREF),
            1 => Some(COMP2INMSEL_A::ONEHALFVREF),
            2 => Some(COMP2INMSEL_A::THREEQUARTERVREF),
            3 => Some(COMP2INMSEL_A::VREF),
            4 => Some(COMP2INMSEL_A::PA4_DAC1_CH1),
            5 => Some(COMP2INMSEL_A::DAC1_CH2),
            6 => Some(COMP2INMSEL_A::PA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        **self == COMP2INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        **self == COMP2INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        **self == COMP2INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        **self == COMP2INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        **self == COMP2INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        **self == COMP2INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        **self == COMP2INMSEL_A::PA2
    }
}
impl core::ops::Deref for COMP2INMSEL_R {
    type Target = crate::FieldReader<u8, COMP2INMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INMSEL` writer - Comparator 2 inverting input selection"]
pub struct COMP2INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2INMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::VREF)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::PA4_DAC1_CH1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::DAC1_CH2)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::PA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2OUTSEL_A {
    #[doc = "0: No selection"]
    NOSELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAKINPUT = 1,
    #[doc = "2: Timer 1 break input 2"]
    TIMER1BREAKINPUT2 = 2,
    #[doc = "6: Timer 1 OCREF_CLR input"]
    TIMER1OCREFCLEARINPUT = 6,
    #[doc = "7: Timer 1 input capture 1"]
    TIMER1INPUTCAPTURE1 = 7,
    #[doc = "8: Timer 2 input capture 4"]
    TIMER2INPUTCAPTURE4 = 8,
    #[doc = "9: Timer 2 OCREF_CLR input"]
    TIMER2OCREFCLEARINPUT = 9,
    #[doc = "10: Timer 3 input capture 1"]
    TIMER3INPUTCAPTURE1 = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    TIMER3OCREFCLEARINPUT = 11,
}
impl From<COMP2OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub struct COMP2OUTSEL_R(crate::FieldReader<u8, COMP2OUTSEL_A>);
impl COMP2OUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2OUTSEL_A> {
        match self.bits {
            0 => Some(COMP2OUTSEL_A::NOSELECTION),
            1 => Some(COMP2OUTSEL_A::TIMER1BREAKINPUT),
            2 => Some(COMP2OUTSEL_A::TIMER1BREAKINPUT2),
            6 => Some(COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT),
            7 => Some(COMP2OUTSEL_A::TIMER1INPUTCAPTURE1),
            8 => Some(COMP2OUTSEL_A::TIMER2INPUTCAPTURE4),
            9 => Some(COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT),
            10 => Some(COMP2OUTSEL_A::TIMER3INPUTCAPTURE1),
            11 => Some(COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        **self == COMP2OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER1BREAKINPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER1OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER1INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER2INPUTCAPTURE4
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER3INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        **self == COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT
    }
}
impl core::ops::Deref for COMP2OUTSEL_R {
    type Target = crate::FieldReader<u8, COMP2OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub struct COMP2OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2OUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::NOSELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1BREAKINPUT)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1BREAKINPUT2)
    }
    #[doc = "Timer 1 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT)
    }
    #[doc = "Timer 1 input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1INPUTCAPTURE1)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2INPUTCAPTURE4)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3INPUTCAPTURE1)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Comparator 2 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2POL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP2POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub struct COMP2POL_R(crate::FieldReader<bool, COMP2POL_A>);
impl COMP2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POL_A {
        match self.bits {
            false => COMP2POL_A::NOTINVERTED,
            true => COMP2POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        **self == COMP2POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == COMP2POL_A::INVERTED
    }
}
impl core::ops::Deref for COMP2POL_R {
    type Target = crate::FieldReader<bool, COMP2POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub struct COMP2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::INVERTED)
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
#[doc = "Comparator 2 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2_BLANKING_A {
    #[doc = "0: No blanking"]
    NOBLANKING = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    TIM1OC5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
    TIM2OC3 = 2,
    #[doc = "3: TIM3 OC3 selected as blanking source"]
    TIM3OC3 = 3,
}
impl From<COMP2_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub struct COMP2_BLANKING_R(crate::FieldReader<u8, COMP2_BLANKING_A>);
impl COMP2_BLANKING_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP2_BLANKING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2_BLANKING_A> {
        match self.bits {
            0 => Some(COMP2_BLANKING_A::NOBLANKING),
            1 => Some(COMP2_BLANKING_A::TIM1OC5),
            2 => Some(COMP2_BLANKING_A::TIM2OC3),
            3 => Some(COMP2_BLANKING_A::TIM3OC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        **self == COMP2_BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM1OC5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        **self == COMP2_BLANKING_A::TIM1OC5
    }
    #[doc = "Checks if the value of the field is `TIM2OC3`"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        **self == COMP2_BLANKING_A::TIM2OC3
    }
    #[doc = "Checks if the value of the field is `TIM3OC3`"]
    #[inline(always)]
    pub fn is_tim3oc3(&self) -> bool {
        **self == COMP2_BLANKING_A::TIM3OC3
    }
}
impl core::ops::Deref for COMP2_BLANKING_R {
    type Target = crate::FieldReader<u8, COMP2_BLANKING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub struct COMP2_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BLANKING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2_BLANKING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::NOBLANKING)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM1OC5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM2OC3)
    }
    #[doc = "TIM3 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM3OC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Comparator 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP2OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub struct COMP2OUT_R(crate::FieldReader<bool, COMP2OUT_A>);
impl COMP2OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2OUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUT_A {
        match self.bits {
            false => COMP2OUT_A::LOW,
            true => COMP2OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == COMP2OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == COMP2OUT_A::HIGH
    }
}
impl core::ops::Deref for COMP2OUT_R {
    type Target = crate::FieldReader<bool, COMP2OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub struct COMP2LOCK_R(crate::FieldReader<bool, COMP2LOCK_A>);
impl COMP2LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::UNLOCKED,
            true => COMP2LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == COMP2LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == COMP2LOCK_A::LOCKED
    }
}
impl core::ops::Deref for COMP2LOCK_R {
    type Target = crate::FieldReader<bool, COMP2LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub struct COMP2LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::LOCKED)
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
#[doc = "Field `COMP2INMSEL3` reader - Comparator 2 inverting input selection"]
pub struct COMP2INMSEL3_R(crate::FieldReader<bool, bool>);
impl COMP2INMSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2INMSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2INMSEL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2INMSEL3` writer - Comparator 2 inverting input selection"]
pub struct COMP2INMSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INMSEL3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&self) -> COMP2INMSEL3_R {
        COMP2INMSEL3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W {
        COMP2INMSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W {
        COMP2OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W {
        COMP2POL_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W {
        COMP2_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W {
        COMP2LOCK_W { w: self }
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&mut self) -> COMP2INMSEL3_W {
        COMP2INMSEL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
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
