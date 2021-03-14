#[doc = "Reader of register HASH_CSR10"]
pub type R = crate::R<u32, super::HASH_CSR10>;
#[doc = "Writer for register HASH_CSR10"]
pub type W = crate::W<u32, super::HASH_CSR10>;
#[doc = "Register HASH_CSR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS10`"]
pub type CS10_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS10`"]
pub struct CS10_W<'a> {
    w: &'a mut W,
}
impl<'a> CS10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS10"]
    #[inline(always)]
    pub fn cs10(&self) -> CS10_R {
        CS10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS10"]
    #[inline(always)]
    pub fn cs10(&mut self) -> CS10_W {
        CS10_W { w: self }
    }
}
