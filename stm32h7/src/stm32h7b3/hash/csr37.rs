#[doc = "Reader of register CSR37"]
pub type R = crate::R<u32, super::CSR37>;
#[doc = "Writer for register CSR37"]
pub type W = crate::W<u32, super::CSR37>;
#[doc = "Register CSR37 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR37 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR37`"]
pub type CSR37_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR37`"]
pub struct CSR37_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR37_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR37"]
    #[inline(always)]
    pub fn csr37(&self) -> CSR37_R {
        CSR37_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR37"]
    #[inline(always)]
    pub fn csr37(&mut self) -> CSR37_W {
        CSR37_W { w: self }
    }
}
