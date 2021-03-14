#[doc = "Reader of register CSR14"]
pub type R = crate::R<u32, super::CSR14>;
#[doc = "Writer for register CSR14"]
pub type W = crate::W<u32, super::CSR14>;
#[doc = "Register CSR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR14`"]
pub type CSR14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR14`"]
pub struct CSR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR14"]
    #[inline(always)]
    pub fn csr14(&self) -> CSR14_R {
        CSR14_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR14"]
    #[inline(always)]
    pub fn csr14(&mut self) -> CSR14_W {
        CSR14_W { w: self }
    }
}
