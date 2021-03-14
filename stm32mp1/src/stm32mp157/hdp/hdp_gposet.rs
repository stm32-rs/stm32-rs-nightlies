#[doc = "Writer for register HDP_GPOSET"]
pub type W = crate::W<u32, super::HDP_GPOSET>;
#[doc = "Register HDP_GPOSET `reset()`'s with value 0"]
impl crate::ResetValue for super::HDP_GPOSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HDPGPOSET`"]
pub struct HDPGPOSET_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPGPOSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOSET"]
    #[inline(always)]
    pub fn hdpgposet(&mut self) -> HDPGPOSET_W {
        HDPGPOSET_W { w: self }
    }
}
