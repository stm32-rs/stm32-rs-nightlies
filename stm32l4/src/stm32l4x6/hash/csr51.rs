#[doc = "Reader of register CSR51"]
pub type R = crate::R<u32, super::CSR51>;
#[doc = "Writer for register CSR51"]
pub type W = crate::W<u32, super::CSR51>;
#[doc = "Register CSR51 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR51 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR51`"]
pub type CSR51_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR51`"]
pub struct CSR51_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR51"]
    #[inline(always)]
    pub fn csr51(&self) -> CSR51_R {
        CSR51_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR51"]
    #[inline(always)]
    pub fn csr51(&mut self) -> CSR51_W {
        CSR51_W { w: self }
    }
}
