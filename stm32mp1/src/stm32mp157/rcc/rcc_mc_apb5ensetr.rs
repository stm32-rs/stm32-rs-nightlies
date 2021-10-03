#[doc = "Register `RCC_MC_APB5ENSETR` reader"]
pub struct R(crate::R<RCC_MC_APB5ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB5ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB5ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB5ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB5ENSETR` writer"]
pub struct W(crate::W<RCC_MC_APB5ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB5ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_APB5ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB5ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI6EN` reader - SPI6EN"]
pub struct SPI6EN_R(crate::FieldReader<bool, bool>);
impl SPI6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI6EN` writer - SPI6EN"]
pub struct SPI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6EN_W<'a> {
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
#[doc = "Field `I2C4EN` reader - I2C4EN"]
pub struct I2C4EN_R(crate::FieldReader<bool, bool>);
impl I2C4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4EN` writer - I2C4EN"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
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
#[doc = "Field `I2C6EN` reader - I2C6EN"]
pub struct I2C6EN_R(crate::FieldReader<bool, bool>);
impl I2C6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C6EN` writer - I2C6EN"]
pub struct I2C6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6EN_W<'a> {
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
#[doc = "Field `USART1EN` reader - USART1EN"]
pub struct USART1EN_R(crate::FieldReader<bool, bool>);
impl USART1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1EN` writer - USART1EN"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
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
#[doc = "Field `RTCAPBEN` reader - RTCAPBEN"]
pub struct RTCAPBEN_R(crate::FieldReader<bool, bool>);
impl RTCAPBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAPBEN` writer - RTCAPBEN"]
pub struct RTCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBEN_W<'a> {
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
#[doc = "Field `TZC1EN` reader - TZC1EN"]
pub struct TZC1EN_R(crate::FieldReader<bool, bool>);
impl TZC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZC1EN` writer - TZC1EN"]
pub struct TZC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1EN_W<'a> {
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
#[doc = "Field `TZC2EN` reader - TZC2EN"]
pub struct TZC2EN_R(crate::FieldReader<bool, bool>);
impl TZC2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZC2EN` writer - TZC2EN"]
pub struct TZC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2EN_W<'a> {
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
#[doc = "Field `TZPCEN` reader - TZPCEN"]
pub struct TZPCEN_R(crate::FieldReader<bool, bool>);
impl TZPCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZPCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZPCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZPCEN` writer - TZPCEN"]
pub struct TZPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCEN_W<'a> {
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
#[doc = "Field `BSECEN` reader - BSECEN"]
pub struct BSECEN_R(crate::FieldReader<bool, bool>);
impl BSECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSECEN` writer - BSECEN"]
pub struct BSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECEN_W<'a> {
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
#[doc = "Field `STGENEN` reader - STGENEN"]
pub struct STGENEN_R(crate::FieldReader<bool, bool>);
impl STGENEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENEN` writer - STGENEN"]
pub struct STGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENEN_W<'a> {
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
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W {
        SPI6EN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&mut self) -> I2C6EN_W {
        I2C6EN_W { w: self }
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&mut self) -> TZC1EN_W {
        TZC1EN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&mut self) -> TZC2EN_W {
        TZC2EN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&mut self) -> TZPCEN_W {
        TZPCEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&mut self) -> BSECEN_W {
        BSECEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&mut self) -> STGENEN_W {
        STGENEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5ensetr](index.html) module"]
pub struct RCC_MC_APB5ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB5ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb5ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB5ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB5ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_APB5ENSETR to value 0"]
impl crate::Resettable for RCC_MC_APB5ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
