#[doc = "Reader of register RCC_SPI6CKSELR"]
pub type R = crate::R<u32, super::RCC_SPI6CKSELR>;
#[doc = "Writer for register RCC_SPI6CKSELR"]
pub type W = crate::W<u32, super::RCC_SPI6CKSELR>;
#[doc = "Register RCC_SPI6CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPI6CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI6SRC`"]
pub type SPI6SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI6SRC`"]
pub struct SPI6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI6SRC"]
    #[inline(always)]
    pub fn spi6src(&self) -> SPI6SRC_R {
        SPI6SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI6SRC"]
    #[inline(always)]
    pub fn spi6src(&mut self) -> SPI6SRC_W {
        SPI6SRC_W { w: self }
    }
}
