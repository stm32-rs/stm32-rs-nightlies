#[doc = "Register `C2ICR` reader"]
pub struct R(crate::R<C2ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2ICR` writer"]
pub struct W(crate::W<C2ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ICR_SPEC>;
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
impl From<crate::W<C2ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt(N) semaphore n clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC0_A {
    #[doc = "0: Always reads 0"]
    NOEFFECT = 0,
}
impl From<ISC0_A> for bool {
    #[inline(always)]
    fn from(variant: ISC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISC0` reader - Interrupt(N) semaphore n clear bit"]
pub struct ISC0_R(crate::FieldReader<bool, ISC0_A>);
impl ISC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ISC0_A> {
        match self.bits {
            false => Some(ISC0_A::NOEFFECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == ISC0_A::NOEFFECT
    }
}
impl core::ops::Deref for ISC0_R {
    type Target = crate::FieldReader<bool, ISC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt(N) semaphore n clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC0_AW {
    #[doc = "0: Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    NOEFFECT = 0,
    #[doc = "1: Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    CLEAR = 1,
}
impl From<ISC0_AW> for bool {
    #[inline(always)]
    fn from(variant: ISC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISC0` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC0_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC0_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC1_A = ISC0_A;
#[doc = "Field `ISC1` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC1_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC1_AW = ISC0_AW;
#[doc = "Field `ISC1` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC1_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC1_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC2_A = ISC0_A;
#[doc = "Field `ISC2` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC2_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC2_AW = ISC0_AW;
#[doc = "Field `ISC2` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC2_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC2_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC3_A = ISC0_A;
#[doc = "Field `ISC3` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC3_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC3_AW = ISC0_AW;
#[doc = "Field `ISC3` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC3_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC3_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC4_A = ISC0_A;
#[doc = "Field `ISC4` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC4_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC4_AW = ISC0_AW;
#[doc = "Field `ISC4` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC4_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC4_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC5_A = ISC0_A;
#[doc = "Field `ISC5` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC5_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC5_AW = ISC0_AW;
#[doc = "Field `ISC5` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC5_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC5_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC6_A = ISC0_A;
#[doc = "Field `ISC6` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC6_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC6_AW = ISC0_AW;
#[doc = "Field `ISC6` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC6_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC6_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC7_A = ISC0_A;
#[doc = "Field `ISC7` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC7_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC7_AW = ISC0_AW;
#[doc = "Field `ISC7` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC7_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC7_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC8_A = ISC0_A;
#[doc = "Field `ISC8` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC8_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC8_AW = ISC0_AW;
#[doc = "Field `ISC8` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC8_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC8_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC9_A = ISC0_A;
#[doc = "Field `ISC9` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC9_R = ISC0_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC9_AW = ISC0_AW;
#[doc = "Field `ISC9` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC9_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC9_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC10_A {
    #[doc = "0: Always reads 0"]
    NOEFFECT = 0,
}
impl From<ISC10_A> for bool {
    #[inline(always)]
    fn from(variant: ISC10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISC10` reader - Interrupt(N) semaphore n clear bit"]
pub struct ISC10_R(crate::FieldReader<bool, ISC10_A>);
impl ISC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISC10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ISC10_A> {
        match self.bits {
            false => Some(ISC10_A::NOEFFECT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == ISC10_A::NOEFFECT
    }
}
impl core::ops::Deref for ISC10_R {
    type Target = crate::FieldReader<bool, ISC10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt(N) semaphore n clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISC10_AW {
    #[doc = "0: Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    NOEFFECT = 0,
    #[doc = "1: Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    CLEAR = 1,
}
impl From<ISC10_AW> for bool {
    #[inline(always)]
    fn from(variant: ISC10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISC10` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC10_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC10_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC11_A = ISC10_A;
#[doc = "Field `ISC11` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC11_R = ISC10_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC11_AW = ISC10_AW;
#[doc = "Field `ISC11` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC11_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC11_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC12_A = ISC10_A;
#[doc = "Field `ISC12` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC12_R = ISC10_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC12_AW = ISC10_AW;
#[doc = "Field `ISC12` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC12_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC12_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC13_A = ISC10_A;
#[doc = "Field `ISC13` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC13_R = ISC10_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC13_AW = ISC10_AW;
#[doc = "Field `ISC13` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC13_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC13_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC14_A = ISC10_A;
#[doc = "Field `ISC14` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC14_R = ISC10_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC14_AW = ISC10_AW;
#[doc = "Field `ISC14` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC14_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC14_AW::CLEAR)
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
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC15_A = ISC10_A;
#[doc = "Field `ISC15` reader - Interrupt(N) semaphore n clear bit"]
pub type ISC15_R = ISC10_R;
#[doc = "Interrupt(N) semaphore n clear bit"]
pub type ISC15_AW = ISC10_AW;
#[doc = "Field `ISC15` writer - Interrupt(N) semaphore n clear bit"]
pub struct ISC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISC15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx not affected"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC15_AW::NOEFFECT)
    }
    #[doc = "Interrupt semaphore x status ISFx and masked status MISFx cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC15_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc8(&self) -> ISC8_R {
        ISC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc9(&self) -> ISC9_R {
        ISC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc10(&self) -> ISC10_R {
        ISC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc11(&self) -> ISC11_R {
        ISC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc12(&self) -> ISC12_R {
        ISC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc13(&self) -> ISC13_R {
        ISC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc14(&self) -> ISC14_R {
        ISC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc15(&self) -> ISC15_R {
        ISC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc0(&mut self) -> ISC0_W {
        ISC0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc1(&mut self) -> ISC1_W {
        ISC1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc2(&mut self) -> ISC2_W {
        ISC2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc3(&mut self) -> ISC3_W {
        ISC3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc4(&mut self) -> ISC4_W {
        ISC4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc5(&mut self) -> ISC5_W {
        ISC5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc6(&mut self) -> ISC6_W {
        ISC6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc7(&mut self) -> ISC7_W {
        ISC7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc8(&mut self) -> ISC8_W {
        ISC8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc9(&mut self) -> ISC9_W {
        ISC9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc10(&mut self) -> ISC10_W {
        ISC10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc11(&mut self) -> ISC11_W {
        ISC11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc12(&mut self) -> ISC12_W {
        ISC12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc13(&mut self) -> ISC13_W {
        ISC13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc14(&mut self) -> ISC14_W {
        ISC14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc15(&mut self) -> ISC15_W {
        ISC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2icr](index.html) module"]
pub struct C2ICR_SPEC;
impl crate::RegisterSpec for C2ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2icr::R](R) reader structure"]
impl crate::Readable for C2ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2icr::W](W) writer structure"]
impl crate::Writable for C2ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2ICR to value 0"]
impl crate::Resettable for C2ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
