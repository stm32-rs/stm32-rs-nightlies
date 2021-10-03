#[doc = "Register `RCC_MC_APB5LPENSETR` reader"]
pub struct R(crate::R<RCC_MC_APB5LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB5LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB5LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB5LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_APB5LPENSETR` writer"]
pub struct W(crate::W<RCC_MC_APB5LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB5LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_APB5LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB5LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI6LPEN` reader - SPI6LPEN"]
pub struct SPI6LPEN_R(crate::FieldReader<bool, bool>);
impl SPI6LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI6LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI6LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI6LPEN` writer - SPI6LPEN"]
pub struct SPI6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6LPEN_W<'a> {
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
#[doc = "Field `I2C4LPEN` reader - I2C4LPEN"]
pub struct I2C4LPEN_R(crate::FieldReader<bool, bool>);
impl I2C4LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4LPEN` writer - I2C4LPEN"]
pub struct I2C4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4LPEN_W<'a> {
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
#[doc = "Field `I2C6LPEN` reader - I2C6LPEN"]
pub struct I2C6LPEN_R(crate::FieldReader<bool, bool>);
impl I2C6LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C6LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C6LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C6LPEN` writer - I2C6LPEN"]
pub struct I2C6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6LPEN_W<'a> {
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
#[doc = "Field `USART1LPEN` reader - USART1LPEN"]
pub struct USART1LPEN_R(crate::FieldReader<bool, bool>);
impl USART1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1LPEN` writer - USART1LPEN"]
pub struct USART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1LPEN_W<'a> {
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
#[doc = "Field `RTCAPBLPEN` reader - RTCAPBLPEN"]
pub struct RTCAPBLPEN_R(crate::FieldReader<bool, bool>);
impl RTCAPBLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAPBLPEN` writer - RTCAPBLPEN"]
pub struct RTCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBLPEN_W<'a> {
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
#[doc = "Field `TZC1LPEN` reader - TZC1LPEN"]
pub struct TZC1LPEN_R(crate::FieldReader<bool, bool>);
impl TZC1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZC1LPEN` writer - TZC1LPEN"]
pub struct TZC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1LPEN_W<'a> {
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
#[doc = "Field `TZC2LPEN` reader - TZC2LPEN"]
pub struct TZC2LPEN_R(crate::FieldReader<bool, bool>);
impl TZC2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZC2LPEN` writer - TZC2LPEN"]
pub struct TZC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2LPEN_W<'a> {
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
#[doc = "Field `TZPCLPEN` reader - TZPCLPEN"]
pub struct TZPCLPEN_R(crate::FieldReader<bool, bool>);
impl TZPCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZPCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZPCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZPCLPEN` writer - TZPCLPEN"]
pub struct TZPCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCLPEN_W<'a> {
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
#[doc = "Field `BSECLPEN` reader - BSECLPEN"]
pub struct BSECLPEN_R(crate::FieldReader<bool, bool>);
impl BSECLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSECLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSECLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSECLPEN` writer - BSECLPEN"]
pub struct BSECLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECLPEN_W<'a> {
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
#[doc = "Field `STGENLPEN` reader - STGENLPEN"]
pub struct STGENLPEN_R(crate::FieldReader<bool, bool>);
impl STGENLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENLPEN` writer - STGENLPEN"]
pub struct STGENLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENLPEN_W<'a> {
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
#[doc = "Field `STGENSTPEN` reader - STGENSTPEN"]
pub struct STGENSTPEN_R(crate::FieldReader<bool, bool>);
impl STGENSTPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STGENSTPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STGENSTPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STGENSTPEN` writer - STGENSTPEN"]
pub struct STGENSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENSTPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W {
        SPI6LPEN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W {
        I2C4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W {
        I2C6LPEN_W { w: self }
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W {
        USART1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W {
        RTCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W {
        TZC1LPEN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W {
        TZC2LPEN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W {
        TZPCLPEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W {
        BSECLPEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W {
        STGENLPEN_W { w: self }
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W {
        STGENSTPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_apb5lpensetr](index.html) module"]
pub struct RCC_MC_APB5LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB5LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_apb5lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_APB5LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_apb5lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_APB5LPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_APB5LPENSETR to value 0x0011_391d"]
impl crate::Resettable for RCC_MC_APB5LPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_391d
    }
}
