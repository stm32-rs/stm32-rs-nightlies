#[doc = "Reader of register HASH_CSR21"]
pub type R = crate::R<u32, super::HASH_CSR21>;
#[doc = "Writer for register HASH_CSR21"]
pub type W = crate::W<u32, super::HASH_CSR21>;
#[doc = "Register HASH_CSR21 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR21 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS21`"]
pub type CS21_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS21`"]
pub struct CS21_W<'a> {
    w: &'a mut W,
}
impl<'a> CS21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS21"]
    #[inline(always)]
    pub fn cs21(&self) -> CS21_R {
        CS21_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS21"]
    #[inline(always)]
    pub fn cs21(&mut self) -> CS21_W {
        CS21_W { w: self }
    }
}
