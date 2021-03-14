#[doc = "Reader of register SETD1R"]
pub type R = crate::R<u32, super::SETD1R>;
#[doc = "Writer for register SETD1R"]
pub type W = crate::W<u32, super::SETD1R>;
#[doc = "Register SETD1R `reset()`'s with value 0"]
impl crate::ResetValue for super::SETD1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Registers update (transfer preload to active)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATE_A {
    #[doc = "0: Register update event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Register update event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UPDATE`"]
pub type UPDATE_R = crate::R<bool, UPDATE_A>;
impl UPDATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::NOEFFECT,
            true => UPDATE_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == UPDATE_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == UPDATE_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `UPDATE`"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDATE_A::NOEFFECT)
    }
    #[doc = "Register update event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(UPDATE_A::SETACTIVE)
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
#[doc = "External Event 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTEVNT10_A {
    #[doc = "0: External event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: External event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<EXTEVNT10_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTEVNT10`"]
pub type EXTEVNT10_R = crate::R<bool, EXTEVNT10_A>;
impl EXTEVNT10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEVNT10_A {
        match self.bits {
            false => EXTEVNT10_A::NOEFFECT,
            true => EXTEVNT10_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EXTEVNT10_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == EXTEVNT10_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `EXTEVNT10`"]
pub struct EXTEVNT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "External Event 9"]
pub type EXTEVNT9_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT9`"]
pub type EXTEVNT9_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT9`"]
pub struct EXTEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "External Event 8"]
pub type EXTEVNT8_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT8`"]
pub type EXTEVNT8_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT8`"]
pub struct EXTEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "External Event 7"]
pub type EXTEVNT7_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT7`"]
pub type EXTEVNT7_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT7`"]
pub struct EXTEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
#[doc = "External Event 6"]
pub type EXTEVNT6_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT6`"]
pub type EXTEVNT6_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT6`"]
pub struct EXTEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "External Event 5"]
pub type EXTEVNT5_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT5`"]
pub type EXTEVNT5_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT5`"]
pub struct EXTEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "External Event 4"]
pub type EXTEVNT4_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT4`"]
pub type EXTEVNT4_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT4`"]
pub struct EXTEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "External Event 3"]
pub type EXTEVNT3_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT3`"]
pub type EXTEVNT3_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT3`"]
pub struct EXTEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
#[doc = "External Event 2"]
pub type EXTEVNT2_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT2`"]
pub type EXTEVNT2_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT2`"]
pub struct EXTEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
#[doc = "External Event 1"]
pub type EXTEVNT1_A = EXTEVNT10_A;
#[doc = "Reader of field `EXTEVNT1`"]
pub type EXTEVNT1_R = crate::R<bool, EXTEVNT10_A>;
#[doc = "Write proxy for field `EXTEVNT1`"]
pub struct EXTEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Timer Event 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVNT9_A {
    #[doc = "0: Timer event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<TIMEVNT9_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEVNT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEVNT9`"]
pub type TIMEVNT9_R = crate::R<bool, TIMEVNT9_A>;
impl TIMEVNT9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVNT9_A {
        match self.bits {
            false => TIMEVNT9_A::NOEFFECT,
            true => TIMEVNT9_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TIMEVNT9_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == TIMEVNT9_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `TIMEVNT9`"]
pub struct TIMEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Timer Event 8"]
pub type TIMEVNT8_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT8`"]
pub type TIMEVNT8_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT8`"]
pub struct TIMEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
#[doc = "Timer Event 7"]
pub type TIMEVNT7_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT7`"]
pub type TIMEVNT7_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT7`"]
pub struct TIMEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
#[doc = "Timer Event 6"]
pub type TIMEVNT6_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT6`"]
pub type TIMEVNT6_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT6`"]
pub struct TIMEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
#[doc = "Timer Event 5"]
pub type TIMEVNT5_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT5`"]
pub type TIMEVNT5_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT5`"]
pub struct TIMEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
#[doc = "Timer Event 4"]
pub type TIMEVNT4_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT4`"]
pub type TIMEVNT4_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT4`"]
pub struct TIMEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
#[doc = "Timer Event 3"]
pub type TIMEVNT3_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT3`"]
pub type TIMEVNT3_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT3`"]
pub struct TIMEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Timer Event 2"]
pub type TIMEVNT2_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT2`"]
pub type TIMEVNT2_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT2`"]
pub struct TIMEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Timer Event 1"]
pub type TIMEVNT1_A = TIMEVNT9_A;
#[doc = "Reader of field `TIMEVNT1`"]
pub type TIMEVNT1_R = crate::R<bool, TIMEVNT9_A>;
#[doc = "Write proxy for field `TIMEVNT1`"]
pub struct TIMEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Master Compare 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCMP4_A {
    #[doc = "0: Master timer compare event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer compare event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<MSTCMP4_A> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTCMP4`"]
pub type MSTCMP4_R = crate::R<bool, MSTCMP4_A>;
impl MSTCMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTCMP4_A {
        match self.bits {
            false => MSTCMP4_A::NOEFFECT,
            true => MSTCMP4_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP4_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == MSTCMP4_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `MSTCMP4`"]
pub struct MSTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(MSTCMP4_A::SETACTIVE)
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
#[doc = "Master Compare 3"]
pub type MSTCMP3_A = MSTCMP4_A;
#[doc = "Reader of field `MSTCMP3`"]
pub type MSTCMP3_R = crate::R<bool, MSTCMP4_A>;
#[doc = "Write proxy for field `MSTCMP3`"]
pub struct MSTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(MSTCMP4_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Master Compare 2"]
pub type MSTCMP2_A = MSTCMP4_A;
#[doc = "Reader of field `MSTCMP2`"]
pub type MSTCMP2_R = crate::R<bool, MSTCMP4_A>;
#[doc = "Write proxy for field `MSTCMP2`"]
pub struct MSTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(MSTCMP4_A::SETACTIVE)
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
#[doc = "Master Compare 1"]
pub type MSTCMP1_A = MSTCMP4_A;
#[doc = "Reader of field `MSTCMP1`"]
pub type MSTCMP1_R = crate::R<bool, MSTCMP4_A>;
#[doc = "Write proxy for field `MSTCMP1`"]
pub struct MSTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(MSTCMP4_A::SETACTIVE)
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
#[doc = "Master Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPER_A {
    #[doc = "0: Master timer counter roll-over/reset has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer counter roll-over/reset forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<MSTPER_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTPER`"]
pub type MSTPER_R = crate::R<bool, MSTPER_A>;
impl MSTPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPER_A {
        match self.bits {
            false => MSTPER_A::NOEFFECT,
            true => MSTPER_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTPER_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == MSTPER_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `MSTPER`"]
pub struct MSTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTPER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer counter roll-over/reset has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NOEFFECT)
    }
    #[doc = "Master timer counter roll-over/reset forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(MSTPER_A::SETACTIVE)
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
#[doc = "Timer A compare 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP4_A {
    #[doc = "0: Timer compare event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer compare event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<CMP4_A> for bool {
    #[inline(always)]
    fn from(variant: CMP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMP4`"]
pub type CMP4_R = crate::R<bool, CMP4_A>;
impl CMP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP4_A {
        match self.bits {
            false => CMP4_A::NOEFFECT,
            true => CMP4_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CMP4_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == CMP4_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `CMP4`"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(CMP4_A::SETACTIVE)
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
#[doc = "Timer A compare 3"]
pub type CMP3_A = CMP4_A;
#[doc = "Reader of field `CMP3`"]
pub type CMP3_R = crate::R<bool, CMP4_A>;
#[doc = "Write proxy for field `CMP3`"]
pub struct CMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(CMP4_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Timer A compare 2"]
pub type CMP2_A = CMP4_A;
#[doc = "Reader of field `CMP2`"]
pub type CMP2_R = crate::R<bool, CMP4_A>;
#[doc = "Write proxy for field `CMP2`"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(CMP4_A::SETACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Timer A compare 1"]
pub type CMP1_A = CMP4_A;
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, CMP4_A>;
#[doc = "Write proxy for field `CMP1`"]
pub struct CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(CMP4_A::SETACTIVE)
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
#[doc = "Timer A Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    #[doc = "0: Timer period event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer period event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<bool, PER_A>;
impl PER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::NOEFFECT,
            true => PER_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PER_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == PER_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer period event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PER_A::NOEFFECT)
    }
    #[doc = "Timer period event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(PER_A::SETACTIVE)
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
#[doc = "Timer A resynchronizaton\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNC_A {
    #[doc = "0: Timer reset event coming solely from software or SYNC input event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer reset event coming solely from software or SYNC input event forces the output to its active state"]
    SETACTIVE = 1,
}
impl From<RESYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESYNC`"]
pub type RESYNC_R = crate::R<bool, RESYNC_A>;
impl RESYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNC_A {
        match self.bits {
            false => RESYNC_A::NOEFFECT,
            true => RESYNC_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RESYNC_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == RESYNC_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `RESYNC`"]
pub struct RESYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer reset event coming solely from software or SYNC input event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RESYNC_A::NOEFFECT)
    }
    #[doc = "Timer reset event coming solely from software or SYNC input event forces the output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(RESYNC_A::SETACTIVE)
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
#[doc = "Software Set trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SST_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Force output to its active state"]
    SETACTIVE = 1,
}
impl From<SST_A> for bool {
    #[inline(always)]
    fn from(variant: SST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SST`"]
pub type SST_R = crate::R<bool, SST_A>;
impl SST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SST_A {
        match self.bits {
            false => SST_A::NOEFFECT,
            true => SST_A::SETACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETACTIVE`"]
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == SST_A::SETACTIVE
    }
}
#[doc = "Write proxy for field `SST`"]
pub struct SST_W<'a> {
    w: &'a mut W,
}
impl<'a> SST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SST_A::NOEFFECT)
    }
    #[doc = "Force output to its active state"]
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(SST_A::SETACTIVE)
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
impl R {
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Registers update (transfer preload to active)"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bit 30 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W {
        EXTEVNT10_W { w: self }
    }
    #[doc = "Bit 29 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W {
        EXTEVNT9_W { w: self }
    }
    #[doc = "Bit 28 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W {
        EXTEVNT8_W { w: self }
    }
    #[doc = "Bit 27 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W {
        EXTEVNT7_W { w: self }
    }
    #[doc = "Bit 26 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W {
        EXTEVNT6_W { w: self }
    }
    #[doc = "Bit 25 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W {
        EXTEVNT5_W { w: self }
    }
    #[doc = "Bit 24 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W {
        EXTEVNT4_W { w: self }
    }
    #[doc = "Bit 23 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W {
        EXTEVNT3_W { w: self }
    }
    #[doc = "Bit 22 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W {
        EXTEVNT2_W { w: self }
    }
    #[doc = "Bit 21 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W {
        EXTEVNT1_W { w: self }
    }
    #[doc = "Bit 20 - Timer Event 9"]
    #[inline(always)]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W {
        TIMEVNT9_W { w: self }
    }
    #[doc = "Bit 19 - Timer Event 8"]
    #[inline(always)]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W {
        TIMEVNT8_W { w: self }
    }
    #[doc = "Bit 18 - Timer Event 7"]
    #[inline(always)]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W {
        TIMEVNT7_W { w: self }
    }
    #[doc = "Bit 17 - Timer Event 6"]
    #[inline(always)]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W {
        TIMEVNT6_W { w: self }
    }
    #[doc = "Bit 16 - Timer Event 5"]
    #[inline(always)]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W {
        TIMEVNT5_W { w: self }
    }
    #[doc = "Bit 15 - Timer Event 4"]
    #[inline(always)]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W {
        TIMEVNT4_W { w: self }
    }
    #[doc = "Bit 14 - Timer Event 3"]
    #[inline(always)]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W {
        TIMEVNT3_W { w: self }
    }
    #[doc = "Bit 13 - Timer Event 2"]
    #[inline(always)]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W {
        TIMEVNT2_W { w: self }
    }
    #[doc = "Bit 12 - Timer Event 1"]
    #[inline(always)]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W {
        TIMEVNT1_W { w: self }
    }
    #[doc = "Bit 11 - Master Compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 10 - Master Compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 9 - Master Compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 8 - Master Compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 7 - Master Period"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W {
        MSTPER_W { w: self }
    }
    #[doc = "Bit 6 - Timer A compare 4"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 5 - Timer A compare 3"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W {
        CMP3_W { w: self }
    }
    #[doc = "Bit 4 - Timer A compare 2"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 3 - Timer A compare 1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W {
        CMP1_W { w: self }
    }
    #[doc = "Bit 2 - Timer A Period"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    #[doc = "Bit 1 - Timer A resynchronizaton"]
    #[inline(always)]
    pub fn resync(&mut self) -> RESYNC_W {
        RESYNC_W { w: self }
    }
    #[doc = "Bit 0 - Software Set trigger"]
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W {
        SST_W { w: self }
    }
}
