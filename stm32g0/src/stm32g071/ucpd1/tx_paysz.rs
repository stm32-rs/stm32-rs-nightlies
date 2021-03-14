#[doc = "Reader of register TX_PAYSZ"]
pub type R = crate::R<u32, super::TX_PAYSZ>;
#[doc = "Writer for register TX_PAYSZ"]
pub type W = crate::W<u32, super::TX_PAYSZ>;
#[doc = "Register TX_PAYSZ `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_PAYSZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPAYSZ`"]
pub type TXPAYSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXPAYSZ`"]
pub struct TXPAYSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAYSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXPAYSZ"]
    #[inline(always)]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W {
        TXPAYSZ_W { w: self }
    }
}
