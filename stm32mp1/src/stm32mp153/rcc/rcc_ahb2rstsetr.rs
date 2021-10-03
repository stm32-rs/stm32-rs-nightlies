#[doc = "Register `RCC_AHB2RSTSETR` reader"]
pub struct R(crate::R<RCC_AHB2RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB2RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB2RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB2RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB2RSTSETR` writer"]
pub struct W(crate::W<RCC_AHB2RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB2RSTSETR_SPEC>;
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
impl From<crate::W<RCC_AHB2RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB2RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1RST"]
pub struct DMA1RST_R(crate::FieldReader<bool, bool>);
impl DMA1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1RST` writer - DMA1RST"]
pub struct DMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RST_W<'a> {
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
#[doc = "Field `DMA2RST` reader - DMA2RST"]
pub struct DMA2RST_R(crate::FieldReader<bool, bool>);
impl DMA2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2RST` writer - DMA2RST"]
pub struct DMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2RST_W<'a> {
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
#[doc = "Field `DMAMUXRST` reader - DMAMUXRST"]
pub struct DMAMUXRST_R(crate::FieldReader<bool, bool>);
impl DMAMUXRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUXRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUXRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUXRST` writer - DMAMUXRST"]
pub struct DMAMUXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUXRST_W<'a> {
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
#[doc = "Field `ADC12RST` reader - ADC12RST"]
pub struct ADC12RST_R(crate::FieldReader<bool, bool>);
impl ADC12RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC12RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC12RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12RST` writer - ADC12RST"]
pub struct ADC12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RST_W<'a> {
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
#[doc = "Field `USBORST` reader - USBORST"]
pub struct USBORST_R(crate::FieldReader<bool, bool>);
impl USBORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBORST` writer - USBORST"]
pub struct USBORST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBORST_W<'a> {
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
#[doc = "Field `SDMMC3RST` reader - SDMMC3RST"]
pub struct SDMMC3RST_R(crate::FieldReader<bool, bool>);
impl SDMMC3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC3RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC3RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC3RST` writer - SDMMC3RST"]
pub struct SDMMC3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC3RST_W<'a> {
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
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&self) -> USBORST_R {
        USBORST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&self) -> SDMMC3RST_R {
        SDMMC3RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W {
        DMA1RST_W { w: self }
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W {
        DMA2RST_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W {
        DMAMUXRST_W { w: self }
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W {
        ADC12RST_W { w: self }
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&mut self) -> USBORST_W {
        USBORST_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&mut self) -> SDMMC3RST_W {
        SDMMC3RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb2rstsetr](index.html) module"]
pub struct RCC_AHB2RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_AHB2RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb2rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_AHB2RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb2rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_AHB2RSTSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHB2RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB2RSTSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
