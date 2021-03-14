#[doc = "Writer for register PRGKEYR"]
pub type W = crate::W<u32, super::PRGKEYR>;
#[doc = "Register PRGKEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRGKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PRGKEYR`"]
pub struct PRGKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Program memory key"]
    #[inline(always)]
    pub fn prgkeyr(&mut self) -> PRGKEYR_W {
        PRGKEYR_W { w: self }
    }
}
