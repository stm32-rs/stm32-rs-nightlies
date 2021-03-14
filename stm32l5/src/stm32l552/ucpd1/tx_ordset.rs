#[doc = "Reader of register TX_ORDSET"]
pub type R = crate::R<u32, super::TX_ORDSET>;
#[doc = "Writer for register TX_ORDSET"]
pub type W = crate::W<u32, super::TX_ORDSET>;
#[doc = "Register TX_ORDSET `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_ORDSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXORDSET`"]
pub type TXORDSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXORDSET`"]
pub struct TXORDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXORDSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&mut self) -> TXORDSET_W {
        TXORDSET_W { w: self }
    }
}
