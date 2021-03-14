#[doc = "Reader of register RCC_UART6CKSELR"]
pub type R = crate::R<u32, super::RCC_UART6CKSELR>;
#[doc = "Writer for register RCC_UART6CKSELR"]
pub type W = crate::W<u32, super::RCC_UART6CKSELR>;
#[doc = "Register RCC_UART6CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_UART6CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART6SRC`"]
pub type UART6SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART6SRC`"]
pub struct UART6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART6SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART6SRC"]
    #[inline(always)]
    pub fn uart6src(&self) -> UART6SRC_R {
        UART6SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART6SRC"]
    #[inline(always)]
    pub fn uart6src(&mut self) -> UART6SRC_W {
        UART6SRC_W { w: self }
    }
}
