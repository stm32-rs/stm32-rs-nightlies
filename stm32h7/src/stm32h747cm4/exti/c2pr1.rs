#[doc = "Register `C2PR1` reader"]
pub struct R(crate::R<C2PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2PR1` writer"]
pub struct W(crate::W<C2PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2PR1_SPEC>;
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
impl From<crate::W<C2PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR0_A> for bool {
    #[inline(always)]
    fn from(variant: PR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` reader - Configurable event input pending"]
pub struct PR0_R(crate::FieldReader<bool, PR0_A>);
impl PR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR0_A {
        match self.bits {
            false => PR0_A::NOTPENDING,
            true => PR0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == PR0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == PR0_A::PENDING
    }
}
impl core::ops::Deref for PR0_R {
    type Target = crate::FieldReader<bool, PR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR0_AW> for bool {
    #[inline(always)]
    fn from(variant: PR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` writer - Configurable event input pending"]
pub struct PR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR1_A = PR0_A;
#[doc = "Field `PR1` reader - Configurable event input pending"]
pub type PR1_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR1_AW = PR0_AW;
#[doc = "Field `PR1` writer - Configurable event input pending"]
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR1_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR2_A = PR0_A;
#[doc = "Field `PR2` reader - Configurable event input pending"]
pub type PR2_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR2_AW = PR0_AW;
#[doc = "Field `PR2` writer - Configurable event input pending"]
pub struct PR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR2_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR3_A = PR0_A;
#[doc = "Field `PR3` reader - Configurable event input pending"]
pub type PR3_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR3_AW = PR0_AW;
#[doc = "Field `PR3` writer - Configurable event input pending"]
pub struct PR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR3_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR4_A = PR0_A;
#[doc = "Field `PR4` reader - Configurable event input pending"]
pub type PR4_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR4_AW = PR0_AW;
#[doc = "Field `PR4` writer - Configurable event input pending"]
pub struct PR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR4_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR5_A = PR0_A;
#[doc = "Field `PR5` reader - Configurable event input pending"]
pub type PR5_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR5_AW = PR0_AW;
#[doc = "Field `PR5` writer - Configurable event input pending"]
pub struct PR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR5_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR6_A = PR0_A;
#[doc = "Field `PR6` reader - Configurable event input pending"]
pub type PR6_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR6_AW = PR0_AW;
#[doc = "Field `PR6` writer - Configurable event input pending"]
pub struct PR6_W<'a> {
    w: &'a mut W,
}
impl<'a> PR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR6_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR7_A = PR0_A;
#[doc = "Field `PR7` reader - Configurable event input pending"]
pub type PR7_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR7_AW = PR0_AW;
#[doc = "Field `PR7` writer - Configurable event input pending"]
pub struct PR7_W<'a> {
    w: &'a mut W,
}
impl<'a> PR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR7_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR8_A = PR0_A;
#[doc = "Field `PR8` reader - Configurable event input pending"]
pub type PR8_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR8_AW = PR0_AW;
#[doc = "Field `PR8` writer - Configurable event input pending"]
pub struct PR8_W<'a> {
    w: &'a mut W,
}
impl<'a> PR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR8_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR9_A = PR0_A;
#[doc = "Field `PR9` reader - Configurable event input pending"]
pub type PR9_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR9_AW = PR0_AW;
#[doc = "Field `PR9` writer - Configurable event input pending"]
pub struct PR9_W<'a> {
    w: &'a mut W,
}
impl<'a> PR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR9_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR10_A = PR0_A;
#[doc = "Field `PR10` reader - Configurable event input pending"]
pub type PR10_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR10_AW = PR0_AW;
#[doc = "Field `PR10` writer - Configurable event input pending"]
pub struct PR10_W<'a> {
    w: &'a mut W,
}
impl<'a> PR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR10_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR11_A = PR0_A;
#[doc = "Field `PR11` reader - Configurable event input pending"]
pub type PR11_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR11_AW = PR0_AW;
#[doc = "Field `PR11` writer - Configurable event input pending"]
pub struct PR11_W<'a> {
    w: &'a mut W,
}
impl<'a> PR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR11_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR12_A = PR0_A;
#[doc = "Field `PR12` reader - Configurable event input pending"]
pub type PR12_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR12_AW = PR0_AW;
#[doc = "Field `PR12` writer - Configurable event input pending"]
pub struct PR12_W<'a> {
    w: &'a mut W,
}
impl<'a> PR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR12_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR13_A = PR0_A;
#[doc = "Field `PR13` reader - Configurable event input pending"]
pub type PR13_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR13_AW = PR0_AW;
#[doc = "Field `PR13` writer - Configurable event input pending"]
pub struct PR13_W<'a> {
    w: &'a mut W,
}
impl<'a> PR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR13_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR14_A = PR0_A;
#[doc = "Field `PR14` reader - Configurable event input pending"]
pub type PR14_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR14_AW = PR0_AW;
#[doc = "Field `PR14` writer - Configurable event input pending"]
pub struct PR14_W<'a> {
    w: &'a mut W,
}
impl<'a> PR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR14_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR15_A = PR0_A;
#[doc = "Field `PR15` reader - Configurable event input pending"]
pub type PR15_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR15_AW = PR0_AW;
#[doc = "Field `PR15` writer - Configurable event input pending"]
pub struct PR15_W<'a> {
    w: &'a mut W,
}
impl<'a> PR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR15_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR16_A = PR0_A;
#[doc = "Field `PR16` reader - Configurable event input pending"]
pub type PR16_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR16_AW = PR0_AW;
#[doc = "Field `PR16` writer - Configurable event input pending"]
pub struct PR16_W<'a> {
    w: &'a mut W,
}
impl<'a> PR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR16_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR16_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR17_A = PR0_A;
#[doc = "Field `PR17` reader - Configurable event input pending"]
pub type PR17_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR17_AW = PR0_AW;
#[doc = "Field `PR17` writer - Configurable event input pending"]
pub struct PR17_W<'a> {
    w: &'a mut W,
}
impl<'a> PR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR17_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR17_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR18_A = PR0_A;
#[doc = "Field `PR18` reader - Configurable event input pending"]
pub type PR18_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR18_AW = PR0_AW;
#[doc = "Field `PR18` writer - Configurable event input pending"]
pub struct PR18_W<'a> {
    w: &'a mut W,
}
impl<'a> PR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR18_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR18_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR19_A = PR0_A;
#[doc = "Field `PR19` reader - Configurable event input pending"]
pub type PR19_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR19_AW = PR0_AW;
#[doc = "Field `PR19` writer - Configurable event input pending"]
pub struct PR19_W<'a> {
    w: &'a mut W,
}
impl<'a> PR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR19_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR19_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR20_A = PR0_A;
#[doc = "Field `PR20` reader - Configurable event input pending"]
pub type PR20_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR20_AW = PR0_AW;
#[doc = "Field `PR20` writer - Configurable event input pending"]
pub struct PR20_W<'a> {
    w: &'a mut W,
}
impl<'a> PR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR20_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR20_AW::CLEAR)
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
#[doc = "Configurable event input pending"]
pub type PR21_A = PR0_A;
#[doc = "Field `PR21` reader - Configurable event input pending"]
pub type PR21_R = PR0_R;
#[doc = "Configurable event input pending"]
pub type PR21_AW = PR0_AW;
#[doc = "Field `PR21` writer - Configurable event input pending"]
pub struct PR21_W<'a> {
    w: &'a mut W,
}
impl<'a> PR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR21_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR21_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W { w: self }
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W {
        PR2_W { w: self }
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W { w: self }
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W { w: self }
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W { w: self }
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W {
        PR6_W { w: self }
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W {
        PR7_W { w: self }
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W {
        PR8_W { w: self }
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W {
        PR9_W { w: self }
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W {
        PR10_W { w: self }
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W {
        PR11_W { w: self }
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W {
        PR12_W { w: self }
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W {
        PR13_W { w: self }
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W {
        PR14_W { w: self }
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W {
        PR15_W { w: self }
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W {
        PR16_W { w: self }
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W {
        PR17_W { w: self }
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W {
        PR18_W { w: self }
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W {
        PR19_W { w: self }
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr20(&mut self) -> PR20_W {
        PR20_W { w: self }
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr21(&mut self) -> PR21_W {
        PR21_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2pr1](index.html) module"]
pub struct C2PR1_SPEC;
impl crate::RegisterSpec for C2PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2pr1::R](R) reader structure"]
impl crate::Readable for C2PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2pr1::W](W) writer structure"]
impl crate::Writable for C2PR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2PR1 to value 0"]
impl crate::Resettable for C2PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
