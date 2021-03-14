#[doc = "Writer for register KEYR1"]
pub type W = crate::W<u32, super::KEYR1>;
#[doc = "Register KEYR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `AES_KEYR1`"]
pub struct AES_KEYR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (key \\[63:32\\])"]
    #[inline(always)]
    pub fn aes_keyr1(&mut self) -> AES_KEYR1_W {
        AES_KEYR1_W { w: self }
    }
}
