#[doc = "Reader of register RXCRC"]
pub type R = crate::R<u32, super::RXCRC>;
#[doc = "Writer for register RXCRC"]
pub type W = crate::W<u32, super::RXCRC>;
#[doc = "Register RXCRC `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXCRC`"]
pub type RXCRC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXCRC`"]
pub struct RXCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&mut self) -> RXCRC_W {
        RXCRC_W { w: self }
    }
}
