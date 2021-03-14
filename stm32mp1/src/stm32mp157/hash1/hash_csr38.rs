#[doc = "Reader of register HASH_CSR38"]
pub type R = crate::R<u32, super::HASH_CSR38>;
#[doc = "Writer for register HASH_CSR38"]
pub type W = crate::W<u32, super::HASH_CSR38>;
#[doc = "Register HASH_CSR38 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS38`"]
pub type CS38_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS38`"]
pub struct CS38_W<'a> {
    w: &'a mut W,
}
impl<'a> CS38_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS38"]
    #[inline(always)]
    pub fn cs38(&self) -> CS38_R {
        CS38_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS38"]
    #[inline(always)]
    pub fn cs38(&mut self) -> CS38_W {
        CS38_W { w: self }
    }
}
