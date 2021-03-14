#[doc = "Reader of register CSR25"]
pub type R = crate::R<u32, super::CSR25>;
#[doc = "Writer for register CSR25"]
pub type W = crate::W<u32, super::CSR25>;
#[doc = "Register CSR25 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR25`"]
pub type CSR25_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR25`"]
pub struct CSR25_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&self) -> CSR25_R {
        CSR25_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&mut self) -> CSR25_W {
        CSR25_W { w: self }
    }
}
