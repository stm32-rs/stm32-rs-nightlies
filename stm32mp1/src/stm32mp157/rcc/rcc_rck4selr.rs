#[doc = "Reader of register RCC_RCK4SELR"]
pub type R = crate::R<u32, super::RCC_RCK4SELR>;
#[doc = "Writer for register RCC_RCK4SELR"]
pub type W = crate::W<u32, super::RCC_RCK4SELR>;
#[doc = "Register RCC_RCK4SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK4SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `PLL4SRC`"]
pub type PLL4SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL4SRC`"]
pub struct PLL4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLL4SRCRDY`"]
pub type PLL4SRCRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    pub fn pll4src(&self) -> PLL4SRC_R {
        PLL4SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL4SRCRDY"]
    #[inline(always)]
    pub fn pll4srcrdy(&self) -> PLL4SRCRDY_R {
        PLL4SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    pub fn pll4src(&mut self) -> PLL4SRC_W {
        PLL4SRC_W { w: self }
    }
}
