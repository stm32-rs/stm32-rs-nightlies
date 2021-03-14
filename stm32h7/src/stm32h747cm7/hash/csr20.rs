#[doc = "Reader of register CSR20"]
pub type R = crate::R<u32, super::CSR20>;
#[doc = "Writer for register CSR20"]
pub type W = crate::W<u32, super::CSR20>;
#[doc = "Register CSR20 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR20`"]
pub type CSR20_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR20`"]
pub struct CSR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&self) -> CSR20_R {
        CSR20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&mut self) -> CSR20_W {
        CSR20_W { w: self }
    }
}
