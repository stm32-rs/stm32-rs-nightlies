#[doc = "Reader of register SWIER2"]
pub type R = crate::R<u32, super::SWIER2>;
#[doc = "Writer for register SWIER2"]
pub type W = crate::W<u32, super::SWIER2>;
#[doc = "Register SWIER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software interrupt on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER32_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIER32_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWIER32`"]
pub type SWIER32_R = crate::R<bool, SWIER32_A>;
impl SWIER32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWIER32_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SWIER32_A::PEND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER32_A::PEND
    }
}
#[doc = "Write proxy for field `SWIER32`"]
pub struct SWIER32_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER32_A::PEND)
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
#[doc = "Software interrupt on line 33"]
pub type SWIER33_A = SWIER32_A;
#[doc = "Reader of field `SWIER33`"]
pub type SWIER33_R = crate::R<bool, SWIER32_A>;
#[doc = "Write proxy for field `SWIER33`"]
pub struct SWIER33_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER33_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER32_A::PEND)
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
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&self) -> SWIER32_R {
        SWIER32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&self) -> SWIER33_R {
        SWIER33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&mut self) -> SWIER32_W {
        SWIER32_W { w: self }
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&mut self) -> SWIER33_W {
        SWIER33_W { w: self }
    }
}
