#[doc = "Register `RCC_MP_APB2ENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB2ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB2ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB2ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB2ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB2ENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB2ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB2ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB2ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB2ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1EN` reader - TIM1EN"]
pub struct TIM1EN_R(crate::FieldReader<bool, bool>);
impl TIM1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1EN` writer - TIM1EN"]
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
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
#[doc = "Field `TIM8EN` reader - TIM8EN"]
pub struct TIM8EN_R(crate::FieldReader<bool, bool>);
impl TIM8EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8EN` writer - TIM8EN"]
pub struct TIM8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8EN_W<'a> {
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
#[doc = "Field `TIM15EN` reader - TIM15EN"]
pub struct TIM15EN_R(crate::FieldReader<bool, bool>);
impl TIM15EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15EN` writer - TIM15EN"]
pub struct TIM15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15EN_W<'a> {
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
#[doc = "Field `TIM16EN` reader - TIM16EN"]
pub struct TIM16EN_R(crate::FieldReader<bool, bool>);
impl TIM16EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16EN` writer - TIM16EN"]
pub struct TIM16EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16EN_W<'a> {
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
#[doc = "Field `TIM17EN` reader - TIM17EN"]
pub struct TIM17EN_R(crate::FieldReader<bool, bool>);
impl TIM17EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17EN` writer - TIM17EN"]
pub struct TIM17EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17EN_W<'a> {
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
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub struct SPI1EN_R(crate::FieldReader<bool, bool>);
impl SPI1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
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
#[doc = "Field `SPI4EN` reader - SPI4EN"]
pub struct SPI4EN_R(crate::FieldReader<bool, bool>);
impl SPI4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI4EN` writer - SPI4EN"]
pub struct SPI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4EN_W<'a> {
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
#[doc = "Field `SPI5EN` reader - SPI5EN"]
pub struct SPI5EN_R(crate::FieldReader<bool, bool>);
impl SPI5EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI5EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI5EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI5EN` writer - SPI5EN"]
pub struct SPI5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5EN_W<'a> {
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
#[doc = "Field `USART6EN` reader - USART6EN"]
pub struct USART6EN_R(crate::FieldReader<bool, bool>);
impl USART6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART6EN` writer - USART6EN"]
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
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
#[doc = "Field `SAI1EN` reader - SAI1EN"]
pub struct SAI1EN_R(crate::FieldReader<bool, bool>);
impl SAI1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1EN` writer - SAI1EN"]
pub struct SAI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1EN_W<'a> {
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
#[doc = "Field `SAI2EN` reader - SAI2EN"]
pub struct SAI2EN_R(crate::FieldReader<bool, bool>);
impl SAI2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2EN` writer - SAI2EN"]
pub struct SAI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2EN_W<'a> {
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
#[doc = "Field `SAI3EN` reader - SAI3EN"]
pub struct SAI3EN_R(crate::FieldReader<bool, bool>);
impl SAI3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI3EN` writer - SAI3EN"]
pub struct SAI3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3EN_W<'a> {
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
#[doc = "Field `DFSDMEN` reader - DFSDMEN"]
pub struct DFSDMEN_R(crate::FieldReader<bool, bool>);
impl DFSDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDMEN` writer - DFSDMEN"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
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
#[doc = "Field `ADFSDMEN` reader - ADFSDMEN"]
pub struct ADFSDMEN_R(crate::FieldReader<bool, bool>);
impl ADFSDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADFSDMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADFSDMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADFSDMEN` writer - ADFSDMEN"]
pub struct ADFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMEN_W<'a> {
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
#[doc = "Field `FDCANEN` reader - FDCANEN"]
pub struct FDCANEN_R(crate::FieldReader<bool, bool>);
impl FDCANEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCANEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANEN` writer - FDCANEN"]
pub struct FDCANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANEN_W<'a> {
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
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&self) -> ADFSDMEN_R {
        ADFSDMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W {
        TIM8EN_W { w: self }
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W {
        TIM15EN_W { w: self }
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W {
        TIM16EN_W { w: self }
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W {
        TIM17EN_W { w: self }
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W {
        SPI4EN_W { w: self }
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W {
        SPI5EN_W { w: self }
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W {
        SAI1EN_W { w: self }
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W {
        SAI2EN_W { w: self }
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&mut self) -> SAI3EN_W {
        SAI3EN_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&mut self) -> ADFSDMEN_W {
        ADFSDMEN_W { w: self }
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W {
        FDCANEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb2ensetr](index.html) module"]
pub struct RCC_MP_APB2ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB2ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb2ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB2ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb2ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB2ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB2ENSETR to value 0"]
impl crate::Resettable for RCC_MP_APB2ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
