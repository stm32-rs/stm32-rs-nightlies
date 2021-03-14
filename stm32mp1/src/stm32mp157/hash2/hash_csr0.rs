#[doc = "Reader of register HASH_CSR0"]
pub type R = crate::R<u32, super::HASH_CSR0>;
#[doc = "Writer for register HASH_CSR0"]
pub type W = crate::W<u32, super::HASH_CSR0>;
#[doc = "Register HASH_CSR0 `reset()`'s with value 0x02"]
impl crate::ResetValue for super::HASH_CSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `CS0`"]
pub type CS0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS0`"]
pub struct CS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS0"]
    #[inline(always)]
    pub fn cs0(&self) -> CS0_R {
        CS0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS0"]
    #[inline(always)]
    pub fn cs0(&mut self) -> CS0_W {
        CS0_W { w: self }
    }
}
