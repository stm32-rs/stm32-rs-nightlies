#[doc = "Reader of register HASH_CSR52"]
pub type R = crate::R<u32, super::HASH_CSR52>;
#[doc = "Writer for register HASH_CSR52"]
pub type W = crate::W<u32, super::HASH_CSR52>;
#[doc = "Register HASH_CSR52 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR52 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS52`"]
pub type CS52_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS52`"]
pub struct CS52_W<'a> {
    w: &'a mut W,
}
impl<'a> CS52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    pub fn cs52(&self) -> CS52_R {
        CS52_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    pub fn cs52(&mut self) -> CS52_W {
        CS52_W { w: self }
    }
}
