#[doc = "Reader of register HASH_CSR26"]
pub type R = crate::R<u32, super::HASH_CSR26>;
#[doc = "Writer for register HASH_CSR26"]
pub type W = crate::W<u32, super::HASH_CSR26>;
#[doc = "Register HASH_CSR26 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR26 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS26`"]
pub type CS26_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS26`"]
pub struct CS26_W<'a> {
    w: &'a mut W,
}
impl<'a> CS26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS26"]
    #[inline(always)]
    pub fn cs26(&self) -> CS26_R {
        CS26_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS26"]
    #[inline(always)]
    pub fn cs26(&mut self) -> CS26_W {
        CS26_W { w: self }
    }
}
