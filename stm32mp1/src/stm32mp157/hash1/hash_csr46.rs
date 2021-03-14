#[doc = "Reader of register HASH_CSR46"]
pub type R = crate::R<u32, super::HASH_CSR46>;
#[doc = "Writer for register HASH_CSR46"]
pub type W = crate::W<u32, super::HASH_CSR46>;
#[doc = "Register HASH_CSR46 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR46 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS46`"]
pub type CS46_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS46`"]
pub struct CS46_W<'a> {
    w: &'a mut W,
}
impl<'a> CS46_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS46"]
    #[inline(always)]
    pub fn cs46(&self) -> CS46_R {
        CS46_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS46"]
    #[inline(always)]
    pub fn cs46(&mut self) -> CS46_W {
        CS46_W { w: self }
    }
}
