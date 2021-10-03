#[doc = "Register `RSTD2R` reader"]
pub struct R(crate::R<RSTD2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTD2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTD2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTD2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTD2R` writer"]
pub struct W(crate::W<RSTD2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTD2R_SPEC>;
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
impl From<crate::W<RSTD2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTD2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "UPDATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATE_A {
    #[doc = "0: Register update event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Register update event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE` reader - UPDATE"]
pub struct UPDATE_R(crate::FieldReader<bool, UPDATE_A>);
impl UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::NOEFFECT,
            true => UPDATE_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == UPDATE_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == UPDATE_A::SETINACTIVE
    }
}
impl core::ops::Deref for UPDATE_R {
    type Target = crate::FieldReader<bool, UPDATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE` writer - UPDATE"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register update event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(UPDATE_A::NOEFFECT)
    }
    #[doc = "Register update event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(UPDATE_A::SETINACTIVE)
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
#[doc = "EXTEVNT10"]
pub type EXTEVNT10_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT10` reader - EXTEVNT10"]
pub type EXTEVNT10_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT10` writer - EXTEVNT10"]
pub struct EXTEVNT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT10_A::SETINACTIVE)
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
#[doc = "EXTEVNT9"]
pub type EXTEVNT9_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT9` reader - EXTEVNT9"]
pub type EXTEVNT9_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT9` writer - EXTEVNT9"]
pub struct EXTEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT9_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT9_A::SETINACTIVE)
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
#[doc = "EXTEVNT8"]
pub type EXTEVNT8_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT8` reader - EXTEVNT8"]
pub type EXTEVNT8_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT8` writer - EXTEVNT8"]
pub struct EXTEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT8_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT8_A::SETINACTIVE)
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
#[doc = "EXTEVNT7"]
pub type EXTEVNT7_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT7` reader - EXTEVNT7"]
pub type EXTEVNT7_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT7` writer - EXTEVNT7"]
pub struct EXTEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT7_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT7_A::SETINACTIVE)
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
#[doc = "EXTEVNT6"]
pub type EXTEVNT6_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT6` reader - EXTEVNT6"]
pub type EXTEVNT6_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT6` writer - EXTEVNT6"]
pub struct EXTEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT6_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT6_A::SETINACTIVE)
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
#[doc = "EXTEVNT5"]
pub type EXTEVNT5_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT5` reader - EXTEVNT5"]
pub type EXTEVNT5_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT5` writer - EXTEVNT5"]
pub struct EXTEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT5_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT5_A::SETINACTIVE)
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
#[doc = "EXTEVNT4"]
pub type EXTEVNT4_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT4` reader - EXTEVNT4"]
pub type EXTEVNT4_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT4` writer - EXTEVNT4"]
pub struct EXTEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT4_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT4_A::SETINACTIVE)
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
#[doc = "EXTEVNT3"]
pub type EXTEVNT3_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT3` reader - EXTEVNT3"]
pub type EXTEVNT3_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT3` writer - EXTEVNT3"]
pub struct EXTEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT3_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT3_A::SETINACTIVE)
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
#[doc = "EXTEVNT2"]
pub type EXTEVNT2_A = EXTEVNT1_A;
#[doc = "Field `EXTEVNT2` reader - EXTEVNT2"]
pub type EXTEVNT2_R = EXTEVNT1_R;
#[doc = "Field `EXTEVNT2` writer - EXTEVNT2"]
pub struct EXTEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT2_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT2_A::SETINACTIVE)
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
#[doc = "EXTEVNT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTEVNT1_A {
    #[doc = "0: External event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: External event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<EXTEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTEVNT1` reader - EXTEVNT1"]
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
            true => EXTEVNT1_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == EXTEVNT1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == EXTEVNT1_A::SETINACTIVE
    }
}
impl core::ops::Deref for EXTEVNT1_R {
    type Target = crate::FieldReader<bool, EXTEVNT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT1` writer - EXTEVNT1"]
pub struct EXTEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTEVNT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::NOEFFECT)
    }
    #[doc = "External event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(EXTEVNT1_A::SETINACTIVE)
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
#[doc = "TIMEVNT9"]
pub type TIMEVNT9_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT9` reader - TIMEVNT9"]
pub type TIMEVNT9_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT9` writer - TIMEVNT9"]
pub struct TIMEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT9_A::SETINACTIVE)
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
#[doc = "TIMEVNT8"]
pub type TIMEVNT8_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT8` reader - TIMEVNT8"]
pub type TIMEVNT8_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT8` writer - TIMEVNT8"]
pub struct TIMEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT8_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT8_A::SETINACTIVE)
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
#[doc = "TIMEVNT7"]
pub type TIMEVNT7_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT7` reader - TIMEVNT7"]
pub type TIMEVNT7_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT7` writer - TIMEVNT7"]
pub struct TIMEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT7_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT7_A::SETINACTIVE)
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
#[doc = "TIMEVNT6"]
pub type TIMEVNT6_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT6` reader - TIMEVNT6"]
pub type TIMEVNT6_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT6` writer - TIMEVNT6"]
pub struct TIMEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT6_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT6_A::SETINACTIVE)
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
#[doc = "TIMEVNT5"]
pub type TIMEVNT5_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT5` reader - TIMEVNT5"]
pub type TIMEVNT5_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT5` writer - TIMEVNT5"]
pub struct TIMEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT5_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT5_A::SETINACTIVE)
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
#[doc = "TIMEVNT4"]
pub type TIMEVNT4_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT4` reader - TIMEVNT4"]
pub type TIMEVNT4_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT4` writer - TIMEVNT4"]
pub struct TIMEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT4_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT4_A::SETINACTIVE)
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
#[doc = "TIMEVNT3"]
pub type TIMEVNT3_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT3` reader - TIMEVNT3"]
pub type TIMEVNT3_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT3` writer - TIMEVNT3"]
pub struct TIMEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT3_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT3_A::SETINACTIVE)
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
#[doc = "TIMEVNT2"]
pub type TIMEVNT2_A = TIMEVNT1_A;
#[doc = "Field `TIMEVNT2` reader - TIMEVNT2"]
pub type TIMEVNT2_R = TIMEVNT1_R;
#[doc = "Field `TIMEVNT2` writer - TIMEVNT2"]
pub struct TIMEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT2_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT2_A::SETINACTIVE)
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
#[doc = "TIMEVNT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVNT1_A {
    #[doc = "0: Timer event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<TIMEVNT1_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEVNT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEVNT1` reader - TIMEVNT1"]
pub struct TIMEVNT1_R(crate::FieldReader<bool, TIMEVNT1_A>);
impl TIMEVNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEVNT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEVNT1_A {
        match self.bits {
            false => TIMEVNT1_A::NOEFFECT,
            true => TIMEVNT1_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TIMEVNT1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == TIMEVNT1_A::SETINACTIVE
    }
}
impl core::ops::Deref for TIMEVNT1_R {
    type Target = crate::FieldReader<bool, TIMEVNT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEVNT1` writer - TIMEVNT1"]
pub struct TIMEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEVNT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVNT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::NOEFFECT)
    }
    #[doc = "Timer event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(TIMEVNT1_A::SETINACTIVE)
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
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP4_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP4_A::SETINACTIVE)
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
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP3_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP3_A::SETINACTIVE)
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
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP2_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP2_A::SETINACTIVE)
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
#[doc = "MSTCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTCMP1_A {
    #[doc = "0: Master timer compare event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer compare event forces the output to its inactive state"]
    SETINACTIVE = 1,
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
            true => MSTCMP1_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTCMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == MSTCMP1_A::SETINACTIVE
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
    #[doc = "Master timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCMP1_A::NOEFFECT)
    }
    #[doc = "Master timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTCMP1_A::SETINACTIVE)
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
#[doc = "MSTPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTPER_A {
    #[doc = "0: Master timer counter roll-over/reset has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Master timer counter roll-over/reset forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<MSTPER_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPER` reader - MSTPER"]
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
            true => MSTPER_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == MSTPER_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == MSTPER_A::SETINACTIVE
    }
}
impl core::ops::Deref for MSTPER_R {
    type Target = crate::FieldReader<bool, MSTPER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTPER` writer - MSTPER"]
pub struct MSTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTPER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master timer counter roll-over/reset has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTPER_A::NOEFFECT)
    }
    #[doc = "Master timer counter roll-over/reset forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(MSTPER_A::SETINACTIVE)
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
#[doc = "CMP4"]
pub type CMP4_A = CMP1_A;
#[doc = "Field `CMP4` reader - CMP4"]
pub type CMP4_R = CMP1_R;
#[doc = "Field `CMP4` writer - CMP4"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP4_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP4_A::SETINACTIVE)
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
#[doc = "CMP3"]
pub type CMP3_A = CMP1_A;
#[doc = "Field `CMP3` reader - CMP3"]
pub type CMP3_R = CMP1_R;
#[doc = "Field `CMP3` writer - CMP3"]
pub struct CMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP3_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP3_A::SETINACTIVE)
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
#[doc = "CMP2"]
pub type CMP2_A = CMP1_A;
#[doc = "Field `CMP2` reader - CMP2"]
pub type CMP2_R = CMP1_R;
#[doc = "Field `CMP2` writer - CMP2"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP2_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP2_A::SETINACTIVE)
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
#[doc = "CMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP1_A {
    #[doc = "0: Timer compare event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer compare event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<CMP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1` reader - CMP1"]
pub struct CMP1_R(crate::FieldReader<bool, CMP1_A>);
impl CMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP1_A {
        match self.bits {
            false => CMP1_A::NOEFFECT,
            true => CMP1_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == CMP1_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == CMP1_A::SETINACTIVE
    }
}
impl core::ops::Deref for CMP1_R {
    type Target = crate::FieldReader<bool, CMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1` writer - CMP1"]
pub struct CMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer compare event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CMP1_A::NOEFFECT)
    }
    #[doc = "Timer compare event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(CMP1_A::SETINACTIVE)
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
#[doc = "PER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    #[doc = "0: Timer period event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer period event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - PER"]
pub struct PER_R(crate::FieldReader<bool, PER_A>);
impl PER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::NOEFFECT,
            true => PER_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == PER_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == PER_A::SETINACTIVE
    }
}
impl core::ops::Deref for PER_R {
    type Target = crate::FieldReader<bool, PER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER` writer - PER"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer period event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PER_A::NOEFFECT)
    }
    #[doc = "Timer period event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(PER_A::SETINACTIVE)
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
#[doc = "RESYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNC_A {
    #[doc = "0: Timer reset event coming solely from software or SYNC input event has no effect"]
    NOEFFECT = 0,
    #[doc = "1: Timer reset event coming solely from software or SYNC input event forces the output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<RESYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESYNC` reader - RESYNC"]
pub struct RESYNC_R(crate::FieldReader<bool, RESYNC_A>);
impl RESYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNC_A {
        match self.bits {
            false => RESYNC_A::NOEFFECT,
            true => RESYNC_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == RESYNC_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == RESYNC_A::SETINACTIVE
    }
}
impl core::ops::Deref for RESYNC_R {
    type Target = crate::FieldReader<bool, RESYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESYNC` writer - RESYNC"]
pub struct RESYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer reset event coming solely from software or SYNC input event has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RESYNC_A::NOEFFECT)
    }
    #[doc = "Timer reset event coming solely from software or SYNC input event forces the output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(RESYNC_A::SETINACTIVE)
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
#[doc = "SRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRT_A {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Force output to its inactive state"]
    SETINACTIVE = 1,
}
impl From<SRT_A> for bool {
    #[inline(always)]
    fn from(variant: SRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRT` reader - SRT"]
pub struct SRT_R(crate::FieldReader<bool, SRT_A>);
impl SRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRT_A {
        match self.bits {
            false => SRT_A::NOEFFECT,
            true => SRT_A::SETINACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == SRT_A::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SETINACTIVE`"]
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        **self == SRT_A::SETINACTIVE
    }
}
impl core::ops::Deref for SRT_R {
    type Target = crate::FieldReader<bool, SRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRT` writer - SRT"]
pub struct SRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SRT_A::NOEFFECT)
    }
    #[doc = "Force output to its inactive state"]
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(SRT_A::SETINACTIVE)
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
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W {
        EXTEVNT10_W { w: self }
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W {
        EXTEVNT9_W { w: self }
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W {
        EXTEVNT8_W { w: self }
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W {
        EXTEVNT7_W { w: self }
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W {
        EXTEVNT6_W { w: self }
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W {
        EXTEVNT5_W { w: self }
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W {
        EXTEVNT4_W { w: self }
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W {
        EXTEVNT3_W { w: self }
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W {
        EXTEVNT2_W { w: self }
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W {
        EXTEVNT1_W { w: self }
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W {
        TIMEVNT9_W { w: self }
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W {
        TIMEVNT8_W { w: self }
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W {
        TIMEVNT7_W { w: self }
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W {
        TIMEVNT6_W { w: self }
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W {
        TIMEVNT5_W { w: self }
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W {
        TIMEVNT4_W { w: self }
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W {
        TIMEVNT3_W { w: self }
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W {
        TIMEVNT2_W { w: self }
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W {
        TIMEVNT1_W { w: self }
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W {
        MSTPER_W { w: self }
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W {
        CMP3_W { w: self }
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W {
        CMP1_W { w: self }
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    pub fn resync(&mut self) -> RESYNC_W {
        RESYNC_W { w: self }
    }
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W {
        SRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstd2r](index.html) module"]
pub struct RSTD2R_SPEC;
impl crate::RegisterSpec for RSTD2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstd2r::R](R) reader structure"]
impl crate::Readable for RSTD2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstd2r::W](W) writer structure"]
impl crate::Writable for RSTD2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTD2R to value 0"]
impl crate::Resettable for RSTD2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
