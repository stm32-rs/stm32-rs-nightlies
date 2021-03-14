#[doc = "Reader of register RCC_ADCCKSELR"]
pub type R = crate::R<u32, super::RCC_ADCCKSELR>;
#[doc = "Writer for register RCC_ADCCKSELR"]
pub type W = crate::W<u32, super::RCC_ADCCKSELR>;
#[doc = "Register RCC_ADCCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_ADCCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCSRC`"]
pub type ADCSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCSRC`"]
pub struct ADCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&mut self) -> ADCSRC_W {
        ADCSRC_W { w: self }
    }
}
