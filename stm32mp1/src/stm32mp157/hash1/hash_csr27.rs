#[doc = "Reader of register HASH_CSR27"]
pub type R = crate::R<u32, super::HASH_CSR27>;
#[doc = "Writer for register HASH_CSR27"]
pub type W = crate::W<u32, super::HASH_CSR27>;
#[doc = "Register HASH_CSR27 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR27 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS27`"]
pub type CS27_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS27`"]
pub struct CS27_W<'a> {
    w: &'a mut W,
}
impl<'a> CS27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS27"]
    #[inline(always)]
    pub fn cs27(&self) -> CS27_R {
        CS27_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS27"]
    #[inline(always)]
    pub fn cs27(&mut self) -> CS27_W {
        CS27_W { w: self }
    }
}
