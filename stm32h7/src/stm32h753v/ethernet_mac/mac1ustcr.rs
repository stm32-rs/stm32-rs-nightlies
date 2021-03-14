#[doc = "Reader of register MAC1USTCR"]
pub type R = crate::R<u32, super::MAC1USTCR>;
#[doc = "Writer for register MAC1USTCR"]
pub type W = crate::W<u32, super::MAC1USTCR>;
#[doc = "Register MAC1USTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC1USTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIC_1US_CNTR`"]
pub type TIC_1US_CNTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIC_1US_CNTR`"]
pub struct TIC_1US_CNTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIC_1US_CNTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - 1 µs tick Counter"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 1 µs tick Counter"]
    #[inline(always)]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W {
        TIC_1US_CNTR_W { w: self }
    }
}
