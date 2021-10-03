#[doc = "Register `DMA_HWCFGR1` reader"]
pub struct R(crate::R<DMA_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_DEF0` reader - DMA_DEF0"]
pub struct DMA_DEF0_R(crate::FieldReader<u8, u8>);
impl DMA_DEF0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF1` reader - DMA_DEF1"]
pub struct DMA_DEF1_R(crate::FieldReader<u8, u8>);
impl DMA_DEF1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF2` reader - DMA_DEF2"]
pub struct DMA_DEF2_R(crate::FieldReader<u8, u8>);
impl DMA_DEF2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF3` reader - DMA_DEF3"]
pub struct DMA_DEF3_R(crate::FieldReader<u8, u8>);
impl DMA_DEF3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF4` reader - DMA_DEF4"]
pub struct DMA_DEF4_R(crate::FieldReader<u8, u8>);
impl DMA_DEF4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF5` reader - DMA_DEF5"]
pub struct DMA_DEF5_R(crate::FieldReader<u8, u8>);
impl DMA_DEF5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF6` reader - DMA_DEF6"]
pub struct DMA_DEF6_R(crate::FieldReader<u8, u8>);
impl DMA_DEF6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_DEF7` reader - DMA_DEF7"]
pub struct DMA_DEF7_R(crate::FieldReader<u8, u8>);
impl DMA_DEF7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_DEF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_DEF7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - DMA_DEF0"]
    #[inline(always)]
    pub fn dma_def0(&self) -> DMA_DEF0_R {
        DMA_DEF0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DMA_DEF1"]
    #[inline(always)]
    pub fn dma_def1(&self) -> DMA_DEF1_R {
        DMA_DEF1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - DMA_DEF2"]
    #[inline(always)]
    pub fn dma_def2(&self) -> DMA_DEF2_R {
        DMA_DEF2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DMA_DEF3"]
    #[inline(always)]
    pub fn dma_def3(&self) -> DMA_DEF3_R {
        DMA_DEF3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DMA_DEF4"]
    #[inline(always)]
    pub fn dma_def4(&self) -> DMA_DEF4_R {
        DMA_DEF4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DMA_DEF5"]
    #[inline(always)]
    pub fn dma_def5(&self) -> DMA_DEF5_R {
        DMA_DEF5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DMA_DEF6"]
    #[inline(always)]
    pub fn dma_def6(&self) -> DMA_DEF6_R {
        DMA_DEF6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DMA_DEF7"]
    #[inline(always)]
    pub fn dma_def7(&self) -> DMA_DEF7_R {
        DMA_DEF7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
#[doc = "DMA hardware configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hwcfgr1](index.html) module"]
pub struct DMA_HWCFGR1_SPEC;
impl crate::RegisterSpec for DMA_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_hwcfgr1::R](R) reader structure"]
impl crate::Readable for DMA_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_HWCFGR1 to value 0x2222_2222"]
impl crate::Resettable for DMA_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2222_2222
    }
}
