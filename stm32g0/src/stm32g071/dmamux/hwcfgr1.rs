#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Field `NUM_DMA_STREAMS` reader - number of DMA request line multiplexer (output) channels"]
pub type NUM_DMA_STREAMS_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_PERIPH_REQ` reader - number of DMA request lines from peripherals"]
pub type NUM_DMA_PERIPH_REQ_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_TRIG` reader - number of synchronization inputs"]
pub type NUM_DMA_TRIG_R = crate::FieldReader;
#[doc = "Field `NUM_DMA_REQGEN` reader - number of DMA request generator channels"]
pub type NUM_DMA_REQGEN_R = crate::FieldReader;
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
#[doc = "DMAMUX hardware configuration 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HWCFGR1rs {}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0417_3907"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0417_3907;
}
