#[doc = "Register `BMTRGR` reader"]
pub struct R(crate::R<BMTRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMTRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMTRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMTRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMTRGR` writer"]
pub struct W(crate::W<BMTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMTRGR_SPEC>;
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
impl From<crate::W<BMTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMTRGR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `OCHPEV` reader - OCHPEV"]
pub struct OCHPEV_R(crate::FieldReader<bool, OCHPEV_A>);
impl OCHPEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCHPEV_R(crate::FieldReader::new(bits))
    }
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
        **self == OCHPEV_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == OCHPEV_A::TRIGGER
    }
}
impl core::ops::Deref for OCHPEV_R {
    type Target = crate::FieldReader<bool, OCHPEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCHPEV` writer - OCHPEV"]
pub struct OCHPEV_W<'a> {
    w: &'a mut W,
}
impl<'a> OCHPEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCHPEV_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "EEV8"]
pub type EEV8_A = EEV7_A;
#[doc = "Field `EEV8` reader - EEV8"]
pub type EEV8_R = EEV7_R;
#[doc = "Field `EEV8` writer - EEV8"]
pub struct EEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> EEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEV8_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "EEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEV7_A {
    #[doc = "0: External event X has no effect"]
    NOEFFECT = 0,
    #[doc = "1: External event X triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<EEV7_A> for bool {
    #[inline(always)]
    fn from(variant: EEV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEV7` reader - EEV7"]
pub struct EEV7_R(crate::FieldReader<bool, EEV7_A>);
impl EEV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EEV7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEV7_A {
        match self.bits {
            false => EEV7_A::NOEFFECT,
            true => EEV7_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == EEV7_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == EEV7_A::TRIGGER
    }
}
impl core::ops::Deref for EEV7_R {
    type Target = crate::FieldReader<bool, EEV7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEV7` writer - EEV7"]
pub struct EEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> EEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEV7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event X has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EEV7_A::NOEFFECT)
    }
    #[doc = "External event X triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(EEV7_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "TDEEV8"]
pub type TDEEV8_A = TAEEV7_A;
#[doc = "Field `TDEEV8` reader - TDEEV8"]
pub type TDEEV8_R = TAEEV7_R;
#[doc = "Field `TDEEV8` writer - TDEEV8"]
pub struct TDEEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> TDEEV8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDEEV8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X period following external event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDEEV8_A::NOEFFECT)
    }
    #[doc = "Timer X period following external event Y triggers a burst mode entry"]
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "TAEEV7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAEEV7_A {
    #[doc = "0: Timer X period following external event Y has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X period following external event Y triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TAEEV7_A> for bool {
    #[inline(always)]
    fn from(variant: TAEEV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAEEV7` reader - TAEEV7"]
pub struct TAEEV7_R(crate::FieldReader<bool, TAEEV7_A>);
impl TAEEV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAEEV7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAEEV7_A {
        match self.bits {
            false => TAEEV7_A::NOEFFECT,
            true => TAEEV7_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TAEEV7_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == TAEEV7_A::TRIGGER
    }
}
impl core::ops::Deref for TAEEV7_R {
    type Target = crate::FieldReader<bool, TAEEV7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAEEV7` writer - TAEEV7"]
pub struct TAEEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEEV7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAEEV7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X period following external event Y has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TAEEV7_A::NOEFFECT)
    }
    #[doc = "Timer X period following external event Y triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TAEEV7_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "TECMP2"]
