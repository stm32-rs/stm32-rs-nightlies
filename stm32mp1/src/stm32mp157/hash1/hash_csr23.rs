#[doc = "Reader of register HASH_CSR23"]
pub type R = crate::R<u32, super::HASH_CSR23>;
#[doc = "Writer for register HASH_CSR23"]
pub type W = crate::W<u32, super::HASH_CSR23>;
#[doc = "Register HASH_CSR23 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS23`"]
pub type CS23_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS23`"]
pub struct CS23_W<'a> {
    w: &'a mut W,
}
impl<'a> CS23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS23"]
    #[inline(always)]
    pub fn cs23(&self) -> CS23_R {
        CS23_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS23"]
    #[inline(always)]
    pub fn cs23(&mut self) -> CS23_W {
        CS23_W { w: self }
    }
}
