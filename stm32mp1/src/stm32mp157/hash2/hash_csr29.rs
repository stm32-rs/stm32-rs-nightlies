#[doc = "Reader of register HASH_CSR29"]
pub type R = crate::R<u32, super::HASH_CSR29>;
#[doc = "Writer for register HASH_CSR29"]
pub type W = crate::W<u32, super::HASH_CSR29>;
#[doc = "Register HASH_CSR29 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS29`"]
pub type CS29_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS29`"]
pub struct CS29_W<'a> {
    w: &'a mut W,
}
impl<'a> CS29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS29"]
    #[inline(always)]
    pub fn cs29(&self) -> CS29_R {
        CS29_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS29"]
    #[inline(always)]
    pub fn cs29(&mut self) -> CS29_W {
        CS29_W { w: self }
    }
}
