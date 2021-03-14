#[doc = "Reader of register CSR42"]
pub type R = crate::R<u32, super::CSR42>;
#[doc = "Writer for register CSR42"]
pub type W = crate::W<u32, super::CSR42>;
#[doc = "Register CSR42 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR42 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR42`"]
pub type CSR42_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR42`"]
pub struct CSR42_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR42_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR42"]
    #[inline(always)]
    pub fn csr42(&self) -> CSR42_R {
        CSR42_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR42"]
    #[inline(always)]
    pub fn csr42(&mut self) -> CSR42_W {
        CSR42_W { w: self }
    }
}
