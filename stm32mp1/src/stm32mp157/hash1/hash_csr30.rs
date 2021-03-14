#[doc = "Reader of register HASH_CSR30"]
pub type R = crate::R<u32, super::HASH_CSR30>;
#[doc = "Writer for register HASH_CSR30"]
pub type W = crate::W<u32, super::HASH_CSR30>;
#[doc = "Register HASH_CSR30 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR30 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS30`"]
pub type CS30_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS30`"]
pub struct CS30_W<'a> {
    w: &'a mut W,
}
impl<'a> CS30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS30"]
    #[inline(always)]
    pub fn cs30(&self) -> CS30_R {
        CS30_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS30"]
    #[inline(always)]
    pub fn cs30(&mut self) -> CS30_W {
        CS30_W { w: self }
    }
}
