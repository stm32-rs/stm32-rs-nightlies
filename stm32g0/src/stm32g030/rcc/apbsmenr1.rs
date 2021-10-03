#[doc = "Register `APBSMENR1` reader"]
pub struct R(crate::R<APBSMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBSMENR1` writer"]
pub struct W(crate::W<APBSMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR1_SPEC>;
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
impl From<crate::W<APBSMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode"]
pub struct TIM2SMEN_R(crate::FieldReader<bool, bool>);
impl TIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode"]
pub struct TIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SMEN_W<'a> {
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
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode"]
pub struct TIM3SMEN_R(crate::FieldReader<bool, bool>);
impl TIM3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode"]
pub struct TIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3SMEN_W<'a> {
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
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode"]
pub struct RTCAPBSMEN_R(crate::FieldReader<bool, bool>);
impl RTCAPBSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode"]
pub struct RTCAPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBSMEN_W<'a> {
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
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode"]
pub struct WWDGSMEN_R(crate::FieldReader<bool, bool>);
impl WWDGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode"]
pub struct WWDGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGSMEN_W<'a> {
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
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode"]
pub struct SPI2SMEN_R(crate::FieldReader<bool, bool>);
impl SPI2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode"]
pub struct SPI2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SMEN_W<'a> {
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
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep mode"]
pub struct USART2SMEN_R(crate::FieldReader<bool, bool>);
impl USART2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep mode"]
pub struct USART2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SMEN_W<'a> {
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
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode"]
pub struct I2C1SMEN_R(crate::FieldReader<bool, bool>);
impl I2C1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode"]
pub struct I2C1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SMEN_W<'a> {
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
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode"]
pub struct I2C2SMEN_R(crate::FieldReader<bool, bool>);
impl I2C2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode"]
pub struct I2C2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SMEN_W<'a> {
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
#[doc = "Field `DBGSMEN` reader - Debug support clock enable during Sleep mode"]
pub struct DBGSMEN_R(crate::FieldReader<bool, bool>);
impl DBGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGSMEN` writer - Debug support clock enable during Sleep mode"]
pub struct DBGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSMEN_W<'a> {
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
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode"]
pub struct PWRSMEN_R(crate::FieldReader<bool, bool>);
impl PWRSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode"]
pub struct PWRSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W {
        TIM2SMEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W {
        TIM3SMEN_W { w: self }
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W {
        RTCAPBSMEN_W { w: self }
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W {
        WWDGSMEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W {
        SPI2SMEN_W { w: self }
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W {
        USART2SMEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W {
        I2C1SMEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W {
        I2C2SMEN_W { w: self }
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W {
        DBGSMEN_W { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W {
        PWRSMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr1](index.html) module"]
pub struct APBSMENR1_SPEC;
impl crate::RegisterSpec for APBSMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbsmenr1::R](R) reader structure"]
impl crate::Readable for APBSMENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbsmenr1::W](W) writer structure"]
impl crate::Writable for APBSMENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBSMENR1 to value 0"]
impl crate::Resettable for APBSMENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
