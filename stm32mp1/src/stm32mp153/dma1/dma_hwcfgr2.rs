#[doc = "Register `DMA_HWCFGR2` reader"]
pub struct R(crate::R<DMA_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_SIZE` reader - FIFO_SIZE"]
pub struct FIFO_SIZE_R(crate::FieldReader<u8, u8>);
impl FIFO_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_BUFFERABLE` reader - WRITE_BUFFERABLE"]
pub struct WRITE_BUFFERABLE_R(crate::FieldReader<bool, bool>);
impl WRITE_BUFFERABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_BUFFERABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_BUFFERABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL_WIDTH` reader - CHSEL_WIDTH"]
pub struct CHSEL_WIDTH_R(crate::FieldReader<u8, u8>);
impl CHSEL_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - FIFO_SIZE"]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - WRITE_BUFFERABLE"]
    #[inline(always)]
    pub fn write_bufferable(&self) -> WRITE_BUFFERABLE_R {
        WRITE_BUFFERABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - CHSEL_WIDTH"]
    #[inline(always)]
    pub fn chsel_width(&self) -> CHSEL_WIDTH_R {
        CHSEL_WIDTH_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
#[doc = "DMA hardware configuration 2register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hwcfgr2](index.html) module"]
pub struct DMA_HWCFGR2_SPEC;
impl crate::RegisterSpec for DMA_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_hwcfgr2::R](R) reader structure"]
impl crate::Readable for DMA_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_HWCFGR2 to value 0x01"]
impl crate::Resettable for DMA_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
