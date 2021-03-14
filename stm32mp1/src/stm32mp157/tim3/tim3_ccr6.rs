#[doc = "Reader of register TIM3_CCR6"]
pub type R = crate::R<u16, super::TIM3_CCR6>;
#[doc = "Writer for register TIM3_CCR6"]
pub type W = crate::W<u16, super::TIM3_CCR6>;
#[doc = "Register TIM3_CCR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM3_CCR6 {
    type Type = u16;
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
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W {
        CCR6_W { w: self }
    }
}
