#[doc = "Reader of register COMP4_CSR"]
pub type R = crate::R<u32, super::COMP4_CSR>;
#[doc = "Writer for register COMP4_CSR"]
pub type W = crate::W<u32, super::COMP4_CSR>;
#[doc = "Register COMP4_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP4_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `COMP4EN`"]
pub type COMP4EN_R = crate::R<bool, COMP4EN_A>;
impl COMP4EN_R {
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
        *self == COMP4EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP4EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMP4EN`"]
pub struct COMP4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Reader of field `COMP4INMSEL`"]
pub type COMP4INMSEL_R = crate::R<u8, COMP4INMSEL_A>;
impl COMP4INMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP4INMSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP4INMSEL_A::ONEQUARTERVREF),
            1 => Val(COMP4INMSEL_A::ONEHALFVREF),
            2 => Val(COMP4INMSEL_A::THREEQUARTERVREF),
            3 => Val(COMP4INMSEL_A::VREF),
            4 => Val(COMP4INMSEL_A::PA4_DAC1_CH1),
            5 => Val(COMP4INMSEL_A::DAC1_CH2),
            7 => Val(COMP4INMSEL_A::PB2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP4INMSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP4INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP4INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP4INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == COMP4INMSEL_A::PB2
    }
}
#[doc = "Write proxy for field `COMP4INMSEL`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
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
#[doc = "Reader of field `COMP4OUTSEL`"]
pub type COMP4OUTSEL_R = crate::R<u8, COMP4OUTSEL_A>;
impl COMP4OUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP4OUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP4OUTSEL_A::NOSELECTION),
            1 => Val(COMP4OUTSEL_A::TIMER1BREAKINPUT),
            2 => Val(COMP4OUTSEL_A::TIMER1BREAKINPUT2),
            6 => Val(COMP4OUTSEL_A::TIMER3INPUTCAPTURE3),
            8 => Val(COMP4OUTSEL_A::TIMER15INPUTCAPTURE2),
            10 => Val(COMP4OUTSEL_A::TIMER15OCREFCLEARINPUT),
            11 => Val(COMP4OUTSEL_A::TIMER3OCREFCLEARINPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP4OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER1BREAKINPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUTCAPTURE3`"]
    #[inline(always)]
    pub fn is_timer3input_capture3(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER3INPUTCAPTURE3
    }
    #[doc = "Checks if the value of the field is `TIMER15INPUTCAPTURE2`"]
    #[inline(always)]
    pub fn is_timer15input_capture2(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER15INPUTCAPTURE2
    }
    #[doc = "Checks if the value of the field is `TIMER15OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer15ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER15OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL_A::TIMER3OCREFCLEARINPUT
    }
}
#[doc = "Write proxy for field `COMP4OUTSEL`"]
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
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
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
#[doc = "Reader of field `COMP4POL`"]
pub type COMP4POL_R = crate::R<bool, COMP4POL_A>;
impl COMP4POL_R {
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
        *self == COMP4POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP4POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `COMP4POL`"]
pub struct COMP4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
#[doc = "Reader of field `COMP4_BLANKING`"]
pub type COMP4_BLANKING_R = crate::R<u8, COMP4_BLANKING_A>;
impl COMP4_BLANKING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP4_BLANKING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP4_BLANKING_A::NOBLANKING),
            1 => Val(COMP4_BLANKING_A::TIM3OC4),
            3 => Val(COMP4_BLANKING_A::TIM15OC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOBLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP4_BLANKING_A::NOBLANKING
    }
    #[doc = "Checks if the value of the field is `TIM3OC4`"]
    #[inline(always)]
    pub fn is_tim3oc4(&self) -> bool {
        *self == COMP4_BLANKING_A::TIM3OC4
    }
    #[doc = "Checks if the value of the field is `TIM15OC1`"]
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        *self == COMP4_BLANKING_A::TIM15OC1
    }
}
#[doc = "Write proxy for field `COMP4_BLANKING`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
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
#[doc = "Reader of field `COMP4OUT`"]
pub type COMP4OUT_R = crate::R<bool, COMP4OUT_A>;
impl COMP4OUT_R {
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
        *self == COMP4OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP4OUT_A::HIGH
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
#[doc = "Reader of field `COMP4LOCK`"]
pub type COMP4LOCK_R = crate::R<bool, COMP4LOCK_A>;
impl COMP4LOCK_R {
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
        *self == COMP4LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP4LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `COMP4LOCK`"]
pub struct COMP4LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP4LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `COMP4MODE`"]
pub type COMP4MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP4MODE`"]
pub struct COMP4MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP4INPSEL`"]
pub type COMP4INPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP4INPSEL`"]
pub struct COMP4INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4INPSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `COMP4HYST`"]
pub type COMP4HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP4HYST`"]
pub struct COMP4HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP4HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> COMP4MODE_R {
        COMP4MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> COMP4INPSEL_R {
        COMP4INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> COMP4HYST_R {
        COMP4HYST_R::new(((self.bits >> 16) & 0x03) as u8)
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
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&mut self) -> COMP4MODE_W {
        COMP4MODE_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    pub fn comp4inpsel(&mut self) -> COMP4INPSEL_W {
        COMP4INPSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&mut self) -> COMP4HYST_W {
        COMP4HYST_W { w: self }
    }
}
