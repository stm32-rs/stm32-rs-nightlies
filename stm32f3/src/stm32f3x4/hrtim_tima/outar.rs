#[doc = "Reader of register OUTAR"]
pub type R = crate::R<u32, super::OUTAR>;
#[doc = "Writer for register OUTAR"]
pub type W = crate::W<u32, super::OUTAR>;
#[doc = "Register OUTAR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Output 2 Deadtime upon burst mode Idle entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIDL2_A {
    #[doc = "0: The programmed idle state is applied immediately to the output"]
    DISABLED = 0,
    #[doc = "1: Deadtime (inactive level) is inserted on output before entering the idle mode"]
    ENABLED = 1,
}
impl From<DIDL2_A> for bool {
    #[inline(always)]
    fn from(variant: DIDL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIDL2`"]
pub type DIDL2_R = crate::R<bool, DIDL2_A>;
impl DIDL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIDL2_A {
        match self.bits {
            false => DIDL2_A::DISABLED,
            true => DIDL2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIDL2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIDL2_A::ENABLED
    }
}
#[doc = "Write proxy for field `DIDL2`"]
pub struct DIDL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIDL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIDL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The programmed idle state is applied immediately to the output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIDL2_A::DISABLED)
    }
    #[doc = "Deadtime (inactive level) is inserted on output before entering the idle mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIDL2_A::ENABLED)
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
#[doc = "Output 2 Chopper enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHP2_A {
    #[doc = "0: Output signal not altered"]
    DISABLED = 0,
    #[doc = "1: Output signal is chopped by a carrier signal"]
    ENABLED = 1,
}
impl From<CHP2_A> for bool {
    #[inline(always)]
    fn from(variant: CHP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHP2`"]
pub type CHP2_R = crate::R<bool, CHP2_A>;
impl CHP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHP2_A {
        match self.bits {
            false => CHP2_A::DISABLED,
            true => CHP2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHP2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHP2_A::ENABLED
    }
}
#[doc = "Write proxy for field `CHP2`"]
pub struct CHP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output signal not altered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHP2_A::DISABLED)
    }
    #[doc = "Output signal is chopped by a carrier signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHP2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Output 2 Fault state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAULT2_A {
    #[doc = "0: No action: the output is not affected by the fault input and stays in run mode"]
    DISABLED = 0,
    #[doc = "1: Output goes to active state after a fault event"]
    SETACTIVE = 1,
    #[doc = "2: Output goes to inactive state after a fault event"]
    SETINACTIVE = 2,
    #[doc = "3: Output goes to high-z state after a fault event"]
    SETHIGHZ = 3,
}
impl From<FAULT2_A> for u8 {
    #[inline(always)]
    fn from(variant: FAULT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAULT2`"]
pub type FAULT2_R = crate::R<u8, FAULT2_A>;
impl FAULT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT2_A {
        match self.bits {
            0 => FAULT2_A::DISABLED,
            1 => FAULT2_A::SETACTIVE,
            2 => FAULT2_A::SETINACTIVE,
            3 => FAULT2_A::SETHIGHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == FAULT2_A::SETACTIVE
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == FAULT2_A::SETINACTIVE
    }
    #[doc = "Checks if the value of the field is `SETHIGHZ`"]
    #[inline(always)]
    pub fn is_set_high_z(&self) -> bool {
        *self == FAULT2_A::SETHIGHZ
    }
}
#[doc = "Write proxy for field `FAULT2`"]
pub struct FAULT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FAULT2_A::DISABLED)
    }
    #[doc = "Output goes to active state after a fault event"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(FAULT2_A::SETACTIVE)
    }
    #[doc = "Output goes to inactive state after a fault event"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(FAULT2_A::SETINACTIVE)
    }
    #[doc = "Output goes to high-z state after a fault event"]
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut W {
        self.variant(FAULT2_A::SETHIGHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Output 2 Idle State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLES2_A {
    #[doc = "0: Output idle state is inactive"]
    INACTIVE = 0,
    #[doc = "1: Output idle state is active"]
    ACTIVE = 1,
}
impl From<IDLES2_A> for bool {
    #[inline(always)]
    fn from(variant: IDLES2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLES2`"]
pub type IDLES2_R = crate::R<bool, IDLES2_A>;
impl IDLES2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLES2_A {
        match self.bits {
            false => IDLES2_A::INACTIVE,
            true => IDLES2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES2_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLES2_A::ACTIVE
    }
}
#[doc = "Write proxy for field `IDLES2`"]
pub struct IDLES2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLES2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLES2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output idle state is inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IDLES2_A::INACTIVE)
    }
    #[doc = "Output idle state is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLES2_A::ACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Output 2 Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEM2_A {
    #[doc = "0: No action: the output is not affected by the burst mode operation"]
    NOEFFECT = 0,
    #[doc = "1: The output is in idle state when requested by the burst mode controller"]
    SETIDLE = 1,
}
impl From<IDLEM2_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLEM2`"]
pub type IDLEM2_R = crate::R<bool, IDLEM2_A>;
impl IDLEM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEM2_A {
        match self.bits {
            false => IDLEM2_A::NOEFFECT,
            true => IDLEM2_A::SETIDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLEM2_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETIDLE`"]
    #[inline(always)]
    pub fn is_set_idle(&self) -> bool {
        *self == IDLEM2_A::SETIDLE
    }
}
#[doc = "Write proxy for field `IDLEM2`"]
pub struct IDLEM2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLEM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action: the output is not affected by the burst mode operation"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IDLEM2_A::NOEFFECT)
    }
    #[doc = "The output is in idle state when requested by the burst mode controller"]
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut W {
        self.variant(IDLEM2_A::SETIDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Output 2 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
    #[doc = "0: Positive polarity (output active high)"]
    ACTIVEHIGH = 0,
    #[doc = "1: Negative polarity (output active low)"]
    ACTIVELOW = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, POL2_A>;
impl POL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::ACTIVEHIGH,
            true => POL2_A::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL2_A::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL2_A::ACTIVELOW
    }
}
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Positive polarity (output active high)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL2_A::ACTIVEHIGH)
    }
    #[doc = "Negative polarity (output active low)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL2_A::ACTIVELOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Delayed Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLYPRT_A {
    #[doc = "0: Output 1 delayed idle on external event 6"]
    OUTPUT1_EE6 = 0,
    #[doc = "1: Output 2 delayed idle on external event 6"]
    OUTPUT2_EE6 = 1,
    #[doc = "2: Output 1 and 2 delayed idle on external event 6"]
    OUTPUT1_2_EE6 = 2,
    #[doc = "3: Balanced idle on external event 6"]
    BALANCED_EE6 = 3,
    #[doc = "4: Output 1 delayed idle on external event 7"]
    OUTPUT1_EE7 = 4,
    #[doc = "5: Output 2 delayed idle on external event 7"]
    OUTPUT2_EE7 = 5,
    #[doc = "6: Output 1 and 2 delayed idle on external event 7"]
    OUTPUT1_2_EE7 = 6,
    #[doc = "7: Balanced idle on external event 7"]
    BALANCED_EE7 = 7,
}
impl From<DLYPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DLYPRT`"]
pub type DLYPRT_R = crate::R<u8, DLYPRT_A>;
impl DLYPRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRT_A {
        match self.bits {
            0 => DLYPRT_A::OUTPUT1_EE6,
            1 => DLYPRT_A::OUTPUT2_EE6,
            2 => DLYPRT_A::OUTPUT1_2_EE6,
            3 => DLYPRT_A::BALANCED_EE6,
            4 => DLYPRT_A::OUTPUT1_EE7,
            5 => DLYPRT_A::OUTPUT2_EE7,
            6 => DLYPRT_A::OUTPUT1_2_EE7,
            7 => DLYPRT_A::BALANCED_EE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT1_EE6`"]
    #[inline(always)]
    pub fn is_output1_ee6(&self) -> bool {
        *self == DLYPRT_A::OUTPUT1_EE6
    }
    #[doc = "Checks if the value of the field is `OUTPUT2_EE6`"]
    #[inline(always)]
    pub fn is_output2_ee6(&self) -> bool {
        *self == DLYPRT_A::OUTPUT2_EE6
    }
    #[doc = "Checks if the value of the field is `OUTPUT1_2_EE6`"]
    #[inline(always)]
    pub fn is_output1_2_ee6(&self) -> bool {
        *self == DLYPRT_A::OUTPUT1_2_EE6
    }
    #[doc = "Checks if the value of the field is `BALANCED_EE6`"]
    #[inline(always)]
    pub fn is_balanced_ee6(&self) -> bool {
        *self == DLYPRT_A::BALANCED_EE6
    }
    #[doc = "Checks if the value of the field is `OUTPUT1_EE7`"]
    #[inline(always)]
    pub fn is_output1_ee7(&self) -> bool {
        *self == DLYPRT_A::OUTPUT1_EE7
    }
    #[doc = "Checks if the value of the field is `OUTPUT2_EE7`"]
    #[inline(always)]
    pub fn is_output2_ee7(&self) -> bool {
        *self == DLYPRT_A::OUTPUT2_EE7
    }
    #[doc = "Checks if the value of the field is `OUTPUT1_2_EE7`"]
    #[inline(always)]
    pub fn is_output1_2_ee7(&self) -> bool {
        *self == DLYPRT_A::OUTPUT1_2_EE7
    }
    #[doc = "Checks if the value of the field is `BALANCED_EE7`"]
    #[inline(always)]
    pub fn is_balanced_ee7(&self) -> bool {
        *self == DLYPRT_A::BALANCED_EE7
    }
}
#[doc = "Write proxy for field `DLYPRT`"]
pub struct DLYPRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYPRT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output 1 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT1_EE6)
    }
    #[doc = "Output 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output2_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT2_EE6)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 6"]
    #[inline(always)]
    pub fn output1_2_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT1_2_EE6)
    }
    #[doc = "Balanced idle on external event 6"]
    #[inline(always)]
    pub fn balanced_ee6(self) -> &'a mut W {
        self.variant(DLYPRT_A::BALANCED_EE6)
    }
    #[doc = "Output 1 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT1_EE7)
    }
    #[doc = "Output 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output2_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT2_EE7)
    }
    #[doc = "Output 1 and 2 delayed idle on external event 7"]
    #[inline(always)]
    pub fn output1_2_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::OUTPUT1_2_EE7)
    }
    #[doc = "Balanced idle on external event 7"]
    #[inline(always)]
    pub fn balanced_ee7(self) -> &'a mut W {
        self.variant(DLYPRT_A::BALANCED_EE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Delayed Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYPRTEN_A {
    #[doc = "0: No action"]
    DISABLED = 0,
    #[doc = "1: Delayed protection is enabled, as per DLYPRT bits"]
    ENABLED = 1,
}
impl From<DLYPRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLYPRTEN`"]
pub type DLYPRTEN_R = crate::R<bool, DLYPRTEN_A>;
impl DLYPRTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTEN_A {
        match self.bits {
            false => DLYPRTEN_A::DISABLED,
            true => DLYPRTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DLYPRTEN`"]
pub struct DLYPRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYPRTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::DISABLED)
    }
    #[doc = "Delayed protection is enabled, as per DLYPRT bits"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Deadtime enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN_A {
    #[doc = "0: Output 1 and 2 signals are independent"]
    DISABLED = 0,
    #[doc = "1: Deadtime is inserted between output 1 and output 2"]
    ENABLED = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN`"]
pub type DTEN_R = crate::R<bool, DTEN_A>;
impl DTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::DISABLED,
            true => DTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DTEN`"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output 1 and 2 signals are independent"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::DISABLED)
    }
    #[doc = "Deadtime is inserted between output 1 and output 2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_A = DIDL2_A;
