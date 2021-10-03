#[doc = "Register `RCC_MC_AHB2LPENCLRR` reader"]
pub struct R(crate::R<RCC_MC_AHB2LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB2LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB2LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB2LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB2LPENCLRR` writer"]
pub struct W(crate::W<RCC_MC_AHB2LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB2LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB2LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB2LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1LPEN` reader - DMA1LPEN"]
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
#[doc = "Field `DMA1LPEN` writer - DMA1LPEN"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DMA2LPEN` reader - DMA2LPEN"]
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
#[doc = "Field `DMA2LPEN` writer - DMA2LPEN"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DMAMUXLPEN` reader - DMAMUXLPEN"]
pub struct DMAMUXLPEN_R(crate::FieldReader<bool, bool>);
impl DMAMUXLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUXLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUXLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUXLPEN` writer - DMAMUXLPEN"]
pub struct DMAMUXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUXLPEN_W<'a> {
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
#[doc = "Field `ADC12LPEN` reader - ADC12LPEN"]
pub struct ADC12LPEN_R(crate::FieldReader<bool, bool>);
impl ADC12LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12LPEN` writer - ADC12LPEN"]
pub struct ADC12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LPEN_W<'a> {
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
#[doc = "Field `USBOLPEN` reader - USBOLPEN"]
pub struct USBOLPEN_R(crate::FieldReader<bool, bool>);
impl USBOLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBOLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBOLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBOLPEN` writer - USBOLPEN"]
pub struct USBOLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOLPEN_W<'a> {
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
#[doc = "Field `SDMMC3LPEN` reader - SDMMC3LPEN"]
pub struct SDMMC3LPEN_R(crate::FieldReader<bool, bool>);
impl SDMMC3LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC3LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC3LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC3LPEN` writer - SDMMC3LPEN"]
pub struct SDMMC3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC3LPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&self) -> DMAMUXLPEN_R {
        DMAMUXLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&self) -> USBOLPEN_R {
        USBOLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&self) -> SDMMC3LPEN_R {
        SDMMC3LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W {
        DMA1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W {
        DMA2LPEN_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&mut self) -> DMAMUXLPEN_W {
        DMAMUXLPEN_W { w: self }
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W {
        ADC12LPEN_W { w: self }
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&mut self) -> USBOLPEN_W {
        USBOLPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&mut self) -> SDMMC3LPEN_W {
        SDMMC3LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb2lpenclrr](index.html) module"]
pub struct RCC_MC_AHB2LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB2LPENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb2lpenclrr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB2LPENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb2lpenclrr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB2LPENCLRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_AHB2LPENCLRR to value 0x0001_0127"]
impl crate::Resettable for RCC_MC_AHB2LPENCLRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0127
    }
}