pub type TECMP2_A = TACMP1_A;
#[doc = "Field `TECMP2` reader - TECMP2"]
pub type TECMP2_R = TACMP1_R;
#[doc = "Field `TECMP2` writer - TECMP2"]
pub struct TECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECMP2_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "TECMP1"]
pub type TECMP1_A = TACMP1_A;
#[doc = "Field `TECMP1` reader - TECMP1"]
pub type TECMP1_R = TACMP1_R;
#[doc = "Field `TECMP1` writer - TECMP1"]
pub struct TECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TECMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TECMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TECMP1_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TECMP1_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "TEREP"]
pub type TEREP_A = TAREP_A;
#[doc = "Field `TEREP` reader - TEREP"]
pub type TEREP_R = TAREP_R;
#[doc = "Field `TEREP` writer - TEREP"]
pub struct TEREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEREP_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "TERST"]
pub type TERST_A = TARST_A;
#[doc = "Field `TERST` reader - TERST"]
pub type TERST_R = TARST_R;
#[doc = "Field `TERST` writer - TERST"]
pub struct TERST_W<'a> {
    w: &'a mut W,
}
impl<'a> TERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "TDCMP2"]
pub type TDCMP2_A = TACMP1_A;
#[doc = "Field `TDCMP2` reader - TDCMP2"]
pub type TDCMP2_R = TACMP1_R;
#[doc = "Field `TDCMP2` writer - TDCMP2"]
pub struct TDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDCMP2_A::TRIGGER)
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
#[doc = "TDCMP1"]
pub type TDCMP1_A = TACMP1_A;
#[doc = "Field `TDCMP1` reader - TDCMP1"]
pub type TDCMP1_R = TACMP1_R;
#[doc = "Field `TDCMP1` writer - TDCMP1"]
pub struct TDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDCMP1_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "TDREP"]
pub type TDREP_A = TAREP_A;
#[doc = "Field `TDREP` reader - TDREP"]
pub type TDREP_R = TAREP_R;
#[doc = "Field `TDREP` writer - TDREP"]
pub struct TDREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TDREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDREP_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "TDRST"]
pub type TDRST_A = TARST_A;
#[doc = "Field `TDRST` reader - TDRST"]
pub type TDRST_R = TARST_R;
#[doc = "Field `TDRST` writer - TDRST"]
pub struct TDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TDRST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TDRST_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "TCCMP2"]
pub type TCCMP2_A = TACMP1_A;
#[doc = "Field `TCCMP2` reader - TCCMP2"]
pub type TCCMP2_R = TACMP1_R;
#[doc = "Field `TCCMP2` writer - TCCMP2"]
pub struct TCCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TCCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TCCMP2_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "TCCMP1"]
pub type TCCMP1_A = TACMP1_A;
#[doc = "Field `TCCMP1` reader - TCCMP1"]
pub type TCCMP1_R = TACMP1_R;
#[doc = "Field `TCCMP1` writer - TCCMP1"]
pub struct TCCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TCCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TCCMP1_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "TCREP"]
pub type TCREP_A = TAREP_A;
#[doc = "Field `TCREP` reader - TCREP"]
pub type TCREP_R = TAREP_R;
#[doc = "Field `TCREP` writer - TCREP"]
pub struct TCREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TCREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TCREP_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "TCRST"]
pub type TCRST_A = TARST_A;
#[doc = "Field `TCRST` reader - TCRST"]
pub type TCRST_R = TARST_R;
#[doc = "Field `TCRST` writer - TCRST"]
pub struct TCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TCRST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TCRST_A::TRIGGER)
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
#[doc = "TBCMP2"]
pub type TBCMP2_A = TACMP1_A;
#[doc = "Field `TBCMP2` reader - TBCMP2"]
pub type TBCMP2_R = TACMP1_R;
#[doc = "Field `TBCMP2` writer - TBCMP2"]
pub struct TBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TBCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TBCMP2_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "TBCMP1"]
pub type TBCMP1_A = TACMP1_A;
#[doc = "Field `TBCMP1` reader - TBCMP1"]
pub type TBCMP1_R = TACMP1_R;
#[doc = "Field `TBCMP1` writer - TBCMP1"]
pub struct TBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TBCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TBCMP1_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "TBREP"]
pub type TBREP_A = TAREP_A;
#[doc = "Field `TBREP` reader - TBREP"]
pub type TBREP_R = TAREP_R;
#[doc = "Field `TBREP` writer - TBREP"]
pub struct TBREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TBREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TBREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TBREP_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "TBRST"]
pub type TBRST_A = TARST_A;
#[doc = "Field `TBRST` reader - TBRST"]
pub type TBRST_R = TARST_R;
#[doc = "Field `TBRST` writer - TBRST"]
pub struct TBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TBRST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TBRST_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "TACMP2"]
pub type TACMP2_A = TACMP1_A;
#[doc = "Field `TACMP2` reader - TACMP2"]
pub type TACMP2_R = TACMP1_R;
#[doc = "Field `TACMP2` writer - TACMP2"]
pub struct TACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TACMP2_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TACMP2_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "TACMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACMP1_A {
    #[doc = "0: Timer X compare Y event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X compare Y event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: TACMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACMP1` reader - TACMP1"]
