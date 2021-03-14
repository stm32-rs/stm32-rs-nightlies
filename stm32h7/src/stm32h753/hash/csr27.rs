#[doc = "Reader of register CSR27"]
pub type R = crate::R<u32, super::CSR27>;
#[doc = "Writer for register CSR27"]
pub type W = crate::W<u32, super::CSR27>;
#[doc = "Register CSR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR27`"]
pub type CSR27_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR27`"]
pub struct CSR27_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    pub fn csr27(&self) -> CSR27_R {
        CSR27_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    pub fn csr27(&mut self) -> CSR27_W {
        CSR27_W { w: self }
    }
}
