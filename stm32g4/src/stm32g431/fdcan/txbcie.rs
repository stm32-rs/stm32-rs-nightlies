#[doc = "Reader of register TXBCIE"]
pub type R = crate::R<u32, super::TXBCIE>;
#[doc = "Writer for register TXBCIE"]
pub type W = crate::W<u32, super::TXBCIE>;
#[doc = "Register TXBCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFIE`"]
pub type CFIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFIE`"]
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CFIE"]
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
}
