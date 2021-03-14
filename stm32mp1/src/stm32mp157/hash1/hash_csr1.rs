#[doc = "Reader of register HASH_CSR1"]
pub type R = crate::R<u32, super::HASH_CSR1>;
#[doc = "Writer for register HASH_CSR1"]
pub type W = crate::W<u32, super::HASH_CSR1>;
#[doc = "Register HASH_CSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS1`"]
pub type CS1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS1`"]
pub struct CS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS1"]
    #[inline(always)]
    pub fn cs1(&self) -> CS1_R {
        CS1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS1"]
    #[inline(always)]
    pub fn cs1(&mut self) -> CS1_W {
        CS1_W { w: self }
    }
}
