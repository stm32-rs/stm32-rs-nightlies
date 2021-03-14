#[doc = "Reader of register RCC_APB4DIVR"]
pub type R = crate::R<u32, super::RCC_APB4DIVR>;
#[doc = "Writer for register RCC_APB4DIVR"]
pub type W = crate::W<u32, super::RCC_APB4DIVR>;
#[doc = "Register RCC_APB4DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB4DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `APB4DIV`"]
pub type APB4DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB4DIV`"]
pub struct APB4DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB4DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `APB4DIVRDY`"]
pub type APB4DIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - APB4DIV"]
    #[inline(always)]
    pub fn apb4div(&self) -> APB4DIV_R {
        APB4DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB4DIVRDY"]
    #[inline(always)]
    pub fn apb4divrdy(&self) -> APB4DIVRDY_R {
        APB4DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB4DIV"]
    #[inline(always)]
    pub fn apb4div(&mut self) -> APB4DIV_W {
        APB4DIV_W { w: self }
    }
}
