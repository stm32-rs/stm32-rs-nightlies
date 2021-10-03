#[doc = "Register `RCC_MP_APB2LPENCLRR` reader"]
pub struct R(crate::R<RCC_MP_APB2LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB2LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB2LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB2LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB2LPENCLRR` writer"]
pub struct W(crate::W<RCC_MP_APB2LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB2LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MP_APB2LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB2LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1LPEN"]
pub struct TIM1LPEN_R(crate::FieldReader<bool, bool>);
impl TIM1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1LPEN"]
pub struct TIM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1LPEN_W<'a> {
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
#[doc = "Field `TIM8LPEN` reader - TIM8LPEN"]
pub struct TIM8LPEN_R(crate::FieldReader<bool, bool>);
impl TIM8LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8LPEN` writer - TIM8LPEN"]
pub struct TIM8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8LPEN_W<'a> {
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
#[doc = "Field `TIM15LPEN` reader - TIM15LPEN"]
pub struct TIM15LPEN_R(crate::FieldReader<bool, bool>);
impl TIM15LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15LPEN` writer - TIM15LPEN"]
pub struct TIM15LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15LPEN_W<'a> {
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
#[doc = "Field `TIM16LPEN` reader - TIM16LPEN"]
pub struct TIM16LPEN_R(crate::FieldReader<bool, bool>);
impl TIM16LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16LPEN` writer - TIM16LPEN"]
pub struct TIM16LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16LPEN_W<'a> {
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
#[doc = "Field `TIM17LPEN` reader - TIM17LPEN"]
pub struct TIM17LPEN_R(crate::FieldReader<bool, bool>);
impl TIM17LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17LPEN` writer - TIM17LPEN"]
pub struct TIM17LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17LPEN_W<'a> {
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
#[doc = "Field `SPI1LPEN` reader - SPI1LPEN"]
pub struct SPI1LPEN_R(crate::FieldReader<bool, bool>);
impl SPI1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1LPEN` writer - SPI1LPEN"]
pub struct SPI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1LPEN_W<'a> {
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
#[doc = "Field `SPI4LPEN` reader - SPI4LPEN"]
pub struct SPI4LPEN_R(crate::FieldReader<bool, bool>);
impl SPI4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI4LPEN` writer - SPI4LPEN"]
pub struct SPI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4LPEN_W<'a> {
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
#[doc = "Field `SPI5LPEN` reader - SPI5LPEN"]
pub struct SPI5LPEN_R(crate::FieldReader<bool, bool>);
impl SPI5LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI5LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI5LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI5LPEN` writer - SPI5LPEN"]
pub struct SPI5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5LPEN_W<'a> {
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
#[doc = "Field `USART6LPEN` reader - USART6LPEN"]
pub struct USART6LPEN_R(crate::FieldReader<bool, bool>);
impl USART6LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART6LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART6LPEN` writer - USART6LPEN"]
pub struct USART6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6LPEN_W<'a> {
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
#[doc = "Field `SAI1LPEN` reader - SAI1LPEN"]
pub struct SAI1LPEN_R(crate::FieldReader<bool, bool>);
impl SAI1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1LPEN` writer - SAI1LPEN"]
pub struct SAI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1LPEN_W<'a> {
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
#[doc = "Field `SAI2LPEN` reader - SAI2LPEN"]
pub struct SAI2LPEN_R(crate::FieldReader<bool, bool>);
impl SAI2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2LPEN` writer - SAI2LPEN"]
pub struct SAI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2LPEN_W<'a> {
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
#[doc = "Field `SAI3LPEN` reader - SAI3LPEN"]
pub struct SAI3LPEN_R(crate::FieldReader<bool, bool>);
impl SAI3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI3LPEN` writer - SAI3LPEN"]
pub struct SAI3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3LPEN_W<'a> {
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
#[doc = "Field `DFSDMLPEN` reader - DFSDMLPEN"]
pub struct DFSDMLPEN_R(crate::FieldReader<bool, bool>);
impl DFSDMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMLPEN` writer - DFSDMLPEN"]
pub struct DFSDMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMLPEN_W<'a> {
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
#[doc = "Field `ADFSDMLPEN` reader - ADFSDMLPEN"]
pub struct ADFSDMLPEN_R(crate::FieldReader<bool, bool>);
impl ADFSDMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADFSDMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFSDMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFSDMLPEN` writer - ADFSDMLPEN"]
pub struct ADFSDMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMLPEN_W<'a> {
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
#[doc = "Field `FDCANLPEN` reader - FDCANLPEN"]
pub struct FDCANLPEN_R(crate::FieldReader<bool, bool>);
impl FDCANLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCANLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANLPEN` writer - FDCANLPEN"]
pub struct FDCANLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&self) -> DFSDMLPEN_R {
        DFSDMLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&self) -> ADFSDMLPEN_R {
        ADFSDMLPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W {
        TIM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W {
        TIM8LPEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W {
        TIM15LPEN_W { w: self }
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W {
        TIM16LPEN_W { w: self }
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W {
        TIM17LPEN_W { w: self }
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W {
        SPI1LPEN_W { w: self }
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W {
        SPI4LPEN_W { w: self }
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W {
        SPI5LPEN_W { w: self }
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W {
        USART6LPEN_W { w: self }
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W {
        SAI1LPEN_W { w: self }
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W {
        SAI2LPEN_W { w: self }
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W {
        SAI3LPEN_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&mut self) -> DFSDMLPEN_W {
        DFSDMLPEN_W { w: self }
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&mut self) -> ADFSDMLPEN_W {
        ADFSDMLPEN_W { w: self }
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W {
        FDCANLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2lpenclrr](index.html) module"]
pub struct RCC_MP_APB2LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB2LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb2lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB2LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB2LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB2LPENCLRR to value 0x0137_271f"]
impl crate::Resettable for RCC_MP_APB2LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0137_271f
    }
}
