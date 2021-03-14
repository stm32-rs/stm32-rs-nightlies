#[doc = "Reader of register RCC_APB3DIVR"]
pub type R = crate::R<u32, super::RCC_APB3DIVR>;
#[doc = "Writer for register RCC_APB3DIVR"]
pub type W = crate::W<u32, super::RCC_APB3DIVR>;
#[doc = "Register RCC_APB3DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB3DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `APB3DIV`"]
pub type APB3DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APB3DIV`"]
pub struct APB3DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB3DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `APB3DIVRDY`"]
pub type APB3DIVRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&self) -> APB3DIV_R {
        APB3DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB3DIVRDY"]
    #[inline(always)]
    pub fn apb3divrdy(&self) -> APB3DIVRDY_R {
        APB3DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&mut self) -> APB3DIV_W {
        APB3DIV_W { w: self }
    }
}
