#[doc = "Reader of register HASH_CSR6"]
pub type R = crate::R<u32, super::HASH_CSR6>;
#[doc = "Writer for register HASH_CSR6"]
pub type W = crate::W<u32, super::HASH_CSR6>;
#[doc = "Register HASH_CSR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS6`"]
pub type CS6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS6`"]
pub struct CS6_W<'a> {
    w: &'a mut W,
}
impl<'a> CS6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS6"]
    #[inline(always)]
    pub fn cs6(&self) -> CS6_R {
        CS6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS6"]
    #[inline(always)]
    pub fn cs6(&mut self) -> CS6_W {
        CS6_W { w: self }
    }
}
