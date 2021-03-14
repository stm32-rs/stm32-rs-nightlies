#[doc = "Reader of register MACSPI2R"]
pub type R = crate::R<u32, super::MACSPI2R>;
#[doc = "Writer for register MACSPI2R"]
pub type W = crate::W<u32, super::MACSPI2R>;
#[doc = "Register MACSPI2R `reset()`'s with value 0"]
impl crate::ResetValue for super::MACSPI2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI2`"]
pub type SPI2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI2`"]
pub struct SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W {
        SPI2_W { w: self }
    }
}
