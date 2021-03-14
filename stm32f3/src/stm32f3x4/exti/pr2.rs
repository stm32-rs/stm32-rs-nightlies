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
#[doc = "Pending bit on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR32_A {
    #[doc = "0: No trigger request occurred"]
    NOTPENDING = 0,
    #[doc = "1: Selected trigger request occurred"]
    PENDING = 1,
}
impl From<PR32_A> for bool {
    #[inline(always)]
    fn from(variant: PR32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PR32`"]
pub type PR32_R = crate::R<bool, PR32_A>;
impl PR32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR32_A {
        match self.bits {
            false => PR32_A::NOTPENDING,
            true => PR32_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR32_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR32_A::PENDING
    }
}
#[doc = "Pending bit on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR32_AW {
    #[doc = "1: Clears pending bit"]
    CLEAR = 1,
}
impl From<PR32_AW> for bool {
    #[inline(always)]
    fn from(variant: PR32_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PR32`"]
pub struct PR32_W<'a> {
    w: &'a mut W,
}
impl<'a> PR32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR32_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR32_AW::CLEAR)
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
#[doc = "Pending bit on line 33"]
pub type PR33_A = PR32_A;
#[doc = "Reader of field `PR33`"]
pub type PR33_R = crate::R<bool, PR32_A>;
#[doc = "Pending bit on line 33"]
pub type PR33_AW = PR32_AW;
#[doc = "Write proxy for field `PR33`"]
pub struct PR33_W<'a> {
    w: &'a mut W,
}
impl<'a> PR33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PR33_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR32_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&self) -> PR32_R {
        PR32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&self) -> PR33_R {
        PR33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit on line 32"]
    #[inline(always)]
    pub fn pr32(&mut self) -> PR32_W {
        PR32_W { w: self }
    }
    #[doc = "Bit 1 - Pending bit on line 33"]
    #[inline(always)]
    pub fn pr33(&mut self) -> PR33_W {
        PR33_W { w: self }
    }
}
