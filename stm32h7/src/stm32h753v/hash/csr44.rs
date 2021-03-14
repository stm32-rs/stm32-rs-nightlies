#[doc = "Reader of register CSR44"]
pub type R = crate::R<u32, super::CSR44>;
#[doc = "Writer for register CSR44"]
pub type W = crate::W<u32, super::CSR44>;
#[doc = "Register CSR44 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR44 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR44`"]
pub type CSR44_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR44`"]
pub struct CSR44_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR44_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR44"]
    #[inline(always)]
    pub fn csr44(&self) -> CSR44_R {
        CSR44_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR44"]
    #[inline(always)]
    pub fn csr44(&mut self) -> CSR44_W {
        CSR44_W { w: self }
    }
}
