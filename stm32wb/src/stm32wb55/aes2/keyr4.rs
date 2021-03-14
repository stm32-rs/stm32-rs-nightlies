#[doc = "Reader of register KEYR4"]
pub type R = crate::R<u32, super::KEYR4>;
#[doc = "Writer for register KEYR4"]
pub type W = crate::W<u32, super::KEYR4>;
#[doc = "Register KEYR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_KEYR4`"]
pub type AES_KEYR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_KEYR4`"]
pub struct AES_KEYR4_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[159:128\\])"]
    #[inline(always)]
    pub fn aes_keyr4(&self) -> AES_KEYR4_R {
        AES_KEYR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[159:128\\])"]
    #[inline(always)]
    pub fn aes_keyr4(&mut self) -> AES_KEYR4_W {
        AES_KEYR4_W { w: self }
    }
}
