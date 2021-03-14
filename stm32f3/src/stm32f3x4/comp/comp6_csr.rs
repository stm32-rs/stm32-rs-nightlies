#[doc = "Reader of register COMP6_CSR"]
pub type R = crate::R<u32, super::COMP6_CSR>;
#[doc = "Writer for register COMP6_CSR"]
pub type W = crate::W<u32, super::COMP6_CSR>;
#[doc = "Register COMP6_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP6_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `COMP6EN`"]
pub type COMP6EN_R = crate::R<bool, COMP6EN_A>;
impl COMP6EN_R {
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
        *self == COMP6EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP6EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMP6EN`"]
pub struct COMP6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `COMP6INMSEL`"]
pub type COMP6INMSEL_R = crate::R<u8, COMP6INMSEL_A>;
impl COMP6INMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP6INMSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP6INMSEL_A::ONEQUARTERVREF),
            1 => Val(COMP6INMSEL_A::ONEHALFVREF),
            2 => Val(COMP6INMSEL_A::THREEQUARTERVREF),
            3 => Val(COMP6INMSEL_A::VREF),
            4 => Val(COMP6INMSEL_A::PA4_DAC1_CH1),
            5 => Val(COMP6INMSEL_A::DAC1_CH2),
            7 => Val(COMP6INMSEL_A::PB15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP6INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP6INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP6INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == COMP6INMSEL_A::PB15
    }
}
#[doc = "Write proxy for field `COMP6INMSEL`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
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
#[doc = "Reader of field `COMP6OUTSEL`"]
pub type COMP6OUTSEL_R = crate::R<u8, COMP6OUTSEL_A>;
impl COMP6OUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP6OUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP6OUTSEL_A::NOSELECTION),
            1 => Val(COMP6OUTSEL_A::TIMER1BREAKINPUT),
            2 => Val(COMP6OUTSEL_A::TIMER1BREAKINPUT2),
            6 => Val(COMP6OUTSEL_A::TIMER2INPUTCAPTURE2),
            8 => Val(COMP6OUTSEL_A::TIMER2OCREFCLEARINPUT),
            9 => Val(COMP6OUTSEL_A::TIMER16OCREFCLEARINPUT),
            10 => Val(COMP6OUTSEL_A::TIMER16INPUTCAPTURE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP6OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER1BREAKINPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE2`"]
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER2INPUTCAPTURE2
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER2OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER16OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER16INPUTCAPTURE1
    }
}
#[doc = "Write proxy for field `COMP6OUTSEL`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
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
#[doc = "Reader of field `COMP6POL`"]
pub type COMP6POL_R = crate::R<bool, COMP6POL_A>;
impl COMP6POL_R {
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
        *self == COMP6POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP6POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `COMP6POL`"]
pub struct COMP6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `COMP6_BLANKING`"]
pub type COMP6_BLANKING_R = crate::R<u8, COMP6_BLANKING_A>;
impl COMP6_BLANKING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP6_BLANKING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP6_BLANKING_A::NOBLANKING),
            3 => Val(COMP6_BLANKING_A::TIM2OC4),
            4 => Val(COMP6_BLANKING_A::TIM15OC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP6_BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM2OC4`"]
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        *self == COMP6_BLANKING_A::TIM2OC4
    }
    #[doc = "Checks if the value of the field is `TIM15OC2`"]
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        *self == COMP6_BLANKING_A::TIM15OC2
    }
}
#[doc = "Write proxy for field `COMP6_BLANKING`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
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
#[doc = "Reader of field `COMP6OUT`"]
pub type COMP6OUT_R = crate::R<bool, COMP6OUT_A>;
impl COMP6OUT_R {
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
        *self == COMP6OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP6OUT_A::HIGH
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
#[doc = "Reader of field `COMP6LOCK`"]
pub type COMP6LOCK_R = crate::R<bool, COMP6LOCK_A>;
impl COMP6LOCK_R {
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
        *self == COMP6LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP6LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `COMP6LOCK`"]
pub struct COMP6LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP6LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COMP6INMSEL3`"]
pub type COMP6INMSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP6INMSEL3`"]
pub struct COMP6INMSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP6INMSEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
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
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel3(&self) -> COMP6INMSEL3_R {
        COMP6INMSEL3_R::new(((self.bits >> 22) & 0x01) != 0)
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
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel3(&mut self) -> COMP6INMSEL3_W {
        COMP6INMSEL3_W { w: self }
    }
}
