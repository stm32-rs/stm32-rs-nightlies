#[doc = "Reader of register CSR1"]
pub type R = crate::R<u32, super::CSR1>;
#[doc = "Writer for register CSR1"]
pub type W = crate::W<u32, super::CSR1>;
#[doc = "Register CSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR1`"]
pub type CSR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR1`"]
pub struct CSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR1"]
    #[inline(always)]
    pub fn csr1(&self) -> CSR1_R {
        CSR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR1"]
    #[inline(always)]
    pub fn csr1(&mut self) -> CSR1_W {
        CSR1_W { w: self }
    }
}
