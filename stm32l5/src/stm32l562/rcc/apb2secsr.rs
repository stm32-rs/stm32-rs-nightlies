#[doc = "Register `APB2SECSR` reader"]
pub struct R(crate::R<APB2SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFSDM1SECF` reader - DFSDM1SECF"]
pub struct DFSDM1SECF_R(crate::FieldReader<bool, bool>);
impl DFSDM1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2SECF` reader - SAI2SECF"]
pub struct SAI2SECF_R(crate::FieldReader<bool, bool>);
impl SAI2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SECF` reader - SAI1SECF"]
pub struct SAI1SECF_R(crate::FieldReader<bool, bool>);
impl SAI1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAI1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17SECF` reader - TIM17SECF"]
pub struct TIM17SECF_R(crate::FieldReader<bool, bool>);
impl TIM17SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16SECF` reader - TIM16SECF"]
pub struct TIM16SECF_R(crate::FieldReader<bool, bool>);
impl TIM16SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15SECF` reader - TIM15SECF"]
pub struct TIM15SECF_R(crate::FieldReader<bool, bool>);
impl TIM15SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SECF` reader - USART1SECF"]
pub struct USART1SECF_R(crate::FieldReader<bool, bool>);
impl USART1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8SECF` reader - TIM8SECF"]
pub struct TIM8SECF_R(crate::FieldReader<bool, bool>);
impl TIM8SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1SECF` reader - SPI1SECF"]
pub struct SPI1SECF_R(crate::FieldReader<bool, bool>);
impl SPI1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1SECF` reader - TIM1SECF"]
pub struct TIM1SECF_R(crate::FieldReader<bool, bool>);
impl TIM1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSCFGSECF` reader - SYSCFGSECF"]
pub struct SYSCFGSECF_R(crate::FieldReader<bool, bool>);
impl SYSCFGSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCFGSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 24 - DFSDM1SECF"]
    #[inline(always)]
    pub fn dfsdm1secf(&self) -> DFSDM1SECF_R {
        DFSDM1SECF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI2SECF"]
    #[inline(always)]
    pub fn sai2secf(&self) -> SAI2SECF_R {
        SAI2SECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI1SECF"]
    #[inline(always)]
    pub fn sai1secf(&self) -> SAI1SECF_R {
        SAI1SECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17SECF"]
    #[inline(always)]
    pub fn tim17secf(&self) -> TIM17SECF_R {
        TIM17SECF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16SECF"]
    #[inline(always)]
    pub fn tim16secf(&self) -> TIM16SECF_R {
        TIM16SECF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15SECF"]
    #[inline(always)]
    pub fn tim15secf(&self) -> TIM15SECF_R {
        TIM15SECF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1SECF"]
    #[inline(always)]
    pub fn usart1secf(&self) -> USART1SECF_R {
        USART1SECF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM8SECF"]
    #[inline(always)]
    pub fn tim8secf(&self) -> TIM8SECF_R {
        TIM8SECF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1SECF"]
    #[inline(always)]
    pub fn spi1secf(&self) -> SPI1SECF_R {
        SPI1SECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1SECF"]
    #[inline(always)]
    pub fn tim1secf(&self) -> TIM1SECF_R {
        TIM1SECF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSCFGSECF"]
    #[inline(always)]
    pub fn syscfgsecf(&self) -> SYSCFGSECF_R {
        SYSCFGSECF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RCC APB2 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2secsr](index.html) module"]
pub struct APB2SECSR_SPEC;
impl crate::RegisterSpec for APB2SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2secsr::R](R) reader structure"]
impl crate::Readable for APB2SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APB2SECSR to value 0"]
impl crate::Resettable for APB2SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
