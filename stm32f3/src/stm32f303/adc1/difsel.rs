#[doc = "Register `DIFSEL` reader"]
pub struct R(crate::R<DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIFSEL` writer"]
pub struct W(crate::W<DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIFSEL_SPEC>;
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
impl From<crate::W<DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Differential mode for channels 15 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFSEL_10_A {
    #[doc = "0: Input channel is configured in single-ended mode"]
    SINGLEENDED = 0,
    #[doc = "1: Input channel is configured in differential mode"]
    DIFFERENTIAL = 1,
}
impl From<DIFSEL_10_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSEL_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFSEL_10` reader - Differential mode for channels 15 to 1"]
pub struct DIFSEL_10_R(crate::FieldReader<bool, DIFSEL_10_A>);
impl DIFSEL_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFSEL_10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSEL_10_A {
        match self.bits {
            false => DIFSEL_10_A::SINGLEENDED,
            true => DIFSEL_10_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLEENDED`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        **self == DIFSEL_10_A::SINGLEENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        **self == DIFSEL_10_A::DIFFERENTIAL
    }
}
impl core::ops::Deref for DIFSEL_10_R {
    type Target = crate::FieldReader<bool, DIFSEL_10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFSEL_10` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_10_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_10_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_11_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_11` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_11_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_11` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_11_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_11_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_12_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_12` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_12_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_12` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_12_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_12_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_13_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_13` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_13_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_13` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_13_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_13_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_14_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_14` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_14_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_14` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_14_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_14_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_15_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_15` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_15_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_15` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_15_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_15_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_16_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_16` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_16_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_16` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_16_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_16_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_17_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_17` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_17_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_17` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_17_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_17_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_18_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_18` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_18_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_18` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_18_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_18_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_19_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_19` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_19_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_19` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_19_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_19_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_110_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_110` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_110_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_110` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_110_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_110_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_110_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_110_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_110_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_111_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_111` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_111_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_111` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_111_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_111_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_111_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_111_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_111_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_112_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_112` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_112_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_112` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_112_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_112_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_112_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_112_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_112_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_113_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_113` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_113_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_113` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_113_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_113_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_113_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_113_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_113_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_114_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_114` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_114_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_114` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_114_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_114_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_114_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_114_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_114_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_115_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_115` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_115_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_115` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_115_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_115_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_115_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_115_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_115_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_116_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_116` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_116_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_116` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_116_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_116_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_116_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_116_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_116_A::DIFFERENTIAL)
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
#[doc = "Differential mode for channels 15 to 1"]
pub type DIFSEL_117_A = DIFSEL_10_A;
#[doc = "Field `DIFSEL_117` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_117_R = DIFSEL_10_R;
#[doc = "Field `DIFSEL_117` writer - Differential mode for channels 15 to 1"]
pub struct DIFSEL_117_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFSEL_117_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFSEL_117_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input channel is configured in single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(DIFSEL_117_A::SINGLEENDED)
    }
    #[doc = "Input channel is configured in differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(DIFSEL_117_A::DIFFERENTIAL)
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
impl R {
    #[doc = "Bit 1 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_10(&self) -> DIFSEL_10_R {
        DIFSEL_10_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_11(&self) -> DIFSEL_11_R {
        DIFSEL_11_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_12(&self) -> DIFSEL_12_R {
        DIFSEL_12_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_13(&self) -> DIFSEL_13_R {
        DIFSEL_13_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_14(&self) -> DIFSEL_14_R {
        DIFSEL_14_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_15(&self) -> DIFSEL_15_R {
        DIFSEL_15_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_16(&self) -> DIFSEL_16_R {
        DIFSEL_16_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_17(&self) -> DIFSEL_17_R {
        DIFSEL_17_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_18(&self) -> DIFSEL_18_R {
        DIFSEL_18_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_19(&self) -> DIFSEL_19_R {
        DIFSEL_19_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_110(&self) -> DIFSEL_110_R {
        DIFSEL_110_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_111(&self) -> DIFSEL_111_R {
        DIFSEL_111_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_112(&self) -> DIFSEL_112_R {
        DIFSEL_112_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_113(&self) -> DIFSEL_113_R {
        DIFSEL_113_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_114(&self) -> DIFSEL_114_R {
        DIFSEL_114_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_115(&self) -> DIFSEL_115_R {
        DIFSEL_115_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_116(&self) -> DIFSEL_116_R {
        DIFSEL_116_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_117(&self) -> DIFSEL_117_R {
        DIFSEL_117_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_10(&mut self) -> DIFSEL_10_W {
        DIFSEL_10_W { w: self }
    }
    #[doc = "Bit 2 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_11(&mut self) -> DIFSEL_11_W {
        DIFSEL_11_W { w: self }
    }
    #[doc = "Bit 3 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_12(&mut self) -> DIFSEL_12_W {
        DIFSEL_12_W { w: self }
    }
    #[doc = "Bit 4 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_13(&mut self) -> DIFSEL_13_W {
        DIFSEL_13_W { w: self }
    }
    #[doc = "Bit 5 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_14(&mut self) -> DIFSEL_14_W {
        DIFSEL_14_W { w: self }
    }
    #[doc = "Bit 6 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_15(&mut self) -> DIFSEL_15_W {
        DIFSEL_15_W { w: self }
    }
    #[doc = "Bit 7 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_16(&mut self) -> DIFSEL_16_W {
        DIFSEL_16_W { w: self }
    }
    #[doc = "Bit 8 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_17(&mut self) -> DIFSEL_17_W {
        DIFSEL_17_W { w: self }
    }
    #[doc = "Bit 9 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_18(&mut self) -> DIFSEL_18_W {
        DIFSEL_18_W { w: self }
    }
    #[doc = "Bit 10 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_19(&mut self) -> DIFSEL_19_W {
        DIFSEL_19_W { w: self }
    }
    #[doc = "Bit 11 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_110(&mut self) -> DIFSEL_110_W {
        DIFSEL_110_W { w: self }
    }
    #[doc = "Bit 12 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_111(&mut self) -> DIFSEL_111_W {
        DIFSEL_111_W { w: self }
    }
    #[doc = "Bit 13 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_112(&mut self) -> DIFSEL_112_W {
        DIFSEL_112_W { w: self }
    }
    #[doc = "Bit 14 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_113(&mut self) -> DIFSEL_113_W {
        DIFSEL_113_W { w: self }
    }
    #[doc = "Bit 15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_114(&mut self) -> DIFSEL_114_W {
        DIFSEL_114_W { w: self }
    }
    #[doc = "Bit 16 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_115(&mut self) -> DIFSEL_115_W {
        DIFSEL_115_W { w: self }
    }
    #[doc = "Bit 17 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_116(&mut self) -> DIFSEL_116_W {
        DIFSEL_116_W { w: self }
    }
    #[doc = "Bit 18 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_117(&mut self) -> DIFSEL_117_W {
        DIFSEL_117_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Differential Mode Selection Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsel](index.html) module"]
pub struct DIFSEL_SPEC;
impl crate::RegisterSpec for DIFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [difsel::R](R) reader structure"]
impl crate::Readable for DIFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [difsel::W](W) writer structure"]
impl crate::Writable for DIFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
