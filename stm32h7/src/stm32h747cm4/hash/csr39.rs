#[doc = "Reader of register CSR39"]
pub type R = crate::R<u32, super::CSR39>;
#[doc = "Writer for register CSR39"]
pub type W = crate::W<u32, super::CSR39>;
#[doc = "Register CSR39 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR39 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR39`"]
pub type CSR39_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR39`"]
pub struct CSR39_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR39"]
    #[inline(always)]
    pub fn csr39(&self) -> CSR39_R {
        CSR39_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR39"]
    #[inline(always)]
    pub fn csr39(&mut self) -> CSR39_W {
        CSR39_W { w: self }
    }
}
