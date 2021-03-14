#[doc = "Reader of register TZC_INT_CLEAR"]
pub type R = crate::R<u32, super::TZC_INT_CLEAR>;
#[doc = "Writer for register TZC_INT_CLEAR"]
pub type W = crate::W<u32, super::TZC_INT_CLEAR>;
#[doc = "Register TZC_INT_CLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TZC_INT_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:1 - CLEAR"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
}
