#[doc = "Writer for register KEYR"]
pub type W = crate::W<u32, super::KEYR>;
#[doc = "Register KEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEYR`"]
pub struct KEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - KEYR"]
    #[inline(always)]
    pub fn keyr(&mut self) -> KEYR_W {
        KEYR_W { w: self }
    }
}
