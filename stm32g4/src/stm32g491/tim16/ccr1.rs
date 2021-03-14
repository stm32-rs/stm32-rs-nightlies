#[doc = "Reader of register CCR1"]
pub type R = crate::R<u32, super::CCR1>;
#[doc = "Writer for register CCR1"]
pub type W = crate::W<u32, super::CCR1>;
#[doc = "Register CCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR1`"]
pub type CCR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR1`"]
pub struct CCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W {
        CCR1_W { w: self }
    }
}
