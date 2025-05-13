///Register `DMAC0CATxDR` reader
pub type R = crate::R<DMAC0CATX_DRrs>;
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
        f.debug_struct("DMAC0CATxDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
/**Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0catx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0CATxDR)*/
pub struct DMAC0CATX_DRrs;
impl crate::RegisterSpec for DMAC0CATX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0catx_dr::R`](R) reader structure
impl crate::Readable for DMAC0CATX_DRrs {}
///`reset()` method sets DMAC0CATxDR to value 0
impl crate::Resettable for DMAC0CATX_DRrs {}
