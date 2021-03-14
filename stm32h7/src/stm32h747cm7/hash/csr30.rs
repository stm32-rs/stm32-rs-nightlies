#[doc = "Reader of register CSR30"]
pub type R = crate::R<u32, super::CSR30>;
#[doc = "Writer for register CSR30"]
pub type W = crate::W<u32, super::CSR30>;
#[doc = "Register CSR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR30`"]
pub type CSR30_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR30`"]
pub struct CSR30_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    pub fn csr30(&self) -> CSR30_R {
        CSR30_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    pub fn csr30(&mut self) -> CSR30_W {
        CSR30_W { w: self }
    }
}
