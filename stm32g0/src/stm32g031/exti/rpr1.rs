#[doc = "Register `RPR1` reader"]
pub struct R(crate::R<RPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR1` writer"]
pub struct W(crate::W<RPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR1_SPEC>;
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
impl From<crate::W<RPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "configurable event inputs x rising edge Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF0_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<RPIF0_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` reader - configurable event inputs x rising edge Pending bit."]
pub struct RPIF0_R(crate::FieldReader<bool, RPIF0_A>);
impl RPIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPIF0_A {
        match self.bits {
            false => RPIF0_A::NOTPENDING,
            true => RPIF0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RPIF0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RPIF0_A::PENDING
    }
}
impl core::ops::Deref for RPIF0_R {
    type Target = crate::FieldReader<bool, RPIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "configurable event inputs x rising edge Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF0_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<RPIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: RPIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF0_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF1_A = RPIF0_A;
#[doc = "Field `RPIF1` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF1_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF1_AW = RPIF0_AW;
#[doc = "Field `RPIF1` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF1_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF2_A = RPIF0_A;
#[doc = "Field `RPIF2` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF2_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF2_AW = RPIF0_AW;
#[doc = "Field `RPIF2` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF2_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF3_A = RPIF0_A;
#[doc = "Field `RPIF3` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF3_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF3_AW = RPIF0_AW;
#[doc = "Field `RPIF3` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF3_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF4_A = RPIF0_A;
#[doc = "Field `RPIF4` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF4_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF4_AW = RPIF0_AW;
#[doc = "Field `RPIF4` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF4_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit"]
pub type RPIF5_A = RPIF0_A;
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge Pending bit"]
pub type RPIF5_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit"]
pub type RPIF5_AW = RPIF0_AW;
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge Pending bit"]
pub struct RPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF5_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF6_A = RPIF0_A;
#[doc = "Field `RPIF6` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF6_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF6_AW = RPIF0_AW;
#[doc = "Field `RPIF6` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF6_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF7_A = RPIF0_A;
#[doc = "Field `RPIF7` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF7_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF7_AW = RPIF0_AW;
#[doc = "Field `RPIF7` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF7_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF8_A = RPIF0_A;
#[doc = "Field `RPIF8` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF8_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF8_AW = RPIF0_AW;
#[doc = "Field `RPIF8` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF8_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF9_A = RPIF0_A;
#[doc = "Field `RPIF9` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF9_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF9_AW = RPIF0_AW;
#[doc = "Field `RPIF9` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF9_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF10_A = RPIF0_A;
#[doc = "Field `RPIF10` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF10_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF10_AW = RPIF0_AW;
#[doc = "Field `RPIF10` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF10_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF11_A = RPIF0_A;
#[doc = "Field `RPIF11` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF11_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF11_AW = RPIF0_AW;
#[doc = "Field `RPIF11` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF11_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF12_A = RPIF0_A;
#[doc = "Field `RPIF12` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF12_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF12_AW = RPIF0_AW;
#[doc = "Field `RPIF12` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF12_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF13_A = RPIF0_A;
#[doc = "Field `RPIF13` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF13_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF13_AW = RPIF0_AW;
#[doc = "Field `RPIF13` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF13_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF14_A = RPIF0_A;
#[doc = "Field `RPIF14` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF14_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF14_AW = RPIF0_AW;
#[doc = "Field `RPIF14` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF14_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF15_A = RPIF0_A;
#[doc = "Field `RPIF15` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF15_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF15_AW = RPIF0_AW;
#[doc = "Field `RPIF15` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF15_AW::CLEAR)
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
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF16_A = RPIF0_A;
#[doc = "Field `RPIF16` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF16_R = RPIF0_R;
#[doc = "configurable event inputs x rising edge Pending bit."]
pub type RPIF16_AW = RPIF0_AW;
#[doc = "Field `RPIF16` writer - configurable event inputs x rising edge Pending bit."]
pub struct RPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIF16_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF16_AW::CLEAR)
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
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W {
        RPIF0_W { w: self }
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W {
        RPIF1_W { w: self }
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W {
        RPIF2_W { w: self }
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W {
        RPIF3_W { w: self }
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W {
        RPIF4_W { w: self }
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W {
        RPIF5_W { w: self }
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W {
        RPIF6_W { w: self }
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W {
        RPIF7_W { w: self }
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W {
        RPIF8_W { w: self }
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W {
        RPIF9_W { w: self }
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W {
        RPIF10_W { w: self }
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W {
        RPIF11_W { w: self }
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W {
        RPIF12_W { w: self }
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W {
        RPIF13_W { w: self }
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W {
        RPIF14_W { w: self }
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W {
        RPIF15_W { w: self }
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif16(&mut self) -> RPIF16_W {
        RPIF16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr1](index.html) module"]
pub struct RPR1_SPEC;
impl crate::RegisterSpec for RPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr1::R](R) reader structure"]
impl crate::Readable for RPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr1::W](W) writer structure"]
impl crate::Writable for RPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
