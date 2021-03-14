#[doc = "Reader of register HASH_CSR31"]
pub type R = crate::R<u32, super::HASH_CSR31>;
#[doc = "Writer for register HASH_CSR31"]
pub type W = crate::W<u32, super::HASH_CSR31>;
#[doc = "Register HASH_CSR31 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR31 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS31`"]
pub type CS31_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS31`"]
pub struct CS31_W<'a> {
    w: &'a mut W,
}
impl<'a> CS31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS31"]
    #[inline(always)]
    pub fn cs31(&self) -> CS31_R {
        CS31_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS31"]
    #[inline(always)]
    pub fn cs31(&mut self) -> CS31_W {
        CS31_W { w: self }
    }
}
