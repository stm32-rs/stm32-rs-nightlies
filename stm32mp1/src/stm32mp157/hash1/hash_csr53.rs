#[doc = "Reader of register HASH_CSR53"]
pub type R = crate::R<u32, super::HASH_CSR53>;
#[doc = "Writer for register HASH_CSR53"]
pub type W = crate::W<u32, super::HASH_CSR53>;
#[doc = "Register HASH_CSR53 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR53 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS53`"]
pub type CS53_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS53`"]
pub struct CS53_W<'a> {
    w: &'a mut W,
}
impl<'a> CS53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS53"]
    #[inline(always)]
    pub fn cs53(&self) -> CS53_R {
        CS53_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS53"]
    #[inline(always)]
    pub fn cs53(&mut self) -> CS53_W {
        CS53_W { w: self }
    }
}
