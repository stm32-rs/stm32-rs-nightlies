#[doc = "Reader of register HASH_CSR8"]
pub type R = crate::R<u32, super::HASH_CSR8>;
#[doc = "Writer for register HASH_CSR8"]
pub type W = crate::W<u32, super::HASH_CSR8>;
#[doc = "Register HASH_CSR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS8`"]
pub type CS8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS8`"]
pub struct CS8_W<'a> {
    w: &'a mut W,
}
impl<'a> CS8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS8"]
    #[inline(always)]
    pub fn cs8(&self) -> CS8_R {
        CS8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS8"]
    #[inline(always)]
    pub fn cs8(&mut self) -> CS8_W {
        CS8_W { w: self }
    }
}
