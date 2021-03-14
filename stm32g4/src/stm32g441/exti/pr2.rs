#[doc = "Reader of register PR2"]
pub type R = crate::R<u32, super::PR2>;
#[doc = "Writer for register PR2"]
pub type W = crate::W<u32, super::PR2>;
#[doc = "Register PR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pending bit 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF32_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PIF32_A> for bool {
    #[inline(always)]
    fn from(variant: PIF32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIF32`"]
pub type PIF32_R = crate::R<bool, PIF32_A>;
impl PIF32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF32_A {
        match self.bits {
            false => PIF32_A::NOTPENDING,
            true => PIF32_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF32_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF32_A::PENDING
    }
}
#[doc = "Pending bit 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF32_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PIF32_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF32_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PIF32`"]
pub struct PIF32_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF32_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF32_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Pending bit 33"]
pub type PIF33_A = PIF32_A;
#[doc = "Reader of field `PIF33`"]
pub type PIF33_R = crate::R<bool, PIF32_A>;
#[doc = "Pending bit 33"]
pub type PIF33_AW = PIF32_AW;
#[doc = "Write proxy for field `PIF33`"]
pub struct PIF33_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF33_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF32_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Pending bit 40"]
pub type PIF40_A = PIF32_A;
#[doc = "Reader of field `PIF40`"]
pub type PIF40_R = crate::R<bool, PIF32_A>;
#[doc = "Pending bit 40"]
pub type PIF40_AW = PIF32_AW;
#[doc = "Write proxy for field `PIF40`"]
pub struct PIF40_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF40_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF32_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Pending bit 41"]
pub type PIF41_A = PIF32_A;
#[doc = "Reader of field `PIF41`"]
pub type PIF41_R = crate::R<bool, PIF32_A>;
#[doc = "Pending bit 41"]
pub type PIF41_AW = PIF32_AW;
#[doc = "Write proxy for field `PIF41`"]
pub struct PIF41_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIF41_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF32_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pending bit 32"]
    #[inline(always)]
    pub fn pif32(&self) -> PIF32_R {
        PIF32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending bit 33"]
    #[inline(always)]
    pub fn pif33(&self) -> PIF33_R {
        PIF33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pending bit 40"]
    #[inline(always)]
    pub fn pif40(&self) -> PIF40_R {
        PIF40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pending bit 41"]
    #[inline(always)]
    pub fn pif41(&self) -> PIF41_R {
        PIF41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 32"]
    #[inline(always)]
    pub fn pif32(&mut self) -> PIF32_W {
        PIF32_W { w: self }
    }
    #[doc = "Bit 1 - Pending bit 33"]
    #[inline(always)]
    pub fn pif33(&mut self) -> PIF33_W {
        PIF33_W { w: self }
    }
    #[doc = "Bit 8 - Pending bit 40"]
    #[inline(always)]
    pub fn pif40(&mut self) -> PIF40_W {
        PIF40_W { w: self }
    }
    #[doc = "Bit 9 - Pending bit 41"]
    #[inline(always)]
    pub fn pif41(&mut self) -> PIF41_W {
        PIF41_W { w: self }
    }
}
