#[doc = "Reader of register HASH_CSR11"]
pub type R = crate::R<u32, super::HASH_CSR11>;
#[doc = "Writer for register HASH_CSR11"]
pub type W = crate::W<u32, super::HASH_CSR11>;
#[doc = "Register HASH_CSR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS11`"]
pub type CS11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS11`"]
pub struct CS11_W<'a> {
    w: &'a mut W,
}
impl<'a> CS11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS11"]
    #[inline(always)]
    pub fn cs11(&self) -> CS11_R {
        CS11_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS11"]
    #[inline(always)]
    pub fn cs11(&mut self) -> CS11_W {
        CS11_W { w: self }
    }
}
