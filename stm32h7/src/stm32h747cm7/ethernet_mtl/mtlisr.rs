#[doc = "Reader of register MTLISR"]
pub type R = crate::R<u32, super::MTLISR>;
#[doc = "Writer for register MTLISR"]
pub type W = crate::W<u32, super::MTLISR>;
#[doc = "Register MTLISR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTLISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Q0IS`"]
pub type Q0IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q0IS`"]
pub struct Q0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0IS_W<'a> {
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
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&mut self) -> Q0IS_W {
        Q0IS_W { w: self }
    }
}
