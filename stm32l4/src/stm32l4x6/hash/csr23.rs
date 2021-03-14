#[doc = "Reader of register CSR23"]
pub type R = crate::R<u32, super::CSR23>;
#[doc = "Writer for register CSR23"]
pub type W = crate::W<u32, super::CSR23>;
#[doc = "Register CSR23 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR23`"]
pub type CSR23_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR23`"]
pub struct CSR23_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR23"]
    #[inline(always)]
    pub fn csr23(&self) -> CSR23_R {
        CSR23_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR23"]
    #[inline(always)]
    pub fn csr23(&mut self) -> CSR23_W {
        CSR23_W { w: self }
    }
}
