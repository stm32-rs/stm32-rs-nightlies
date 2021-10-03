#[doc = "Register `APB4RSTR` reader"]
pub struct R(crate::R<APB4RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB4RSTR` writer"]
pub struct W(crate::W<APB4RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4RSTR_SPEC>;
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
impl From<crate::W<APB4RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYSCFG block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG block reset Set and reset by software."]
pub struct SYSCFGRST_R(crate::FieldReader<bool, SYSCFGRST_A>);
impl SYSCFGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRST_A> {
        match self.bits {
            true => Some(SYSCFGRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SYSCFGRST_A::RESET
    }
}
impl core::ops::Deref for SYSCFGRST_R {
    type Target = crate::FieldReader<bool, SYSCFGRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG block reset Set and reset by software."]
pub struct SYSCFGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::RESET)
    }
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
#[doc = "LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_A = SYSCFGRST_A;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_R = SYSCFGRST_R;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub struct LPUART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPUART1RST_A::RESET)
    }
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
#[doc = "SPI6 block reset Set and reset by software."]
pub type SPI6RST_A = SYSCFGRST_A;
#[doc = "Field `SPI6RST` reader - SPI6 block reset Set and reset by software."]
pub type SPI6RST_R = SYSCFGRST_R;
#[doc = "Field `SPI6RST` writer - SPI6 block reset Set and reset by software."]
pub struct SPI6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI6RST_A::RESET)
    }
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
#[doc = "I2C4 block reset Set and reset by software."]
pub type I2C4RST_A = SYSCFGRST_A;
#[doc = "Field `I2C4RST` reader - I2C4 block reset Set and reset by software."]
pub type I2C4RST_R = SYSCFGRST_R;
#[doc = "Field `I2C4RST` writer - I2C4 block reset Set and reset by software."]
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C4RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_A = SYSCFGRST_A;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software."]
pub type LPTIM2RST_R = SYSCFGRST_R;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software."]
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::RESET)
    }
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
#[doc = "LPTIM3 block reset Set and reset by software."]
pub type LPTIM3RST_A = SYSCFGRST_A;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset Set and reset by software."]
pub type LPTIM3RST_R = SYSCFGRST_R;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset Set and reset by software."]
pub struct LPTIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM3RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "DAC2 (containing one converter) reset Set and reset by software."]
pub type DAC2RST_A = SYSCFGRST_A;
#[doc = "Field `DAC2RST` reader - DAC2 (containing one converter) reset Set and reset by software."]
pub type DAC2RST_R = SYSCFGRST_R;
#[doc = "Field `DAC2RST` writer - DAC2 (containing one converter) reset Set and reset by software."]
pub struct DAC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DAC2RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "COMP1 and 2 blocks reset Set and reset by software."]
pub type COMP12RST_A = SYSCFGRST_A;
#[doc = "Field `COMP12RST` reader - COMP1 and 2 blocks reset Set and reset by software."]
pub type COMP12RST_R = SYSCFGRST_R;
#[doc = "Field `COMP12RST` writer - COMP1 and 2 blocks reset Set and reset by software."]
pub struct COMP12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP12RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(COMP12RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "VREF block reset Set and reset by software."]
pub type VREFRST_A = SYSCFGRST_A;
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub type VREFRST_R = SYSCFGRST_R;
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub struct VREFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VREFRST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Digital temperature sensor block reset Set and reset by software."]
pub type DTSRST_A = SYSCFGRST_A;
#[doc = "Field `DTSRST` reader - Digital temperature sensor block reset Set and reset by software."]
pub type DTSRST_R = SYSCFGRST_R;
#[doc = "Field `DTSRST` writer - Digital temperature sensor block reset Set and reset by software."]
pub struct DTSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTSRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DTSRST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "DFSDM2 block reset Set and reset by software."]
pub type DFSDM2RST_A = SYSCFGRST_A;
#[doc = "Field `DFSDM2RST` reader - DFSDM2 block reset Set and reset by software."]
pub type DFSDM2RST_R = SYSCFGRST_R;
#[doc = "Field `DFSDM2RST` writer - DFSDM2 block reset Set and reset by software."]
pub struct DFSDM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDM2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DFSDM2RST_A::RESET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SYSCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) reset Set and reset by software."]
    #[inline(always)]
    pub fn dac2rst(&self) -> DAC2RST_R {
        DAC2RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - COMP1 and 2 blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn comp12rst(&self) -> COMP12RST_R {
        COMP12RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor block reset Set and reset by software."]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DFSDM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn dfsdm2rst(&self) -> DFSDM2RST_R {
        DFSDM2RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG block reset Set and reset by software."]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W {
        SYSCFGRST_W { w: self }
    }
    #[doc = "Bit 3 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W {
        LPUART1RST_W { w: self }
    }
    #[doc = "Bit 5 - SPI6 block reset Set and reset by software."]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W {
        SPI6RST_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 block reset Set and reset by software."]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W {
        LPTIM3RST_W { w: self }
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) reset Set and reset by software."]
    #[inline(always)]
    pub fn dac2rst(&mut self) -> DAC2RST_W {
        DAC2RST_W { w: self }
    }
    #[doc = "Bit 14 - COMP1 and 2 blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn comp12rst(&mut self) -> COMP12RST_W {
        COMP12RST_W { w: self }
    }
    #[doc = "Bit 15 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W {
        VREFRST_W { w: self }
    }
    #[doc = "Bit 26 - Digital temperature sensor block reset Set and reset by software."]
    #[inline(always)]
    pub fn dtsrst(&mut self) -> DTSRST_W {
        DTSRST_W { w: self }
    }
    #[doc = "Bit 27 - DFSDM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn dfsdm2rst(&mut self) -> DFSDM2RST_W {
        DFSDM2RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4rstr](index.html) module"]
pub struct APB4RSTR_SPEC;
impl crate::RegisterSpec for APB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb4rstr::R](R) reader structure"]
impl crate::Readable for APB4RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb4rstr::W](W) writer structure"]
impl crate::Writable for APB4RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB4RSTR to value 0"]
impl crate::Resettable for APB4RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