pub struct TACMP1_R(crate::FieldReader<bool, TACMP1_A>);
impl TACMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TACMP1_A {
        match self.bits {
            false => TACMP1_A::NOEFFECT,
            true => TACMP1_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TACMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == TACMP1_A::TRIGGER
    }
}
impl core::ops::Deref for TACMP1_R {
    type Target = crate::FieldReader<bool, TACMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACMP1` writer - TACMP1"]
pub struct TACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TACMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Y event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TACMP1_A::NOEFFECT)
    }
    #[doc = "Timer X compare Y event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TACMP1_A::TRIGGER)
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
#[doc = "TAREP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAREP_A {
    #[doc = "0: Timer X repetition event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X repetition event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TAREP_A> for bool {
    #[inline(always)]
    fn from(variant: TAREP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAREP` reader - TAREP"]
pub struct TAREP_R(crate::FieldReader<bool, TAREP_A>);
impl TAREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAREP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAREP_A {
        match self.bits {
            false => TAREP_A::NOEFFECT,
            true => TAREP_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TAREP_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == TAREP_A::TRIGGER
    }
}
impl core::ops::Deref for TAREP_R {
    type Target = crate::FieldReader<bool, TAREP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAREP` writer - TAREP"]
pub struct TAREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TAREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAREP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X repetition event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TAREP_A::NOEFFECT)
    }
    #[doc = "Timer X repetition event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TAREP_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "TARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARST_A {
    #[doc = "0: Timer X reset/roll-over event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X reset/roll-over event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<TARST_A> for bool {
    #[inline(always)]
    fn from(variant: TARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TARST` reader - TARST"]
pub struct TARST_R(crate::FieldReader<bool, TARST_A>);
impl TARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARST_A {
        match self.bits {
            false => TARST_A::NOEFFECT,
            true => TARST_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TARST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == TARST_A::TRIGGER
    }
}
impl core::ops::Deref for TARST_R {
    type Target = crate::FieldReader<bool, TARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARST` writer - TARST"]
pub struct TARST_W<'a> {
    w: &'a mut W,
}
impl<'a> TARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X reset/roll-over event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TARST_A::NOEFFECT)
    }
    #[doc = "Timer X reset/roll-over event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TARST_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "MSTCMP4"]
pub type MSTCMP4_A = MSTCMP1_A;
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub type MSTCMP4_R = MSTCMP1_R;
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub struct MSTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP4_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "MSTCMP3"]
pub type MSTCMP3_A = MSTCMP1_A;
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub type MSTCMP3_R = MSTCMP1_R;
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub struct MSTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP3_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP3_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "MSTCMP2"]
pub type MSTCMP2_A = MSTCMP1_A;
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub type MSTCMP2_R = MSTCMP1_R;
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub struct MSTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP2_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP2_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "MSTCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare X event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer compare X event triggers a burst mode entry"]
    TRIGGER = 1,
}
impl From<MSTCMP1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub struct MSTCMP1_R(crate::FieldReader<bool, MSTCMP1_A>);
impl MSTCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTCMP1_A {
        match self.bits {
            false => MSTCMP1_A::NOEFFECT,
            true => MSTCMP1_A::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTCMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == MSTCMP1_A::TRIGGER
    }
}
impl core::ops::Deref for MSTCMP1_R {
    type Target = crate::FieldReader<bool, MSTCMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub struct MSTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare X event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NOEFFECT)
    }
    #[doc = "Master timer compare X event triggers a burst mode entry"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(MSTCMP1_A::TRIGGER)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
#[doc = "Field `MSTREP` reader - MSTREP"]
pub struct MSTREP_R(crate::FieldReader<bool, MSTREP_A>);
impl MSTREP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTREP_R(crate::FieldReader::new(bits))
    }
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
        **self == MSTREP_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == MSTREP_A::TRIGGER
    }
}
impl core::ops::Deref for MSTREP_R {
    type Target = crate::FieldReader<bool, MSTREP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTREP` writer - MSTREP"]
pub struct MSTREP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTREP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTREP_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `MSTRST` reader - MSTRST"]
pub struct MSTRST_R(crate::FieldReader<bool, MSTRST_A>);
impl MSTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTRST_R(crate::FieldReader::new(bits))
    }
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
        **self == MSTRST_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == MSTRST_A::TRIGGER
    }
}
impl core::ops::Deref for MSTRST_R {
    type Target = crate::FieldReader<bool, MSTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTRST` writer - MSTRST"]
pub struct MSTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTRST_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
#[doc = "Field `SW` reader - SW"]
pub struct SW_R(crate::FieldReader<bool, SW_A>);
impl SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
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
        **self == SW_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline(always)]
    pub fn is_trigger(&self) -> bool {
        **self == SW_A::TRIGGER
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - SW"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMTRGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmtrgr](index.html) module"]
pub struct BMTRGR_SPEC;
impl crate::RegisterSpec for BMTRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmtrgr::R](R) reader structure"]
impl crate::Readable for BMTRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmtrgr::W](W) writer structure"]
impl crate::Writable for BMTRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMTRGR to value 0"]
impl crate::Resettable for BMTRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
