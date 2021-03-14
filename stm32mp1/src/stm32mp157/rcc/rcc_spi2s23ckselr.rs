#[doc = "Reader of register RCC_SPI2S23CKSELR"]
pub type R = crate::R<u32, super::RCC_SPI2S23CKSELR>;
#[doc = "Writer for register RCC_SPI2S23CKSELR"]
pub type W = crate::W<u32, super::RCC_SPI2S23CKSELR>;
#[doc = "Register RCC_SPI2S23CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPI2S23CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI23SRC`"]
pub type SPI23SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI23SRC`"]
pub struct SPI23SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI23SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    pub fn spi23src(&self) -> SPI23SRC_R {
        SPI23SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI23SRC"]
    #[inline(always)]
    pub fn spi23src(&mut self) -> SPI23SRC_W {
        SPI23SRC_W { w: self }
    }
}
