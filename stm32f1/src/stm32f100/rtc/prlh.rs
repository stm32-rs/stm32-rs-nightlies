#[doc = "Writer for register PRLH"]
pub type W = crate::W<u32, super::PRLH>;
#[doc = "Register PRLH `reset()`'s with value 0"]
impl crate::ResetValue for super::PRLH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PRLH`"]
pub struct PRLH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRLH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub fn prlh(&mut self) -> PRLH_W {
        PRLH_W { w: self }
    }
}
