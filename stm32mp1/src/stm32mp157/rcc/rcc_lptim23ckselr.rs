#[doc = "Reader of register RCC_LPTIM23CKSELR"]
pub type R = crate::R<u32, super::RCC_LPTIM23CKSELR>;
#[doc = "Writer for register RCC_LPTIM23CKSELR"]
pub type W = crate::W<u32, super::RCC_LPTIM23CKSELR>;
#[doc = "Register RCC_LPTIM23CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_LPTIM23CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPTIM23SRC`"]
pub type LPTIM23SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPTIM23SRC`"]
pub struct LPTIM23SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM23SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    pub fn lptim23src(&self) -> LPTIM23SRC_R {
        LPTIM23SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    pub fn lptim23src(&mut self) -> LPTIM23SRC_W {
        LPTIM23SRC_W { w: self }
    }
}
