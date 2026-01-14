///Register `DMAC1CATxBR` reader
pub type R = crate::R<DMAC1CATX_BRrs>;
///Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1CATxBR")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
/**Channel 0 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1catx_br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC1CATxBR)*/
pub struct DMAC1CATX_BRrs;
impl crate::RegisterSpec for DMAC1CATX_BRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1catx_br::R`](R) reader structure
impl crate::Readable for DMAC1CATX_BRrs {}
///`reset()` method sets DMAC1CATxBR to value 0
impl crate::Resettable for DMAC1CATX_BRrs {}
