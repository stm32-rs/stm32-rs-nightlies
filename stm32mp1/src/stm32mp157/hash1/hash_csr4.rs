#[doc = "Reader of register HASH_CSR4"]
pub type R = crate::R<u32, super::HASH_CSR4>;
#[doc = "Writer for register HASH_CSR4"]
pub type W = crate::W<u32, super::HASH_CSR4>;
#[doc = "Register HASH_CSR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS4`"]
pub type CS4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS4`"]
pub struct CS4_W<'a> {
    w: &'a mut W,
}
impl<'a> CS4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS4"]
    #[inline(always)]
    pub fn cs4(&self) -> CS4_R {
        CS4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS4"]
    #[inline(always)]
    pub fn cs4(&mut self) -> CS4_W {
        CS4_W { w: self }
    }
}
