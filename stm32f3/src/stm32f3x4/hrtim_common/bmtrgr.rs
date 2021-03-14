#[doc = "Reader of register BMTRGR"]
pub type R = crate::R<u32, super::BMTRGR>;
#[doc = "Writer for register BMTRGR"]
pub type W = crate::W<u32, super::BMTRGR>;
#[doc = "Register BMTRGR `reset()`'s with value 0"]
impl crate::ResetValue for super::BMTRGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "OCHPEV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCHPEV_A {
    #[doc = "0: Rising edge on an on-chip event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Rising edge on an on-chip event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<OCHPEV_A> for bool {
    #[inline(always)]
    fn from(variant: OCHPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCHPEV`"]
pub type OCHPEV_R = crate::R<bool, OCHPEV_A>;
impl OCHPEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCHPEV_A {
        match self.bits {
            false => OCHPEV_A::NOEFFECT,
            true => OCHPEV_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OCHPEV_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == OCHPEV_A::TRIGGER
    }
}
#[doc = "Write proxy for field `OCHPEV`"]
pub struct OCHPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> OCHPEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCHPEV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rising edge on an on-chip event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OCHPEV_A::NOEFFECT)
    }
    #[doc = "Rising edge on an on-chip event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(OCHPEV_A::TRIGGER)
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
#[doc = "EEV8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEV8_A {
    #[doc = "0: External event X has no effect"]
    NOEFFECT = 0,
    #[doc = "1: External event X triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<EEV8_A> for bool {
    #[inline(always)]
    fn from(variant: EEV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EEV8`"]
pub type EEV8_R = crate::R<bool, EEV8_A>;
impl EEV8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEV8_A {
        match self.bits {
            false => EEV8_A::NOEFFECT,
            true => EEV8_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == EEV8_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == EEV8_A::TRIGGER
    }
}
#[doc = "Write proxy for field `EEV8`"]
pub struct EEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> EEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EEV8_A::NOEFFECT)
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(EEV8_A::TRIGGER)
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
#[doc = "EEV7"]
pub type EEV7_A = EEV8_A;
#[doc = "Reader of field `EEV7`"]
pub type EEV7_R = crate::R<bool, EEV8_A>;
#[doc = "Write proxy for field `EEV7`"]
pub struct EEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> EEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EEV8_A::NOEFFECT)
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(EEV8_A::TRIGGER)
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
#[doc = "TDEEV8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDEEV8_A {
    #[doc = "0: has no effect"]
    NOEFFECT = 0,
    #[doc = "1: triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TDEEV8_A> for bool {
    #[inline(always)]
    fn from(variant: TDEEV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDEEV8`"]
pub type TDEEV8_R = crate::R<bool, TDEEV8_A>;
impl TDEEV8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDEEV8_A {
        match self.bits {
            false => TDEEV8_A::NOEFFECT,
            true => TDEEV8_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TDEEV8_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TDEEV8_A::TRIGGER
    }
}
#[doc = "Write proxy for field `TDEEV8`"]
pub struct TDEEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> TDEEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDEEV8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDEEV8_A::NOEFFECT)
    }
    #[doc = "triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDEEV8_A::TRIGGER)
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
#[doc = "TAEEV7"]
pub type TAEEV7_A = TDEEV8_A;
#[doc = "Reader of field `TAEEV7`"]
pub type TAEEV7_R = crate::R<bool, TDEEV8_A>;
#[doc = "Write proxy for field `TAEEV7`"]
pub struct TAEEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEEV7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDEEV8_A::NOEFFECT)
    }
    #[doc = "triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDEEV8_A::TRIGGER)
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
#[doc = "TECMP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TECMP2_A {
    #[doc = "0: Timer X compare Y event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X compare Y event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TECMP2_A> for bool {
    #[inline(always)]
    fn from(variant: TECMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TECMP2`"]
pub type TECMP2_R = crate::R<bool, TECMP2_A>;
impl TECMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TECMP2_A {
        match self.bits {
            false => TECMP2_A::NOEFFECT,
            true => TECMP2_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TECMP2_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TECMP2_A::TRIGGER
    }
}
#[doc = "Write proxy for field `TECMP2`"]
pub struct TECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TECMP1"]
pub type TECMP1_A = TECMP2_A;
#[doc = "Reader of field `TECMP1`"]
pub type TECMP1_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TECMP1`"]
pub struct TECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TEREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEREP_A {
    #[doc = "0: Timer X repetition event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X repetition event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TEREP_A> for bool {
    #[inline(always)]
    fn from(variant: TEREP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEREP`"]
pub type TEREP_R = crate::R<bool, TEREP_A>;
impl TEREP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEREP_A {
        match self.bits {
            false => TEREP_A::NOEFFECT,
            true => TEREP_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TEREP_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TEREP_A::TRIGGER
    }
}
#[doc = "Write proxy for field `TEREP`"]
pub struct TEREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TEREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TEREP_A::TRIGGER)
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
#[doc = "TERST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERST_A {
    #[doc = "0: Timer X reset/roll-over event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X reset/roll-over event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TERST_A> for bool {
    #[inline(always)]
    fn from(variant: TERST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TERST`"]
pub type TERST_R = crate::R<bool, TERST_A>;
impl TERST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERST_A {
        match self.bits {
            false => TERST_A::NOEFFECT,
            true => TERST_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TERST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == TERST_A::TRIGGER
    }
}
#[doc = "Write proxy for field `TERST`"]
pub struct TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> TERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TERST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TERST_A::TRIGGER)
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
#[doc = "TDCMP2"]
pub type TDCMP2_A = TECMP2_A;
#[doc = "Reader of field `TDCMP2`"]
pub type TDCMP2_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TDCMP2`"]
pub struct TDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TDCMP1"]
pub type TDCMP1_A = TECMP2_A;
#[doc = "Reader of field `TDCMP1`"]
pub type TDCMP1_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TDCMP1`"]
pub struct TDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TDREP"]
pub type TDREP_A = TEREP_A;
#[doc = "Reader of field `TDREP`"]
pub type TDREP_R = crate::R<bool, TEREP_A>;
#[doc = "Write proxy for field `TDREP`"]
pub struct TDREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TDREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TEREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TEREP_A::TRIGGER)
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
#[doc = "TDRST"]
pub type TDRST_A = TERST_A;
#[doc = "Reader of field `TDRST`"]
pub type TDRST_R = crate::R<bool, TERST_A>;
#[doc = "Write proxy for field `TDRST`"]
pub struct TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TERST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TERST_A::TRIGGER)
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
#[doc = "TCCMP2"]
pub type TCCMP2_A = TECMP2_A;
#[doc = "Reader of field `TCCMP2`"]
pub type TCCMP2_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TCCMP2`"]
pub struct TCCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TCCMP1"]
pub type TCCMP1_A = TECMP2_A;
#[doc = "Reader of field `TCCMP1`"]
pub type TCCMP1_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TCCMP1`"]
pub struct TCCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TCREP"]
pub type TCREP_A = TEREP_A;
#[doc = "Reader of field `TCREP`"]
pub type TCREP_R = crate::R<bool, TEREP_A>;
#[doc = "Write proxy for field `TCREP`"]
pub struct TCREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TEREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TEREP_A::TRIGGER)
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
#[doc = "TCRST"]
pub type TCRST_A = TERST_A;
#[doc = "Reader of field `TCRST`"]
pub type TCRST_R = crate::R<bool, TERST_A>;
#[doc = "Write proxy for field `TCRST`"]
pub struct TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TERST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TERST_A::TRIGGER)
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
#[doc = "TBCMP2"]
pub type TBCMP2_A = TECMP2_A;
#[doc = "Reader of field `TBCMP2`"]
pub type TBCMP2_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TBCMP2`"]
pub struct TBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TBCMP1"]
pub type TBCMP1_A = TECMP2_A;
#[doc = "Reader of field `TBCMP1`"]
pub type TBCMP1_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TBCMP1`"]
pub struct TBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TBREP"]
pub type TBREP_A = TEREP_A;
#[doc = "Reader of field `TBREP`"]
pub type TBREP_R = crate::R<bool, TEREP_A>;
#[doc = "Write proxy for field `TBREP`"]
pub struct TBREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TEREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TEREP_A::TRIGGER)
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
#[doc = "TBRST"]
pub type TBRST_A = TERST_A;
#[doc = "Reader of field `TBRST`"]
pub type TBRST_R = crate::R<bool, TERST_A>;
#[doc = "Write proxy for field `TBRST`"]
pub struct TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TERST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TERST_A::TRIGGER)
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
#[doc = "TACMP2"]
pub type TACMP2_A = TECMP2_A;
#[doc = "Reader of field `TACMP2`"]
pub type TACMP2_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TACMP2`"]
pub struct TACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TACMP1"]
pub type TACMP1_A = TECMP2_A;
#[doc = "Reader of field `TACMP1`"]
pub type TACMP1_R = crate::R<bool, TECMP2_A>;
#[doc = "Write proxy for field `TACMP1`"]
pub struct TACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP2_A::TRIGGER)
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
#[doc = "TAREP"]
pub type TAREP_A = TEREP_A;
#[doc = "Reader of field `TAREP`"]
pub type TAREP_R = crate::R<bool, TEREP_A>;
#[doc = "Write proxy for field `TAREP`"]
pub struct TAREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TAREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TEREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TEREP_A::TRIGGER)
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
#[doc = "TARST"]
pub type TARST_A = TERST_A;
#[doc = "Reader of field `TARST`"]
pub type TARST_R = crate::R<bool, TERST_A>;
#[doc = "Write proxy for field `TARST`"]
pub struct TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> TARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TERST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TERST_A::TRIGGER)
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
#[doc = "MSTCMP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCMP4_A {
    #[doc = "0: Master timer compare X event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer compare X event triggers a burst mode entry"]
    TRIGGER = 1,
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
            true => MSTCMP4_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTCMP4_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTCMP4_A::TRIGGER
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
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP4_A::TRIGGER)
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
#[doc = "MSTCMP3"]
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
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP4_A::TRIGGER)
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
#[doc = "MSTCMP2"]
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
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP4_A::TRIGGER)
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
#[doc = "MSTCMP1"]
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
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP4_A::TRIGGER)
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
#[doc = "MSTREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTREP_A {
    #[doc = "0: Master timer repetition event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer repetition event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<MSTREP_A> for bool {
    #[inline(always)]
    fn from(variant: MSTREP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTREP`"]
pub type MSTREP_R = crate::R<bool, MSTREP_A>;
impl MSTREP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTREP_A {
        match self.bits {
            false => MSTREP_A::NOEFFECT,
            true => MSTREP_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTREP_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTREP_A::TRIGGER
    }
}
#[doc = "Write proxy for field `MSTREP`"]
pub struct MSTREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTREP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTREP_A::NOEFFECT)
    }
    #[doc = "Master timer repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTREP_A::TRIGGER)
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
#[doc = "MSTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRST_A {
    #[doc = "0: Master timer reset/roll-over event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer reset/roll-over event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<MSTRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTRST`"]
pub type MSTRST_R = crate::R<bool, MSTRST_A>;
impl MSTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTRST_A {
        match self.bits {
            false => MSTRST_A::NOEFFECT,
            true => MSTRST_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTRST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == MSTRST_A::TRIGGER
    }
}
#[doc = "Write proxy for field `MSTRST`"]
pub struct MSTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master timer reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTRST_A::NOEFFECT)
    }
    #[doc = "Master timer reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTRST_A::TRIGGER)
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
#[doc = "SW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Trigger immediate burst mode operation"]
    TRIGGER = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<bool, SW_A>;