#[doc = "Reader of field `DIDL1`"]
pub type DIDL1_R = crate::R<bool, DIDL2_A>;
#[doc = "Write proxy for field `DIDL1`"]
pub struct DIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIDL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIDL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The programmed idle state is applied immediately to the output"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIDL2_A::DISABLED)
    }
    #[doc = "Deadtime (inactive level) is inserted on output before entering the idle mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIDL2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Output 1 Chopper enable"]
pub type CHP1_A = CHP2_A;
#[doc = "Reader of field `CHP1`"]
pub type CHP1_R = crate::R<bool, CHP2_A>;
#[doc = "Write proxy for field `CHP1`"]
pub struct CHP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output signal not altered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHP2_A::DISABLED)
    }
    #[doc = "Output signal is chopped by a carrier signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHP2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Output 1 Fault state"]
pub type FAULT1_A = FAULT2_A;
#[doc = "Reader of field `FAULT1`"]
pub type FAULT1_R = crate::R<u8, FAULT2_A>;
#[doc = "Write proxy for field `FAULT1`"]
pub struct FAULT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No action: the output is not affected by the fault input and stays in run mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FAULT2_A::DISABLED)
    }
    #[doc = "Output goes to active state after a fault event"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(FAULT2_A::SETACTIVE)
    }
    #[doc = "Output goes to inactive state after a fault event"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(FAULT2_A::SETINACTIVE)
    }
    #[doc = "Output goes to high-z state after a fault event"]
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut W {
        self.variant(FAULT2_A::SETHIGHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Output 1 Idle State"]
pub type IDLES1_A = IDLES2_A;
#[doc = "Reader of field `IDLES1`"]
pub type IDLES1_R = crate::R<bool, IDLES2_A>;
#[doc = "Write proxy for field `IDLES1`"]
pub struct IDLES1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLES1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLES1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output idle state is inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IDLES2_A::INACTIVE)
    }
    #[doc = "Output idle state is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLES2_A::ACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Output 1 Idle mode"]
