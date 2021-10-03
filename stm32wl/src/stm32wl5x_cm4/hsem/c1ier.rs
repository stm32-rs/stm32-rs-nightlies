#[doc = "Register `C1IER` reader"]
pub struct R(crate::R<C1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IER` writer"]
pub struct W(crate::W<C1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IER_SPEC>;
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
impl From<crate::W<C1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt semaphore n enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISE0_A {
    #[doc = "0: Interrupt generation disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt generation enabled"]
    ENABLED = 1,
}
impl From<ISE0_A> for bool {
    #[inline(always)]
    fn from(variant: ISE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISE0` reader - Interrupt semaphore n enable bit"]
pub struct ISE0_R(crate::FieldReader<bool, ISE0_A>);
impl ISE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISE0_A {
        match self.bits {
            false => ISE0_A::DISABLED,
            true => ISE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ISE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ISE0_A::ENABLED
    }
}
impl core::ops::Deref for ISE0_R {
    type Target = crate::FieldReader<bool, ISE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISE0` writer - Interrupt semaphore n enable bit"]
pub struct ISE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE0_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE0_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE1_A = ISE0_A;
#[doc = "Field `ISE1` reader - Interrupt semaphore n enable bit"]
pub type ISE1_R = ISE0_R;
#[doc = "Field `ISE1` writer - Interrupt semaphore n enable bit"]
pub struct ISE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE1_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE1_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE2_A = ISE0_A;
#[doc = "Field `ISE2` reader - Interrupt semaphore n enable bit"]
pub type ISE2_R = ISE0_R;
#[doc = "Field `ISE2` writer - Interrupt semaphore n enable bit"]
pub struct ISE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE2_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE2_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE3_A = ISE0_A;
#[doc = "Field `ISE3` reader - Interrupt semaphore n enable bit"]
pub type ISE3_R = ISE0_R;
#[doc = "Field `ISE3` writer - Interrupt semaphore n enable bit"]
pub struct ISE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE3_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE3_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE4_A = ISE0_A;
#[doc = "Field `ISE4` reader - Interrupt semaphore n enable bit"]
pub type ISE4_R = ISE0_R;
#[doc = "Field `ISE4` writer - Interrupt semaphore n enable bit"]
pub struct ISE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE4_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE4_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE5_A = ISE0_A;
#[doc = "Field `ISE5` reader - Interrupt semaphore n enable bit"]
pub type ISE5_R = ISE0_R;
#[doc = "Field `ISE5` writer - Interrupt semaphore n enable bit"]
pub struct ISE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE5_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE5_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE6_A = ISE0_A;
#[doc = "Field `ISE6` reader - Interrupt semaphore n enable bit"]
pub type ISE6_R = ISE0_R;
#[doc = "Field `ISE6` writer - Interrupt semaphore n enable bit"]
pub struct ISE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE6_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE6_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE7_A = ISE0_A;
#[doc = "Field `ISE7` reader - Interrupt semaphore n enable bit"]
pub type ISE7_R = ISE0_R;
#[doc = "Field `ISE7` writer - Interrupt semaphore n enable bit"]
pub struct ISE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE7_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE7_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE8_A = ISE0_A;
#[doc = "Field `ISE8` reader - Interrupt semaphore n enable bit"]
pub type ISE8_R = ISE0_R;
#[doc = "Field `ISE8` writer - Interrupt semaphore n enable bit"]
pub struct ISE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE8_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE8_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE9_A = ISE0_A;
#[doc = "Field `ISE9` reader - Interrupt semaphore n enable bit"]
pub type ISE9_R = ISE0_R;
#[doc = "Field `ISE9` writer - Interrupt semaphore n enable bit"]
pub struct ISE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE9_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE9_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISE10_A {
    #[doc = "0: Interrupt generation disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt generation enabled"]
    ENABLED = 1,
}
impl From<ISE10_A> for bool {
    #[inline(always)]
    fn from(variant: ISE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISE10` reader - Interrupt semaphore n enable bit"]
pub struct ISE10_R(crate::FieldReader<bool, ISE10_A>);
impl ISE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISE10_A {
        match self.bits {
            false => ISE10_A::DISABLED,
            true => ISE10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ISE10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ISE10_A::ENABLED
    }
}
impl core::ops::Deref for ISE10_R {
    type Target = crate::FieldReader<bool, ISE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISE10` writer - Interrupt semaphore n enable bit"]
pub struct ISE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE10_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE10_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE11_A = ISE10_A;
#[doc = "Field `ISE11` reader - Interrupt semaphore n enable bit"]
pub type ISE11_R = ISE10_R;
#[doc = "Field `ISE11` writer - Interrupt semaphore n enable bit"]
pub struct ISE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE11_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE11_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE12_A = ISE10_A;
#[doc = "Field `ISE12` reader - Interrupt semaphore n enable bit"]
pub type ISE12_R = ISE10_R;
#[doc = "Field `ISE12` writer - Interrupt semaphore n enable bit"]
pub struct ISE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE12_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE12_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE13_A = ISE10_A;
#[doc = "Field `ISE13` reader - Interrupt semaphore n enable bit"]
pub type ISE13_R = ISE10_R;
#[doc = "Field `ISE13` writer - Interrupt semaphore n enable bit"]
pub struct ISE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE13_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE13_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE14_A = ISE10_A;
#[doc = "Field `ISE14` reader - Interrupt semaphore n enable bit"]
pub type ISE14_R = ISE10_R;
#[doc = "Field `ISE14` writer - Interrupt semaphore n enable bit"]
pub struct ISE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE14_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE14_A::ENABLED)
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
#[doc = "Interrupt semaphore n enable bit"]
pub type ISE15_A = ISE10_A;
#[doc = "Field `ISE15` reader - Interrupt semaphore n enable bit"]
pub type ISE15_R = ISE10_R;
#[doc = "Field `ISE15` writer - Interrupt semaphore n enable bit"]
pub struct ISE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISE15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE15_A::DISABLED)
    }
    #[doc = "Interrupt generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE15_A::ENABLED)
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
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise0(&self) -> ISE0_R {
        ISE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise1(&self) -> ISE1_R {
        ISE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise2(&self) -> ISE2_R {
        ISE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise3(&self) -> ISE3_R {
        ISE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise4(&self) -> ISE4_R {
        ISE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise5(&self) -> ISE5_R {
        ISE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise6(&self) -> ISE6_R {
        ISE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise7(&self) -> ISE7_R {
        ISE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise8(&self) -> ISE8_R {
        ISE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise9(&self) -> ISE9_R {
        ISE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise10(&self) -> ISE10_R {
        ISE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise11(&self) -> ISE11_R {
        ISE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise12(&self) -> ISE12_R {
        ISE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise13(&self) -> ISE13_R {
        ISE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise14(&self) -> ISE14_R {
        ISE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise15(&self) -> ISE15_R {
        ISE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise0(&mut self) -> ISE0_W {
        ISE0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise1(&mut self) -> ISE1_W {
        ISE1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise2(&mut self) -> ISE2_W {
        ISE2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise3(&mut self) -> ISE3_W {
        ISE3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise4(&mut self) -> ISE4_W {
        ISE4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise5(&mut self) -> ISE5_W {
        ISE5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise6(&mut self) -> ISE6_W {
        ISE6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise7(&mut self) -> ISE7_W {
        ISE7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise8(&mut self) -> ISE8_W {
        ISE8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise9(&mut self) -> ISE9_W {
        ISE9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise10(&mut self) -> ISE10_W {
        ISE10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise11(&mut self) -> ISE11_W {
        ISE11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise12(&mut self) -> ISE12_W {
        ISE12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise13(&mut self) -> ISE13_W {
        ISE13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise14(&mut self) -> ISE14_W {
        ISE14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise15(&mut self) -> ISE15_W {
        ISE15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1ier](index.html) module"]
pub struct C1IER_SPEC;
impl crate::RegisterSpec for C1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1ier::R](R) reader structure"]
impl crate::Readable for C1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1ier::W](W) writer structure"]
impl crate::Writable for C1IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1IER to value 0"]
impl crate::Resettable for C1IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
