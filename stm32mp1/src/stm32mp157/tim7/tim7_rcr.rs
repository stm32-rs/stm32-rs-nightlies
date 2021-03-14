#[doc = "Reader of register TIM7_RCR"]
pub type R = crate::R<u16, super::TIM7_RCR>;
#[doc = "Writer for register TIM7_RCR"]
pub type W = crate::W<u16, super::TIM7_RCR>;
#[doc = "Register TIM7_RCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM7_RCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REP`"]
pub struct REP_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W { w: self }
    }
}
