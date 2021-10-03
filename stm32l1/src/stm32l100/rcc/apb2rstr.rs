#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USART1RST"]
pub type USART1RST_A = SYSCFGRST_A;
#[doc = "Field `USART1RST` reader - USART1RST"]
pub type USART1RST_R = SYSCFGRST_R;
#[doc = "Field `USART1RST` writer - USART1RST"]
pub struct USART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART1RST_A::RESET)
    }
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
#[doc = "SPI1RST"]
pub type SPI1RST_A = SYSCFGRST_A;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = SYSCFGRST_R;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI1RST_A::RESET)
    }
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
#[doc = "SDIORST"]
pub type SDIORST_A = SYSCFGRST_A;
#[doc = "Field `SDIORST` reader - SDIORST"]
pub type SDIORST_R = SYSCFGRST_R;
#[doc = "Field `SDIORST` writer - SDIORST"]
pub struct SDIORST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIORST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDIORST_A::RESET)
    }
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
#[doc = "ADC1RST"]
pub type ADC1RST_A = SYSCFGRST_A;
#[doc = "Field `ADC1RST` reader - ADC1RST"]
pub type ADC1RST_R = SYSCFGRST_R;
#[doc = "Field `ADC1RST` writer - ADC1RST"]
pub struct ADC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADC1RST_A::RESET)
    }
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
#[doc = "TM11RST"]
pub type TM11RST_A = SYSCFGRST_A;
#[doc = "Field `TM11RST` reader - TM11RST"]
pub type TM11RST_R = SYSCFGRST_R;
#[doc = "Field `TM11RST` writer - TM11RST"]
pub struct TM11RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TM11RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TM11RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TM11RST_A::RESET)
    }
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
#[doc = "TM10RST"]
pub type TM10RST_A = SYSCFGRST_A;
#[doc = "Field `TM10RST` reader - TM10RST"]
pub type TM10RST_R = SYSCFGRST_R;
#[doc = "Field `TM10RST` writer - TM10RST"]
pub struct TM10RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TM10RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TM10RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TM10RST_A::RESET)
    }
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
#[doc = "TIM9RST"]
pub type TIM9RST_A = SYSCFGRST_A;
#[doc = "Field `TIM9RST` reader - TIM9RST"]
pub type TIM9RST_R = SYSCFGRST_R;
#[doc = "Field `TIM9RST` writer - TIM9RST"]
pub struct TIM9RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM9RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM9RST_A::RESET)
    }
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
#[doc = "SYSCFGRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRST_A {
    #[doc = "1: Reset the module"]
    RESET = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFGRST"]
pub struct SYSCFGRST_R(crate::FieldReader<bool, SYSCFGRST_A>);
impl SYSCFGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRST_A> {
        match self.bits {
            true => Some(SYSCFGRST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SYSCFGRST_A::RESET
    }
}
impl core::ops::Deref for SYSCFGRST_R {
    type Target = crate::FieldReader<bool, SYSCFGRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFGRST"]
pub struct SYSCFGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::RESET)
    }
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
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TM11RST"]
    #[inline(always)]
    pub fn tm11rst(&self) -> TM11RST_R {
        TM11RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TM10RST"]
    #[inline(always)]
    pub fn tm10rst(&self) -> TM10RST_R {
        TM10RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM9RST"]
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W { w: self }
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 11 - SDIORST"]
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SDIORST_W {
        SDIORST_W { w: self }
    }
    #[doc = "Bit 9 - ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W {
        ADC1RST_W { w: self }
    }
    #[doc = "Bit 4 - TM11RST"]
    #[inline(always)]
    pub fn tm11rst(&mut self) -> TM11RST_W {
        TM11RST_W { w: self }
    }
    #[doc = "Bit 3 - TM10RST"]
    #[inline(always)]
    pub fn tm10rst(&mut self) -> TM10RST_W {
        TM10RST_W { w: self }
    }
    #[doc = "Bit 2 - TIM9RST"]
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W {
        TIM9RST_W { w: self }
    }
    #[doc = "Bit 0 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W {
        SYSCFGRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
