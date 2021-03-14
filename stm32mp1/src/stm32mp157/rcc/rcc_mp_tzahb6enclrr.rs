#[doc = "Reader of register RCC_MP_TZAHB6ENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_TZAHB6ENCLRR>;
#[doc = "Writer for register RCC_MP_TZAHB6ENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_TZAHB6ENCLRR>;
#[doc = "Register RCC_MP_TZAHB6ENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_TZAHB6ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDMAEN`"]
pub type MDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMAEN`"]
pub struct MDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAEN_W<'a> {
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
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W {
        MDMAEN_W { w: self }
    }
}
