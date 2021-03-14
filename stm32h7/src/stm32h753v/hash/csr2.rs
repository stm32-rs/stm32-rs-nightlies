#[doc = "Reader of register CSR2"]
pub type R = crate::R<u32, super::CSR2>;
#[doc = "Writer for register CSR2"]
pub type W = crate::W<u32, super::CSR2>;
#[doc = "Register CSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR2`"]
pub type CSR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR2`"]
pub struct CSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR2"]
    #[inline(always)]
    pub fn csr2(&self) -> CSR2_R {
        CSR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR2"]
    #[inline(always)]
    pub fn csr2(&mut self) -> CSR2_W {
        CSR2_W { w: self }
    }
}
