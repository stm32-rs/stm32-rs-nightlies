#[doc = "Writer for register NSKEYR"]
pub type W = crate::W<u32, super::NSKEYR>;
#[doc = "Register NSKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::NSKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NSKEYR`"]
pub struct NSKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> NSKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - NSKEYR"]
    #[inline(always)]
    pub fn nskeyr(&mut self) -> NSKEYR_W {
        NSKEYR_W { w: self }
    }
}
