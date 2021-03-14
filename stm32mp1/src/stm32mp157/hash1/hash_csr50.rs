#[doc = "Reader of register HASH_CSR50"]
pub type R = crate::R<u32, super::HASH_CSR50>;
#[doc = "Writer for register HASH_CSR50"]
pub type W = crate::W<u32, super::HASH_CSR50>;
#[doc = "Register HASH_CSR50 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR50 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS50`"]
pub type CS50_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS50`"]
pub struct CS50_W<'a> {
    w: &'a mut W,
}
impl<'a> CS50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS50"]
    #[inline(always)]
    pub fn cs50(&self) -> CS50_R {
        CS50_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS50"]
    #[inline(always)]
    pub fn cs50(&mut self) -> CS50_W {
        CS50_W { w: self }
    }
}
