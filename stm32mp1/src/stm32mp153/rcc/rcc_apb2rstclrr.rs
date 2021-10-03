#[doc = "Register `RCC_APB2RSTCLRR` reader"]
pub struct R(crate::R<RCC_APB2RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2RSTCLRR` writer"]
pub struct W(crate::W<RCC_APB2RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_APB2RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1RST` reader - TIM1RST"]
pub struct TIM1RST_R(crate::FieldReader<bool, bool>);
impl TIM1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1RST` writer - TIM1RST"]
pub struct TIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1RST_W<'a> {
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
#[doc = "Field `TIM8RST` reader - TIM8RST"]
pub struct TIM8RST_R(crate::FieldReader<bool, bool>);
impl TIM8RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8RST` writer - TIM8RST"]
pub struct TIM8RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8RST_W<'a> {
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
#[doc = "Field `TIM15RST` reader - TIM15RST"]
pub struct TIM15RST_R(crate::FieldReader<bool, bool>);
impl TIM15RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15RST` writer - TIM15RST"]
pub struct TIM15RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15RST_W<'a> {
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
#[doc = "Field `TIM16RST` reader - TIM16RST"]
pub struct TIM16RST_R(crate::FieldReader<bool, bool>);
impl TIM16RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16RST` writer - TIM16RST"]
pub struct TIM16RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16RST_W<'a> {
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
#[doc = "Field `TIM17RST` reader - TIM17RST"]
pub struct TIM17RST_R(crate::FieldReader<bool, bool>);
impl TIM17RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17RST` writer - TIM17RST"]
pub struct TIM17RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17RST_W<'a> {
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
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub struct SPI1RST_R(crate::FieldReader<bool, bool>);
impl SPI1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
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
#[doc = "Field `SPI4RST` reader - SPI4RST"]
pub struct SPI4RST_R(crate::FieldReader<bool, bool>);
impl SPI4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI4RST` writer - SPI4RST"]
pub struct SPI4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4RST_W<'a> {
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
#[doc = "Field `SPI5RST` reader - SPI5RST"]
pub struct SPI5RST_R(crate::FieldReader<bool, bool>);
impl SPI5RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI5RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI5RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI5RST` writer - SPI5RST"]
pub struct SPI5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5RST_W<'a> {
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
#[doc = "Field `USART6RST` reader - USART6RST"]
pub struct USART6RST_R(crate::FieldReader<bool, bool>);
impl USART6RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART6RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART6RST` writer - USART6RST"]
pub struct USART6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6RST_W<'a> {
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
#[doc = "Field `SAI1RST` reader - SAI1RST"]
pub struct SAI1RST_R(crate::FieldReader<bool, bool>);
impl SAI1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1RST` writer - SAI1RST"]
pub struct SAI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1RST_W<'a> {
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
#[doc = "Field `SAI2RST` reader - SAI2RST"]
pub struct SAI2RST_R(crate::FieldReader<bool, bool>);
impl SAI2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2RST` writer - SAI2RST"]
pub struct SAI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2RST_W<'a> {
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
#[doc = "Field `SAI3RST` reader - SAI3RST"]
pub struct SAI3RST_R(crate::FieldReader<bool, bool>);
impl SAI3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI3RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI3RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI3RST` writer - SAI3RST"]
pub struct SAI3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3RST_W<'a> {
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
#[doc = "Field `DFSDMRST` reader - DFSDMRST"]
pub struct DFSDMRST_R(crate::FieldReader<bool, bool>);
impl DFSDMRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMRST` writer - DFSDMRST"]
pub struct DFSDMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMRST_W<'a> {
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
#[doc = "Field `FDCANRST` reader - FDCANRST"]
pub struct FDCANRST_R(crate::FieldReader<bool, bool>);
impl FDCANRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCANRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANRST` writer - FDCANRST"]
pub struct FDCANRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANRST_W<'a> {
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
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W {
        TIM1RST_W { w: self }
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W {
        TIM8RST_W { w: self }
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W {
        TIM15RST_W { w: self }
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W {
        TIM16RST_W { w: self }
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W {
        TIM17RST_W { w: self }
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W {
        SPI4RST_W { w: self }
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W {
        SPI5RST_W { w: self }
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W {
        USART6RST_W { w: self }
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W {
        SAI1RST_W { w: self }
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W {
        SAI2RST_W { w: self }
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&mut self) -> SAI3RST_W {
        SAI3RST_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W {
        DFSDMRST_W { w: self }
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W {
        FDCANRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2rstclrr](index.html) module"]
pub struct RCC_APB2RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_APB2RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_APB2RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_APB2RSTCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB2RSTCLRR to value 0"]
impl crate::Resettable for RCC_APB2RSTCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
