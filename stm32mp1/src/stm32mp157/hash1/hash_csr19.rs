#[doc = "Reader of register HASH_CSR19"]
pub type R = crate::R<u32, super::HASH_CSR19>;
#[doc = "Writer for register HASH_CSR19"]
pub type W = crate::W<u32, super::HASH_CSR19>;
#[doc = "Register HASH_CSR19 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR19 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS19`"]
pub type CS19_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS19`"]
pub struct CS19_W<'a> {
    w: &'a mut W,
}
impl<'a> CS19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS19"]
    #[inline(always)]
    pub fn cs19(&self) -> CS19_R {
        CS19_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS19"]
    #[inline(always)]
    pub fn cs19(&mut self) -> CS19_W {
        CS19_W { w: self }
    }
}
