#[doc = "Writer for register LVEKEYR"]
pub type W = crate::W<u32, super::LVEKEYR>;
#[doc = "Register LVEKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::LVEKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `LVEKEYR`"]
pub struct LVEKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> LVEKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - LVEKEYR"]
    #[inline(always)]
    pub fn lvekeyr(&mut self) -> LVEKEYR_W {
        LVEKEYR_W { w: self }
    }
}
