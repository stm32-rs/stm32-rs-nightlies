#[doc = "Reader of register HASH_CSR34"]
pub type R = crate::R<u32, super::HASH_CSR34>;
#[doc = "Writer for register HASH_CSR34"]
pub type W = crate::W<u32, super::HASH_CSR34>;
#[doc = "Register HASH_CSR34 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS34`"]
pub type CS34_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS34`"]
pub struct CS34_W<'a> {
    w: &'a mut W,
}
impl<'a> CS34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS34"]
    #[inline(always)]
    pub fn cs34(&self) -> CS34_R {
        CS34_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS34"]
    #[inline(always)]
    pub fn cs34(&mut self) -> CS34_W {
        CS34_W { w: self }
    }
}
