#[doc = "Reader of register CSR38"]
pub type R = crate::R<u32, super::CSR38>;
#[doc = "Writer for register CSR38"]
pub type W = crate::W<u32, super::CSR38>;
#[doc = "Register CSR38 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR38`"]
pub type CSR38_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR38`"]
pub struct CSR38_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    pub fn csr38(&self) -> CSR38_R {
        CSR38_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR38"]
    #[inline(always)]
    pub fn csr38(&mut self) -> CSR38_W {
        CSR38_W { w: self }
    }
}
