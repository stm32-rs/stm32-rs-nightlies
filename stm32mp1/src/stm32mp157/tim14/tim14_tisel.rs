#[doc = "Reader of register TIM14_TISEL"]
pub type R = crate::R<u16, super::TIM14_TISEL>;
#[doc = "Writer for register TIM14_TISEL"]
pub type W = crate::W<u16, super::TIM14_TISEL>;
#[doc = "Register TIM14_TISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM14_TISEL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI1SEL`"]
pub type TI1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI1SEL`"]
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
}
