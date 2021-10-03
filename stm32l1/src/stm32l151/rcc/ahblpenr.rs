#[doc = "Register `AHBLPENR` reader"]
pub struct R(crate::R<AHBLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBLPENR` writer"]
pub struct W(crate::W<AHBLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBLPENR_SPEC>;
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
impl From<crate::W<AHBLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub struct DMA2LPEN_R(crate::FieldReader<bool, bool>);
impl DMA2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub struct DMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2LPEN_W<'a> {
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
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub struct DMA1LPEN_R(crate::FieldReader<bool, bool>);
impl DMA1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub struct DMA1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1LPEN_W<'a> {
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
#[doc = "Field `SRAMLPEN` reader - SRAM clock enable during Sleep mode"]
pub struct SRAMLPEN_R(crate::FieldReader<bool, bool>);
impl SRAMLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAMLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAMLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMLPEN` writer - SRAM clock enable during Sleep mode"]
pub struct SRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLPEN_W<'a> {
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
#[doc = "Field `FLITFLPEN` reader - FLITF clock enable during Sleep mode"]
pub struct FLITFLPEN_R(crate::FieldReader<bool, bool>);
impl FLITFLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLITFLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLITFLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLITFLPEN` writer - FLITF clock enable during Sleep mode"]
pub struct FLITFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITFLPEN_W<'a> {
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
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub struct CRCLPEN_R(crate::FieldReader<bool, bool>);
impl CRCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub struct CRCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCLPEN_W<'a> {
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
#[doc = "Field `GPIOGLPEN` reader - IO port G clock enable during Sleep mode"]
pub struct GPIOGLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOGLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOGLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOGLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOGLPEN` writer - IO port G clock enable during Sleep mode"]
pub struct GPIOGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGLPEN_W<'a> {
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
#[doc = "Field `GPIOFLPEN` reader - IO port F clock enable during Sleep mode"]
pub struct GPIOFLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOFLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOFLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOFLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOFLPEN` writer - IO port F clock enable during Sleep mode"]
pub struct GPIOFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFLPEN_W<'a> {
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
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub struct GPIOHLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOHLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub struct GPIOHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHLPEN_W<'a> {
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
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub struct GPIOELPEN_R(crate::FieldReader<bool, bool>);
impl GPIOELPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOELPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOELPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub struct GPIOELPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOELPEN_W<'a> {
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
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub struct GPIODLPEN_R(crate::FieldReader<bool, bool>);
impl GPIODLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub struct GPIODLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODLPEN_W<'a> {
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
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub struct GPIOCLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub struct GPIOCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCLPEN_W<'a> {
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
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub struct GPIOBLPEN_R(crate::FieldReader<bool, bool>);
impl GPIOBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub struct GPIOBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBLPEN_W<'a> {
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
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during Sleep mode"]
pub struct GPIOALPEN_R(crate::FieldReader<bool, bool>);
impl GPIOALPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOALPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOALPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during Sleep mode"]
pub struct GPIOALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOALPEN_W<'a> {
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
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&self) -> SRAMLPEN_R {
        SRAMLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W {
        DMA2LPEN_W { w: self }
    }
    #[doc = "Bit 24 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W {
        DMA1LPEN_W { w: self }
    }
    #[doc = "Bit 16 - SRAM clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sramlpen(&mut self) -> SRAMLPEN_W {
        SRAMLPEN_W { w: self }
    }
    #[doc = "Bit 15 - FLITF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W {
        FLITFLPEN_W { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W {
        CRCLPEN_W { w: self }
    }
    #[doc = "Bit 7 - IO port G clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W {
        GPIOGLPEN_W { w: self }
    }
    #[doc = "Bit 6 - IO port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W {
        GPIOFLPEN_W { w: self }
    }
    #[doc = "Bit 5 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W {
        GPIOHLPEN_W { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W {
        GPIOELPEN_W { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W {
        GPIODLPEN_W { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W {
        GPIOCLPEN_W { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W {
        GPIOBLPEN_W { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W {
        GPIOALPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahblpenr](index.html) module"]
pub struct AHBLPENR_SPEC;
impl crate::RegisterSpec for AHBLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahblpenr::R](R) reader structure"]
impl crate::Readable for AHBLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahblpenr::W](W) writer structure"]
impl crate::Writable for AHBLPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBLPENR to value 0x0101_903f"]
impl crate::Resettable for AHBLPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_903f
    }
}
