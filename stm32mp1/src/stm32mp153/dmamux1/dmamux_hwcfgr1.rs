#[doc = "Register `DMAMUX_HWCFGR1` reader"]
pub type R = crate::R<DMAMUX_HWCFGR1rs>;
#[doc = "Field `NUM_DMA_STREAMS` reader - NUM_DMA_STREAMS"]
pub type NUM_DMA_STREAMS_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_PERIPH_REQ` reader - NUM_DMA_PERIPH_REQ"]
pub type NUM_DMA_PERIPH_REQ_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_TRIG` reader - NUM_DMA_TRIG"]
pub type NUM_DMA_TRIG_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_REQGEN` reader - NUM_DMA_REQGEN"]
pub type NUM_DMA_REQGEN_R = crate::FieldReader;
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
#[doc = "DMAMUX hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamux_hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMUX_HWCFGR1rs;
impl crate::RegisterSpec for DMAMUX_HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamux_hwcfgr1::R`](R) reader structure"]
impl crate::Readable for DMAMUX_HWCFGR1rs {}
#[doc = "`reset()` method sets DMAMUX_HWCFGR1 to value 0x0808_6c10"]
impl crate::Resettable for DMAMUX_HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0808_6c10;
}
