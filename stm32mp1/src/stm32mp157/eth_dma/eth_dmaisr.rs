#[doc = "Register `ETH_DMAISR` reader"]
pub struct R(crate::R<ETH_DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub struct DC0IS_R(crate::FieldReader<bool, bool>);
impl DC0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC1IS` reader - DC1IS"]
pub struct DC1IS_R(crate::FieldReader<bool, bool>);
impl DC1IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC1IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub struct MTLIS_R(crate::FieldReader<bool, bool>);
impl MTLIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTLIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTLIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub struct MACIS_R(crate::FieldReader<bool, bool>);
impl MACIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MACIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DC1IS"]
    #[inline(always)]
    pub fn dc1is(&self) -> DC1IS_R {
        DC1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaisr](index.html) module"]
pub struct ETH_DMAISR_SPEC;
impl crate::RegisterSpec for ETH_DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaisr::R](R) reader structure"]
impl crate::Readable for ETH_DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETH_DMAISR to value 0x8000"]
impl crate::Resettable for ETH_DMAISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
