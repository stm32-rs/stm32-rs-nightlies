#[doc = "Reader of register ETH_MACSPI1R"]
pub type R = crate::R<u32, super::ETH_MACSPI1R>;
#[doc = "Writer for register ETH_MACSPI1R"]
pub type W = crate::W<u32, super::ETH_MACSPI1R>;
#[doc = "Register ETH_MACSPI1R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACSPI1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI1`"]
pub type SPI1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI1`"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
}
