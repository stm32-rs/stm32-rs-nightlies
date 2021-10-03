#[doc = "Register `RCC_APB5RSTCLRR` reader"]
pub struct R(crate::R<RCC_APB5RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB5RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB5RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB5RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB5RSTCLRR` writer"]
pub struct W(crate::W<RCC_APB5RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB5RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_APB5RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB5RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI6RST` reader - SPI6RST"]
pub struct SPI6RST_R(crate::FieldReader<bool, bool>);
impl SPI6RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI6RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI6RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI6RST` writer - SPI6RST"]
pub struct SPI6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6RST_W<'a> {
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
#[doc = "Field `I2C4RST` reader - I2C4RST"]
pub struct I2C4RST_R(crate::FieldReader<bool, bool>);
impl I2C4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4RST` writer - I2C4RST"]
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
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
#[doc = "Field `I2C6RST` reader - I2C6RST"]
pub struct I2C6RST_R(crate::FieldReader<bool, bool>);
impl I2C6RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C6RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C6RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C6RST` writer - I2C6RST"]
pub struct I2C6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6RST_W<'a> {
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
#[doc = "Field `USART1RST` reader - USART1RST"]
pub struct USART1RST_R(crate::FieldReader<bool, bool>);
impl USART1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1RST` writer - USART1RST"]
pub struct USART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1RST_W<'a> {
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
#[doc = "Field `STGENRST` reader - STGENRST"]
pub struct STGENRST_R(crate::FieldReader<bool, bool>);
impl STGENRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENRST` writer - STGENRST"]
pub struct STGENRST_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W {
        SPI6RST_W { w: self }
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&mut self) -> I2C6RST_W {
        I2C6RST_W { w: self }
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W { w: self }
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&mut self) -> STGENRST_W {
        STGENRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb5rstclrr](index.html) module"]
pub struct RCC_APB5RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_APB5RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb5rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_APB5RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb5rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_APB5RSTCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB5RSTCLRR to value 0"]
impl crate::Resettable for RCC_APB5RSTCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
