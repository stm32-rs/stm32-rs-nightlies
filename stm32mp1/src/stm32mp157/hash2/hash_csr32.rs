#[doc = "Reader of register HASH_CSR32"]
pub type R = crate::R<u32, super::HASH_CSR32>;
#[doc = "Writer for register HASH_CSR32"]
pub type W = crate::W<u32, super::HASH_CSR32>;
#[doc = "Register HASH_CSR32 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS32`"]
pub type CS32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS32`"]
pub struct CS32_W<'a> {
    w: &'a mut W,
}
impl<'a> CS32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS32"]
    #[inline(always)]
    pub fn cs32(&self) -> CS32_R {
        CS32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS32"]
    #[inline(always)]
    pub fn cs32(&mut self) -> CS32_W {
        CS32_W { w: self }
    }
}
