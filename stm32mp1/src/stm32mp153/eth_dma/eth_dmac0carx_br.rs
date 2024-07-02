///Register `ETH_DMAC0CARxBR` reader
pub type R = crate::R<ETH_DMAC0CARX_BRrs>;
///Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Receive Buffer Address Pointer
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_DMAC0CARxBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
/**Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`eth_dmac0carx_br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:ETH_DMAC0CARxBR)*/
pub struct ETH_DMAC0CARX_BRrs;
impl crate::RegisterSpec for ETH_DMAC0CARX_BRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_dmac0carx_br::R`](R) reader structure
impl crate::Readable for ETH_DMAC0CARX_BRrs {}
///`reset()` method sets ETH_DMAC0CARxBR to value 0
impl crate::Resettable for ETH_DMAC0CARX_BRrs {
    const RESET_VALUE: u32 = 0;
}
