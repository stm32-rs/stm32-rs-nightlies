#[doc = "Reader of register CSR41"]
pub type R = crate::R<u32, super::CSR41>;
#[doc = "Writer for register CSR41"]
pub type W = crate::W<u32, super::CSR41>;
#[doc = "Register CSR41 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR41 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR41`"]
pub type CSR41_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR41`"]
pub struct CSR41_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR41"]
    #[inline(always)]
    pub fn csr41(&self) -> CSR41_R {
        CSR41_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR41"]
    #[inline(always)]
    pub fn csr41(&mut self) -> CSR41_W {
        CSR41_W { w: self }
    }
}
