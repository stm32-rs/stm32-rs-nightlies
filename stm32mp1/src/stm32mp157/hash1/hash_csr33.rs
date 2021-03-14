#[doc = "Reader of register HASH_CSR33"]
pub type R = crate::R<u32, super::HASH_CSR33>;
#[doc = "Writer for register HASH_CSR33"]
pub type W = crate::W<u32, super::HASH_CSR33>;
#[doc = "Register HASH_CSR33 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR33 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS33`"]
pub type CS33_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS33`"]
pub struct CS33_W<'a> {
    w: &'a mut W,
}
impl<'a> CS33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS33"]
    #[inline(always)]
    pub fn cs33(&self) -> CS33_R {
        CS33_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS33"]
    #[inline(always)]
    pub fn cs33(&mut self) -> CS33_W {
        CS33_W { w: self }
    }
}
