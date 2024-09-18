///Register `ETH_DMAC0CARxDR` reader
pub type R = crate::R<ETH_DMAC0CARX_DRrs>;
///Field `CURRDESAPTR` reader - Application Transmit Descriptor Address Pointer
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0CARxDR")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
/**Channel 0 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0carx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:ETH_DMAC0CARxDR)*/
pub struct ETH_DMAC0CARX_DRrs;
impl crate::RegisterSpec for ETH_DMAC0CARX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0carx_dr::R`](R) reader structure
impl crate::Readable for ETH_DMAC0CARX_DRrs {}
///`reset()` method sets ETH_DMAC0CARxDR to value 0
impl crate::Resettable for ETH_DMAC0CARX_DRrs {
    const RESET_VALUE: u32 = 0;
}
