#[doc = "Writer for register SECKEYR"]
pub type W = crate::W<u32, super::SECKEYR>;
#[doc = "Register SECKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SECKEYR`"]
pub struct SECKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - SECKEYR"]
    #[inline(always)]
    pub fn seckeyr(&mut self) -> SECKEYR_W {
        SECKEYR_W { w: self }
    }
}
