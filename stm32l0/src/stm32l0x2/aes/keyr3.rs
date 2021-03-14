#[doc = "Reader of register KEYR3"]
pub type R = crate::R<u32, super::KEYR3>;
#[doc = "Writer for register KEYR3"]
pub type W = crate::W<u32, super::KEYR3>;
#[doc = "Register KEYR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY3`"]
pub type KEY3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY3`"]
pub struct KEY3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[127:96\\])"]
    #[inline(always)]
    pub fn key3(&mut self) -> KEY3_W {
        KEY3_W { w: self }
    }
}
