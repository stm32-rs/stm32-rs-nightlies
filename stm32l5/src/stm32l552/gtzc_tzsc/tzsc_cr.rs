#[doc = "Reader of register TZSC_CR"]
pub type R = crate::R<u32, super::TZSC_CR>;
#[doc = "Writer for register TZSC_CR"]
pub type W = crate::W<u32, super::TZSC_CR>;
#[doc = "Register TZSC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::TZSC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCK`"]
pub type LCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCK`"]
pub struct LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&mut self) -> LCK_W {
        LCK_W { w: self }
    }
}
