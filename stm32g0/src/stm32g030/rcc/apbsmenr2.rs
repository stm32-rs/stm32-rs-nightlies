#[doc = "Register `APBSMENR2` reader"]
pub struct R(crate::R<APBSMENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBSMENR2` writer"]
pub struct W(crate::W<APBSMENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR2_SPEC>;
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
impl From<crate::W<APBSMENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub struct SYSCFGSMEN_R(crate::FieldReader<bool, bool>);
impl SYSCFGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub struct SYSCFGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSMEN_W<'a> {
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
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode"]
pub struct TIM1SMEN_R(crate::FieldReader<bool, bool>);
impl TIM1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode"]
pub struct TIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SMEN_W<'a> {
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
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode"]
pub struct SPI1SMEN_R(crate::FieldReader<bool, bool>);
impl SPI1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode"]
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
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
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep mode"]
pub struct USART1SMEN_R(crate::FieldReader<bool, bool>);
impl USART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep mode"]
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
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
#[doc = "Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode"]
pub struct TIM14SMEN_R(crate::FieldReader<bool, bool>);
impl TIM14SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM14SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM14SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode"]
pub struct TIM14SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14SMEN_W<'a> {
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
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub struct TIM16SMEN_R(crate::FieldReader<bool, bool>);
impl TIM16SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub struct TIM16SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SMEN_W<'a> {
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
#[doc = "Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub struct TIM17SMEN_R(crate::FieldReader<bool, bool>);
impl TIM17SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub struct TIM17SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17SMEN_W<'a> {
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
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode"]
pub struct ADCSMEN_R(crate::FieldReader<bool, bool>);
impl ADCSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode"]
pub struct ADCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSMEN_W<'a> {
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
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14smen(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W {
        SYSCFGSMEN_W { w: self }
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W {
        TIM1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14smen(&mut self) -> TIM14SMEN_W {
        TIM14SMEN_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W {
        TIM16SMEN_W { w: self }
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim17smen(&mut self) -> TIM17SMEN_W {
        TIM17SMEN_W { w: self }
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W {
        ADCSMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbsmenr2](index.html) module"]
pub struct APBSMENR2_SPEC;
impl crate::RegisterSpec for APBSMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbsmenr2::R](R) reader structure"]
impl crate::Readable for APBSMENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbsmenr2::W](W) writer structure"]
impl crate::Writable for APBSMENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBSMENR2 to value 0"]
impl crate::Resettable for APBSMENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
