#[doc = "Reader of register CSR35"]
pub type R = crate::R<u32, super::CSR35>;
#[doc = "Writer for register CSR35"]
pub type W = crate::W<u32, super::CSR35>;
#[doc = "Register CSR35 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR35 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR35`"]
pub type CSR35_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR35`"]
pub struct CSR35_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    pub fn csr35(&self) -> CSR35_R {
        CSR35_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    pub fn csr35(&mut self) -> CSR35_W {
        CSR35_W { w: self }
    }
}
