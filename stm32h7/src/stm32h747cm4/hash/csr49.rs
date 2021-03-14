#[doc = "Reader of register CSR49"]
pub type R = crate::R<u32, super::CSR49>;
#[doc = "Writer for register CSR49"]
pub type W = crate::W<u32, super::CSR49>;
#[doc = "Register CSR49 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR49 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR49`"]
pub type CSR49_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR49`"]
pub struct CSR49_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    pub fn csr49(&self) -> CSR49_R {
        CSR49_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    pub fn csr49(&mut self) -> CSR49_W {
        CSR49_W { w: self }
    }
}
