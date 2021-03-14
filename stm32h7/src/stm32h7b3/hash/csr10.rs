#[doc = "Reader of register CSR10"]
pub type R = crate::R<u32, super::CSR10>;
#[doc = "Writer for register CSR10"]
pub type W = crate::W<u32, super::CSR10>;
#[doc = "Register CSR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR10`"]
pub type CSR10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR10`"]
pub struct CSR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR10"]
    #[inline(always)]
    pub fn csr10(&self) -> CSR10_R {
        CSR10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR10"]
    #[inline(always)]
    pub fn csr10(&mut self) -> CSR10_W {
        CSR10_W { w: self }
    }
}
