#[doc = "Reader of register TIM3_ARR"]
pub type R = crate::R<u16, super::TIM3_ARR>;
#[doc = "Writer for register TIM3_ARR"]
pub type W = crate::W<u16, super::TIM3_ARR>;
#[doc = "Register TIM3_ARR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TIM3_ARR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `ARR`"]
pub type ARR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR`"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
}
