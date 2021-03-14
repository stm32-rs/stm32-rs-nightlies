#[doc = "Reader of register RCC_APB2DIVR"]
pub type R = crate::R<u32, super::RCC_APB2DIVR>;
#[doc = "Writer for register RCC_APB2DIVR"]
pub type W = crate::W<u32, super::RCC_APB2DIVR>;
#[doc = "Register RCC_APB2DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB2DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `APB2DIV`"]
pub type APB2DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB2DIV`"]
pub struct APB2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `APB2DIVRDY`"]
pub type APB2DIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB2DIVRDY"]
    #[inline(always)]
    pub fn apb2divrdy(&self) -> APB2DIVRDY_R {
        APB2DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&mut self) -> APB2DIV_W {
        APB2DIV_W { w: self }
    }
}
