#[doc = "Register `RCC_MP_APB1LPENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB1LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB1LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB1LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB1LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB1LPENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB1LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB1LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB1LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB1LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2LPEN"]
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
#[doc = "Field `TIM2LPEN` writer - TIM2LPEN"]
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
#[doc = "Field `TIM3LPEN` reader - TIM3LPEN"]
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
#[doc = "Field `TIM3LPEN` writer - TIM3LPEN"]
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
#[doc = "Field `TIM4LPEN` reader - TIM4LPEN"]
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
#[doc = "Field `TIM4LPEN` writer - TIM4LPEN"]
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
#[doc = "Field `TIM5LPEN` reader - TIM5LPEN"]
pub struct TIM5LPEN_R(crate::FieldReader<bool, bool>);
impl TIM5LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5LPEN` writer - TIM5LPEN"]
pub struct TIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5LPEN_W<'a> {
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
#[doc = "Field `TIM6LPEN` reader - TIM6LPEN"]
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
#[doc = "Field `TIM6LPEN` writer - TIM6LPEN"]
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
#[doc = "Field `TIM7LPEN` reader - TIM7LPEN"]
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
#[doc = "Field `TIM7LPEN` writer - TIM7LPEN"]
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
#[doc = "Field `TIM12LPEN` reader - TIM12LPEN"]
pub struct TIM12LPEN_R(crate::FieldReader<bool, bool>);
impl TIM12LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM12LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM12LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM12LPEN` writer - TIM12LPEN"]
pub struct TIM12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12LPEN_W<'a> {
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
#[doc = "Field `TIM13LPEN` reader - TIM13LPEN"]
pub struct TIM13LPEN_R(crate::FieldReader<bool, bool>);
impl TIM13LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM13LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM13LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM13LPEN` writer - TIM13LPEN"]
pub struct TIM13LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13LPEN_W<'a> {
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
#[doc = "Field `TIM14LPEN` reader - TIM14LPEN"]
pub struct TIM14LPEN_R(crate::FieldReader<bool, bool>);
impl TIM14LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM14LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM14LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM14LPEN` writer - TIM14LPEN"]
pub struct TIM14LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14LPEN_W<'a> {
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
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1LPEN"]
pub struct LPTIM1LPEN_R(crate::FieldReader<bool, bool>);
impl LPTIM1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1LPEN"]
pub struct LPTIM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1LPEN_W<'a> {
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
#[doc = "Field `SPI2LPEN` reader - SPI2LPEN"]
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
#[doc = "Field `SPI2LPEN` writer - SPI2LPEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SPI3LPEN` reader - SPI3LPEN"]
pub struct SPI3LPEN_R(crate::FieldReader<bool, bool>);
impl SPI3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3LPEN` writer - SPI3LPEN"]
pub struct SPI3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3LPEN_W<'a> {
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
#[doc = "Field `USART2LPEN` reader - USART2LPEN"]
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
#[doc = "Field `USART2LPEN` writer - USART2LPEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `USART3LPEN` reader - USART3LPEN"]
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
#[doc = "Field `USART3LPEN` writer - USART3LPEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `UART4LPEN` reader - UART4LPEN"]
pub struct UART4LPEN_R(crate::FieldReader<bool, bool>);
impl UART4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4LPEN` writer - UART4LPEN"]
pub struct UART4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4LPEN_W<'a> {
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
#[doc = "Field `UART5LPEN` reader - UART5LPEN"]
pub struct UART5LPEN_R(crate::FieldReader<bool, bool>);
impl UART5LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART5LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5LPEN` writer - UART5LPEN"]
pub struct UART5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5LPEN_W<'a> {
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
#[doc = "Field `UART7LPEN` reader - UART7LPEN"]
pub struct UART7LPEN_R(crate::FieldReader<bool, bool>);
impl UART7LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART7LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART7LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7LPEN` writer - UART7LPEN"]
pub struct UART7LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7LPEN_W<'a> {
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
#[doc = "Field `UART8LPEN` reader - UART8LPEN"]
pub struct UART8LPEN_R(crate::FieldReader<bool, bool>);
impl UART8LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART8LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART8LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART8LPEN` writer - UART8LPEN"]
pub struct UART8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8LPEN_W<'a> {
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
#[doc = "Field `I2C1LPEN` reader - I2C1LPEN"]
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
#[doc = "Field `I2C1LPEN` writer - I2C1LPEN"]
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
#[doc = "Field `I2C2LPEN` reader - I2C2LPEN"]
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
#[doc = "Field `I2C2LPEN` writer - I2C2LPEN"]
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
#[doc = "Field `I2C3LPEN` reader - I2C3LPEN"]
pub struct I2C3LPEN_R(crate::FieldReader<bool, bool>);
impl I2C3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3LPEN` writer - I2C3LPEN"]
pub struct I2C3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3LPEN_W<'a> {
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
#[doc = "Field `I2C5LPEN` reader - I2C5LPEN"]
pub struct I2C5LPEN_R(crate::FieldReader<bool, bool>);
impl I2C5LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C5LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C5LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C5LPEN` writer - I2C5LPEN"]
pub struct I2C5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C5LPEN_W<'a> {
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
#[doc = "Field `SPDIFLPEN` reader - SPDIFLPEN"]
pub struct SPDIFLPEN_R(crate::FieldReader<bool, bool>);
impl SPDIFLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDIFLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDIFLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIFLPEN` writer - SPDIFLPEN"]
pub struct SPDIFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFLPEN_W<'a> {
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
#[doc = "Field `CECLPEN` reader - CECLPEN"]
pub struct CECLPEN_R(crate::FieldReader<bool, bool>);
impl CECLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECLPEN` writer - CECLPEN"]
pub struct CECLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECLPEN_W<'a> {
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
#[doc = "Field `DAC12LPEN` reader - DAC12LPEN"]
pub struct DAC12LPEN_R(crate::FieldReader<bool, bool>);
impl DAC12LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC12LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC12LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC12LPEN` writer - DAC12LPEN"]
pub struct DAC12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC12LPEN_W<'a> {
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
#[doc = "Field `MDIOSLPEN` reader - MDIOSLPEN"]
pub struct MDIOSLPEN_R(crate::FieldReader<bool, bool>);
impl MDIOSLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDIOSLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDIOSLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIOSLPEN` writer - MDIOSLPEN"]
pub struct MDIOSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOSLPEN_W<'a> {
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
    #[doc = "Bit 0 - TIM2LPEN"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3LPEN"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4LPEN"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5LPEN"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6LPEN"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7LPEN"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12LPEN"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13LPEN"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14LPEN"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1LPEN"]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPI2LPEN"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI3LPEN"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART2LPEN"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USART3LPEN"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART4LPEN"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - UART5LPEN"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - UART7LPEN"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART8LPEN"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1LPEN"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2LPEN"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3LPEN"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - I2C5LPEN"]
    #[inline(always)]
    pub fn i2c5lpen(&self) -> I2C5LPEN_R {
        I2C5LPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SPDIFLPEN"]
    #[inline(always)]
    pub fn spdiflpen(&self) -> SPDIFLPEN_R {
        SPDIFLPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CECLPEN"]
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC12LPEN"]
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - MDIOSLPEN"]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MDIOSLPEN_R {
        MDIOSLPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2LPEN"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W {
        TIM2LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM3LPEN"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W {
        TIM3LPEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM4LPEN"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W {
        TIM4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - TIM5LPEN"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W {
        TIM5LPEN_W { w: self }
    }
    #[doc = "Bit 4 - TIM6LPEN"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W {
        TIM6LPEN_W { w: self }
    }
    #[doc = "Bit 5 - TIM7LPEN"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W {
        TIM7LPEN_W { w: self }
    }
    #[doc = "Bit 6 - TIM12LPEN"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W {
        TIM12LPEN_W { w: self }
    }
    #[doc = "Bit 7 - TIM13LPEN"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W {
        TIM13LPEN_W { w: self }
    }
    #[doc = "Bit 8 - TIM14LPEN"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W {
        TIM14LPEN_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1LPEN"]
    #[inline(always)]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W {
        LPTIM1LPEN_W { w: self }
    }
    #[doc = "Bit 11 - SPI2LPEN"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W {
        SPI2LPEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI3LPEN"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W {
        SPI3LPEN_W { w: self }
    }
    #[doc = "Bit 14 - USART2LPEN"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W {
        USART2LPEN_W { w: self }
    }
    #[doc = "Bit 15 - USART3LPEN"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W {
        USART3LPEN_W { w: self }
    }
    #[doc = "Bit 16 - UART4LPEN"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W {
        UART4LPEN_W { w: self }
    }
    #[doc = "Bit 17 - UART5LPEN"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W {
        UART5LPEN_W { w: self }
    }
    #[doc = "Bit 18 - UART7LPEN"]
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W {
        UART7LPEN_W { w: self }
    }
    #[doc = "Bit 19 - UART8LPEN"]
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W {
        UART8LPEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1LPEN"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W {
        I2C1LPEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2LPEN"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W {
        I2C2LPEN_W { w: self }
    }
    #[doc = "Bit 23 - I2C3LPEN"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W {
        I2C3LPEN_W { w: self }
    }
    #[doc = "Bit 24 - I2C5LPEN"]
    #[inline(always)]
    pub fn i2c5lpen(&mut self) -> I2C5LPEN_W {
        I2C5LPEN_W { w: self }
    }
    #[doc = "Bit 26 - SPDIFLPEN"]
    #[inline(always)]
    pub fn spdiflpen(&mut self) -> SPDIFLPEN_W {
        SPDIFLPEN_W { w: self }
    }
    #[doc = "Bit 27 - CECLPEN"]
    #[inline(always)]
    pub fn ceclpen(&mut self) -> CECLPEN_W {
        CECLPEN_W { w: self }
    }
    #[doc = "Bit 29 - DAC12LPEN"]
    #[inline(always)]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W {
        DAC12LPEN_W { w: self }
    }
    #[doc = "Bit 31 - MDIOSLPEN"]
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MDIOSLPEN_W {
        MDIOSLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb1lpensetr](index.html) module"]
pub struct RCC_MP_APB1LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB1LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb1lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB1LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb1lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB1LPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB1LPENSETR to value 0xadef_dbff"]
impl crate::Resettable for RCC_MP_APB1LPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xadef_dbff
    }
}
