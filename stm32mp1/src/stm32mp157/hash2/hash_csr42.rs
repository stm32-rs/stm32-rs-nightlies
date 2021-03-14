#[doc = "Reader of register HASH_CSR42"]
pub type R = crate::R<u32, super::HASH_CSR42>;
#[doc = "Writer for register HASH_CSR42"]
pub type W = crate::W<u32, super::HASH_CSR42>;
#[doc = "Register HASH_CSR42 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR42 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS42`"]
pub type CS42_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS42`"]
pub struct CS42_W<'a> {
    w: &'a mut W,
}
impl<'a> CS42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS42"]
    #[inline(always)]
    pub fn cs42(&self) -> CS42_R {
        CS42_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS42"]
    #[inline(always)]
    pub fn cs42(&mut self) -> CS42_W {
        CS42_W { w: self }
    }
}
