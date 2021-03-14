#[doc = "Reader of register CSR5"]
pub type R = crate::R<u32, super::CSR5>;
#[doc = "Writer for register CSR5"]
pub type W = crate::W<u32, super::CSR5>;
#[doc = "Register CSR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR5`"]
pub type CSR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR5`"]
pub struct CSR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&self) -> CSR5_R {
        CSR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&mut self) -> CSR5_W {
        CSR5_W { w: self }
    }
}
