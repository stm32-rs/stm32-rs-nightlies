#[doc = "Register `APB1LPENR` reader"]
pub struct R(crate::R<APB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LPENR` writer"]
pub struct W(crate::W<APB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LPENR_SPEC>;
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
impl From<crate::W<APB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPLPEN` reader - COMP interface clock enable during Sleep mode"]
pub struct COMPLPEN_R(crate::FieldReader<bool, bool>);
impl COMPLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPLPEN` writer - COMP interface clock enable during Sleep mode"]
pub struct COMPLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `DACLPEN` reader - DAC interface clock enable during Sleep mode"]
pub struct DACLPEN_R(crate::FieldReader<bool, bool>);
impl DACLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACLPEN` writer - DAC interface clock enable during Sleep mode"]
pub struct DACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub struct PWRLPEN_R(crate::FieldReader<bool, bool>);
impl PWRLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub struct PWRLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `USBLPEN` reader - USB clock enable during Sleep mode"]
pub struct USBLPEN_R(crate::FieldReader<bool, bool>);
impl USBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBLPEN` writer - USB clock enable during Sleep mode"]
pub struct USBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `I2C2LPEN` reader - I2C 2 clock enable during Sleep mode"]
pub struct I2C2LPEN_R(crate::FieldReader<bool, bool>);
impl I2C2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2LPEN` writer - I2C 2 clock enable during Sleep mode"]
pub struct I2C2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `I2C1LPEN` reader - I2C 1 clock enable during Sleep mode"]
pub struct I2C1LPEN_R(crate::FieldReader<bool, bool>);
impl I2C1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1LPEN` writer - I2C 1 clock enable during Sleep mode"]
pub struct I2C1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `USART3LPEN` reader - USART 3 clock enable during Sleep mode"]
pub struct USART3LPEN_R(crate::FieldReader<bool, bool>);
impl USART3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3LPEN` writer - USART 3 clock enable during Sleep mode"]
pub struct USART3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3LPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `USART2LPEN` reader - USART 2 clock enable during Sleep mode"]
pub struct USART2LPEN_R(crate::FieldReader<bool, bool>);
impl USART2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2LPEN` writer - USART 2 clock enable during Sleep mode"]
pub struct USART2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2LPEN_W<'a> {
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
#[doc = "Field `SPI2LPEN` reader - SPI 2 clock enable during Sleep mode"]
pub struct SPI2LPEN_R(crate::FieldReader<bool, bool>);
impl SPI2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2LPEN` writer - SPI 2 clock enable during Sleep mode"]
pub struct SPI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2LPEN_W<'a> {
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
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub struct WWDGLPEN_R(crate::FieldReader<bool, bool>);
impl WWDGLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub struct WWDGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `LCDLPEN` reader - LCD clock enable during Sleep mode"]
pub struct LCDLPEN_R(crate::FieldReader<bool, bool>);
impl LCDLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDLPEN` writer - LCD clock enable during Sleep mode"]
pub struct LCDLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDLPEN_W<'a> {
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
#[doc = "Field `TIM7LPEN` reader - Timer 7 clock enable during Sleep mode"]
pub struct TIM7LPEN_R(crate::FieldReader<bool, bool>);
impl TIM7LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7LPEN` writer - Timer 7 clock enable during Sleep mode"]
pub struct TIM7LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7LPEN_W<'a> {
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
#[doc = "Field `TIM6LPEN` reader - Timer 6 clock enable during Sleep mode"]
pub struct TIM6LPEN_R(crate::FieldReader<bool, bool>);
impl TIM6LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6LPEN` writer - Timer 6 clock enable during Sleep mode"]
pub struct TIM6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6LPEN_W<'a> {
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
#[doc = "Field `TIM4LPEN` reader - Timer 4 clock enable during Sleep mode"]
pub struct TIM4LPEN_R(crate::FieldReader<bool, bool>);
impl TIM4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4LPEN` writer - Timer 4 clock enable during Sleep mode"]
pub struct TIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4LPEN_W<'a> {
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
#[doc = "Field `TIM3LPEN` reader - Timer 3 clock enable during Sleep mode"]
pub struct TIM3LPEN_R(crate::FieldReader<bool, bool>);
impl TIM3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3LPEN` writer - Timer 3 clock enable during Sleep mode"]
pub struct TIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3LPEN_W<'a> {
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
#[doc = "Field `TIM2LPEN` reader - Timer 2 clock enable during Sleep mode"]
pub struct TIM2LPEN_R(crate::FieldReader<bool, bool>);
impl TIM2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2LPEN` writer - Timer 2 clock enable during Sleep mode"]
pub struct TIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2LPEN_W<'a> {
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
impl R {
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn complpen(&self) -> COMPLPEN_R {
        COMPLPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usblpen(&self) -> USBLPEN_R {
        USBLPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lcdlpen(&self) -> LCDLPEN_R {
        LCDLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - COMP interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn complpen(&mut self) -> COMPLPEN_W {
        COMPLPEN_W { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W {
        DACLPEN_W { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W {
        PWRLPEN_W { w: self }
    }
    #[doc = "Bit 23 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usblpen(&mut self) -> USBLPEN_W {
        USBLPEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W {
        I2C2LPEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W {
        I2C1LPEN_W { w: self }
    }
    #[doc = "Bit 18 - USART 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W {
        USART3LPEN_W { w: self }
    }
    #[doc = "Bit 17 - USART 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W {
        USART2LPEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W {
        SPI2LPEN_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W {
        WWDGLPEN_W { w: self }
    }
    #[doc = "Bit 9 - LCD clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lcdlpen(&mut self) -> LCDLPEN_W {
        LCDLPEN_W { w: self }
    }
    #[doc = "Bit 5 - Timer 7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W {
        TIM7LPEN_W { w: self }
    }
    #[doc = "Bit 4 - Timer 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W {
        TIM6LPEN_W { w: self }
    }
    #[doc = "Bit 2 - Timer 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W {
        TIM4LPEN_W { w: self }
    }
    #[doc = "Bit 1 - Timer 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W {
        TIM3LPEN_W { w: self }
    }
    #[doc = "Bit 0 - Timer 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W {
        TIM2LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](index.html) module"]
pub struct APB1LPENR_SPEC;
impl crate::RegisterSpec for APB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lpenr::R](R) reader structure"]
impl crate::Readable for APB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](W) writer structure"]
impl crate::Writable for APB1LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LPENR to value 0"]
impl crate::Resettable for APB1LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
