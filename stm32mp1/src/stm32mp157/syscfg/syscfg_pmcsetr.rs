#[doc = "Reader of register SYSCFG_PMCSETR"]
pub type R = crate::R<u32, super::SYSCFG_PMCSETR>;
#[doc = "Writer for register SYSCFG_PMCSETR"]
pub type W = crate::W<u32, super::SYSCFG_PMCSETR>;
#[doc = "Register SYSCFG_PMCSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_PMCSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C1_FMP`"]
pub type I2C1_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_FMP`"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
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
#[doc = "Reader of field `I2C2_FMP`"]
pub type I2C2_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2_FMP`"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2C3_FMP`"]
pub type I2C3_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3_FMP`"]
pub struct I2C3_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_FMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2C4_FMP`"]
pub type I2C4_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4_FMP`"]
pub struct I2C4_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4_FMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `I2C5_FMP`"]
pub type I2C5_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C5_FMP`"]
pub struct I2C5_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C5_FMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `I2C6_FMP`"]
pub type I2C6_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C6_FMP`"]
pub struct I2C6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6_FMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EN_BOOSTER`"]
pub type EN_BOOSTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_BOOSTER`"]
pub struct EN_BOOSTER_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_BOOSTER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ANASWVDD`"]
pub type ANASWVDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANASWVDD`"]
pub struct ANASWVDD_W<'a> {
    w: &'a mut W,
}
impl<'a> ANASWVDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ETH_CLK_SEL`"]
pub type ETH_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH_CLK_SEL`"]
pub struct ETH_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `ETH_REF_CLK_SEL`"]
pub type ETH_REF_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH_REF_CLK_SEL`"]
pub struct ETH_REF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_REF_CLK_SEL_W<'a> {
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
#[doc = "Reader of field `ETH_SELMII`"]
pub type ETH_SELMII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH_SELMII`"]
pub struct ETH_SELMII_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_SELMII_W<'a> {
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
#[doc = "Reader of field `ETH_SEL`"]
pub type ETH_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETH_SEL`"]
pub struct ETH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `ANA0_SEL`"]
pub type ANA0_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANA0_SEL`"]
pub struct ANA0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA0_SEL_W<'a> {
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
#[doc = "Reader of field `ANA1_SEL`"]
pub type ANA1_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANA1_SEL`"]
pub struct ANA1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA1_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    pub fn i2c4_fmp(&self) -> I2C4_FMP_R {
        I2C4_FMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    pub fn i2c5_fmp(&self) -> I2C5_FMP_R {
        I2C5_FMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    pub fn i2c6_fmp(&self) -> I2C6_FMP_R {
        I2C6_FMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    pub fn en_booster(&self) -> EN_BOOSTER_R {
        EN_BOOSTER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    pub fn eth_clk_sel(&self) -> ETH_CLK_SEL_R {
        ETH_CLK_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    pub fn eth_ref_clk_sel(&self) -> ETH_REF_CLK_SEL_R {
        ETH_REF_CLK_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    pub fn eth_selmii(&self) -> ETH_SELMII_R {
        ETH_SELMII_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    pub fn eth_sel(&self) -> ETH_SEL_R {
        ETH_SEL_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    pub fn ana0_sel(&self) -> ANA0_SEL_R {
        ANA0_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    pub fn ana1_sel(&self) -> ANA1_SEL_R {
        ANA1_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1_FMP"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    #[doc = "Bit 1 - I2C2_FMP"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bit 2 - I2C3_FMP"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W {
        I2C3_FMP_W { w: self }
    }
    #[doc = "Bit 3 - I2C4_FMP"]
    #[inline(always)]
    pub fn i2c4_fmp(&mut self) -> I2C4_FMP_W {
        I2C4_FMP_W { w: self }
    }
    #[doc = "Bit 4 - I2C5_FMP"]
    #[inline(always)]
    pub fn i2c5_fmp(&mut self) -> I2C5_FMP_W {
        I2C5_FMP_W { w: self }
    }
    #[doc = "Bit 5 - I2C6_FMP"]
    #[inline(always)]
    pub fn i2c6_fmp(&mut self) -> I2C6_FMP_W {
        I2C6_FMP_W { w: self }
    }
    #[doc = "Bit 8 - EN_BOOSTER"]
    #[inline(always)]
    pub fn en_booster(&mut self) -> EN_BOOSTER_W {
        EN_BOOSTER_W { w: self }
    }
    #[doc = "Bit 9 - ANASWVDD"]
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W {
        ANASWVDD_W { w: self }
    }
    #[doc = "Bit 16 - ETH_CLK_SEL"]
    #[inline(always)]
    pub fn eth_clk_sel(&mut self) -> ETH_CLK_SEL_W {
        ETH_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 17 - ETH_REF_CLK_SEL"]
    #[inline(always)]
    pub fn eth_ref_clk_sel(&mut self) -> ETH_REF_CLK_SEL_W {
        ETH_REF_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 20 - ETH_SELMII"]
    #[inline(always)]
    pub fn eth_selmii(&mut self) -> ETH_SELMII_W {
        ETH_SELMII_W { w: self }
    }
    #[doc = "Bits 21:23 - ETH_SEL"]
    #[inline(always)]
    pub fn eth_sel(&mut self) -> ETH_SEL_W {
        ETH_SEL_W { w: self }
    }
    #[doc = "Bit 24 - ANA0_SEL"]
    #[inline(always)]
    pub fn ana0_sel(&mut self) -> ANA0_SEL_W {
        ANA0_SEL_W { w: self }
    }
    #[doc = "Bit 25 - ANA1_SEL"]
    #[inline(always)]
    pub fn ana1_sel(&mut self) -> ANA1_SEL_W {
        ANA1_SEL_W { w: self }
    }
}
