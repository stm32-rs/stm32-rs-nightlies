#[doc = "Reader of register HASH_CSR39"]
pub type R = crate::R<u32, super::HASH_CSR39>;
#[doc = "Writer for register HASH_CSR39"]
pub type W = crate::W<u32, super::HASH_CSR39>;
#[doc = "Register HASH_CSR39 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR39 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS39`"]
pub type CS39_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS39`"]
pub struct CS39_W<'a> {
    w: &'a mut W,
}
impl<'a> CS39_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS39"]
    #[inline(always)]
    pub fn cs39(&self) -> CS39_R {
        CS39_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS39"]
    #[inline(always)]
    pub fn cs39(&mut self) -> CS39_W {
        CS39_W { w: self }
    }
}
