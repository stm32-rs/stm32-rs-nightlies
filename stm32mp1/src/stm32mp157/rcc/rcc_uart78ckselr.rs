#[doc = "Reader of register RCC_UART78CKSELR"]
pub type R = crate::R<u32, super::RCC_UART78CKSELR>;
#[doc = "Writer for register RCC_UART78CKSELR"]
pub type W = crate::W<u32, super::RCC_UART78CKSELR>;
#[doc = "Register RCC_UART78CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_UART78CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART78SRC`"]
pub type UART78SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART78SRC`"]
pub struct UART78SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART78SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART78SRC"]
    #[inline(always)]
    pub fn uart78src(&self) -> UART78SRC_R {
        UART78SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART78SRC"]
    #[inline(always)]
    pub fn uart78src(&mut self) -> UART78SRC_W {
        UART78SRC_W { w: self }
    }
}
