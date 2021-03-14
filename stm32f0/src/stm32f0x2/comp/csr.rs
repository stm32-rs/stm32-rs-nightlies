#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Writer for register CSR"]
pub type W = crate::W<u32, super::CSR>;
#[doc = "Register CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1EN_A {
    #[doc = "0: Comparator 1 disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator 1 enabled"]
    ENABLED = 1,
}
impl From<COMP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1EN`"]
pub type COMP1EN_R = crate::R<bool, COMP1EN_A>;
impl COMP1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1EN_A {
        match self.bits {
            false => COMP1EN_A::DISABLED,
            true => COMP1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMP1EN`"]
pub struct COMP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::DISABLED)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP1EN_A::ENABLED)
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
#[doc = "Comparator 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP1MODE_A {
    #[doc = "0: High speed / full power"]
    HIGHSPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUMSPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOWSPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERYLOWSPEED = 3,
}
impl From<COMP1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP1MODE`"]
pub type COMP1MODE_R = crate::R<u8, COMP1MODE_A>;
impl COMP1MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1MODE_A {
        match self.bits {
            0 => COMP1MODE_A::HIGHSPEED,
            1 => COMP1MODE_A::MEDIUMSPEED,
            2 => COMP1MODE_A::LOWSPEED,
            3 => COMP1MODE_A::VERYLOWSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP1MODE_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP1MODE_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP1MODE_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `VERYLOWSPEED`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP1MODE_A::VERYLOWSPEED
    }
}
#[doc = "Write proxy for field `COMP1MODE`"]
pub struct COMP1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::HIGHSPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::MEDIUMSPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::LOWSPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(COMP1MODE_A::VERYLOWSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Comparator 1 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP1INSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONEQUARTERVREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONEHALFVREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREEQUARTERVREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    COMP1_INM4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    COMP1_INM5 = 5,
    #[doc = "6: COMP1_INM6 (PA0)"]
    COMP1_INM6 = 6,
}
impl From<COMP1INSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP1INSEL`"]
pub type COMP1INSEL_R = crate::R<u8, COMP1INSEL_A>;
impl COMP1INSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP1INSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP1INSEL_A::ONEQUARTERVREF),
            1 => Val(COMP1INSEL_A::ONEHALFVREF),
            2 => Val(COMP1INSEL_A::THREEQUARTERVREF),
            3 => Val(COMP1INSEL_A::VREF),
            4 => Val(COMP1INSEL_A::COMP1_INM4),
            5 => Val(COMP1INSEL_A::COMP1_INM5),
            6 => Val(COMP1INSEL_A::COMP1_INM6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP1INSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP1INSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP1INSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP1INSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `COMP1_INM4`"]
    #[inline(always)]
    pub fn is_comp1_inm4(&self) -> bool {
        *self == COMP1INSEL_A::COMP1_INM4
    }
    #[doc = "Checks if the value of the field is `COMP1_INM5`"]
    #[inline(always)]
    pub fn is_comp1_inm5(&self) -> bool {
        *self == COMP1INSEL_A::COMP1_INM5
    }
    #[doc = "Checks if the value of the field is `COMP1_INM6`"]
    #[inline(always)]
    pub fn is_comp1_inm6(&self) -> bool {
        *self == COMP1INSEL_A::COMP1_INM6
    }
}
#[doc = "Write proxy for field `COMP1INSEL`"]
pub struct COMP1INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1INSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1INSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::VREF)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp1_inm4(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::COMP1_INM4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp1_inm5(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::COMP1_INM5)
    }
    #[doc = "COMP1_INM6 (PA0)"]
    #[inline(always)]
    pub fn comp1_inm6(self) -> &'a mut W {
        self.variant(COMP1INSEL_A::COMP1_INM6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 1 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP1OUTSEL_A {
    #[doc = "0: No selection"]
    NOSELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAKINPUT = 1,
    #[doc = "2: Timer 1 Input capture 1"]
    TIMER1INPUTCAPTURE1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    TIMER1OCREFCLEARINPUT = 3,
    #[doc = "4: Timer 2 input capture 4"]
    TIMER2INPUTCAPTURE4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    TIMER2OCREFCLEARINPUT = 5,
    #[doc = "6: Timer 3 input capture 1"]
    TIMER3INPUTCAPTURE1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    TIMER3OCREFCLEARINPUT = 7,
}
impl From<COMP1OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP1OUTSEL`"]
pub type COMP1OUTSEL_R = crate::R<u8, COMP1OUTSEL_A>;
impl COMP1OUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1OUTSEL_A {
        match self.bits {
            0 => COMP1OUTSEL_A::NOSELECTION,
            1 => COMP1OUTSEL_A::TIMER1BREAKINPUT,
            2 => COMP1OUTSEL_A::TIMER1INPUTCAPTURE1,
            3 => COMP1OUTSEL_A::TIMER1OCREFCLEARINPUT,
            4 => COMP1OUTSEL_A::TIMER2INPUTCAPTURE4,
            5 => COMP1OUTSEL_A::TIMER2OCREFCLEARINPUT,
            6 => COMP1OUTSEL_A::TIMER3INPUTCAPTURE1,
            7 => COMP1OUTSEL_A::TIMER3OCREFCLEARINPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP1OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER1INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER1OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER1OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER2INPUTCAPTURE4
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER2OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER3INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP1OUTSEL_A::TIMER3OCREFCLEARINPUT
    }
}
#[doc = "Write proxy for field `COMP1OUTSEL`"]
pub struct COMP1OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1OUTSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::NOSELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER1BREAKINPUT)
    }
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER1INPUTCAPTURE1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER1OCREFCLEARINPUT)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER2INPUTCAPTURE4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER2OCREFCLEARINPUT)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER3INPUTCAPTURE1)
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP1OUTSEL_A::TIMER3OCREFCLEARINPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Comparator 1 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1POL_A {
    #[doc = "0: Output is not inverted"]
    NOTINVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP1POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1POL`"]
pub type COMP1POL_R = crate::R<bool, COMP1POL_A>;
impl COMP1POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1POL_A {
        match self.bits {
            false => COMP1POL_A::NOTINVERTED,
            true => COMP1POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `COMP1POL`"]
pub struct COMP1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP1POL_A::NOTINVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP1POL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Comparator 1 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP1HYST_A {
    #[doc = "0: No hysteresis"]
    NOHYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOWHYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUMHYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGHHYSTERESIS = 3,
}
impl From<COMP1HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP1HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP1HYST`"]
pub type COMP1HYST_R = crate::R<u8, COMP1HYST_A>;
impl COMP1HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1HYST_A {
        match self.bits {
            0 => COMP1HYST_A::NOHYSTERESIS,
            1 => COMP1HYST_A::LOWHYSTERESIS,
            2 => COMP1HYST_A::MEDIUMHYSTERESIS,
            3 => COMP1HYST_A::HIGHHYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOHYSTERESIS`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::NOHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `LOWHYSTERESIS`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::LOWHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `MEDIUMHYSTERESIS`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::MEDIUMHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `HIGHHYSTERESIS`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP1HYST_A::HIGHHYSTERESIS
    }
}
#[doc = "Write proxy for field `COMP1HYST`"]
pub struct COMP1HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1HYST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::NOHYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::LOWHYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::MEDIUMHYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(COMP1HYST_A::HIGHHYSTERESIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Comparator 1 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP1OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1OUT`"]
pub type COMP1OUT_R = crate::R<bool, COMP1OUT_A>;
impl COMP1OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1OUT_A {
        match self.bits {
            false => COMP1OUT_A::LOW,
            true => COMP1OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP1OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP1OUT_A::HIGH
    }
}
#[doc = "Comparator 1 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1LOCK_A {
    #[doc = "0: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    LOCKED = 1,
}
impl From<COMP1LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1LOCK`"]
pub type COMP1LOCK_R = crate::R<bool, COMP1LOCK_A>;
impl COMP1LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1LOCK_A {
        match self.bits {
            false => COMP1LOCK_A::UNLOCKED,
            true => COMP1LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP1LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP1LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `COMP1LOCK`"]
pub struct COMP1LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator 1 CSR bits (CSR\\[15:0\\]) are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP1LOCK_A::LOCKED)
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
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator 2 disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator 2 enabled"]
    ENABLED = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP2EN`"]
pub type COMP2EN_R = crate::R<bool, COMP2EN_A>;
impl COMP2EN_R {
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
        *self == COMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMP2EN`"]
pub struct COMP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::DISABLED)
    }
    #[doc = "Comparator 2 enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Comparator 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2MODE_A {
    #[doc = "0: High speed / full power"]
    HIGHSPEED = 0,
    #[doc = "1: Medium speed / medium power"]
    MEDIUMSPEED = 1,
    #[doc = "2: Low speed / low power"]
    LOWSPEED = 2,
    #[doc = "3: Very-low speed / ultra-low power"]
    VERYLOWSPEED = 3,
}
impl From<COMP2MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP2MODE`"]
pub type COMP2MODE_R = crate::R<u8, COMP2MODE_A>;
impl COMP2MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2MODE_A {
        match self.bits {
            0 => COMP2MODE_A::HIGHSPEED,
            1 => COMP2MODE_A::MEDIUMSPEED,
            2 => COMP2MODE_A::LOWSPEED,
            3 => COMP2MODE_A::VERYLOWSPEED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == COMP2MODE_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `MEDIUMSPEED`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == COMP2MODE_A::MEDIUMSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == COMP2MODE_A::LOWSPEED
    }
    #[doc = "Checks if the value of the field is `VERYLOWSPEED`"]
    #[inline(always)]
    pub fn is_very_low_speed(&self) -> bool {
        *self == COMP2MODE_A::VERYLOWSPEED
    }
}
#[doc = "Write proxy for field `COMP2MODE`"]
pub struct COMP2MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::HIGHSPEED)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::MEDIUMSPEED)
    }
    #[doc = "Low speed / low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::LOWSPEED)
    }
    #[doc = "Very-low speed / ultra-low power"]
    #[inline(always)]
    pub fn very_low_speed(self) -> &'a mut W {
        self.variant(COMP2MODE_A::VERYLOWSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2INSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONEQUARTERVREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONEHALFVREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREEQUARTERVREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    COMP2_INM4 = 4,
    #[doc = "5: COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    COMP2_INM5 = 5,
    #[doc = "6: COMP1_INM6 (PA2)"]
    COMP2_INM6 = 6,
}
impl From<COMP2INSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP2INSEL`"]
pub type COMP2INSEL_R = crate::R<u8, COMP2INSEL_A>;
impl COMP2INSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP2INSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP2INSEL_A::ONEQUARTERVREF),
            1 => Val(COMP2INSEL_A::ONEHALFVREF),
            2 => Val(COMP2INSEL_A::THREEQUARTERVREF),
            3 => Val(COMP2INSEL_A::VREF),
            4 => Val(COMP2INSEL_A::COMP2_INM4),
            5 => Val(COMP2INSEL_A::COMP2_INM5),
            6 => Val(COMP2INSEL_A::COMP2_INM6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INSEL_A::ONEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `ONEHALFVREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INSEL_A::ONEHALFVREF
    }
    #[doc = "Checks if the value of the field is `THREEQUARTERVREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INSEL_A::THREEQUARTERVREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `COMP2_INM4`"]
    #[inline(always)]
    pub fn is_comp2_inm4(&self) -> bool {
        *self == COMP2INSEL_A::COMP2_INM4
    }
    #[doc = "Checks if the value of the field is `COMP2_INM5`"]
    #[inline(always)]
    pub fn is_comp2_inm5(&self) -> bool {
        *self == COMP2INSEL_A::COMP2_INM5
    }
    #[doc = "Checks if the value of the field is `COMP2_INM6`"]
    #[inline(always)]
    pub fn is_comp2_inm6(&self) -> bool {
        *self == COMP2INSEL_A::COMP2_INM6
    }
}
#[doc = "Write proxy for field `COMP2INSEL`"]
pub struct COMP2INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2INSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2INSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::ONEQUARTERVREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::ONEHALFVREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::THREEQUARTERVREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::VREF)
    }
    #[doc = "COMP1_INM4 (PA4 with DAC_OUT1 if enabled)"]
    #[inline(always)]
    pub fn comp2_inm4(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::COMP2_INM4)
    }
    #[doc = "COMP1_INM5 (PA5 with DAC_OUT2 if present and enabled)"]
    #[inline(always)]
    pub fn comp2_inm5(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::COMP2_INM5)
    }
    #[doc = "COMP1_INM6 (PA2)"]
    #[inline(always)]
    pub fn comp2_inm6(self) -> &'a mut W {
        self.variant(COMP2INSEL_A::COMP2_INM6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Window mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WNDWEN_A {
    #[doc = "0: Window mode disabled"]
    DISABLED = 0,
    #[doc = "1: Window mode enabled"]
    ENABLED = 1,
}
impl From<WNDWEN_A> for bool {
    #[inline(always)]
    fn from(variant: WNDWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WNDWEN`"]
pub type WNDWEN_R = crate::R<bool, WNDWEN_A>;
impl WNDWEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WNDWEN_A {
        match self.bits {
            false => WNDWEN_A::DISABLED,
            true => WNDWEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WNDWEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WNDWEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `WNDWEN`"]
pub struct WNDWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WNDWEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WNDWEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Window mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WNDWEN_A::DISABLED)
    }
    #[doc = "Window mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WNDWEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
    #[doc = "2: Timer 1 Input capture 1"]
    TIMER1INPUTCAPTURE1 = 2,
    #[doc = "3: Timer 1 OCrefclear input"]
    TIMER1OCREFCLEARINPUT = 3,
    #[doc = "4: Timer 2 input capture 4"]
    TIMER2INPUTCAPTURE4 = 4,
    #[doc = "5: Timer 2 OCrefclear input"]
    TIMER2OCREFCLEARINPUT = 5,
    #[doc = "6: Timer 3 input capture 1"]
    TIMER3INPUTCAPTURE1 = 6,
    #[doc = "7: Timer 3 OCrefclear input"]
    TIMER3OCREFCLEARINPUT = 7,
}
impl From<COMP2OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP2OUTSEL`"]
pub type COMP2OUTSEL_R = crate::R<u8, COMP2OUTSEL_A>;
impl COMP2OUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUTSEL_A {
        match self.bits {
            0 => COMP2OUTSEL_A::NOSELECTION,
            1 => COMP2OUTSEL_A::TIMER1BREAKINPUT,
            2 => COMP2OUTSEL_A::TIMER1INPUTCAPTURE1,
            3 => COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT,
            4 => COMP2OUTSEL_A::TIMER2INPUTCAPTURE4,
            5 => COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT,
            6 => COMP2OUTSEL_A::TIMER3INPUTCAPTURE1,
            7 => COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOSELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL_A::NOSELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAKINPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1BREAKINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER1OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUTCAPTURE4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER2INPUTCAPTURE4
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUTCAPTURE1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER3INPUTCAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREFCLEARINPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT
    }
}
#[doc = "Write proxy for field `COMP2OUTSEL`"]
pub struct COMP2OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2OUTSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
    #[doc = "Timer 1 Input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1INPUTCAPTURE1)
    }
    #[doc = "Timer 1 OCrefclear input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1OCREFCLEARINPUT)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2INPUTCAPTURE4)
    }
    #[doc = "Timer 2 OCrefclear input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2OCREFCLEARINPUT)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3INPUTCAPTURE1)
    }
    #[doc = "Timer 3 OCrefclear input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3OCREFCLEARINPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
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
#[doc = "Reader of field `COMP2POL`"]
pub type COMP2POL_R = crate::R<bool, COMP2POL_A>;
impl COMP2POL_R {
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
        *self == COMP2POL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `COMP2POL`"]
pub struct COMP2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Comparator 2 hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2HYST_A {
    #[doc = "0: No hysteresis"]
    NOHYSTERESIS = 0,
    #[doc = "1: Low hysteresis"]
    LOWHYSTERESIS = 1,
    #[doc = "2: Medium hysteresis"]
    MEDIUMHYSTERESIS = 2,
    #[doc = "3: High hysteresis"]
    HIGHHYSTERESIS = 3,
}
impl From<COMP2HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP2HYST`"]
pub type COMP2HYST_R = crate::R<u8, COMP2HYST_A>;
impl COMP2HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2HYST_A {
        match self.bits {
            0 => COMP2HYST_A::NOHYSTERESIS,
            1 => COMP2HYST_A::LOWHYSTERESIS,
            2 => COMP2HYST_A::MEDIUMHYSTERESIS,
            3 => COMP2HYST_A::HIGHHYSTERESIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOHYSTERESIS`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::NOHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `LOWHYSTERESIS`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::LOWHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `MEDIUMHYSTERESIS`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::MEDIUMHYSTERESIS
    }
    #[doc = "Checks if the value of the field is `HIGHHYSTERESIS`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == COMP2HYST_A::HIGHHYSTERESIS
    }
}
#[doc = "Write proxy for field `COMP2HYST`"]
pub struct COMP2HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2HYST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::NOHYSTERESIS)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::LOWHYSTERESIS)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::MEDIUMHYSTERESIS)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(COMP2HYST_A::HIGHHYSTERESIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
#[doc = "Reader of field `COMP2OUT`"]
pub type COMP2OUT_R = crate::R<bool, COMP2OUT_A>;
impl COMP2OUT_R {
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
        *self == COMP2OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT_A::HIGH
    }
}
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LOCK_A {
    #[doc = "0: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
    LOCKED = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP2LOCK`"]
pub type COMP2LOCK_R = crate::R<bool, COMP2LOCK_A>;
impl COMP2LOCK_R {
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
        *self == COMP2LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `COMP2LOCK`"]
pub struct COMP2LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator 2 CSR bits (CSR\\[31:16\\]) are read-only"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Comparator 1 non inverting input DAC switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1SW1_A {
    #[doc = "0: Switch open"]
    OPEN = 0,
    #[doc = "1: Switch closed"]
    CLOSED = 1,
}
impl From<COMP1SW1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1SW1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1SW1`"]
pub type COMP1SW1_R = crate::R<bool, COMP1SW1_A>;
impl COMP1SW1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1SW1_A {
        match self.bits {
            false => COMP1SW1_A::OPEN,
            true => COMP1SW1_A::CLOSED,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN`"]
    #[inline(always)]
    pub fn is_open(&self) -> bool {
        *self == COMP1SW1_A::OPEN
    }
    #[doc = "Checks if the value of the field is `CLOSED`"]
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        *self == COMP1SW1_A::CLOSED
    }
}
#[doc = "Write proxy for field `COMP1SW1`"]
pub struct COMP1SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1SW1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1SW1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Switch open"]
    #[inline(always)]
    pub fn open(self) -> &'a mut W {
        self.variant(COMP1SW1_A::OPEN)
    }
    #[doc = "Switch closed"]
    #[inline(always)]
    pub fn closed(self) -> &'a mut W {
        self.variant(COMP1SW1_A::CLOSED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 0x03) as u8)
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
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    pub fn comp1sw1(&self) -> COMP1SW1_R {
        COMP1SW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&mut self) -> COMP1EN_W {
        COMP1EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&mut self) -> COMP1MODE_W {
        COMP1MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W {
        COMP1INSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W {
        COMP1OUTSEL_W { w: self }
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&mut self) -> COMP1POL_W {
        COMP1POL_W { w: self }
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W {
        COMP1HYST_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W {
        COMP1LOCK_W { w: self }
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W {
        COMP2EN_W { w: self }
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&mut self) -> COMP2MODE_W {
        COMP2MODE_W { w: self }
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W {
        COMP2INSEL_W { w: self }
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&mut self) -> WNDWEN_W {
        WNDWEN_W { w: self }
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W {
        COMP2OUTSEL_W { w: self }
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W {
        COMP2POL_W { w: self }
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W {
        COMP2HYST_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W {
        COMP2LOCK_W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input DAC switch"]
    #[inline(always)]
    pub fn comp1sw1(&mut self) -> COMP1SW1_W {
        COMP1SW1_W { w: self }
    }
}
