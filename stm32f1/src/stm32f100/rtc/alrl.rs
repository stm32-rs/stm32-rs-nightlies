#[doc = "Writer for register ALRL"]
pub type W = crate::W<u32, super::ALRL>;
#[doc = "Register ALRL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ALRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Write proxy for field `ALRL`"]
pub struct ALRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    pub fn alrl(&mut self) -> ALRL_W {
        ALRL_W { w: self }
    }
}
