#[doc = "Reader of register RCC_CECCKSELR"]
pub type R = crate::R<u32, super::RCC_CECCKSELR>;
#[doc = "Writer for register RCC_CECCKSELR"]
pub type W = crate::W<u32, super::RCC_CECCKSELR>;
#[doc = "Register RCC_CECCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_CECCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CECSRC`"]
pub type CECSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CECSRC`"]
pub struct CECSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CECSRC_W {
        CECSRC_W { w: self }
    }
}
