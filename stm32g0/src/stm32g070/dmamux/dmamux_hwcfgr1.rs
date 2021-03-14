#[doc = "Reader of register DMAMUX_HWCFGR1"]
pub type R = crate::R<u32, super::DMAMUX_HWCFGR1>;
#[doc = "Reader of field `NUM_DMA_STREAMS`"]
pub type NUM_DMA_STREAMS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_DMA_PERIPH_REQ`"]
pub type NUM_DMA_PERIPH_REQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_DMA_TRIG`"]
pub type NUM_DMA_TRIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `NUM_DMA_REQGEN`"]
pub type NUM_DMA_REQGEN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - number of DMA request line multiplexer (output) channels"]
    #[inline(always)]
    pub fn num_dma_streams(&self) -> NUM_DMA_STREAMS_R {
        NUM_DMA_STREAMS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - number of DMA request lines from peripherals"]
    #[inline(always)]
    pub fn num_dma_periph_req(&self) -> NUM_DMA_PERIPH_REQ_R {
        NUM_DMA_PERIPH_REQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - number of synchronization inputs"]
    #[inline(always)]
    pub fn num_dma_trig(&self) -> NUM_DMA_TRIG_R {
        NUM_DMA_TRIG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - number of DMA request generator channels"]
    #[inline(always)]
    pub fn num_dma_reqgen(&self) -> NUM_DMA_REQGEN_R {
        NUM_DMA_REQGEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
