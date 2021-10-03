#[doc = "Register `TZSC_PRIVCFGR1` reader"]
pub struct R(crate::R<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_PRIVCFGR1` writer"]
pub struct W(crate::W<TZSC_PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_PRIVCFGR1_SPEC>;
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
impl From<crate::W<TZSC_PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2PRIV` reader - TIM2PRIV"]
pub struct TIM2PRIV_R(crate::FieldReader<bool, bool>);
impl TIM2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2PRIV` writer - TIM2PRIV"]
pub struct TIM2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2PRIV_W<'a> {
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
#[doc = "Field `TIM3PRIV` reader - TIM3PRIV"]
pub struct TIM3PRIV_R(crate::FieldReader<bool, bool>);
impl TIM3PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3PRIV` writer - TIM3PRIV"]
pub struct TIM3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3PRIV_W<'a> {
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
#[doc = "Field `TIM4PRIV` reader - TIM4PRIV"]
pub struct TIM4PRIV_R(crate::FieldReader<bool, bool>);
impl TIM4PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4PRIV` writer - TIM4PRIV"]
pub struct TIM4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4PRIV_W<'a> {
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
#[doc = "Field `TIM5PRIV` reader - TIM5PRIV"]
pub struct TIM5PRIV_R(crate::FieldReader<bool, bool>);
impl TIM5PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5PRIV` writer - TIM5PRIV"]
pub struct TIM5PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5PRIV_W<'a> {
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
#[doc = "Field `TIM6PRIV` reader - TIM6PRIV"]
pub struct TIM6PRIV_R(crate::FieldReader<bool, bool>);
impl TIM6PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6PRIV` writer - TIM6PRIV"]
pub struct TIM6PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6PRIV_W<'a> {
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
#[doc = "Field `TIM7PRIV` reader - TIM7PRIV"]
pub struct TIM7PRIV_R(crate::FieldReader<bool, bool>);
impl TIM7PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7PRIV` writer - TIM7PRIV"]
pub struct TIM7PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7PRIV_W<'a> {
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
#[doc = "Field `WWDGPRIV` reader - WWDGPRIV"]
pub struct WWDGPRIV_R(crate::FieldReader<bool, bool>);
impl WWDGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGPRIV` writer - WWDGPRIV"]
pub struct WWDGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGPRIV_W<'a> {
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
#[doc = "Field `IWDGPRIV` reader - IWDGPRIV"]
pub struct IWDGPRIV_R(crate::FieldReader<bool, bool>);
impl IWDGPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWDGPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IWDGPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWDGPRIV` writer - IWDGPRIV"]
pub struct IWDGPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGPRIV_W<'a> {
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
#[doc = "Field `SPI2PRIV` reader - SPI2PRIV"]
pub struct SPI2PRIV_R(crate::FieldReader<bool, bool>);
impl SPI2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2PRIV` writer - SPI2PRIV"]
pub struct SPI2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2PRIV_W<'a> {
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
#[doc = "Field `SPI3PRIV` reader - SPI3PRIV"]
pub struct SPI3PRIV_R(crate::FieldReader<bool, bool>);
impl SPI3PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3PRIV` writer - SPI3PRIV"]
pub struct SPI3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3PRIV_W<'a> {
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
#[doc = "Field `USART2PRIV` reader - USART2PRIV"]
pub struct USART2PRIV_R(crate::FieldReader<bool, bool>);
impl USART2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2PRIV` writer - USART2PRIV"]
pub struct USART2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2PRIV_W<'a> {
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
#[doc = "Field `USART3PRIV` reader - USART3PRIV"]
pub struct USART3PRIV_R(crate::FieldReader<bool, bool>);
impl USART3PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3PRIV` writer - USART3PRIV"]
pub struct USART3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3PRIV_W<'a> {
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
#[doc = "Field `UART4PRIV` reader - UART4PRIV"]
pub struct UART4PRIV_R(crate::FieldReader<bool, bool>);
impl UART4PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4PRIV` writer - UART4PRIV"]
pub struct UART4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4PRIV_W<'a> {
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
#[doc = "Field `UART5PRIV` reader - UART5PRIV"]
pub struct UART5PRIV_R(crate::FieldReader<bool, bool>);
impl UART5PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5PRIV` writer - UART5PRIV"]
pub struct UART5PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5PRIV_W<'a> {
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
#[doc = "Field `I2C1PRIV` reader - I2C1PRIV"]
pub struct I2C1PRIV_R(crate::FieldReader<bool, bool>);
impl I2C1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1PRIV` writer - I2C1PRIV"]
pub struct I2C1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1PRIV_W<'a> {
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
#[doc = "Field `I2C2PRIV` reader - I2C2PRIV"]
pub struct I2C2PRIV_R(crate::FieldReader<bool, bool>);
impl I2C2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2PRIV` writer - I2C2PRIV"]
pub struct I2C2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2PRIV_W<'a> {
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
#[doc = "Field `I2C3PRIV` reader - I2C3PRIV"]
pub struct I2C3PRIV_R(crate::FieldReader<bool, bool>);
impl I2C3PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3PRIV` writer - I2C3PRIV"]
pub struct I2C3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3PRIV_W<'a> {
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
#[doc = "Field `CRSPRIV` reader - CRSPRIV"]
pub struct CRSPRIV_R(crate::FieldReader<bool, bool>);
impl CRSPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSPRIV` writer - CRSPRIV"]
pub struct CRSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSPRIV_W<'a> {
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
#[doc = "Field `DACPRIV` reader - DACPRIV"]
pub struct DACPRIV_R(crate::FieldReader<bool, bool>);
impl DACPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACPRIV` writer - DACPRIV"]
pub struct DACPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DACPRIV_W<'a> {
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
#[doc = "Field `OPAMPPRIV` reader - OPAMPPRIV"]
pub struct OPAMPPRIV_R(crate::FieldReader<bool, bool>);
impl OPAMPPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPAMPPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPAMPPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPAMPPRIV` writer - OPAMPPRIV"]
pub struct OPAMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAMPPRIV_W<'a> {
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
#[doc = "Field `LPTIM1PRIV` reader - LPTIM1PRIV"]
pub struct LPTIM1PRIV_R(crate::FieldReader<bool, bool>);
impl LPTIM1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1PRIV` writer - LPTIM1PRIV"]
pub struct LPTIM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1PRIV_W<'a> {
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
#[doc = "Field `LPUART1PRIV` reader - LPUART1PRIV"]
pub struct LPUART1PRIV_R(crate::FieldReader<bool, bool>);
impl LPUART1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1PRIV` writer - LPUART1PRIV"]
pub struct LPUART1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1PRIV_W<'a> {
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
#[doc = "Field `I2C4PRIV` reader - I2C4PRIV"]
pub struct I2C4PRIV_R(crate::FieldReader<bool, bool>);
impl I2C4PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4PRIV` writer - I2C4PRIV"]
pub struct I2C4PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4PRIV_W<'a> {
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
#[doc = "Field `LPTIM2PRIV` reader - LPTIM2PRIV"]
pub struct LPTIM2PRIV_R(crate::FieldReader<bool, bool>);
impl LPTIM2PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2PRIV` writer - LPTIM2PRIV"]
pub struct LPTIM2PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2PRIV_W<'a> {
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
#[doc = "Field `LPTIM3PRIV` reader - LPTIM3PRIV"]
pub struct LPTIM3PRIV_R(crate::FieldReader<bool, bool>);
impl LPTIM3PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM3PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3PRIV` writer - LPTIM3PRIV"]
pub struct LPTIM3PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3PRIV_W<'a> {
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
#[doc = "Field `FDCAN1PRIV` reader - FDCAN1PRIV"]
pub struct FDCAN1PRIV_R(crate::FieldReader<bool, bool>);
impl FDCAN1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCAN1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCAN1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCAN1PRIV` writer - FDCAN1PRIV"]
pub struct FDCAN1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1PRIV_W<'a> {
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
#[doc = "Field `USBFSPRIV` reader - USBFSPRIV"]
pub struct USBFSPRIV_R(crate::FieldReader<bool, bool>);
impl USBFSPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFSPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFSPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFSPRIV` writer - USBFSPRIV"]
pub struct USBFSPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSPRIV_W<'a> {
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
#[doc = "Field `UCPD1PRIV` reader - UCPD1PRIV"]
pub struct UCPD1PRIV_R(crate::FieldReader<bool, bool>);
impl UCPD1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPD1PRIV` writer - UCPD1PRIV"]
pub struct UCPD1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1PRIV_W<'a> {
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
#[doc = "Field `VREFBUFPRIV` reader - VREFBUFPRIV"]
pub struct VREFBUFPRIV_R(crate::FieldReader<bool, bool>);
impl VREFBUFPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFBUFPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFBUFPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFBUFPRIV` writer - VREFBUFPRIV"]
pub struct VREFBUFPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUFPRIV_W<'a> {
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
#[doc = "Field `COMPPRIV` reader - COMPPRIV"]
pub struct COMPPRIV_R(crate::FieldReader<bool, bool>);
impl COMPPRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPPRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPPRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPPRIV` writer - COMPPRIV"]
pub struct COMPPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPPRIV_W<'a> {
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
#[doc = "Field `TIM1PRIV` reader - TIM1PRIV"]
pub struct TIM1PRIV_R(crate::FieldReader<bool, bool>);
impl TIM1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1PRIV` writer - TIM1PRIV"]
pub struct TIM1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1PRIV_W<'a> {
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
#[doc = "Field `SPI1PRIV` reader - SPI1PRIV"]
pub struct SPI1PRIV_R(crate::FieldReader<bool, bool>);
impl SPI1PRIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1PRIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1PRIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1PRIV` writer - SPI1PRIV"]
pub struct SPI1PRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1PRIV_W<'a> {
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
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    pub fn tim2priv(&self) -> TIM2PRIV_R {
        TIM2PRIV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    pub fn tim3priv(&self) -> TIM3PRIV_R {
        TIM3PRIV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    pub fn tim4priv(&self) -> TIM4PRIV_R {
        TIM4PRIV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    pub fn tim5priv(&self) -> TIM5PRIV_R {
        TIM5PRIV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    pub fn tim6priv(&self) -> TIM6PRIV_R {
        TIM6PRIV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    pub fn tim7priv(&self) -> TIM7PRIV_R {
        TIM7PRIV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    pub fn wwdgpriv(&self) -> WWDGPRIV_R {
        WWDGPRIV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    pub fn iwdgpriv(&self) -> IWDGPRIV_R {
        IWDGPRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    pub fn spi2priv(&self) -> SPI2PRIV_R {
        SPI2PRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    pub fn spi3priv(&self) -> SPI3PRIV_R {
        SPI3PRIV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    pub fn usart2priv(&self) -> USART2PRIV_R {
        USART2PRIV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    pub fn usart3priv(&self) -> USART3PRIV_R {
        USART3PRIV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    pub fn uart4priv(&self) -> UART4PRIV_R {
        UART4PRIV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    pub fn uart5priv(&self) -> UART5PRIV_R {
        UART5PRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    pub fn i2c1priv(&self) -> I2C1PRIV_R {
        I2C1PRIV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    pub fn i2c2priv(&self) -> I2C2PRIV_R {
        I2C2PRIV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    pub fn i2c3priv(&self) -> I2C3PRIV_R {
        I2C3PRIV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    pub fn crspriv(&self) -> CRSPRIV_R {
        CRSPRIV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    pub fn dacpriv(&self) -> DACPRIV_R {
        DACPRIV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    pub fn opamppriv(&self) -> OPAMPPRIV_R {
        OPAMPPRIV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    pub fn lptim1priv(&self) -> LPTIM1PRIV_R {
        LPTIM1PRIV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    pub fn lpuart1priv(&self) -> LPUART1PRIV_R {
        LPUART1PRIV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    pub fn i2c4priv(&self) -> I2C4PRIV_R {
        I2C4PRIV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    pub fn lptim2priv(&self) -> LPTIM2PRIV_R {
        LPTIM2PRIV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    pub fn lptim3priv(&self) -> LPTIM3PRIV_R {
        LPTIM3PRIV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    pub fn fdcan1priv(&self) -> FDCAN1PRIV_R {
        FDCAN1PRIV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    pub fn usbfspriv(&self) -> USBFSPRIV_R {
        USBFSPRIV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    pub fn ucpd1priv(&self) -> UCPD1PRIV_R {
        UCPD1PRIV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    pub fn vrefbufpriv(&self) -> VREFBUFPRIV_R {
        VREFBUFPRIV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    pub fn comppriv(&self) -> COMPPRIV_R {
        COMPPRIV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    pub fn tim1priv(&self) -> TIM1PRIV_R {
        TIM1PRIV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&self) -> SPI1PRIV_R {
        SPI1PRIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2PRIV"]
    #[inline(always)]
    pub fn tim2priv(&mut self) -> TIM2PRIV_W {
        TIM2PRIV_W { w: self }
    }
    #[doc = "Bit 1 - TIM3PRIV"]
    #[inline(always)]
    pub fn tim3priv(&mut self) -> TIM3PRIV_W {
        TIM3PRIV_W { w: self }
    }
    #[doc = "Bit 2 - TIM4PRIV"]
    #[inline(always)]
    pub fn tim4priv(&mut self) -> TIM4PRIV_W {
        TIM4PRIV_W { w: self }
    }
    #[doc = "Bit 3 - TIM5PRIV"]
    #[inline(always)]
    pub fn tim5priv(&mut self) -> TIM5PRIV_W {
        TIM5PRIV_W { w: self }
    }
    #[doc = "Bit 4 - TIM6PRIV"]
    #[inline(always)]
    pub fn tim6priv(&mut self) -> TIM6PRIV_W {
        TIM6PRIV_W { w: self }
    }
    #[doc = "Bit 5 - TIM7PRIV"]
    #[inline(always)]
    pub fn tim7priv(&mut self) -> TIM7PRIV_W {
        TIM7PRIV_W { w: self }
    }
    #[doc = "Bit 6 - WWDGPRIV"]
    #[inline(always)]
    pub fn wwdgpriv(&mut self) -> WWDGPRIV_W {
        WWDGPRIV_W { w: self }
    }
    #[doc = "Bit 7 - IWDGPRIV"]
    #[inline(always)]
    pub fn iwdgpriv(&mut self) -> IWDGPRIV_W {
        IWDGPRIV_W { w: self }
    }
    #[doc = "Bit 8 - SPI2PRIV"]
    #[inline(always)]
    pub fn spi2priv(&mut self) -> SPI2PRIV_W {
        SPI2PRIV_W { w: self }
    }
    #[doc = "Bit 9 - SPI3PRIV"]
    #[inline(always)]
    pub fn spi3priv(&mut self) -> SPI3PRIV_W {
        SPI3PRIV_W { w: self }
    }
    #[doc = "Bit 10 - USART2PRIV"]
    #[inline(always)]
    pub fn usart2priv(&mut self) -> USART2PRIV_W {
        USART2PRIV_W { w: self }
    }
    #[doc = "Bit 11 - USART3PRIV"]
    #[inline(always)]
    pub fn usart3priv(&mut self) -> USART3PRIV_W {
        USART3PRIV_W { w: self }
    }
    #[doc = "Bit 12 - UART4PRIV"]
    #[inline(always)]
    pub fn uart4priv(&mut self) -> UART4PRIV_W {
        UART4PRIV_W { w: self }
    }
    #[doc = "Bit 13 - UART5PRIV"]
    #[inline(always)]
    pub fn uart5priv(&mut self) -> UART5PRIV_W {
        UART5PRIV_W { w: self }
    }
    #[doc = "Bit 14 - I2C1PRIV"]
    #[inline(always)]
    pub fn i2c1priv(&mut self) -> I2C1PRIV_W {
        I2C1PRIV_W { w: self }
    }
    #[doc = "Bit 15 - I2C2PRIV"]
    #[inline(always)]
    pub fn i2c2priv(&mut self) -> I2C2PRIV_W {
        I2C2PRIV_W { w: self }
    }
    #[doc = "Bit 16 - I2C3PRIV"]
    #[inline(always)]
    pub fn i2c3priv(&mut self) -> I2C3PRIV_W {
        I2C3PRIV_W { w: self }
    }
    #[doc = "Bit 17 - CRSPRIV"]
    #[inline(always)]
    pub fn crspriv(&mut self) -> CRSPRIV_W {
        CRSPRIV_W { w: self }
    }
    #[doc = "Bit 18 - DACPRIV"]
    #[inline(always)]
    pub fn dacpriv(&mut self) -> DACPRIV_W {
        DACPRIV_W { w: self }
    }
    #[doc = "Bit 19 - OPAMPPRIV"]
    #[inline(always)]
    pub fn opamppriv(&mut self) -> OPAMPPRIV_W {
        OPAMPPRIV_W { w: self }
    }
    #[doc = "Bit 20 - LPTIM1PRIV"]
    #[inline(always)]
    pub fn lptim1priv(&mut self) -> LPTIM1PRIV_W {
        LPTIM1PRIV_W { w: self }
    }
    #[doc = "Bit 21 - LPUART1PRIV"]
    #[inline(always)]
    pub fn lpuart1priv(&mut self) -> LPUART1PRIV_W {
        LPUART1PRIV_W { w: self }
    }
    #[doc = "Bit 22 - I2C4PRIV"]
    #[inline(always)]
    pub fn i2c4priv(&mut self) -> I2C4PRIV_W {
        I2C4PRIV_W { w: self }
    }
    #[doc = "Bit 23 - LPTIM2PRIV"]
    #[inline(always)]
    pub fn lptim2priv(&mut self) -> LPTIM2PRIV_W {
        LPTIM2PRIV_W { w: self }
    }
    #[doc = "Bit 24 - LPTIM3PRIV"]
    #[inline(always)]
    pub fn lptim3priv(&mut self) -> LPTIM3PRIV_W {
        LPTIM3PRIV_W { w: self }
    }
    #[doc = "Bit 25 - FDCAN1PRIV"]
    #[inline(always)]
    pub fn fdcan1priv(&mut self) -> FDCAN1PRIV_W {
        FDCAN1PRIV_W { w: self }
    }
    #[doc = "Bit 26 - USBFSPRIV"]
    #[inline(always)]
    pub fn usbfspriv(&mut self) -> USBFSPRIV_W {
        USBFSPRIV_W { w: self }
    }
    #[doc = "Bit 27 - UCPD1PRIV"]
    #[inline(always)]
    pub fn ucpd1priv(&mut self) -> UCPD1PRIV_W {
        UCPD1PRIV_W { w: self }
    }
    #[doc = "Bit 28 - VREFBUFPRIV"]
    #[inline(always)]
    pub fn vrefbufpriv(&mut self) -> VREFBUFPRIV_W {
        VREFBUFPRIV_W { w: self }
    }
    #[doc = "Bit 29 - COMPPRIV"]
    #[inline(always)]
    pub fn comppriv(&mut self) -> COMPPRIV_W {
        COMPPRIV_W { w: self }
    }
    #[doc = "Bit 30 - TIM1PRIV"]
    #[inline(always)]
    pub fn tim1priv(&mut self) -> TIM1PRIV_W {
        TIM1PRIV_W { w: self }
    }
    #[doc = "Bit 31 - SPI1PRIV"]
    #[inline(always)]
    pub fn spi1priv(&mut self) -> SPI1PRIV_W {
        SPI1PRIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC privilege configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_privcfgr1](index.html) module"]
pub struct TZSC_PRIVCFGR1_SPEC;
impl crate::RegisterSpec for TZSC_PRIVCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_privcfgr1::R](R) reader structure"]
impl crate::Readable for TZSC_PRIVCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_privcfgr1::W](W) writer structure"]
impl crate::Writable for TZSC_PRIVCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_PRIVCFGR1 to value 0"]
impl crate::Resettable for TZSC_PRIVCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
