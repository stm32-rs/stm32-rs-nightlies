#[doc = "Reader of register HASH_CSR37"]
pub type R = crate::R<u32, super::HASH_CSR37>;
#[doc = "Writer for register HASH_CSR37"]
pub type W = crate::W<u32, super::HASH_CSR37>;
#[doc = "Register HASH_CSR37 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR37 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS37`"]
pub type CS37_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS37`"]
pub struct CS37_W<'a> {
    w: &'a mut W,
}
impl<'a> CS37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS37"]
    #[inline(always)]
    pub fn cs37(&self) -> CS37_R {
        CS37_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS37"]
    #[inline(always)]
    pub fn cs37(&mut self) -> CS37_W {
        CS37_W { w: self }
    }
}
