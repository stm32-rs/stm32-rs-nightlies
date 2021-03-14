#[doc = "Reader of register CSR47"]
pub type R = crate::R<u32, super::CSR47>;
#[doc = "Writer for register CSR47"]
pub type W = crate::W<u32, super::CSR47>;
#[doc = "Register CSR47 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR47 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR47`"]
pub type CSR47_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR47`"]
pub struct CSR47_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR47_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR47"]
    #[inline(always)]
    pub fn csr47(&self) -> CSR47_R {
        CSR47_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR47"]
    #[inline(always)]
    pub fn csr47(&mut self) -> CSR47_W {
        CSR47_W { w: self }
    }
}
