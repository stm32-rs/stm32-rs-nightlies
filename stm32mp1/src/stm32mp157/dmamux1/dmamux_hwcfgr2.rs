///Register `DMAMUX_HWCFGR2` reader
pub type R = crate::R<DMAMUX_HWCFGR2rs>;
///Field `NUM_DMA_EXT_REQ` reader - NUM_DMA_EXT_REQ
pub type NUM_DMA_EXT_REQ_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - NUM_DMA_EXT_REQ
    #[inline(always)]
    pub fn num_dma_ext_req(&self) -> NUM_DMA_EXT_REQ_R {
        NUM_DMA_EXT_REQ_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX_HWCFGR2")
            .field("num_dma_ext_req", &self.num_dma_ext_req())
            .finish()
    }
}
/**DMAMUX hardware configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`dmamux_hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMAMUX1:DMAMUX_HWCFGR2)*/
pub struct DMAMUX_HWCFGR2rs;
impl crate::RegisterSpec for DMAMUX_HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_hwcfgr2::R`](R) reader structure
impl crate::Readable for DMAMUX_HWCFGR2rs {}
///`reset()` method sets DMAMUX_HWCFGR2 to value 0x08
impl crate::Resettable for DMAMUX_HWCFGR2rs {
    const RESET_VALUE: u32 = 0x08;
}
