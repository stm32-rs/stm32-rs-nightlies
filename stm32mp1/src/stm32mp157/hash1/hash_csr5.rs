#[doc = "Reader of register HASH_CSR5"]
pub type R = crate::R<u32, super::HASH_CSR5>;
#[doc = "Writer for register HASH_CSR5"]
pub type W = crate::W<u32, super::HASH_CSR5>;
#[doc = "Register HASH_CSR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS5`"]
pub type CS5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS5`"]
pub struct CS5_W<'a> {
    w: &'a mut W,
}
impl<'a> CS5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS5"]
    #[inline(always)]
    pub fn cs5(&self) -> CS5_R {
        CS5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS5"]
    #[inline(always)]
    pub fn cs5(&mut self) -> CS5_W {
        CS5_W { w: self }
    }
}
