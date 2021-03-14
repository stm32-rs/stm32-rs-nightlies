#[doc = "Writer for register TXDR"]
pub type W = crate::W<u32, super::TXDR>;
#[doc = "Register TXDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXDR`"]
pub struct TXDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn txdr(&mut self) -> TXDR_W {
        TXDR_W { w: self }
    }
}
