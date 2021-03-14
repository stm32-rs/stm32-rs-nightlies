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
#[doc = "Reader of field `HSE32EN`"]
pub type HSE32EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSE32EN`"]
pub struct HSE32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE32EN_W<'a> {
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
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    pub fn hse32en(&self) -> HSE32EN_R {
        HSE32EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    pub fn hse32en(&mut self) -> HSE32EN_W {
        HSE32EN_W { w: self }
    }
}
