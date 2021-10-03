#[doc = "Register `DMAMUX_HWCFGR1` reader"]
pub struct R(crate::R<DMAMUX_HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_DMA_STREAMS` reader - NUM_DMA_STREAMS"]
pub struct NUM_DMA_STREAMS_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_STREAMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_STREAMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_STREAMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_DMA_PERIPH_REQ` reader - NUM_DMA_PERIPH_REQ"]
pub struct NUM_DMA_PERIPH_REQ_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_PERIPH_REQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_PERIPH_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_PERIPH_REQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_DMA_TRIG` reader - NUM_DMA_TRIG"]
pub struct NUM_DMA_TRIG_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_DMA_REQGEN` reader - NUM_DMA_REQGEN"]
pub struct NUM_DMA_REQGEN_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_REQGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_REQGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_REQGEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - NUM_DMA_STREAMS"]
    #[inline(always)]
    pub fn num_dma_streams(&self) -> NUM_DMA_STREAMS_R {
        NUM_DMA_STREAMS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NUM_DMA_PERIPH_REQ"]
    #[inline(always)]
    pub fn num_dma_periph_req(&self) -> NUM_DMA_PERIPH_REQ_R {
        NUM_DMA_PERIPH_REQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NUM_DMA_TRIG"]
    #[inline(always)]
    pub fn num_dma_trig(&self) -> NUM_DMA_TRIG_R {
        NUM_DMA_TRIG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NUM_DMA_REQGEN"]
    #[inline(always)]
    pub fn num_dma_reqgen(&self) -> NUM_DMA_REQGEN_R {
        NUM_DMA_REQGEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DMAMUX hardware configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_hwcfgr1](index.html) module"]
pub struct DMAMUX_HWCFGR1_SPEC;
impl crate::RegisterSpec for DMAMUX_HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamux_hwcfgr1::R](R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAMUX_HWCFGR1 to value 0x0808_6c10"]
impl crate::Resettable for DMAMUX_HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0808_6c10
    }
}
