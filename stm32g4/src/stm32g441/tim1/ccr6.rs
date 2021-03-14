#[doc = "Reader of register CCR6"]
pub type R = crate::R<u32, super::CCR6>;
#[doc = "Writer for register CCR6"]
pub type W = crate::W<u32, super::CCR6>;
#[doc = "Register CCR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR6`"]
pub type CCR6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR6`"]
pub struct CCR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR6_W<'a> {
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
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W {
        CCR6_W { w: self }
    }
}
