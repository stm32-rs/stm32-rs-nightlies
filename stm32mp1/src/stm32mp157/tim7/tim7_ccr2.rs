#[doc = "Reader of register TIM7_CCR2"]
pub type R = crate::R<u16, super::TIM7_CCR2>;
#[doc = "Writer for register TIM7_CCR2"]
pub type W = crate::W<u16, super::TIM7_CCR2>;
#[doc = "Register TIM7_CCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM7_CCR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR2`"]
pub type CCR2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR2`"]
pub struct CCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR2"]
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR2"]
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W {
        CCR2_W { w: self }
    }
}
