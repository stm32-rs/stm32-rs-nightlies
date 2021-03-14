#[doc = "Reader of register WCCR"]
pub type R = crate::R<u32, super::WCCR>;
#[doc = "Writer for register WCCR"]
pub type W = crate::W<u32, super::WCCR>;
#[doc = "Register WCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFRESH`"]
pub type REFRESH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REFRESH`"]
pub struct REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REFRESH"]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W {
        REFRESH_W { w: self }
    }
}
