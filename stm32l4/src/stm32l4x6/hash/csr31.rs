#[doc = "Reader of register CSR31"]
pub type R = crate::R<u32, super::CSR31>;
#[doc = "Writer for register CSR31"]
pub type W = crate::W<u32, super::CSR31>;
#[doc = "Register CSR31 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR31`"]
pub type CSR31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR31`"]
pub struct CSR31_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&self) -> CSR31_R {
        CSR31_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&mut self) -> CSR31_W {
        CSR31_W { w: self }
    }
}
