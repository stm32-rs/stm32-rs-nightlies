#[doc = "Reader of register CSR28"]
pub type R = crate::R<u32, super::CSR28>;
#[doc = "Writer for register CSR28"]
pub type W = crate::W<u32, super::CSR28>;
#[doc = "Register CSR28 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR28`"]
pub type CSR28_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR28`"]
pub struct CSR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR28"]
    #[inline(always)]
    pub fn csr28(&self) -> CSR28_R {
        CSR28_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR28"]
    #[inline(always)]
    pub fn csr28(&mut self) -> CSR28_W {
        CSR28_W { w: self }
    }
}
