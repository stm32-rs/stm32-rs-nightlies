#[doc = "Reader of register HASH_CSR12"]
pub type R = crate::R<u32, super::HASH_CSR12>;
#[doc = "Writer for register HASH_CSR12"]
pub type W = crate::W<u32, super::HASH_CSR12>;
#[doc = "Register HASH_CSR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS12`"]
pub type CS12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS12`"]
pub struct CS12_W<'a> {
    w: &'a mut W,
}
impl<'a> CS12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS12"]
    #[inline(always)]
    pub fn cs12(&self) -> CS12_R {
        CS12_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS12"]
    #[inline(always)]
    pub fn cs12(&mut self) -> CS12_W {
        CS12_W { w: self }
    }
}
