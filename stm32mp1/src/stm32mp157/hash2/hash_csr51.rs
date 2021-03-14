#[doc = "Reader of register HASH_CSR51"]
pub type R = crate::R<u32, super::HASH_CSR51>;
#[doc = "Writer for register HASH_CSR51"]
pub type W = crate::W<u32, super::HASH_CSR51>;
#[doc = "Register HASH_CSR51 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR51 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS51`"]
pub type CS51_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS51`"]
pub struct CS51_W<'a> {
    w: &'a mut W,
}
impl<'a> CS51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS51"]
    #[inline(always)]
    pub fn cs51(&self) -> CS51_R {
        CS51_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS51"]
    #[inline(always)]
    pub fn cs51(&mut self) -> CS51_W {
        CS51_W { w: self }
    }
}
