#[doc = "Register `FTSR1` reader"]
pub struct R(crate::R<FTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR1` writer"]
pub struct W(crate::W<FTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR1_SPEC>;
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
impl From<crate::W<FTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT21_A = FT0_A;
#[doc = "Field `FT21` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT21_R = FT0_R;
#[doc = "Field `FT21` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FT21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT21_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT21_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT22_A = FT0_A;
#[doc = "Field `FT22` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT22_R = FT0_R;
#[doc = "Field `FT22` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT22_W<'a> {
    w: &'a mut W,
}
impl<'a> FT22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT22_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT22_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FT0_A {
    #[doc = "0: Falling edge trigger is disabled"]
    DISABLED = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    ENABLED = 1,
}
impl From<FT0_A> for bool {
    #[inline(always)]
    fn from(variant: FT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FT0` reader - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT0_R(crate::FieldReader<bool, FT0_A>);
impl FT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT0_A {
        match self.bits {
            false => FT0_A::DISABLED,
            true => FT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FT0_A::ENABLED
    }
}
impl core::ops::Deref for FT0_R {
    type Target = crate::FieldReader<bool, FT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT0` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT0_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT0_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT1_A = FT0_A;
#[doc = "Field `FT1` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT1_R = FT0_R;
#[doc = "Field `FT1` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT1_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT1_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT2_A = FT0_A;
#[doc = "Field `FT2` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT2_R = FT0_R;
#[doc = "Field `FT2` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT2_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT2_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT3_A = FT0_A;
#[doc = "Field `FT3` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT3_R = FT0_R;
#[doc = "Field `FT3` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT3_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT3_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT4_A = FT0_A;
#[doc = "Field `FT4` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT4_R = FT0_R;
#[doc = "Field `FT4` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT4_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT4_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT5_A = FT0_A;
#[doc = "Field `FT5` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT5_R = FT0_R;
#[doc = "Field `FT5` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT5_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT5_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT6_A = FT0_A;
#[doc = "Field `FT6` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT6_R = FT0_R;
#[doc = "Field `FT6` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT6_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT6_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT7_A = FT0_A;
#[doc = "Field `FT7` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT7_R = FT0_R;
#[doc = "Field `FT7` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT7_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT7_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT8_A = FT0_A;
#[doc = "Field `FT8` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT8_R = FT0_R;
#[doc = "Field `FT8` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT8_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT8_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT9_A = FT0_A;
#[doc = "Field `FT9` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT9_R = FT0_R;
#[doc = "Field `FT9` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT9_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT9_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT10_A = FT0_A;
#[doc = "Field `FT10` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT10_R = FT0_R;
#[doc = "Field `FT10` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT10_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT10_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT11_A = FT0_A;
#[doc = "Field `FT11` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT11_R = FT0_R;
#[doc = "Field `FT11` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT11_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT11_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT12_A = FT0_A;
#[doc = "Field `FT12` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT12_R = FT0_R;
#[doc = "Field `FT12` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT12_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT12_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT13_A = FT0_A;
#[doc = "Field `FT13` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT13_R = FT0_R;
#[doc = "Field `FT13` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT13_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT13_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT14_A = FT0_A;
#[doc = "Field `FT14` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT14_R = FT0_R;
#[doc = "Field `FT14` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT14_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT14_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT15_A = FT0_A;
#[doc = "Field `FT15` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT15_R = FT0_R;
#[doc = "Field `FT15` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT15_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT15_A::ENABLED)
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
#[doc = "Falling trigger event configuration bit of Configurable Event input"]
pub type FT16_A = FT0_A;
#[doc = "Field `FT16` reader - Falling trigger event configuration bit of Configurable Event input"]
pub type FT16_R = FT0_R;
#[doc = "Field `FT16` writer - Falling trigger event configuration bit of Configurable Event input"]
pub struct FT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT16_A::DISABLED)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT16_A::ENABLED)
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
impl R {
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft21(&mut self) -> FT21_W {
        FT21_W { w: self }
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft22(&mut self) -> FT22_W {
        FT22_W { w: self }
    }
    #[doc = "Bit 0 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W {
        FT0_W { w: self }
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W {
        FT1_W { w: self }
    }
    #[doc = "Bit 2 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W {
        FT2_W { w: self }
    }
    #[doc = "Bit 3 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W {
        FT3_W { w: self }
    }
    #[doc = "Bit 4 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W {
        FT4_W { w: self }
    }
    #[doc = "Bit 5 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W {
        FT5_W { w: self }
    }
    #[doc = "Bit 6 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W {
        FT6_W { w: self }
    }
    #[doc = "Bit 7 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W {
        FT7_W { w: self }
    }
    #[doc = "Bit 8 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W {
        FT8_W { w: self }
    }
    #[doc = "Bit 9 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W {
        FT9_W { w: self }
    }
    #[doc = "Bit 10 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W {
        FT10_W { w: self }
    }
    #[doc = "Bit 11 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W {
        FT11_W { w: self }
    }
    #[doc = "Bit 12 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W {
        FT12_W { w: self }
    }
    #[doc = "Bit 13 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W {
        FT13_W { w: self }
    }
    #[doc = "Bit 14 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W {
        FT14_W { w: self }
    }
    #[doc = "Bit 15 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W {
        FT15_W { w: self }
    }
    #[doc = "Bit 16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W {
        FT16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](index.html) module"]
pub struct FTSR1_SPEC;
impl crate::RegisterSpec for FTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr1::R](R) reader structure"]
impl crate::Readable for FTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](W) writer structure"]
impl crate::Writable for FTSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTSR1 to value 0"]
impl crate::Resettable for FTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
