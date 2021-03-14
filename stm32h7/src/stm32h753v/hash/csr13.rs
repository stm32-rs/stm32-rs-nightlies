#[doc = "Reader of register CSR13"]
pub type R = crate::R<u32, super::CSR13>;
#[doc = "Writer for register CSR13"]
pub type W = crate::W<u32, super::CSR13>;
#[doc = "Register CSR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR13`"]
pub type CSR13_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR13`"]
pub struct CSR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR13"]
    #[inline(always)]
    pub fn csr13(&self) -> CSR13_R {
        CSR13_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR13"]
    #[inline(always)]
    pub fn csr13(&mut self) -> CSR13_W {
        CSR13_W { w: self }
    }
}