impl SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::NOEFFECT,
            true => SW_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SW_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        *self == SW_A::TRIGGER
    }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SW_A::NOEFFECT)
    }
    #[doc = "Trigger immediate burst mode operation"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(SW_A::TRIGGER)
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
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&self) -> OCHPEV_R {
        OCHPEV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    pub fn eev8(&self) -> EEV8_R {
        EEV8_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    pub fn eev7(&self) -> EEV7_R {
        EEV7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    pub fn tdeev8(&self) -> TDEEV8_R {
        TDEEV8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    pub fn taeev7(&self) -> TAEEV7_R {
        TAEEV7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&self) -> TECMP2_R {
        TECMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&self) -> TECMP1_R {
        TECMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&self) -> TEREP_R {
        TEREP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&self) -> TERST_R {
        TERST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&self) -> TDCMP2_R {
        TDCMP2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&self) -> TDCMP1_R {
        TDCMP1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&self) -> TDREP_R {
        TDREP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&self) -> TDRST_R {
        TDRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&self) -> TCCMP2_R {
        TCCMP2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&self) -> TCCMP1_R {
        TCCMP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&self) -> TCREP_R {
        TCREP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&self) -> TCRST_R {
        TCRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&self) -> TBCMP2_R {
        TBCMP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&self) -> TBCMP1_R {
        TBCMP1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&self) -> TBREP_R {
        TBREP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&self) -> TBRST_R {
        TBRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&self) -> TACMP2_R {
        TACMP2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&self) -> TACMP1_R {
        TACMP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&self) -> TAREP_R {
        TAREP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&self) -> TARST_R {
        TARST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&self) -> MSTREP_R {
        MSTREP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&self) -> MSTRST_R {
        MSTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - OCHPEV"]
    #[inline(always)]
    pub fn ochpev(&mut self) -> OCHPEV_W {
        OCHPEV_W { w: self }
    }
    #[doc = "Bit 30 - EEV8"]
    #[inline(always)]
    pub fn eev8(&mut self) -> EEV8_W {
        EEV8_W { w: self }
    }
    #[doc = "Bit 29 - EEV7"]
    #[inline(always)]
    pub fn eev7(&mut self) -> EEV7_W {
        EEV7_W { w: self }
    }
    #[doc = "Bit 28 - TDEEV8"]
    #[inline(always)]
    pub fn tdeev8(&mut self) -> TDEEV8_W {
        TDEEV8_W { w: self }
    }
    #[doc = "Bit 27 - TAEEV7"]
    #[inline(always)]
    pub fn taeev7(&mut self) -> TAEEV7_W {
        TAEEV7_W { w: self }
    }
    #[doc = "Bit 26 - TECMP2"]
    #[inline(always)]
    pub fn tecmp2(&mut self) -> TECMP2_W {
        TECMP2_W { w: self }
    }
    #[doc = "Bit 25 - TECMP1"]
    #[inline(always)]
    pub fn tecmp1(&mut self) -> TECMP1_W {
        TECMP1_W { w: self }
    }
    #[doc = "Bit 24 - TEREP"]
    #[inline(always)]
    pub fn terep(&mut self) -> TEREP_W {
        TEREP_W { w: self }
    }
    #[doc = "Bit 23 - TERST"]
    #[inline(always)]
    pub fn terst(&mut self) -> TERST_W {
        TERST_W { w: self }
    }
    #[doc = "Bit 22 - TDCMP2"]
    #[inline(always)]
    pub fn tdcmp2(&mut self) -> TDCMP2_W {
        TDCMP2_W { w: self }
    }
    #[doc = "Bit 21 - TDCMP1"]
    #[inline(always)]
    pub fn tdcmp1(&mut self) -> TDCMP1_W {
        TDCMP1_W { w: self }
    }
    #[doc = "Bit 20 - TDREP"]
    #[inline(always)]
    pub fn tdrep(&mut self) -> TDREP_W {
        TDREP_W { w: self }
    }
    #[doc = "Bit 19 - TDRST"]
    #[inline(always)]
    pub fn tdrst(&mut self) -> TDRST_W {
        TDRST_W { w: self }
    }
    #[doc = "Bit 18 - TCCMP2"]
    #[inline(always)]
    pub fn tccmp2(&mut self) -> TCCMP2_W {
        TCCMP2_W { w: self }
    }
    #[doc = "Bit 17 - TCCMP1"]
    #[inline(always)]
    pub fn tccmp1(&mut self) -> TCCMP1_W {
        TCCMP1_W { w: self }
    }
    #[doc = "Bit 16 - TCREP"]
    #[inline(always)]
    pub fn tcrep(&mut self) -> TCREP_W {
        TCREP_W { w: self }
    }
    #[doc = "Bit 15 - TCRST"]
    #[inline(always)]
    pub fn tcrst(&mut self) -> TCRST_W {
        TCRST_W { w: self }
    }
    #[doc = "Bit 14 - TBCMP2"]
    #[inline(always)]
    pub fn tbcmp2(&mut self) -> TBCMP2_W {
        TBCMP2_W { w: self }
    }
    #[doc = "Bit 13 - TBCMP1"]
    #[inline(always)]
    pub fn tbcmp1(&mut self) -> TBCMP1_W {
        TBCMP1_W { w: self }
    }
    #[doc = "Bit 12 - TBREP"]
    #[inline(always)]
    pub fn tbrep(&mut self) -> TBREP_W {
        TBREP_W { w: self }
    }
    #[doc = "Bit 11 - TBRST"]
    #[inline(always)]
    pub fn tbrst(&mut self) -> TBRST_W {
        TBRST_W { w: self }
    }
    #[doc = "Bit 10 - TACMP2"]
    #[inline(always)]
    pub fn tacmp2(&mut self) -> TACMP2_W {
        TACMP2_W { w: self }
    }
    #[doc = "Bit 9 - TACMP1"]
    #[inline(always)]
    pub fn tacmp1(&mut self) -> TACMP1_W {
        TACMP1_W { w: self }
    }
    #[doc = "Bit 8 - TAREP"]
    #[inline(always)]
    pub fn tarep(&mut self) -> TAREP_W {
        TAREP_W { w: self }
    }
    #[doc = "Bit 7 - TARST"]
    #[inline(always)]
    pub fn tarst(&mut self) -> TARST_W {
        TARST_W { w: self }
    }
    #[doc = "Bit 6 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 5 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 4 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 3 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 2 - MSTREP"]
    #[inline(always)]
    pub fn mstrep(&mut self) -> MSTREP_W {
        MSTREP_W { w: self }
    }
    #[doc = "Bit 1 - MSTRST"]
    #[inline(always)]
    pub fn mstrst(&mut self) -> MSTRST_W {
        MSTRST_W { w: self }
    }
    #[doc = "Bit 0 - SW"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
}
