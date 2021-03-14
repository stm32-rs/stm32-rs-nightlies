#[doc = "Reader of register DMAMUX_HWCFGR2"]
pub type R = crate::R<u32, super::DMAMUX_HWCFGR2>;
#[doc = "Reader of field `NUM_DMA_EXT_REQ`"]
pub type NUM_DMA_EXT_REQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of DMA request trigger inputs"]
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
