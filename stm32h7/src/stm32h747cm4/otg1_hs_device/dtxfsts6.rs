#[doc = "Reader of register DTXFSTS6"]
pub type R = crate::R<u32, super::DTXFSTS6>;
#[doc = "Writer for register DTXFSTS6"]
pub type W = crate::W<u32, super::DTXFSTS6>;
#[doc = "Register DTXFSTS6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DTXFSTS6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INEPTFSAV`"]
pub type INEPTFSAV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPTFSAV`"]
pub struct INEPTFSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTFSAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&mut self) -> INEPTFSAV_W {
        INEPTFSAV_W { w: self }
    }
}
