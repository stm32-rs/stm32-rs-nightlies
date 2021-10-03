#[doc = "Register `SYSCFG_PMCSETR` reader"]
pub struct R(crate::R<SYSCFG_PMCSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_PMCSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_PMCSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_PMCSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_PMCSETR` writer"]
pub struct W(crate::W<SYSCFG_PMCSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_PMCSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYSCFG_PMCSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_PMCSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1_FMP"]
pub struct I2C1_FMP_R(crate::FieldReader<bool, bool>);
impl I2C1_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1_FMP"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2_FMP"]
pub struct I2C2_FMP_R(crate::FieldReader<bool, bool>);
impl I2C2_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2_FMP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `I2C3_FMP` reader - I2C3_FMP"]
pub struct I2C3_FMP_R(crate::FieldReader<bool, bool>);
impl I2C3_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3_FMP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `I2C4_FMP` reader - I2C4_FMP"]
pub struct I2C4_FMP_R(crate::FieldReader<bool, bool>);
impl I2C4_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4_FMP` writer - I2C4_FMP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `I2C5_FMP` reader - I2C5_FMP"]
pub struct I2C5_FMP_R(crate::FieldReader<bool, bool>);
impl I2C5_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C5_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C5_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C5_FMP` writer - I2C5_FMP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `I2C6_FMP` reader - I2C6_FMP"]
pub struct I2C6_FMP_R(crate::FieldReader<bool, bool>);
impl I2C6_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C6_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C6_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C6_FMP` writer - I2C6_FMP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EN_BOOSTER` reader - EN_BOOSTER"]
pub struct EN_BOOSTER_R(crate::FieldReader<bool, bool>);
impl EN_BOOSTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_BOOSTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_BOOSTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_BOOSTER` writer - EN_BOOSTER"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ANASWVDD` reader - ANASWVDD"]
pub struct ANASWVDD_R(crate::FieldReader<bool, bool>);
impl ANASWVDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANASWVDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANASWVDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANASWVDD` writer - ANASWVDD"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ETH_CLK_SEL` reader - ETH_CLK_SEL"]
pub struct ETH_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl ETH_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH_CLK_SEL` writer - ETH_CLK_SEL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ETH_REF_CLK_SEL` reader - ETH_REF_CLK_SEL"]
pub struct ETH_REF_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl ETH_REF_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH_REF_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_REF_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH_REF_CLK_SEL` writer - ETH_REF_CLK_SEL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ETH_SELMII` reader - ETH_SELMII"]
pub struct ETH_SELMII_R(crate::FieldReader<bool, bool>);
impl ETH_SELMII_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH_SELMII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_SELMII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH_SELMII` writer - ETH_SELMII"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ETH_SEL` reader - ETH_SEL"]
pub struct ETH_SEL_R(crate::FieldReader<u8, u8>);
impl ETH_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH_SEL` writer - ETH_SEL"]
pub struct ETH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `ANA0_SEL` reader - ANA0_SEL"]
pub struct ANA0_SEL_R(crate::FieldReader<bool, bool>);
impl ANA0_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANA0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA0_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA0_SEL` writer - ANA0_SEL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ANA1_SEL` reader - ANA1_SEL"]
pub struct ANA1_SEL_R(crate::FieldReader<bool, bool>);
impl ANA1_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANA1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA1_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA1_SEL` writer - ANA1_SEL"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG peripheral mode configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_pmcsetr](index.html) module"]
pub struct SYSCFG_PMCSETR_SPEC;
impl crate::RegisterSpec for SYSCFG_PMCSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_pmcsetr::R](R) reader structure"]
impl crate::Readable for SYSCFG_PMCSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_pmcsetr::W](W) writer structure"]
impl crate::Writable for SYSCFG_PMCSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG_PMCSETR to value 0"]
impl crate::Resettable for SYSCFG_PMCSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
