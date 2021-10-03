#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2IE` reader - TIM2IE"]
pub struct TIM2IE_R(crate::FieldReader<bool, bool>);
impl TIM2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2IE` writer - TIM2IE"]
pub struct TIM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2IE_W<'a> {
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
#[doc = "Field `TIM3IE` reader - TIM3IE"]
pub struct TIM3IE_R(crate::FieldReader<bool, bool>);
impl TIM3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3IE` writer - TIM3IE"]
pub struct TIM3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3IE_W<'a> {
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
#[doc = "Field `TIM4IE` reader - TIM4IE"]
pub struct TIM4IE_R(crate::FieldReader<bool, bool>);
impl TIM4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4IE` writer - TIM4IE"]
pub struct TIM4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4IE_W<'a> {
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
#[doc = "Field `TIM5IE` reader - TIM5IE"]
pub struct TIM5IE_R(crate::FieldReader<bool, bool>);
impl TIM5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5IE` writer - TIM5IE"]
pub struct TIM5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5IE_W<'a> {
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
#[doc = "Field `TIM6IE` reader - TIM6IE"]
pub struct TIM6IE_R(crate::FieldReader<bool, bool>);
impl TIM6IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6IE` writer - TIM6IE"]
pub struct TIM6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6IE_W<'a> {
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
#[doc = "Field `TIM7IE` reader - TIM7IE"]
pub struct TIM7IE_R(crate::FieldReader<bool, bool>);
impl TIM7IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7IE` writer - TIM7IE"]
pub struct TIM7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7IE_W<'a> {
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
#[doc = "Field `WWDGIE` reader - WWDGIE"]
pub struct WWDGIE_R(crate::FieldReader<bool, bool>);
impl WWDGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGIE` writer - WWDGIE"]
pub struct WWDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IWDGIE` reader - IWDGIE"]
pub struct IWDGIE_R(crate::FieldReader<bool, bool>);
impl IWDGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDGIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDGIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDGIE` writer - IWDGIE"]
pub struct IWDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGIE_W<'a> {
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
#[doc = "Field `SPI2IE` reader - SPI2IE"]
pub struct SPI2IE_R(crate::FieldReader<bool, bool>);
impl SPI2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2IE` writer - SPI2IE"]
pub struct SPI2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IE_W<'a> {
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
#[doc = "Field `SPI3IE` reader - SPI3IE"]
pub struct SPI3IE_R(crate::FieldReader<bool, bool>);
impl SPI3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3IE` writer - SPI3IE"]
pub struct SPI3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3IE_W<'a> {
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
#[doc = "Field `USART2IE` reader - USART2IE"]
pub struct USART2IE_R(crate::FieldReader<bool, bool>);
impl USART2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2IE` writer - USART2IE"]
pub struct USART2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2IE_W<'a> {
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
#[doc = "Field `USART3IE` reader - USART3IE"]
pub struct USART3IE_R(crate::FieldReader<bool, bool>);
impl USART3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3IE` writer - USART3IE"]
pub struct USART3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3IE_W<'a> {
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
#[doc = "Field `UART4IE` reader - UART4IE"]
pub struct UART4IE_R(crate::FieldReader<bool, bool>);
impl UART4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4IE` writer - UART4IE"]
pub struct UART4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `UART5IE` reader - UART5IE"]
pub struct UART5IE_R(crate::FieldReader<bool, bool>);
impl UART5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5IE` writer - UART5IE"]
pub struct UART5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5IE_W<'a> {
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
#[doc = "Field `I2C1IE` reader - I2C1IE"]
pub struct I2C1IE_R(crate::FieldReader<bool, bool>);
impl I2C1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1IE` writer - I2C1IE"]
pub struct I2C1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IE_W<'a> {
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
#[doc = "Field `I2C2IE` reader - I2C2IE"]
pub struct I2C2IE_R(crate::FieldReader<bool, bool>);
impl I2C2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2IE` writer - I2C2IE"]
pub struct I2C2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IE_W<'a> {
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
#[doc = "Field `I2C3IE` reader - I2C3IE"]
pub struct I2C3IE_R(crate::FieldReader<bool, bool>);
impl I2C3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3IE` writer - I2C3IE"]
pub struct I2C3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3IE_W<'a> {
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
#[doc = "Field `CRSIE` reader - CRSIE"]
pub struct CRSIE_R(crate::FieldReader<bool, bool>);
impl CRSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSIE` writer - CRSIE"]
pub struct CRSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSIE_W<'a> {
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
#[doc = "Field `DACIE` reader - DACIE"]
pub struct DACIE_R(crate::FieldReader<bool, bool>);
impl DACIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACIE` writer - DACIE"]
pub struct DACIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIE_W<'a> {
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
#[doc = "Field `OPAMPIE` reader - OPAMPIE"]
pub struct OPAMPIE_R(crate::FieldReader<bool, bool>);
impl OPAMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMPIE` writer - OPAMPIE"]
pub struct OPAMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `LPTIM1IE` reader - LPTIM1IE"]
pub struct LPTIM1IE_R(crate::FieldReader<bool, bool>);
impl LPTIM1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1IE` writer - LPTIM1IE"]
pub struct LPTIM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1IE_W<'a> {
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
#[doc = "Field `LPUART1IE` reader - LPUART1IE"]
pub struct LPUART1IE_R(crate::FieldReader<bool, bool>);
impl LPUART1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1IE` writer - LPUART1IE"]
pub struct LPUART1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1IE_W<'a> {
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
#[doc = "Field `I2C4IE` reader - I2C4IE"]
pub struct I2C4IE_R(crate::FieldReader<bool, bool>);
impl I2C4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4IE` writer - I2C4IE"]
pub struct I2C4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4IE_W<'a> {
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
#[doc = "Field `LPTIM2IE` reader - LPTIM2IE"]
pub struct LPTIM2IE_R(crate::FieldReader<bool, bool>);
impl LPTIM2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2IE` writer - LPTIM2IE"]
pub struct LPTIM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2IE_W<'a> {
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
#[doc = "Field `LPTIM3IE` reader - LPTIM3IE"]
pub struct LPTIM3IE_R(crate::FieldReader<bool, bool>);
impl LPTIM3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3IE` writer - LPTIM3IE"]
pub struct LPTIM3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3IE_W<'a> {
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
#[doc = "Field `FDCAN1IE` reader - FDCAN1IE"]
pub struct FDCAN1IE_R(crate::FieldReader<bool, bool>);
impl FDCAN1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCAN1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCAN1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCAN1IE` writer - FDCAN1IE"]
pub struct FDCAN1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1IE_W<'a> {
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
#[doc = "Field `USBFSIE` reader - USBFSIE"]
pub struct USBFSIE_R(crate::FieldReader<bool, bool>);
impl USBFSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFSIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFSIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSIE` writer - USBFSIE"]
pub struct USBFSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSIE_W<'a> {
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
#[doc = "Field `UCPD1IE` reader - UCPD1IE"]
pub struct UCPD1IE_R(crate::FieldReader<bool, bool>);
impl UCPD1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPD1IE` writer - UCPD1IE"]
pub struct UCPD1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1IE_W<'a> {
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
#[doc = "Field `VREFBUFIE` reader - VREFBUFIE"]
pub struct VREFBUFIE_R(crate::FieldReader<bool, bool>);
impl VREFBUFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFBUFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFBUFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFBUFIE` writer - VREFBUFIE"]
pub struct VREFBUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFIE_W<'a> {
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
#[doc = "Field `COMPIE` reader - COMPIE"]
pub struct COMPIE_R(crate::FieldReader<bool, bool>);
impl COMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPIE` writer - COMPIE"]
pub struct COMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPIE_W<'a> {
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
#[doc = "Field `TIM1IE` reader - TIM1IE"]
pub struct TIM1IE_R(crate::FieldReader<bool, bool>);
impl TIM1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1IE` writer - TIM1IE"]
pub struct TIM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SPI1IE` reader - SPI1IE"]
pub struct SPI1IE_R(crate::FieldReader<bool, bool>);
impl SPI1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1IE` writer - SPI1IE"]
pub struct SPI1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&self) -> TIM2IE_R {
        TIM2IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&self) -> TIM3IE_R {
        TIM3IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&self) -> TIM4IE_R {
        TIM4IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&self) -> TIM5IE_R {
        TIM5IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&self) -> TIM6IE_R {
        TIM6IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&self) -> TIM7IE_R {
        TIM7IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&self) -> WWDGIE_R {
        WWDGIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&self) -> IWDGIE_R {
        IWDGIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&self) -> SPI2IE_R {
        SPI2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    pub fn spi3ie(&self) -> SPI3IE_R {
        SPI3IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    pub fn usart2ie(&self) -> USART2IE_R {
        USART2IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    pub fn usart3ie(&self) -> USART3IE_R {
        USART3IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    pub fn uart4ie(&self) -> UART4IE_R {
        UART4IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    pub fn uart5ie(&self) -> UART5IE_R {
        UART5IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    pub fn i2c1ie(&self) -> I2C1IE_R {
        I2C1IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    pub fn i2c2ie(&self) -> I2C2IE_R {
        I2C2IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    pub fn i2c3ie(&self) -> I2C3IE_R {
        I2C3IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    pub fn crsie(&self) -> CRSIE_R {
        CRSIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    pub fn dacie(&self) -> DACIE_R {
        DACIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    pub fn opampie(&self) -> OPAMPIE_R {
        OPAMPIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    pub fn lptim1ie(&self) -> LPTIM1IE_R {
        LPTIM1IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    pub fn lpuart1ie(&self) -> LPUART1IE_R {
        LPUART1IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    pub fn i2c4ie(&self) -> I2C4IE_R {
        I2C4IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    pub fn lptim2ie(&self) -> LPTIM2IE_R {
        LPTIM2IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    pub fn lptim3ie(&self) -> LPTIM3IE_R {
        LPTIM3IE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    pub fn fdcan1ie(&self) -> FDCAN1IE_R {
        FDCAN1IE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    pub fn usbfsie(&self) -> USBFSIE_R {
        USBFSIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    pub fn ucpd1ie(&self) -> UCPD1IE_R {
        UCPD1IE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    pub fn vrefbufie(&self) -> VREFBUFIE_R {
        VREFBUFIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    pub fn compie(&self) -> COMPIE_R {
        COMPIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    pub fn tim1ie(&self) -> TIM1IE_R {
        TIM1IE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    pub fn spi1ie(&self) -> SPI1IE_R {
        SPI1IE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2IE"]
    #[inline(always)]
    pub fn tim2ie(&mut self) -> TIM2IE_W {
        TIM2IE_W { w: self }
    }
    #[doc = "Bit 1 - TIM3IE"]
    #[inline(always)]
    pub fn tim3ie(&mut self) -> TIM3IE_W {
        TIM3IE_W { w: self }
    }
    #[doc = "Bit 2 - TIM4IE"]
    #[inline(always)]
    pub fn tim4ie(&mut self) -> TIM4IE_W {
        TIM4IE_W { w: self }
    }
    #[doc = "Bit 3 - TIM5IE"]
    #[inline(always)]
    pub fn tim5ie(&mut self) -> TIM5IE_W {
        TIM5IE_W { w: self }
    }
    #[doc = "Bit 4 - TIM6IE"]
    #[inline(always)]
    pub fn tim6ie(&mut self) -> TIM6IE_W {
        TIM6IE_W { w: self }
    }
    #[doc = "Bit 5 - TIM7IE"]
    #[inline(always)]
    pub fn tim7ie(&mut self) -> TIM7IE_W {
        TIM7IE_W { w: self }
    }
    #[doc = "Bit 6 - WWDGIE"]
    #[inline(always)]
    pub fn wwdgie(&mut self) -> WWDGIE_W {
        WWDGIE_W { w: self }
    }
    #[doc = "Bit 7 - IWDGIE"]
    #[inline(always)]
    pub fn iwdgie(&mut self) -> IWDGIE_W {
        IWDGIE_W { w: self }
    }
    #[doc = "Bit 8 - SPI2IE"]
    #[inline(always)]
    pub fn spi2ie(&mut self) -> SPI2IE_W {
        SPI2IE_W { w: self }
    }
    #[doc = "Bit 9 - SPI3IE"]
    #[inline(always)]
    pub fn spi3ie(&mut self) -> SPI3IE_W {
        SPI3IE_W { w: self }
    }
    #[doc = "Bit 10 - USART2IE"]
    #[inline(always)]
    pub fn usart2ie(&mut self) -> USART2IE_W {
        USART2IE_W { w: self }
    }
    #[doc = "Bit 11 - USART3IE"]
    #[inline(always)]
    pub fn usart3ie(&mut self) -> USART3IE_W {
        USART3IE_W { w: self }
    }
    #[doc = "Bit 12 - UART4IE"]
    #[inline(always)]
    pub fn uart4ie(&mut self) -> UART4IE_W {
        UART4IE_W { w: self }
    }
    #[doc = "Bit 13 - UART5IE"]
    #[inline(always)]
    pub fn uart5ie(&mut self) -> UART5IE_W {
        UART5IE_W { w: self }
    }
    #[doc = "Bit 14 - I2C1IE"]
    #[inline(always)]
    pub fn i2c1ie(&mut self) -> I2C1IE_W {
        I2C1IE_W { w: self }
    }
    #[doc = "Bit 15 - I2C2IE"]
    #[inline(always)]
    pub fn i2c2ie(&mut self) -> I2C2IE_W {
        I2C2IE_W { w: self }
    }
    #[doc = "Bit 16 - I2C3IE"]
    #[inline(always)]
    pub fn i2c3ie(&mut self) -> I2C3IE_W {
        I2C3IE_W { w: self }
    }
    #[doc = "Bit 17 - CRSIE"]
    #[inline(always)]
    pub fn crsie(&mut self) -> CRSIE_W {
        CRSIE_W { w: self }
    }
    #[doc = "Bit 18 - DACIE"]
    #[inline(always)]
    pub fn dacie(&mut self) -> DACIE_W {
        DACIE_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPIE"]
    #[inline(always)]
    pub fn opampie(&mut self) -> OPAMPIE_W {
        OPAMPIE_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1IE"]
    #[inline(always)]
    pub fn lptim1ie(&mut self) -> LPTIM1IE_W {
        LPTIM1IE_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1IE"]
    #[inline(always)]
    pub fn lpuart1ie(&mut self) -> LPUART1IE_W {
        LPUART1IE_W { w: self }
    }
    #[doc = "Bit 22 - I2C4IE"]
    #[inline(always)]
    pub fn i2c4ie(&mut self) -> I2C4IE_W {
        I2C4IE_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2IE"]
    #[inline(always)]
    pub fn lptim2ie(&mut self) -> LPTIM2IE_W {
        LPTIM2IE_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3IE"]
    #[inline(always)]
    pub fn lptim3ie(&mut self) -> LPTIM3IE_W {
        LPTIM3IE_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1IE"]
    #[inline(always)]
    pub fn fdcan1ie(&mut self) -> FDCAN1IE_W {
        FDCAN1IE_W { w: self }
    }
    #[doc = "Bit 26 - USBFSIE"]
    #[inline(always)]
    pub fn usbfsie(&mut self) -> USBFSIE_W {
        USBFSIE_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1IE"]
    #[inline(always)]
    pub fn ucpd1ie(&mut self) -> UCPD1IE_W {
        UCPD1IE_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFIE"]
    #[inline(always)]
    pub fn vrefbufie(&mut self) -> VREFBUFIE_W {
        VREFBUFIE_W { w: self }
    }
    #[doc = "Bit 29 - COMPIE"]
    #[inline(always)]
    pub fn compie(&mut self) -> COMPIE_W {
        COMPIE_W { w: self }
    }
    #[doc = "Bit 30 - TIM1IE"]
    #[inline(always)]
    pub fn tim1ie(&mut self) -> TIM1IE_W {
        TIM1IE_W { w: self }
    }
    #[doc = "Bit 31 - SPI1IE"]
    #[inline(always)]
    pub fn spi1ie(&mut self) -> SPI1IE_W {
        SPI1IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
