#[doc = "Writer for register R3KEYR0"]
pub type W = crate::W<u32, super::R3KEYR0>;
#[doc = "Register R3KEYR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::R3KEYR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `REGx_KEY`"]
pub struct REGX_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - REGx_KEY"]
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W {
        REGX_KEY_W { w: self }
    }
}
