#[doc = "Reader of register CSR19"]
pub type R = crate::R<u32, super::CSR19>;
#[doc = "Writer for register CSR19"]
pub type W = crate::W<u32, super::CSR19>;
#[doc = "Register CSR19 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR19`"]
pub type CSR19_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR19`"]
pub struct CSR19_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    pub fn csr19(&self) -> CSR19_R {
        CSR19_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    pub fn csr19(&mut self) -> CSR19_W {
        CSR19_W { w: self }
    }
}
