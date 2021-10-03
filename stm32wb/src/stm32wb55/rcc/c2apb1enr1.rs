#[doc = "Register `C2APB1ENR1` reader"]
pub struct R(crate::R<C2APB1ENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1ENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1ENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1ENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB1ENR1` writer"]
pub struct W(crate::W<C2APB1ENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1ENR1_SPEC>;
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
impl From<crate::W<C2APB1ENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1ENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM1EN` reader - CPU2 Low power timer 1 clock enable"]
pub struct LPTIM1EN_R(crate::FieldReader<bool, bool>);
impl LPTIM1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1EN` writer - CPU2 Low power timer 1 clock enable"]
pub struct LPTIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1EN_W<'a> {
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
#[doc = "Field `USBEN` reader - CPU2 USB clock enable"]
pub struct USBEN_R(crate::FieldReader<bool, bool>);
impl USBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBEN` writer - CPU2 USB clock enable"]
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
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
#[doc = "Field `CRSEN` reader - CPU2 CRS clock enable"]
pub struct CRSEN_R(crate::FieldReader<bool, bool>);
impl CRSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSEN` writer - CPU2 CRS clock enable"]
pub struct CRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSEN_W<'a> {
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
#[doc = "Field `I2C3EN` reader - CPU2 I2C3 clock enable"]
pub struct I2C3EN_R(crate::FieldReader<bool, bool>);
impl I2C3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3EN` writer - CPU2 I2C3 clock enable"]
pub struct I2C3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3EN_W<'a> {
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
#[doc = "Field `I2C1EN` reader - CPU2 I2C1 clock enable"]
pub struct I2C1EN_R(crate::FieldReader<bool, bool>);
impl I2C1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1EN` writer - CPU2 I2C1 clock enable"]
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
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
#[doc = "Field `SPI2EN` reader - CPU2 SPI2 clock enable"]
pub struct SPI2EN_R(crate::FieldReader<bool, bool>);
impl SPI2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2EN` writer - CPU2 SPI2 clock enable"]
pub struct SPI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2EN_W<'a> {
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
#[doc = "Field `RTCAPBEN` reader - CPU2 RTC APB clock enable"]
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
#[doc = "Field `RTCAPBEN` writer - CPU2 RTC APB clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LCDEN` reader - CPU2 LCD clock enable"]
pub struct LCDEN_R(crate::FieldReader<bool, bool>);
impl LCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDEN` writer - CPU2 LCD clock enable"]
pub struct LCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDEN_W<'a> {
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
#[doc = "Field `TIM2EN` reader - CPU2 TIM2 timer clock enable"]
pub struct TIM2EN_R(crate::FieldReader<bool, bool>);
impl TIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2EN` writer - CPU2 TIM2 timer clock enable"]
pub struct TIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2EN_W<'a> {
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
    #[doc = "Bit 31 - CPU2 Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CPU2 USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPU2 CRS clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPU2 I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU2 I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU2 SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU2 RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU2 LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CPU2 TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CPU2 Low power timer 1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W {
        LPTIM1EN_W { w: self }
    }
    #[doc = "Bit 26 - CPU2 USB clock enable"]
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
    #[doc = "Bit 24 - CPU2 CRS clock enable"]
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W {
        CRSEN_W { w: self }
    }
    #[doc = "Bit 23 - CPU2 I2C3 clock enable"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W {
        I2C3EN_W { w: self }
    }
    #[doc = "Bit 21 - CPU2 I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    #[doc = "Bit 14 - CPU2 SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W {
        SPI2EN_W { w: self }
    }
    #[doc = "Bit 10 - CPU2 RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    #[doc = "Bit 9 - CPU2 LCD clock enable"]
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W {
        LCDEN_W { w: self }
    }
    #[doc = "Bit 0 - CPU2 TIM2 timer clock enable"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W {
        TIM2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB1ENR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1enr1](index.html) module"]
pub struct C2APB1ENR1_SPEC;
impl crate::RegisterSpec for C2APB1ENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb1enr1::R](R) reader structure"]
impl crate::Readable for C2APB1ENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb1enr1::W](W) writer structure"]
impl crate::Writable for C2APB1ENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB1ENR1 to value 0x0400"]
impl crate::Resettable for C2APB1ENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
