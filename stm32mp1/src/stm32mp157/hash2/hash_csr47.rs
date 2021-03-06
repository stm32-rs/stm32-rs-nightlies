#[doc = "Reader of register HASH_CSR47"]
pub type R = crate::R<u32, super::HASH_CSR47>;
#[doc = "Writer for register HASH_CSR47"]
pub type W = crate::W<u32, super::HASH_CSR47>;
#[doc = "Register HASH_CSR47 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR47 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS47`"]
pub type CS47_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS47`"]
pub struct CS47_W<'a> {
    w: &'a mut W,
}
impl<'a> CS47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS47"]
    #[inline(always)]
    pub fn cs47(&self) -> CS47_R {
        CS47_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS47"]
    #[inline(always)]
    pub fn cs47(&mut self) -> CS47_W {
        CS47_W { w: self }
    }
}
