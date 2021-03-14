#[doc = "Reader of register RCC_RCK12SELR"]
pub type R = crate::R<u32, super::RCC_RCK12SELR>;
#[doc = "Writer for register RCC_RCK12SELR"]
pub type W = crate::W<u32, super::RCC_RCK12SELR>;
#[doc = "Register RCC_RCK12SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK12SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `PLL12SRC`"]
pub type PLL12SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL12SRC`"]
pub struct PLL12SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL12SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLL12SRCRDY`"]
pub type PLL12SRCRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&self) -> PLL12SRC_R {
        PLL12SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL12SRCRDY"]
    #[inline(always)]
    pub fn pll12srcrdy(&self) -> PLL12SRCRDY_R {
        PLL12SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&mut self) -> PLL12SRC_W {
        PLL12SRC_W { w: self }
    }
}
