#[doc = "Reader of register RCC_UART35CKSELR"]
pub type R = crate::R<u32, super::RCC_UART35CKSELR>;
#[doc = "Writer for register RCC_UART35CKSELR"]
pub type W = crate::W<u32, super::RCC_UART35CKSELR>;
#[doc = "Register RCC_UART35CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_UART35CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART35SRC`"]
pub type UART35SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART35SRC`"]
pub struct UART35SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART35SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART35SRC"]
    #[inline(always)]
    pub fn uart35src(&self) -> UART35SRC_R {
        UART35SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART35SRC"]
    #[inline(always)]
    pub fn uart35src(&mut self) -> UART35SRC_W {
        UART35SRC_W { w: self }
    }
}
