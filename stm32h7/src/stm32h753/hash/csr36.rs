#[doc = "Reader of register CSR36"]
pub type R = crate::R<u32, super::CSR36>;
#[doc = "Writer for register CSR36"]
pub type W = crate::W<u32, super::CSR36>;
#[doc = "Register CSR36 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR36`"]
pub type CSR36_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR36`"]
pub struct CSR36_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR36"]
    #[inline(always)]
    pub fn csr36(&self) -> CSR36_R {
        CSR36_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR36"]
    #[inline(always)]
    pub fn csr36(&mut self) -> CSR36_W {
        CSR36_W { w: self }
    }
}
