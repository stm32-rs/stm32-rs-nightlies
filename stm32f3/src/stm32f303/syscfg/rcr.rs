#[doc = "Register `RCR` reader"]
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR` writer"]
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CCM SRAM page write protection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGE0_WP_A {
    #[doc = "0: Write protection of pagex is disabled"]
    DISABLED = 0,
    #[doc = "1: Write protection of pagex is enabled"]
    ENABLED = 1,
}
impl From<PAGE0_WP_A> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAGE0_WP` reader - CCM SRAM page write protection bit"]
pub struct PAGE0_WP_R(crate::FieldReader<bool, PAGE0_WP_A>);
impl PAGE0_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAGE0_WP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGE0_WP_A {
        match self.bits {
            false => PAGE0_WP_A::DISABLED,
            true => PAGE0_WP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PAGE0_WP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PAGE0_WP_A::ENABLED
    }
}
impl core::ops::Deref for PAGE0_WP_R {
    type Target = crate::FieldReader<bool, PAGE0_WP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAGE0_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE0_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE0_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE0_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE1_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE1_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE1_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE1_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE1_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE1_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE1_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE1_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE2_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE2_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE2_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE2_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE2_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE2_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE2_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE2_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE3_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE3_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE3_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE3_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE3_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE3_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE3_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE3_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE4_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE4_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE4_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE4_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE4_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE4_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE4_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE4_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE4_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE5_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE5_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE5_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE5_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE5_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE5_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE5_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE5_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE5_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE6_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE6_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE6_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE6_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE6_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE6_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE6_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE6_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE6_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE7_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE7_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE7_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE7_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE7_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE7_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE7_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE7_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE7_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE8_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE8_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE8_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE8_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE8_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE8_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE8_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE8_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE8_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE9_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE9_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE9_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE9_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE9_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE9_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE9_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE9_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE9_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE10_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE10_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE10_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE10_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE10_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE10_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE10_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE10_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE10_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE11_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE11_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE11_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE11_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE11_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE11_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE11_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE11_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE11_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE12_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE12_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE12_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE12_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE12_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE12_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE12_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE12_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE12_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE13_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE13_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE13_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE13_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE13_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE13_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE13_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE13_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE13_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE14_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE14_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE14_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE14_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE14_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE14_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE14_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE14_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE14_WP_A::ENABLED)
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
#[doc = "CCM SRAM page write protection bit"]
pub type PAGE15_WP_A = PAGE0_WP_A;
#[doc = "Field `PAGE15_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE15_WP_R = PAGE0_WP_R;
#[doc = "Field `PAGE15_WP` writer - CCM SRAM page write protection bit"]
pub struct PAGE15_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE15_WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGE15_WP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE15_WP_A::DISABLED)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE15_WP_A::ENABLED)
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
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W {
        PAGE0_WP_W { w: self }
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W {
        PAGE1_WP_W { w: self }
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W {
        PAGE2_WP_W { w: self }
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W {
        PAGE3_WP_W { w: self }
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W {
        PAGE4_WP_W { w: self }
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W {
        PAGE5_WP_W { w: self }
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W {
        PAGE6_WP_W { w: self }
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W {
        PAGE7_WP_W { w: self }
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W {
        PAGE8_WP_W { w: self }
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W {
        PAGE9_WP_W { w: self }
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W {
        PAGE10_WP_W { w: self }
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W {
        PAGE11_WP_W { w: self }
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W {
        PAGE12_WP_W { w: self }
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W {
        PAGE13_WP_W { w: self }
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W {
        PAGE14_WP_W { w: self }
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W {
        PAGE15_WP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM SRAM protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](index.html) module"]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr::R](R) reader structure"]
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr::W](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
