#[doc = "Writer for register HDP_GPOCLR"]
pub type W = crate::W<u32, super::HDP_GPOCLR>;
#[doc = "Register HDP_GPOCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HDP_GPOCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HDPGPOCLR`"]
pub struct HDPGPOCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPGPOCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOCLR"]
    #[inline(always)]
    pub fn hdpgpoclr(&mut self) -> HDPGPOCLR_W {
        HDPGPOCLR_W { w: self }
    }
}
