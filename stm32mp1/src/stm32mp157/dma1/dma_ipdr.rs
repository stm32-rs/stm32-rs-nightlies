///Register `DMA_IPDR` reader
pub type R = crate::R<DMA_IPDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IPDR").field("id", &self.id()).finish()
    }
}
/**DMA IP identification register

You can [`read`](crate::Reg::read) this register and get [`dma_ipdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DMA1:DMA_IPDR)*/
pub struct DMA_IPDRrs;
impl crate::RegisterSpec for DMA_IPDRrs {
    type Ux = u32;
}
///`read()` method returns [`dma_ipdr::R`](R) reader structure
impl crate::Readable for DMA_IPDRrs {}
///`reset()` method sets DMA_IPDR to value 0x0010_0002
impl crate::Resettable for DMA_IPDRrs {
    const RESET_VALUE: u32 = 0x0010_0002;
}
