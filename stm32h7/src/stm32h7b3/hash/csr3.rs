#[doc = "Reader of register CSR3"]
pub type R = crate::R<u32, super::CSR3>;
#[doc = "Writer for register CSR3"]
pub type W = crate::W<u32, super::CSR3>;
#[doc = "Register CSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR3`"]
pub type CSR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR3`"]
pub struct CSR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    pub fn csr3(&self) -> CSR3_R {
        CSR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    pub fn csr3(&mut self) -> CSR3_W {
        CSR3_W { w: self }
    }
}
