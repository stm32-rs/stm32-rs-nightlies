#[doc = "Reader of register HASH_CSR40"]
pub type R = crate::R<u32, super::HASH_CSR40>;
#[doc = "Writer for register HASH_CSR40"]
pub type W = crate::W<u32, super::HASH_CSR40>;
#[doc = "Register HASH_CSR40 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR40 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS40`"]
pub type CS40_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS40`"]
pub struct CS40_W<'a> {
    w: &'a mut W,
}
impl<'a> CS40_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS40"]
    #[inline(always)]
    pub fn cs40(&self) -> CS40_R {
        CS40_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS40"]
    #[inline(always)]
    pub fn cs40(&mut self) -> CS40_W {
        CS40_W { w: self }
    }
}
