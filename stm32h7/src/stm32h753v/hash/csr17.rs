#[doc = "Reader of register CSR17"]
pub type R = crate::R<u32, super::CSR17>;
#[doc = "Writer for register CSR17"]
pub type W = crate::W<u32, super::CSR17>;
#[doc = "Register CSR17 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR17`"]
pub type CSR17_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR17`"]
pub struct CSR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&self) -> CSR17_R {
        CSR17_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&mut self) -> CSR17_W {
        CSR17_W { w: self }
    }
}
