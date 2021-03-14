#[doc = "Reader of register OR1"]
pub type R = crate::R<u32, super::OR1>;
#[doc = "Writer for register OR1"]
pub type W = crate::W<u32, super::OR1>;
#[doc = "Register OR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCREF_CLR`"]
pub type OCREF_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCREF_CLR`"]
pub struct OCREF_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCREF_CLR_W<'a> {
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
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W {
        OCREF_CLR_W { w: self }
    }
}
