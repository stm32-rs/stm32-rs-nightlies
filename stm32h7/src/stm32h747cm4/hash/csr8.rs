#[doc = "Reader of register CSR8"]
pub type R = crate::R<u32, super::CSR8>;
#[doc = "Writer for register CSR8"]
pub type W = crate::W<u32, super::CSR8>;
#[doc = "Register CSR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR8`"]
pub type CSR8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR8`"]
pub struct CSR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    pub fn csr8(&self) -> CSR8_R {
        CSR8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR8"]
    #[inline(always)]
    pub fn csr8(&mut self) -> CSR8_W {
        CSR8_W { w: self }
    }
}
