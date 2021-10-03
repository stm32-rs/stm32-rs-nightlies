#[doc = "Register `AHB1SECSR` reader"]
pub struct R(crate::R<AHB1SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICACHESECF` reader - ICACHESECF"]
pub struct ICACHESECF_R(crate::FieldReader<bool, bool>);
impl ICACHESECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHESECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHESECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTZCSECF` reader - GTZCSECF"]
pub struct GTZCSECF_R(crate::FieldReader<bool, bool>);
impl GTZCSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTZCSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTZCSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCSECF` reader - TSCSECF"]
pub struct TSCSECF_R(crate::FieldReader<bool, bool>);
impl TSCSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSCSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCSECF` reader - CRCSECF"]
pub struct CRCSECF_R(crate::FieldReader<bool, bool>);
impl CRCSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1SECF` reader - SRAM1SECF"]
pub struct SRAM1SECF_R(crate::FieldReader<bool, bool>);
impl SRAM1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHSECF` reader - FLASHSECF"]
pub struct FLASHSECF_R(crate::FieldReader<bool, bool>);
impl FLASHSECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHSECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHSECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1SECF` reader - DMAMUX1SECF"]
pub struct DMAMUX1SECF_R(crate::FieldReader<bool, bool>);
impl DMAMUX1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2SECF` reader - DMA2SECF"]
pub struct DMA2SECF_R(crate::FieldReader<bool, bool>);
impl DMA2SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1SECF` reader - DMA1SECF"]
pub struct DMA1SECF_R(crate::FieldReader<bool, bool>);
impl DMA1SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23 - ICACHESECF"]
    #[inline(always)]
    pub fn icachesecf(&self) -> ICACHESECF_R {
        ICACHESECF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GTZCSECF"]
    #[inline(always)]
    pub fn gtzcsecf(&self) -> GTZCSECF_R {
        GTZCSECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TSCSECF"]
    #[inline(always)]
    pub fn tscsecf(&self) -> TSCSECF_R {
        TSCSECF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRCSECF"]
    #[inline(always)]
    pub fn crcsecf(&self) -> CRCSECF_R {
        CRCSECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM1SECF"]
    #[inline(always)]
    pub fn sram1secf(&self) -> SRAM1SECF_R {
        SRAM1SECF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLASHSECF"]
    #[inline(always)]
    pub fn flashsecf(&self) -> FLASHSECF_R {
        FLASHSECF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1SECF"]
    #[inline(always)]
    pub fn dmamux1secf(&self) -> DMAMUX1SECF_R {
        DMAMUX1SECF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2SECF"]
    #[inline(always)]
    pub fn dma2secf(&self) -> DMA2SECF_R {
        DMA2SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA1SECF"]
    #[inline(always)]
    pub fn dma1secf(&self) -> DMA1SECF_R {
        DMA1SECF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RCC AHB1 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1secsr](index.html) module"]
pub struct AHB1SECSR_SPEC;
impl crate::RegisterSpec for AHB1SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1secsr::R](R) reader structure"]
impl crate::Readable for AHB1SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB1SECSR to value 0x0040_0300"]
impl crate::Resettable for AHB1SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0300
    }
}
