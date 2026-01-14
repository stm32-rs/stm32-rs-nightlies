///Register `HWCFGR1` reader
pub type R = crate::R<HWCFGR1rs>;
///Field `NUM_DMA_STREAMS` reader - NUM_DMA_STREAMS
pub type NUM_DMA_STREAMS_R = crate::FieldReader;
///Field `NUM_DMA_PERIPH_REQ` reader - NUM_DMA_PERIPH_REQ
pub type NUM_DMA_PERIPH_REQ_R = crate::FieldReader;
///Field `NUM_DMA_TRIG` reader - NUM_DMA_TRIG
pub type NUM_DMA_TRIG_R = crate::FieldReader;
///Field `NUM_DMA_REQGEN` reader - NUM_DMA_REQGEN
pub type NUM_DMA_REQGEN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NUM_DMA_STREAMS
    #[inline(always)]
    pub fn num_dma_streams(&self) -> NUM_DMA_STREAMS_R {
        NUM_DMA_STREAMS_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NUM_DMA_PERIPH_REQ
    #[inline(always)]
    pub fn num_dma_periph_req(&self) -> NUM_DMA_PERIPH_REQ_R {
        NUM_DMA_PERIPH_REQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - NUM_DMA_TRIG
    #[inline(always)]
    pub fn num_dma_trig(&self) -> NUM_DMA_TRIG_R {
        NUM_DMA_TRIG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - NUM_DMA_REQGEN
    #[inline(always)]
    pub fn num_dma_reqgen(&self) -> NUM_DMA_REQGEN_R {
        NUM_DMA_REQGEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR1")
            .field("num_dma_streams", &self.num_dma_streams())
            .field("num_dma_periph_req", &self.num_dma_periph_req())
            .field("num_dma_trig", &self.num_dma_trig())
            .field("num_dma_reqgen", &self.num_dma_reqgen())
            .finish()
    }
}
/**DMAMUX hardware configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMAMUX1:HWCFGR1)*/
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr1::R`](R) reader structure
impl crate::Readable for HWCFGR1rs {}
///`reset()` method sets HWCFGR1 to value 0x0808_6c10
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0808_6c10;
}
