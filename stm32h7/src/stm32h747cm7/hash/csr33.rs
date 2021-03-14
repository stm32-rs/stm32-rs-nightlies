#[doc = "Reader of register CSR33"]
pub type R = crate::R<u32, super::CSR33>;
#[doc = "Writer for register CSR33"]
pub type W = crate::W<u32, super::CSR33>;
#[doc = "Register CSR33 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR33 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR33`"]
pub type CSR33_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR33`"]
pub struct CSR33_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR33"]
    #[inline(always)]
    pub fn csr33(&self) -> CSR33_R {
        CSR33_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR33"]
    #[inline(always)]
    pub fn csr33(&mut self) -> CSR33_W {
        CSR33_W { w: self }
    }
}
