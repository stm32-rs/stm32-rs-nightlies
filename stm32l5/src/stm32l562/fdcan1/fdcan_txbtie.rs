#[doc = "Reader of register FDCAN_TXBTIE"]
pub type R = crate::R<u32, super::FDCAN_TXBTIE>;
#[doc = "Writer for register FDCAN_TXBTIE"]
pub type W = crate::W<u32, super::FDCAN_TXBTIE>;
#[doc = "Register FDCAN_TXBTIE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBTIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmission Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
}
