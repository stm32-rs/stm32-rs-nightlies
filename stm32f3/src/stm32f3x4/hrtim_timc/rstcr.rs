#[doc = "Register `RSTCR` reader"]
pub struct R(crate::R<RSTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCR` writer"]
pub struct W(crate::W<RSTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCR_SPEC>;
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
impl From<crate::W<RSTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer E Compare 4"]
pub type TIMECMP4_A = TIMACMP1_A;
#[doc = "Field `TIMECMP4` reader - Timer E Compare 4"]
pub type TIMECMP4_R = TIMACMP1_R;
#[doc = "Field `TIMECMP4` writer - Timer E Compare 4"]
pub struct TIMECMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMECMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMECMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMECMP4_A::RESETCOUNTER)
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
#[doc = "Timer E Compare 2"]
pub type TIMECMP2_A = TIMACMP1_A;
#[doc = "Field `TIMECMP2` reader - Timer E Compare 2"]
pub type TIMECMP2_R = TIMACMP1_R;
#[doc = "Field `TIMECMP2` writer - Timer E Compare 2"]
pub struct TIMECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMECMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMECMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMECMP2_A::RESETCOUNTER)
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
#[doc = "Timer E Compare 1"]
pub type TIMECMP1_A = TIMACMP1_A;
#[doc = "Field `TIMECMP1` reader - Timer E Compare 1"]
pub type TIMECMP1_R = TIMACMP1_R;
#[doc = "Field `TIMECMP1` writer - Timer E Compare 1"]
pub struct TIMECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMECMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMECMP1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMECMP1_A::RESETCOUNTER)
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
#[doc = "Timer D Compare 4"]
pub type TIMDCMP4_A = TIMACMP1_A;
#[doc = "Field `TIMDCMP4` reader - Timer D Compare 4"]
pub type TIMDCMP4_R = TIMACMP1_R;
#[doc = "Field `TIMDCMP4` writer - Timer D Compare 4"]
pub struct TIMDCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMDCMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMDCMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMDCMP4_A::RESETCOUNTER)
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
#[doc = "Timer D Compare 2"]
pub type TIMDCMP2_A = TIMACMP1_A;
#[doc = "Field `TIMDCMP2` reader - Timer D Compare 2"]
pub type TIMDCMP2_R = TIMACMP1_R;
#[doc = "Field `TIMDCMP2` writer - Timer D Compare 2"]
pub struct TIMDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMDCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMDCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMDCMP2_A::RESETCOUNTER)
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
#[doc = "Timer D Compare 1"]
pub type TIMDCMP1_A = TIMACMP1_A;
#[doc = "Field `TIMDCMP1` reader - Timer D Compare 1"]
pub type TIMDCMP1_R = TIMACMP1_R;
#[doc = "Field `TIMDCMP1` writer - Timer D Compare 1"]
pub struct TIMDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMDCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMDCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMDCMP1_A::RESETCOUNTER)
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
#[doc = "Timer B Compare 4"]
pub type TIMBCMP4_A = TIMACMP1_A;
#[doc = "Field `TIMBCMP4` reader - Timer B Compare 4"]
pub type TIMBCMP4_R = TIMACMP1_R;
#[doc = "Field `TIMBCMP4` writer - Timer B Compare 4"]
pub struct TIMBCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMBCMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMBCMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMBCMP4_A::RESETCOUNTER)
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
#[doc = "Timer B Compare 2"]
pub type TIMBCMP2_A = TIMACMP1_A;
#[doc = "Field `TIMBCMP2` reader - Timer B Compare 2"]
pub type TIMBCMP2_R = TIMACMP1_R;
#[doc = "Field `TIMBCMP2` writer - Timer B Compare 2"]
pub struct TIMBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMBCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMBCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMBCMP2_A::RESETCOUNTER)
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
#[doc = "Timer B Compare 1"]
pub type TIMBCMP1_A = TIMACMP1_A;
#[doc = "Field `TIMBCMP1` reader - Timer B Compare 1"]
pub type TIMBCMP1_R = TIMACMP1_R;
#[doc = "Field `TIMBCMP1` writer - Timer B Compare 1"]
pub struct TIMBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMBCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMBCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMBCMP1_A::RESETCOUNTER)
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
#[doc = "Timer A Compare 4"]
pub type TIMACMP4_A = TIMACMP1_A;
#[doc = "Field `TIMACMP4` reader - Timer A Compare 4"]
pub type TIMACMP4_R = TIMACMP1_R;
#[doc = "Field `TIMACMP4` writer - Timer A Compare 4"]
pub struct TIMACMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMACMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMACMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMACMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMACMP4_A::RESETCOUNTER)
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
#[doc = "Timer A Compare 2"]
pub type TIMACMP2_A = TIMACMP1_A;
#[doc = "Field `TIMACMP2` reader - Timer A Compare 2"]
pub type TIMACMP2_R = TIMACMP1_R;
#[doc = "Field `TIMACMP2` writer - Timer A Compare 2"]
pub struct TIMACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMACMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMACMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMACMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMACMP2_A::RESETCOUNTER)
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
#[doc = "Timer A Compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMACMP1_A {
    #[doc = "0: Timer Y compare Z event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon timer Y compare Z event"]
    RESETCOUNTER = 1,
}
impl From<TIMACMP1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMACMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMACMP1` reader - Timer A Compare 1"]
pub struct TIMACMP1_R(crate::FieldReader<bool, TIMACMP1_A>);
impl TIMACMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMACMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMACMP1_A {
        match self.bits {
            false => TIMACMP1_A::NOEFFECT,
            true => TIMACMP1_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TIMACMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == TIMACMP1_A::RESETCOUNTER
    }
}
impl core::ops::Deref for TIMACMP1_R {
    type Target = crate::FieldReader<bool, TIMACMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMACMP1` writer - Timer A Compare 1"]
pub struct TIMACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMACMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMACMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Y compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMACMP1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer Y compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(TIMACMP1_A::RESETCOUNTER)
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
#[doc = "External Event 10"]
pub type EXTEVNT10_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub type EXTEVNT10_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub struct EXTEVNT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::RESETCOUNTER)
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
#[doc = "External Event 9"]
pub type EXTEVNT9_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub type EXTEVNT9_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub struct EXTEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT9_A::RESETCOUNTER)
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
#[doc = "External Event 8"]
pub type EXTEVNT8_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub type EXTEVNT8_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub struct EXTEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT8_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT8_A::RESETCOUNTER)
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
#[doc = "External Event 7"]
pub type EXTEVNT7_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub type EXTEVNT7_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub struct EXTEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT7_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT7_A::RESETCOUNTER)
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
#[doc = "External Event 6"]
pub type EXTEVNT6_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub type EXTEVNT6_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub struct EXTEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT6_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT6_A::RESETCOUNTER)
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
#[doc = "External Event 5"]
pub type EXTEVNT5_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub type EXTEVNT5_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub struct EXTEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT5_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT5_A::RESETCOUNTER)
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
#[doc = "External Event 4"]
pub type EXTEVNT4_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub type EXTEVNT4_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub struct EXTEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT4_A::RESETCOUNTER)
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
#[doc = "External Event 3"]
pub type EXTEVNT3_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub type EXTEVNT3_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub struct EXTEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT3_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT3_A::RESETCOUNTER)
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
#[doc = "External Event 2"]
pub type EXTEVNT2_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub type EXTEVNT2_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub struct EXTEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT2_A::RESETCOUNTER)
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
#[doc = "External Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTEVNT1_A {
    #[doc = "0: External event Z has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon external event Z"]
    RESETCOUNTER = 1,
}
impl From<EXTEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub struct EXTEVNT1_R(crate::FieldReader<bool, EXTEVNT1_A>);
impl EXTEVNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEVNT1_A {
        match self.bits {
            false => EXTEVNT1_A::NOEFFECT,
            true => EXTEVNT1_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == EXTEVNT1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == EXTEVNT1_A::RESETCOUNTER
    }
}
impl core::ops::Deref for EXTEVNT1_R {
    type Target = crate::FieldReader<bool, EXTEVNT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub struct EXTEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event Z has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon external event Z"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::RESETCOUNTER)
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
#[doc = "Master compare 4"]
pub type MSTCMP4_A = MSTCMP1_A;
#[doc = "Field `MSTCMP4` reader - Master compare 4"]
pub type MSTCMP4_R = MSTCMP1_R;
#[doc = "Field `MSTCMP4` writer - Master compare 4"]
pub struct MSTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon master timer compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP4_A::RESETCOUNTER)
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
#[doc = "Master compare 3"]
pub type MSTCMP3_A = MSTCMP1_A;
#[doc = "Field `MSTCMP3` reader - Master compare 3"]
pub type MSTCMP3_R = MSTCMP1_R;
#[doc = "Field `MSTCMP3` writer - Master compare 3"]
pub struct MSTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP3_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon master timer compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP3_A::RESETCOUNTER)
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
#[doc = "Master compare 2"]
pub type MSTCMP2_A = MSTCMP1_A;
#[doc = "Field `MSTCMP2` reader - Master compare 2"]
pub type MSTCMP2_R = MSTCMP1_R;
#[doc = "Field `MSTCMP2` writer - Master compare 2"]
pub struct MSTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon master timer compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP2_A::RESETCOUNTER)
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
#[doc = "Master compare 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare Z event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon master timer compare Z event"]
    RESETCOUNTER = 1,
}
impl From<MSTCMP1_A> for bool {
    #[inline(always)]
    fn from(variant: MSTCMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTCMP1` reader - Master compare 1"]
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
            true => MSTCMP1_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTCMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == MSTCMP1_A::RESETCOUNTER
    }
}
impl core::ops::Deref for MSTCMP1_R {
    type Target = crate::FieldReader<bool, MSTCMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP1` writer - Master compare 1"]
pub struct MSTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTCMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon master timer compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTCMP1_A::RESETCOUNTER)
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
#[doc = "Master timer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPER_A {
    #[doc = "0: Master timer period event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon master timer period event"]
    RESETCOUNTER = 1,
}
impl From<MSTPER_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPER` reader - Master timer Period"]
pub struct MSTPER_R(crate::FieldReader<bool, MSTPER_A>);
impl MSTPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTPER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPER_A {
        match self.bits {
            false => MSTPER_A::NOEFFECT,
            true => MSTPER_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTPER_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == MSTPER_A::RESETCOUNTER
    }
}
impl core::ops::Deref for MSTPER_R {
    type Target = crate::FieldReader<bool, MSTPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTPER` writer - Master timer Period"]
pub struct MSTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer period event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon master timer period event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(MSTPER_A::RESETCOUNTER)
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
#[doc = "Timer A compare 4 reset"]
pub type CMP4_A = CMP2_A;
#[doc = "Field `CMP4` reader - Timer A compare 4 reset"]
pub type CMP4_R = CMP2_R;
#[doc = "Field `CMP4` writer - Timer A compare 4 reset"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer X compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(CMP4_A::RESETCOUNTER)
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
#[doc = "Timer A compare 2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP2_A {
    #[doc = "0: Timer X compare Z event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon timer X compare Z event"]
    RESETCOUNTER = 1,
}
impl From<CMP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP2` reader - Timer A compare 2 reset"]
pub struct CMP2_R(crate::FieldReader<bool, CMP2_A>);
impl CMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP2_A {
        match self.bits {
            false => CMP2_A::NOEFFECT,
            true => CMP2_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == CMP2_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == CMP2_A::RESETCOUNTER
    }
}
impl core::ops::Deref for CMP2_R {
    type Target = crate::FieldReader<bool, CMP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2` writer - Timer A compare 2 reset"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer X compare Z event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP2_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon timer X compare Z event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(CMP2_A::RESETCOUNTER)
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
#[doc = "Timer A Update reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDT_A {
    #[doc = "0: Update event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer X counter is reset upon update event"]
    RESETCOUNTER = 1,
}
impl From<UPDT_A> for bool {
    #[inline(always)]
    fn from(variant: UPDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDT` reader - Timer A Update reset"]
pub struct UPDT_R(crate::FieldReader<bool, UPDT_A>);
impl UPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDT_A {
        match self.bits {
            false => UPDT_A::NOEFFECT,
            true => UPDT_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == UPDT_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        **self == UPDT_A::RESETCOUNTER
    }
}
impl core::ops::Deref for UPDT_R {
    type Target = crate::FieldReader<bool, UPDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDT` writer - Timer A Update reset"]
pub struct UPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDT_A::NOEFFECT)
    }
    #[doc = "Timer X counter is reset upon update event"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(UPDT_A::RESETCOUNTER)
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
impl R {
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    pub fn timacmp4(&self) -> TIMACMP4_R {
        TIMACMP4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    pub fn timacmp2(&self) -> TIMACMP2_R {
        TIMACMP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    pub fn timacmp1(&self) -> TIMACMP1_R {
        TIMACMP1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&mut self) -> TIMECMP4_W {
        TIMECMP4_W { w: self }
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&mut self) -> TIMECMP2_W {
        TIMECMP2_W { w: self }
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&mut self) -> TIMECMP1_W {
        TIMECMP1_W { w: self }
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W {
        TIMDCMP4_W { w: self }
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W {
        TIMDCMP2_W { w: self }
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W {
        TIMDCMP1_W { w: self }
    }
    #[doc = "Bit 24 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W {
        TIMBCMP4_W { w: self }
    }
    #[doc = "Bit 23 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W {
        TIMBCMP2_W { w: self }
    }
    #[doc = "Bit 22 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W {
        TIMBCMP1_W { w: self }
    }
    #[doc = "Bit 21 - Timer A Compare 4"]
    #[inline(always)]
    pub fn timacmp4(&mut self) -> TIMACMP4_W {
        TIMACMP4_W { w: self }
    }
    #[doc = "Bit 20 - Timer A Compare 2"]
    #[inline(always)]
    pub fn timacmp2(&mut self) -> TIMACMP2_W {
        TIMACMP2_W { w: self }
    }
    #[doc = "Bit 19 - Timer A Compare 1"]
    #[inline(always)]
    pub fn timacmp1(&mut self) -> TIMACMP1_W {
        TIMACMP1_W { w: self }
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W {
        EXTEVNT10_W { w: self }
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W {
        EXTEVNT9_W { w: self }
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W {
        EXTEVNT8_W { w: self }
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W {
        EXTEVNT7_W { w: self }
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W {
        EXTEVNT6_W { w: self }
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W {
        EXTEVNT5_W { w: self }
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W {
        EXTEVNT4_W { w: self }
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W {
        EXTEVNT3_W { w: self }
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W {
        EXTEVNT2_W { w: self }
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W {
        EXTEVNT1_W { w: self }
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W {
        MSTPER_W { w: self }
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&mut self) -> UPDT_W {
        UPDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcr](index.html) module"]
pub struct RSTCR_SPEC;
impl crate::RegisterSpec for RSTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcr::R](R) reader structure"]
impl crate::Readable for RSTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcr::W](W) writer structure"]
impl crate::Writable for RSTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCR to value 0"]
impl crate::Resettable for RSTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
