///Register `ETH_DMAC0CATxDR` reader
pub type R = crate::R<ETH_DMAC0CATX_DRrs>;
///Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0CATxDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
/**Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0catx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CATxDR)*/
pub struct ETH_DMAC0CATX_DRrs;
impl crate::RegisterSpec for ETH_DMAC0CATX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0catx_dr::R`](R) reader structure
impl crate::Readable for ETH_DMAC0CATX_DRrs {}
///`reset()` method sets ETH_DMAC0CATxDR to value 0
impl crate::Resettable for ETH_DMAC0CATX_DRrs {
    const RESET_VALUE: u32 = 0;
}
