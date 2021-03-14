#[doc = "Reader of register PLLSAI1CFGR"]
pub type R = crate::R<u32, super::PLLSAI1CFGR>;
#[doc = "Writer for register PLLSAI1CFGR"]
pub type W = crate::W<u32, super::PLLSAI1CFGR>;
#[doc = "Register PLLSAI1CFGR `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::PLLSAI1CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `PLLSAI1PDIV`"]
pub type PLLSAI1PDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1PDIV`"]
pub struct PLLSAI1PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1R`"]
pub type PLLSAI1R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1R`"]
pub struct PLLSAI1R_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1REN`"]
pub type PLLSAI1REN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1REN`"]
pub struct PLLSAI1REN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1REN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1Q`"]
pub type PLLSAI1Q_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1Q`"]
pub struct PLLSAI1Q_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1QEN`"]
pub type PLLSAI1QEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1QEN`"]
pub struct PLLSAI1QEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1QEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1P`"]
pub type PLLSAI1P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1P`"]
pub struct PLLSAI1P_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1PEN`"]
pub type PLLSAI1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSAI1PEN`"]
pub struct PLLSAI1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1N`"]
pub type PLLSAI1N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1N`"]
pub struct PLLSAI1N_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1M`"]
pub type PLLSAI1M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1M`"]
pub struct PLLSAI1M_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PLLSAI1SRC`"]
pub type PLLSAI1SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAI1SRC`"]
pub struct PLLSAI1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> PLLSAI1PDIV_R {
        PLLSAI1PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI1 input clock"]
    #[inline(always)]
    pub fn pllsai1m(&self) -> PLLSAI1M_R {
        PLLSAI1M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - PLLSAI1SRC"]
    #[inline(always)]
    pub fn pllsai1src(&self) -> PLLSAI1SRC_R {
        PLLSAI1SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&mut self) -> PLLSAI1PDIV_W {
        PLLSAI1PDIV_W { w: self }
    }
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W {
        PLLSAI1R_W { w: self }
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W {
        PLLSAI1REN_W { w: self }
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W {
        PLLSAI1Q_W { w: self }
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W {
        PLLSAI1QEN_W { w: self }
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W {
        PLLSAI1P_W { w: self }
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W {
        PLLSAI1PEN_W { w: self }
    }
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W {
        PLLSAI1N_W { w: self }
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI1 input clock"]
    #[inline(always)]
    pub fn pllsai1m(&mut self) -> PLLSAI1M_W {
        PLLSAI1M_W { w: self }
    }
    #[doc = "Bits 0:1 - PLLSAI1SRC"]
    #[inline(always)]
    pub fn pllsai1src(&mut self) -> PLLSAI1SRC_W {
        PLLSAI1SRC_W { w: self }
    }
}
