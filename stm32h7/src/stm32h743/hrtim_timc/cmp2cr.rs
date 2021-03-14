#[doc = "Reader of register CMP2CR"]
pub type R = crate::R<u32, super::CMP2CR>;
#[doc = "Writer for register CMP2CR"]
pub type W = crate::W<u32, super::CMP2CR>;
#[doc = "Register CMP2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP2x`"]
pub type CMP2X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMP2x`"]
pub struct CMP2X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&self) -> CMP2X_R {
        CMP2X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 2 value"]
    #[inline(always)]
    pub fn cmp2x(&mut self) -> CMP2X_W {
        CMP2X_W { w: self }
    }
}
