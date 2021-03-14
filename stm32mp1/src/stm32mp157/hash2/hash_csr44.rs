#[doc = "Reader of register HASH_CSR44"]
pub type R = crate::R<u32, super::HASH_CSR44>;
#[doc = "Writer for register HASH_CSR44"]
pub type W = crate::W<u32, super::HASH_CSR44>;
#[doc = "Register HASH_CSR44 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR44 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS44`"]
pub type CS44_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS44`"]
pub struct CS44_W<'a> {
    w: &'a mut W,
}
impl<'a> CS44_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS44"]
    #[inline(always)]
    pub fn cs44(&self) -> CS44_R {
        CS44_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS44"]
    #[inline(always)]
    pub fn cs44(&mut self) -> CS44_W {
        CS44_W { w: self }
    }
}
