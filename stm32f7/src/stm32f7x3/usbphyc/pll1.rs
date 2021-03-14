#[doc = "Reader of register PLL1"]
pub type R = crate::R<u32, super::PLL1>;
#[doc = "Writer for register PLL1"]
pub type W = crate::W<u32, super::PLL1>;
#[doc = "Register PLL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLL1EN`"]
pub type PLL1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL1EN`"]
pub struct PLL1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PLL1SEL`"]
pub type PLL1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLL1SEL`"]
pub struct PLL1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&self) -> PLL1EN_R {
        PLL1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&self) -> PLL1SEL_R {
        PLL1SEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PLL1 inside PHY"]
    #[inline(always)]
    pub fn pll1en(&mut self) -> PLL1EN_W {
        PLL1EN_W { w: self }
    }
    #[doc = "Bits 1:3 - : Controls the PHY PLL1 input clock frequency selection"]
    #[inline(always)]
    pub fn pll1sel(&mut self) -> PLL1SEL_W {
        PLL1SEL_W { w: self }
    }
}
