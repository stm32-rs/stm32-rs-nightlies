#[doc = "Reader of register CSR11"]
pub type R = crate::R<u32, super::CSR11>;
#[doc = "Writer for register CSR11"]
pub type W = crate::W<u32, super::CSR11>;
#[doc = "Register CSR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR11`"]
pub type CSR11_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR11`"]
pub struct CSR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR11"]
    #[inline(always)]
    pub fn csr11(&self) -> CSR11_R {
        CSR11_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR11"]
    #[inline(always)]
    pub fn csr11(&mut self) -> CSR11_W {
        CSR11_W { w: self }
    }
}
