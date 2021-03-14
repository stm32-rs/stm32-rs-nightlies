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
#[doc = "Write proxy for field `FKEYR`"]
pub struct FKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> FKEYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Flash Key"]
    #[inline(always)]
    pub fn fkeyr(&mut self) -> FKEYR_W {
        FKEYR_W { w: self }
    }
}
