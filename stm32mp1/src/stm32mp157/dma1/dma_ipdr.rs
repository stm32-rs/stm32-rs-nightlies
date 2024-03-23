#[doc = "Register `DMA_IPDR` reader"]
pub type R = crate::R<DMA_IPDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "DMA IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_ipdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IPDRrs;
impl crate::RegisterSpec for DMA_IPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_ipdr::R`](R) reader structure"]
impl crate::Readable for DMA_IPDRrs {}
#[doc = "`reset()` method sets DMA_IPDR to value 0x0010_0002"]
impl crate::Resettable for DMA_IPDRrs {
    const RESET_VALUE: u32 = 0x0010_0002;
}
