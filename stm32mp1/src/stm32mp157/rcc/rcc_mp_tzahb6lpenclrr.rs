#[doc = "Reader of register RCC_MP_TZAHB6LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_TZAHB6LPENCLRR>;
#[doc = "Writer for register RCC_MP_TZAHB6LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_TZAHB6LPENCLRR>;
#[doc = "Register RCC_MP_TZAHB6LPENCLRR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_MP_TZAHB6LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `MDMALPEN`"]
pub type MDMALPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMALPEN`"]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
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
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
}
