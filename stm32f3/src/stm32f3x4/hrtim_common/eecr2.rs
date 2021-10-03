#[doc = "Register `EECR2` reader"]
pub struct R(crate::R<EECR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR2` writer"]
pub struct W(crate::W<EECR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR2_SPEC>;
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
impl From<crate::W<EECR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Event 10 Sensitivity"]
pub type EE10SNS_A = EE6SNS_A;
#[doc = "Field `EE10SNS` reader - External Event 10 Sensitivity"]
pub type EE10SNS_R = EE6SNS_R;
#[doc = "Field `EE10SNS` writer - External Event 10 Sensitivity"]
pub struct EE10SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10SNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE10SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE10SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE10SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE10SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "External Event 10 Polarity"]
pub type EE10POL_A = EE6POL_A;
#[doc = "Field `EE10POL` reader - External Event 10 Polarity"]
pub type EE10POL_R = EE6POL_R;
#[doc = "Field `EE10POL` writer - External Event 10 Polarity"]
pub struct EE10POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE10POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE10POL_A::ACTIVELOW)
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
#[doc = "External Event 10 Source"]
pub type EE10SRC_A = EE6SRC_A;
#[doc = "Field `EE10SRC` reader - External Event 10 Source"]
pub type EE10SRC_R = EE6SRC_R;
#[doc = "Field `EE10SRC` writer - External Event 10 Source"]
pub struct EE10SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE10SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE10SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE10SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE10SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE10SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "External Event 9 Sensitivity"]
pub type EE9SNS_A = EE6SNS_A;
#[doc = "Field `EE9SNS` reader - External Event 9 Sensitivity"]
pub type EE9SNS_R = EE6SNS_R;
#[doc = "Field `EE9SNS` writer - External Event 9 Sensitivity"]
pub struct EE9SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9SNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE9SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE9SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE9SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE9SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "External Event 9 Polarity"]
pub type EE9POL_A = EE6POL_A;
#[doc = "Field `EE9POL` reader - External Event 9 Polarity"]
pub type EE9POL_R = EE6POL_R;
#[doc = "Field `EE9POL` writer - External Event 9 Polarity"]
pub struct EE9POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE9POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE9POL_A::ACTIVELOW)
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
#[doc = "External Event 9 Source"]
pub type EE9SRC_A = EE6SRC_A;
#[doc = "Field `EE9SRC` reader - External Event 9 Source"]
pub type EE9SRC_R = EE6SRC_R;
#[doc = "Field `EE9SRC` writer - External Event 9 Source"]
pub struct EE9SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE9SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE9SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE9SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE9SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE9SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "External Event 8 Sensitivity"]
pub type EE8SNS_A = EE6SNS_A;
#[doc = "Field `EE8SNS` reader - External Event 8 Sensitivity"]
pub type EE8SNS_R = EE6SNS_R;
#[doc = "Field `EE8SNS` writer - External Event 8 Sensitivity"]
pub struct EE8SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8SNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE8SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE8SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE8SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE8SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "External Event 8 Polarity"]
pub type EE8POL_A = EE6POL_A;
#[doc = "Field `EE8POL` reader - External Event 8 Polarity"]
pub type EE8POL_R = EE6POL_R;
#[doc = "Field `EE8POL` writer - External Event 8 Polarity"]
pub struct EE8POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE8POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE8POL_A::ACTIVELOW)
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
#[doc = "External Event 8 Source"]
pub type EE8SRC_A = EE6SRC_A;
#[doc = "Field `EE8SRC` reader - External Event 8 Source"]
pub type EE8SRC_R = EE6SRC_R;
#[doc = "Field `EE8SRC` writer - External Event 8 Source"]
pub struct EE8SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE8SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE8SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE8SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE8SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE8SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "External Event 7 Sensitivity"]
pub type EE7SNS_A = EE6SNS_A;
#[doc = "Field `EE7SNS` reader - External Event 7 Sensitivity"]
pub type EE7SNS_R = EE6SNS_R;
#[doc = "Field `EE7SNS` writer - External Event 7 Sensitivity"]
pub struct EE7SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7SNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE7SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE7SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE7SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE7SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "External Event 7 Polarity"]
pub type EE7POL_A = EE6POL_A;
#[doc = "Field `EE7POL` reader - External Event 7 Polarity"]
pub type EE7POL_R = EE6POL_R;
#[doc = "Field `EE7POL` writer - External Event 7 Polarity"]
pub struct EE7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE7POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE7POL_A::ACTIVELOW)
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
#[doc = "External Event 7 Source"]
pub type EE7SRC_A = EE6SRC_A;
#[doc = "Field `EE7SRC` reader - External Event 7 Source"]
pub type EE7SRC_R = EE6SRC_R;
#[doc = "Field `EE7SRC` writer - External Event 7 Source"]
pub struct EE7SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE7SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE7SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE7SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE7SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE7SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "External Event 6 Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE6SNS_A {
    #[doc = "0: On active level defined by EExPOL bit"]
    ACTIVE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both edges"]
    BOTH = 3,
}
impl From<EE6SNS_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6SNS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EE6SNS` reader - External Event 6 Sensitivity"]
pub struct EE6SNS_R(crate::FieldReader<u8, EE6SNS_A>);
impl EE6SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6SNS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6SNS_A {
        match self.bits {
            0 => EE6SNS_A::ACTIVE,
            1 => EE6SNS_A::RISING,
            2 => EE6SNS_A::FALLING,
            3 => EE6SNS_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == EE6SNS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == EE6SNS_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == EE6SNS_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == EE6SNS_A::BOTH
    }
}
impl core::ops::Deref for EE6SNS_R {
    type Target = crate::FieldReader<u8, EE6SNS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6SNS` writer - External Event 6 Sensitivity"]
pub struct EE6SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6SNS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE6SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE6SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE6SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE6SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "External Event 6 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE6POL_A {
    #[doc = "0: External event is active high"]
    ACTIVEHIGH = 0,
    #[doc = "1: External event is active low"]
    ACTIVELOW = 1,
}
impl From<EE6POL_A> for bool {
    #[inline(always)]
    fn from(variant: EE6POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EE6POL` reader - External Event 6 Polarity"]
pub struct EE6POL_R(crate::FieldReader<bool, EE6POL_A>);
impl EE6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE6POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6POL_A {
        match self.bits {
            false => EE6POL_A::ACTIVEHIGH,
            true => EE6POL_A::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == EE6POL_A::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == EE6POL_A::ACTIVELOW
    }
}
impl core::ops::Deref for EE6POL_R {
    type Target = crate::FieldReader<bool, EE6POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6POL` writer - External Event 6 Polarity"]
pub struct EE6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE6POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE6POL_A::ACTIVELOW)
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
#[doc = "External Event 6 Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE6SRC_A {
    #[doc = "0: Source 1"]
    SRC1 = 0,
    #[doc = "1: Source 2"]
    SRC2 = 1,
    #[doc = "2: Source 3"]
    SRC3 = 2,
    #[doc = "3: Source 4"]
    SRC4 = 3,
}
impl From<EE6SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EE6SRC` reader - External Event 6 Source"]
pub struct EE6SRC_R(crate::FieldReader<u8, EE6SRC_A>);
impl EE6SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6SRC_A {
        match self.bits {
            0 => EE6SRC_A::SRC1,
            1 => EE6SRC_A::SRC2,
            2 => EE6SRC_A::SRC3,
            3 => EE6SRC_A::SRC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRC1`"]
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        **self == EE6SRC_A::SRC1
    }
    #[doc = "Checks if the value of the field is `SRC2`"]
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        **self == EE6SRC_A::SRC2
    }
    #[doc = "Checks if the value of the field is `SRC3`"]
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        **self == EE6SRC_A::SRC3
    }
    #[doc = "Checks if the value of the field is `SRC4`"]
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        **self == EE6SRC_A::SRC4
    }
}
impl core::ops::Deref for EE6SRC_R {
    type Target = crate::FieldReader<u8, EE6SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6SRC` writer - External Event 6 Source"]
pub struct EE6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE6SRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE6SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE6SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE6SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE6SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    pub fn ee10sns(&mut self) -> EE10SNS_W {
        EE10SNS_W { w: self }
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    pub fn ee10pol(&mut self) -> EE10POL_W {
        EE10POL_W { w: self }
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    pub fn ee10src(&mut self) -> EE10SRC_W {
        EE10SRC_W { w: self }
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    pub fn ee9sns(&mut self) -> EE9SNS_W {
        EE9SNS_W { w: self }
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    pub fn ee9pol(&mut self) -> EE9POL_W {
        EE9POL_W { w: self }
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    pub fn ee9src(&mut self) -> EE9SRC_W {
        EE9SRC_W { w: self }
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    pub fn ee8sns(&mut self) -> EE8SNS_W {
        EE8SNS_W { w: self }
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    pub fn ee8pol(&mut self) -> EE8POL_W {
        EE8POL_W { w: self }
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    pub fn ee8src(&mut self) -> EE8SRC_W {
        EE8SRC_W { w: self }
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    pub fn ee7sns(&mut self) -> EE7SNS_W {
        EE7SNS_W { w: self }
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    pub fn ee7pol(&mut self) -> EE7POL_W {
        EE7POL_W { w: self }
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    pub fn ee7src(&mut self) -> EE7SRC_W {
        EE7SRC_W { w: self }
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    pub fn ee6sns(&mut self) -> EE6SNS_W {
        EE6SNS_W { w: self }
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    pub fn ee6pol(&mut self) -> EE6POL_W {
        EE6POL_W { w: self }
    }
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    pub fn ee6src(&mut self) -> EE6SRC_W {
        EE6SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr2](index.html) module"]
pub struct EECR2_SPEC;
impl crate::RegisterSpec for EECR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr2::R](R) reader structure"]
impl crate::Readable for EECR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr2::W](W) writer structure"]
impl crate::Writable for EECR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR2 to value 0"]
impl crate::Resettable for EECR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
