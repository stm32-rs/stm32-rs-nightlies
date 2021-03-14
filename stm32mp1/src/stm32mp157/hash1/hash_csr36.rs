#[doc = "Reader of register HASH_CSR36"]
pub type R = crate::R<u32, super::HASH_CSR36>;
#[doc = "Writer for register HASH_CSR36"]
pub type W = crate::W<u32, super::HASH_CSR36>;
#[doc = "Register HASH_CSR36 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR36 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS36`"]
pub type CS36_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS36`"]
pub struct CS36_W<'a> {
    w: &'a mut W,
}
impl<'a> CS36_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS36"]
    #[inline(always)]
    pub fn cs36(&self) -> CS36_R {
        CS36_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS36"]
    #[inline(always)]
    pub fn cs36(&mut self) -> CS36_W {
        CS36_W { w: self }
    }
}
