#[doc = "Reader of register HASH_CSR17"]
pub type R = crate::R<u32, super::HASH_CSR17>;
#[doc = "Writer for register HASH_CSR17"]
pub type W = crate::W<u32, super::HASH_CSR17>;
#[doc = "Register HASH_CSR17 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS17`"]
pub type CS17_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS17`"]
pub struct CS17_W<'a> {
    w: &'a mut W,
}
impl<'a> CS17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS17"]
    #[inline(always)]
    pub fn cs17(&self) -> CS17_R {
        CS17_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS17"]
    #[inline(always)]
    pub fn cs17(&mut self) -> CS17_W {
        CS17_W { w: self }
    }
}
