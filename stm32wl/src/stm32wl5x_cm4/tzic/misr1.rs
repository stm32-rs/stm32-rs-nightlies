#[doc = "Register `MISR1` reader"]
pub struct R(crate::R<MISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TZICMF` reader - TZICMF"]
pub struct TZICMF_R(crate::FieldReader<bool, bool>);
impl TZICMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZICMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZICMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZSCMF` reader - TZSCMF"]
pub struct TZSCMF_R(crate::FieldReader<bool, bool>);
impl TZSCMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZSCMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZSCMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESMF` reader - AESMF"]
pub struct AESMF_R(crate::FieldReader<bool, bool>);
impl AESMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGMF` reader - RNGMF"]
pub struct RNGMF_R(crate::FieldReader<bool, bool>);
impl RNGMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBGHZSPIMF` reader - SUBGHZSPIMF"]
pub struct SUBGHZSPIMF_R(crate::FieldReader<bool, bool>);
impl SUBGHZSPIMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUBGHZSPIMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBGHZSPIMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRMF` reader - PWRMF"]
pub struct PWRMF_R(crate::FieldReader<bool, bool>);
impl PWRMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHIFMF` reader - FLASHIFMF"]
pub struct FLASHIFMF_R(crate::FieldReader<bool, bool>);
impl FLASHIFMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHIFMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHIFMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1MF` reader - DMA1MF"]
pub struct DMA1MF_R(crate::FieldReader<bool, bool>);
impl DMA1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2MF` reader - DMA2MF"]
pub struct DMA2MF_R(crate::FieldReader<bool, bool>);
impl DMA2MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAMUX1MF` reader - DMAMUX1MF"]
pub struct DMAMUX1MF_R(crate::FieldReader<bool, bool>);
impl DMAMUX1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAMUX1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAMUX1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHMF` reader - FLASHMF"]
pub struct FLASHMF_R(crate::FieldReader<bool, bool>);
impl FLASHMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1MF` reader - SRAM1MF"]
pub struct SRAM1MF_R(crate::FieldReader<bool, bool>);
impl SRAM1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2MF` reader - SRAM2MF"]
pub struct SRAM2MF_R(crate::FieldReader<bool, bool>);
impl SRAM2MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAMF` reader - PKAMF"]
pub struct PKAMF_R(crate::FieldReader<bool, bool>);
impl PKAMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TZICMF"]
    #[inline(always)]
    pub fn tzicmf(&self) -> TZICMF_R {
        TZICMF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCMF"]
    #[inline(always)]
    pub fn tzscmf(&self) -> TZSCMF_R {
        TZSCMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESMF"]
    #[inline(always)]
    pub fn aesmf(&self) -> AESMF_R {
        AESMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGMF"]
    #[inline(always)]
    pub fn rngmf(&self) -> RNGMF_R {
        RNGMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIMF"]
    #[inline(always)]
    pub fn subghzspimf(&self) -> SUBGHZSPIMF_R {
        SUBGHZSPIMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRMF"]
    #[inline(always)]
    pub fn pwrmf(&self) -> PWRMF_R {
        PWRMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFMF"]
    #[inline(always)]
    pub fn flashifmf(&self) -> FLASHIFMF_R {
        FLASHIFMF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1MF"]
    #[inline(always)]
    pub fn dma1mf(&self) -> DMA1MF_R {
        DMA1MF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2MF"]
    #[inline(always)]
    pub fn dma2mf(&self) -> DMA2MF_R {
        DMA2MF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1MF"]
    #[inline(always)]
    pub fn dmamux1mf(&self) -> DMAMUX1MF_R {
        DMAMUX1MF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHMF"]
    #[inline(always)]
    pub fn flashmf(&self) -> FLASHMF_R {
        FLASHMF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1MF"]
    #[inline(always)]
    pub fn sram1mf(&self) -> SRAM1MF_R {
        SRAM1MF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2MF"]
    #[inline(always)]
    pub fn sram2mf(&self) -> SRAM2MF_R {
        SRAM2MF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKAMF"]
    #[inline(always)]
    pub fn pkamf(&self) -> PKAMF_R {
        PKAMF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "TZIC status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr1](index.html) module"]
pub struct MISR1_SPEC;
impl crate::RegisterSpec for MISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr1::R](R) reader structure"]
impl crate::Readable for MISR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR1 to value 0"]
impl crate::Resettable for MISR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
