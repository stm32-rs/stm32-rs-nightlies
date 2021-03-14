#[doc = "Reader of register RCC_RCK3SELR"]
pub type R = crate::R<u32, super::RCC_RCK3SELR>;
#[doc = "Writer for register RCC_RCK3SELR"]
pub type W = crate::W<u32, super::RCC_RCK3SELR>;
#[doc = "Register RCC_RCK3SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK3SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `PLL3SRC`"]
pub type PLL3SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL3SRC`"]
pub struct PLL3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PLL3SRCRDY`"]
pub type PLL3SRCRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL3SRCRDY"]
    #[inline(always)]
    pub fn pll3srcrdy(&self) -> PLL3SRCRDY_R {
        PLL3SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W {
        PLL3SRC_W { w: self }
    }
}
