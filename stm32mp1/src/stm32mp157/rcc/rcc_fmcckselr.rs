#[doc = "Reader of register RCC_FMCCKSELR"]
pub type R = crate::R<u32, super::RCC_FMCCKSELR>;
#[doc = "Writer for register RCC_FMCCKSELR"]
pub type W = crate::W<u32, super::RCC_FMCCKSELR>;
#[doc = "Register RCC_FMCCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_FMCCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMCSRC`"]
pub type FMCSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMCSRC`"]
pub struct FMCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&mut self) -> FMCSRC_W {
        FMCSRC_W { w: self }
    }
}
