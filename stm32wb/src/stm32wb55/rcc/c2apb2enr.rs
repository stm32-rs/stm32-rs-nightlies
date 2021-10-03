#[doc = "Register `C2APB2ENR` reader"]
pub struct R(crate::R<C2APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2APB2ENR` writer"]
pub struct W(crate::W<C2APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB2ENR_SPEC>;
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
impl From<crate::W<C2APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAI1EN` reader - CPU2 SAI1 clock enable"]
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
#[doc = "Field `SAI1EN` writer - CPU2 SAI1 clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TIM17EN` reader - CPU2 TIM17 timer clock enable"]
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
#[doc = "Field `TIM17EN` writer - CPU2 TIM17 timer clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TIM16EN` reader - CPU2 TIM16 timer clock enable"]
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
#[doc = "Field `TIM16EN` writer - CPU2 TIM16 timer clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `USART1EN` reader - CPU2 USART1clock enable"]
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
#[doc = "Field `USART1EN` writer - CPU2 USART1clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SPI1EN` reader - CPU2 SPI1 clock enable"]
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
#[doc = "Field `SPI1EN` writer - CPU2 SPI1 clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TIM1EN` reader - CPU2 TIM1 timer clock enable"]
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
#[doc = "Field `TIM1EN` writer - CPU2 TIM1 timer clock enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - CPU2 SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU2 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU2 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU2 USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPU2 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU2 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - CPU2 SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W {
        SAI1EN_W { w: self }
    }
    #[doc = "Bit 18 - CPU2 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W {
        TIM17EN_W { w: self }
    }
    #[doc = "Bit 17 - CPU2 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W {
        TIM16EN_W { w: self }
    }
    #[doc = "Bit 14 - CPU2 USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 12 - CPU2 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 11 - CPU2 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 APB2ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb2enr](index.html) module"]
pub struct C2APB2ENR_SPEC;
impl crate::RegisterSpec for C2APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2apb2enr::R](R) reader structure"]
impl crate::Readable for C2APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2apb2enr::W](W) writer structure"]
impl crate::Writable for C2APB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2APB2ENR to value 0"]
impl crate::Resettable for C2APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
