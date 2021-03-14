#[doc = "Reader of register CSR45"]
pub type R = crate::R<u32, super::CSR45>;
#[doc = "Writer for register CSR45"]
pub type W = crate::W<u32, super::CSR45>;
#[doc = "Register CSR45 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR45 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR45`"]
pub type CSR45_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR45`"]
pub struct CSR45_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR45_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR45"]
    #[inline(always)]
    pub fn csr45(&self) -> CSR45_R {
        CSR45_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR45"]
    #[inline(always)]
    pub fn csr45(&mut self) -> CSR45_W {
        CSR45_W { w: self }
    }
}
