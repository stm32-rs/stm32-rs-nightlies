#[doc = "Reader of register TIM4_CCR3"]
pub type R = crate::R<u16, super::TIM4_CCR3>;
#[doc = "Writer for register TIM4_CCR3"]
pub type W = crate::W<u16, super::TIM4_CCR3>;
#[doc = "Register TIM4_CCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM4_CCR3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCR3`"]
pub type CCR3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CCR3`"]
pub struct CCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W {
        CCR3_W { w: self }
    }
}
