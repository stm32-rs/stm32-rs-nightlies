#[doc = "Reader of register PERBR"]
pub type R = crate::R<u32, super::PERBR>;
#[doc = "Writer for register PERBR"]
pub type W = crate::W<u32, super::PERBR>;
#[doc = "Register PERBR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::PERBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `PERx`"]
pub type PERX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PERx`"]
pub struct PERX_W<'a> {
    w: &'a mut W,
}
impl<'a> PERX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&mut self) -> PERX_W {
        PERX_W { w: self }
    }
}
