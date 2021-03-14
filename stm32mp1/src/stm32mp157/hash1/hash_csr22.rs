#[doc = "Reader of register HASH_CSR22"]
pub type R = crate::R<u32, super::HASH_CSR22>;
#[doc = "Writer for register HASH_CSR22"]
pub type W = crate::W<u32, super::HASH_CSR22>;
#[doc = "Register HASH_CSR22 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS22`"]
pub type CS22_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS22`"]
pub struct CS22_W<'a> {
    w: &'a mut W,
}
impl<'a> CS22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS22"]
    #[inline(always)]
    pub fn cs22(&self) -> CS22_R {
        CS22_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS22"]
    #[inline(always)]
    pub fn cs22(&mut self) -> CS22_W {
        CS22_W { w: self }
    }
}
