#[doc = "Reader of register RCC_TZAHB6RSTCLRR"]
pub type R = crate::R<u32, super::RCC_TZAHB6RSTCLRR>;
#[doc = "Writer for register RCC_TZAHB6RSTCLRR"]
pub type W = crate::W<u32, super::RCC_TZAHB6RSTCLRR>;
#[doc = "Register RCC_TZAHB6RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_TZAHB6RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDMARST`"]
pub type MDMARST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMARST`"]
pub struct MDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMARST_W<'a> {
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
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W {
        MDMARST_W { w: self }
    }
}
