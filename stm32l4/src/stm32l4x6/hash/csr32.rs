#[doc = "Reader of register CSR32"]
pub type R = crate::R<u32, super::CSR32>;
#[doc = "Writer for register CSR32"]
pub type W = crate::W<u32, super::CSR32>;
#[doc = "Register CSR32 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR32 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR32`"]
pub type CSR32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR32`"]
pub struct CSR32_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR32"]
    #[inline(always)]
    pub fn csr32(&self) -> CSR32_R {
        CSR32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR32"]
    #[inline(always)]
    pub fn csr32(&mut self) -> CSR32_W {
        CSR32_W { w: self }
    }
}
