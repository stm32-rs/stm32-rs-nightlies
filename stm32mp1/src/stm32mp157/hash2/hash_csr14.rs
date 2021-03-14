#[doc = "Reader of register HASH_CSR14"]
pub type R = crate::R<u32, super::HASH_CSR14>;
#[doc = "Writer for register HASH_CSR14"]
pub type W = crate::W<u32, super::HASH_CSR14>;
#[doc = "Register HASH_CSR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS14`"]
pub type CS14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS14`"]
pub struct CS14_W<'a> {
    w: &'a mut W,
}
impl<'a> CS14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS14"]
    #[inline(always)]
    pub fn cs14(&self) -> CS14_R {
        CS14_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS14"]
    #[inline(always)]
    pub fn cs14(&mut self) -> CS14_W {
        CS14_W { w: self }
    }
}
