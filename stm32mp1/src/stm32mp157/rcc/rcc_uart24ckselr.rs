#[doc = "Reader of register RCC_UART24CKSELR"]
pub type R = crate::R<u32, super::RCC_UART24CKSELR>;
#[doc = "Writer for register RCC_UART24CKSELR"]
pub type W = crate::W<u32, super::RCC_UART24CKSELR>;
#[doc = "Register RCC_UART24CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_UART24CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART24SRC`"]
pub type UART24SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART24SRC`"]
pub struct UART24SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART24SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART24SRC"]
    #[inline(always)]
    pub fn uart24src(&self) -> UART24SRC_R {
        UART24SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART24SRC"]
    #[inline(always)]
    pub fn uart24src(&mut self) -> UART24SRC_W {
        UART24SRC_W { w: self }
    }
}
