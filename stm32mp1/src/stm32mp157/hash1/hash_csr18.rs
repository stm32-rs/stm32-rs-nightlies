#[doc = "Reader of register HASH_CSR18"]
pub type R = crate::R<u32, super::HASH_CSR18>;
#[doc = "Writer for register HASH_CSR18"]
pub type W = crate::W<u32, super::HASH_CSR18>;
#[doc = "Register HASH_CSR18 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS18`"]
pub type CS18_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS18`"]
pub struct CS18_W<'a> {
    w: &'a mut W,
}
impl<'a> CS18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS18"]
    #[inline(always)]
    pub fn cs18(&self) -> CS18_R {
        CS18_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS18"]
    #[inline(always)]
    pub fn cs18(&mut self) -> CS18_W {
        CS18_W { w: self }
    }
}
