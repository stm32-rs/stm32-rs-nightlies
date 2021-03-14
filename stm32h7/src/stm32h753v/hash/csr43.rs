#[doc = "Reader of register CSR43"]
pub type R = crate::R<u32, super::CSR43>;
#[doc = "Writer for register CSR43"]
pub type W = crate::W<u32, super::CSR43>;
#[doc = "Register CSR43 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR43 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR43`"]
pub type CSR43_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR43`"]
pub struct CSR43_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR43"]
    #[inline(always)]
    pub fn csr43(&self) -> CSR43_R {
        CSR43_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR43"]
    #[inline(always)]
    pub fn csr43(&mut self) -> CSR43_W {
        CSR43_W { w: self }
    }
}
