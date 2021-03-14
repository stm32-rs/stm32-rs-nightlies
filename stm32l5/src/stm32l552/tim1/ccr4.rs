#[doc = "Reader of register CCR4"]
pub type R = crate::R<u32, super::CCR4>;
#[doc = "Writer for register CCR4"]
pub type W = crate::W<u32, super::CCR4>;
#[doc = "Register CCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR4`"]
pub type CCR4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR4`"]
pub struct CCR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W {
        CCR4_W { w: self }
    }
}
