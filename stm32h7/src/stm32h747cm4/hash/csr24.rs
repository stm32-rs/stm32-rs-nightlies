#[doc = "Reader of register CSR24"]
pub type R = crate::R<u32, super::CSR24>;
#[doc = "Writer for register CSR24"]
pub type W = crate::W<u32, super::CSR24>;
#[doc = "Register CSR24 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR24`"]
pub type CSR24_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR24`"]
pub struct CSR24_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR24"]
    #[inline(always)]
    pub fn csr24(&self) -> CSR24_R {
        CSR24_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR24"]
    #[inline(always)]
    pub fn csr24(&mut self) -> CSR24_W {
        CSR24_W { w: self }
    }
}
