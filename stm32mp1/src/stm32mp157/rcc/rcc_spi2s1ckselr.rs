#[doc = "Reader of register RCC_SPI2S1CKSELR"]
pub type R = crate::R<u32, super::RCC_SPI2S1CKSELR>;
#[doc = "Writer for register RCC_SPI2S1CKSELR"]
pub type W = crate::W<u32, super::RCC_SPI2S1CKSELR>;
#[doc = "Register RCC_SPI2S1CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPI2S1CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI1SRC`"]
pub type SPI1SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1SRC`"]
pub struct SPI1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI1SRC"]
    #[inline(always)]
    pub fn spi1src(&self) -> SPI1SRC_R {
        SPI1SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI1SRC"]
    #[inline(always)]
    pub fn spi1src(&mut self) -> SPI1SRC_W {
        SPI1SRC_W { w: self }
    }
}