pub type IDLEM1_A = IDLEM2_A;
#[doc = "Reader of field `IDLEM1`"]
pub type IDLEM1_R = crate::R<bool, IDLEM2_A>;
#[doc = "Write proxy for field `IDLEM1`"]
pub struct IDLEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLEM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action: the output is not affected by the burst mode operation"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IDLEM2_A::NOEFFECT)
    }
    #[doc = "The output is in idle state when requested by the burst mode controller"]
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut W {
        self.variant(IDLEM2_A::SETIDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Output 1 polarity"]
pub type POL1_A = POL2_A;
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, POL2_A>;
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Positive polarity (output active high)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL2_A::ACTIVEHIGH)
    }
    #[doc = "Negative polarity (output active low)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL2_A::ACTIVELOW)
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
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&mut self) -> DIDL2_W {
        DIDL2_W { w: self }
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&mut self) -> CHP2_W {
        CHP2_W { w: self }
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT2_W {
        FAULT2_W { w: self }
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&mut self) -> IDLES2_W {
        IDLES2_W { w: self }
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&mut self) -> IDLEM2_W {
        IDLEM2_W { w: self }
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&mut self) -> DLYPRT_W {
        DLYPRT_W { w: self }
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W {
        DLYPRTEN_W { w: self }
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&mut self) -> DIDL1_W {
        DIDL1_W { w: self }
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&mut self) -> CHP1_W {
        CHP1_W { w: self }
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W {
        FAULT1_W { w: self }
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&mut self) -> IDLES1_W {
        IDLES1_W { w: self }
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&mut self) -> IDLEM1_W {
        IDLEM1_W { w: self }
    }
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
}
