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
#[doc = "Software interrupt on line x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER49_A {
    #[doc = "1: Generates an interrupt request"]
    PEND = 1,
}
impl From<SWIER49_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWIER49`"]
pub type SWIER49_R = crate::R<bool, SWIER49_A>;
impl SWIER49_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWIER49_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SWIER49_A::PEND),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PEND`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER49_A::PEND
    }
}
#[doc = "Write proxy for field `SWIER49`"]
pub struct SWIER49_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER49_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER49_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Software interrupt on line x+32"]
pub type SWIER51_A = SWIER49_A;
#[doc = "Reader of field `SWIER51`"]
pub type SWIER51_R = crate::R<bool, SWIER49_A>;
#[doc = "Write proxy for field `SWIER51`"]
pub struct SWIER51_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIER51_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER49_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&self) -> SWIER49_R {
        SWIER49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&self) -> SWIER51_R {
        SWIER51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&mut self) -> SWIER49_W {
        SWIER49_W { w: self }
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&mut self) -> SWIER51_W {
        SWIER51_W { w: self }
    }
}
