#[doc = "Reader of register COUNT5_TX"]
pub type R = crate::R<u16, super::COUNT5_TX>;
#[doc = "Writer for register COUNT5_TX"]
pub type W = crate::W<u16, super::COUNT5_TX>;
#[doc = "Register COUNT5_TX `reset()`'s with value 0"]
impl crate::ResetValue for super::COUNT5_TX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT5_TX`"]
pub type COUNT5_TX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT5_TX`"]
pub struct COUNT5_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT5_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count5_tx(&self) -> COUNT5_TX_R {
        COUNT5_TX_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Transmission byte count"]
    #[inline(always)]
    pub fn count5_tx(&mut self) -> COUNT5_TX_W {
        COUNT5_TX_W { w: self }
    }
}
