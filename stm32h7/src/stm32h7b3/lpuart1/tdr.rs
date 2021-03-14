#[doc = "Reader of register TDR"]
pub type R = crate::R<u32, super::TDR>;
#[doc = "Writer for register TDR"]
pub type W = crate::W<u32, super::TDR>;
#[doc = "Register TDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDR`"]
pub type TDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TDR`"]
pub struct TDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transmit data value"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W {
        TDR_W { w: self }
    }
}
