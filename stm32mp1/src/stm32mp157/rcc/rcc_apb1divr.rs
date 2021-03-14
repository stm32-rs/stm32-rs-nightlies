#[doc = "Reader of register RCC_APB1DIVR"]
pub type R = crate::R<u32, super::RCC_APB1DIVR>;
#[doc = "Writer for register RCC_APB1DIVR"]
pub type W = crate::W<u32, super::RCC_APB1DIVR>;
#[doc = "Register RCC_APB1DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB1DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `APB1DIV`"]
pub type APB1DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB1DIV`"]
pub struct APB1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `APB1DIVRDY`"]
pub type APB1DIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB1DIVRDY"]
    #[inline(always)]
    pub fn apb1divrdy(&self) -> APB1DIVRDY_R {
        APB1DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    pub fn apb1div(&mut self) -> APB1DIV_W {
        APB1DIV_W { w: self }
    }
}
