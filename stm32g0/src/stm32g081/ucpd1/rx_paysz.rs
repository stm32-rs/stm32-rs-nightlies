#[doc = "Reader of register RX_PAYSZ"]
pub type R = crate::R<u32, super::RX_PAYSZ>;
#[doc = "Writer for register RX_PAYSZ"]
pub type W = crate::W<u32, super::RX_PAYSZ>;
#[doc = "Register RX_PAYSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_PAYSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPAYSZ`"]
pub type RXPAYSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXPAYSZ`"]
pub struct RXPAYSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAYSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - RXPAYSZ"]
    #[inline(always)]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W {
        RXPAYSZ_W { w: self }
    }
}
