#[doc = "Reader of register SECBB2R4"]
pub type R = crate::R<u32, super::SECBB2R4>;
#[doc = "Writer for register SECBB2R4"]
pub type W = crate::W<u32, super::SECBB2R4>;
#[doc = "Register SECBB2R4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SECBB2R4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECBB2`"]
pub type SECBB2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SECBB2`"]
pub struct SECBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB2"]
    #[inline(always)]
    pub fn secbb2(&mut self) -> SECBB2_W {
        SECBB2_W { w: self }
    }
}
