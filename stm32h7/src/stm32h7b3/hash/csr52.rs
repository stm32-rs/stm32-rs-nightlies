#[doc = "Reader of register CSR52"]
pub type R = crate::R<u32, super::CSR52>;
#[doc = "Writer for register CSR52"]
pub type W = crate::W<u32, super::CSR52>;
#[doc = "Register CSR52 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR52 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR52`"]
pub type CSR52_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR52`"]
pub struct CSR52_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    pub fn csr52(&self) -> CSR52_R {
        CSR52_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    pub fn csr52(&mut self) -> CSR52_W {
        CSR52_W { w: self }
    }
}
