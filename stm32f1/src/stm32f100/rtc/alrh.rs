#[doc = "Writer for register ALRH"]
pub type W = crate::W<u32, super::ALRH>;
#[doc = "Register ALRH `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ALRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Write proxy for field `ALRH`"]
pub struct ALRH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    pub fn alrh(&mut self) -> ALRH_W {
        ALRH_W { w: self }
    }
}
