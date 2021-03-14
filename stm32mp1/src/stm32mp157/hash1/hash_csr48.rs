#[doc = "Reader of register HASH_CSR48"]
pub type R = crate::R<u32, super::HASH_CSR48>;
#[doc = "Writer for register HASH_CSR48"]
pub type W = crate::W<u32, super::HASH_CSR48>;
#[doc = "Register HASH_CSR48 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR48 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS48`"]
pub type CS48_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS48`"]
pub struct CS48_W<'a> {
    w: &'a mut W,
}
impl<'a> CS48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS48"]
    #[inline(always)]
    pub fn cs48(&self) -> CS48_R {
        CS48_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS48"]
    #[inline(always)]
    pub fn cs48(&mut self) -> CS48_W {
        CS48_W { w: self }
    }
}
