#[doc = "Reader of register TIMx_RCR"]
pub type R = crate::R<u16, super::TIMX_RCR>;
#[doc = "Writer for register TIMx_RCR"]
pub type W = crate::W<u16, super::TIMX_RCR>;
#[doc = "Register TIMx_RCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMX_RCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REP`"]
pub struct REP_W<'a> {
    w: &'a mut W,
}
impl<'a> REP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REP"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W {
        REP_W { w: self }
    }
}
