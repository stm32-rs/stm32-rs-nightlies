#[doc = "Reader of register RCC_RTCDIVR"]
pub type R = crate::R<u32, super::RCC_RTCDIVR>;
#[doc = "Writer for register RCC_RTCDIVR"]
pub type W = crate::W<u32, super::RCC_RTCDIVR>;
#[doc = "Register RCC_RTCDIVR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RTCDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCDIV`"]
pub type RTCDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCDIV`"]
pub struct RTCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    pub fn rtcdiv(&self) -> RTCDIV_R {
        RTCDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    pub fn rtcdiv(&mut self) -> RTCDIV_W {
        RTCDIV_W { w: self }
    }
}
