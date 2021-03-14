#[doc = "Reader of register ETH_MACSPI0R"]
pub type R = crate::R<u32, super::ETH_MACSPI0R>;
#[doc = "Writer for register ETH_MACSPI0R"]
pub type W = crate::W<u32, super::ETH_MACSPI0R>;
#[doc = "Register ETH_MACSPI0R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACSPI0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI0`"]
pub type SPI0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI0`"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
}
