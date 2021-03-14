#[doc = "Reader of register SECBB1R4"]
pub type R = crate::R<u32, super::SECBB1R4>;
#[doc = "Writer for register SECBB1R4"]
pub type W = crate::W<u32, super::SECBB1R4>;
#[doc = "Register SECBB1R4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SECBB1R4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECBB1`"]
pub type SECBB1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SECBB1`"]
pub struct SECBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    pub fn secbb1(&mut self) -> SECBB1_W {
        SECBB1_W { w: self }
    }
}
