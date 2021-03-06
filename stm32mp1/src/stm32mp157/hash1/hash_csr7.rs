#[doc = "Reader of register HASH_CSR7"]
pub type R = crate::R<u32, super::HASH_CSR7>;
#[doc = "Writer for register HASH_CSR7"]
pub type W = crate::W<u32, super::HASH_CSR7>;
#[doc = "Register HASH_CSR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS7`"]
pub type CS7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS7`"]
pub struct CS7_W<'a> {
    w: &'a mut W,
}
impl<'a> CS7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS7"]
    #[inline(always)]
    pub fn cs7(&self) -> CS7_R {
        CS7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS7"]
    #[inline(always)]
    pub fn cs7(&mut self) -> CS7_W {
        CS7_W { w: self }
    }
}
