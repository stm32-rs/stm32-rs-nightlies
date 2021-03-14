#[doc = "Reader of register LTDC_LIPCR"]
pub type R = crate::R<u32, super::LTDC_LIPCR>;
#[doc = "Writer for register LTDC_LIPCR"]
pub type W = crate::W<u32, super::LTDC_LIPCR>;
#[doc = "Register LTDC_LIPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_LIPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIPOS`"]
pub type LIPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LIPOS`"]
pub struct LIPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LIPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - LIPOS"]
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LIPOS"]
    #[inline(always)]
    pub fn lipos(&mut self) -> LIPOS_W {
        LIPOS_W { w: self }
    }
}
