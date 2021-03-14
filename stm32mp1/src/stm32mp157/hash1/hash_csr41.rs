#[doc = "Reader of register HASH_CSR41"]
pub type R = crate::R<u32, super::HASH_CSR41>;
#[doc = "Writer for register HASH_CSR41"]
pub type W = crate::W<u32, super::HASH_CSR41>;
#[doc = "Register HASH_CSR41 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR41 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS41`"]
pub type CS41_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS41`"]
pub struct CS41_W<'a> {
    w: &'a mut W,
}
impl<'a> CS41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS41"]
    #[inline(always)]
    pub fn cs41(&self) -> CS41_R {
        CS41_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS41"]
    #[inline(always)]
    pub fn cs41(&mut self) -> CS41_W {
        CS41_W { w: self }
    }
}
