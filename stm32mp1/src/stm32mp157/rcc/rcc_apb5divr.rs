#[doc = "Reader of register RCC_APB5DIVR"]
pub type R = crate::R<u32, super::RCC_APB5DIVR>;
#[doc = "Writer for register RCC_APB5DIVR"]
pub type W = crate::W<u32, super::RCC_APB5DIVR>;
#[doc = "Register RCC_APB5DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB5DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `APB5DIV`"]
pub type APB5DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB5DIV`"]
pub struct APB5DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB5DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `APB5DIVRDY`"]
pub type APB5DIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    pub fn apb5div(&self) -> APB5DIV_R {
        APB5DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB5DIVRDY"]
    #[inline(always)]
    pub fn apb5divrdy(&self) -> APB5DIVRDY_R {
        APB5DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    pub fn apb5div(&mut self) -> APB5DIV_W {
        APB5DIV_W { w: self }
    }
}
