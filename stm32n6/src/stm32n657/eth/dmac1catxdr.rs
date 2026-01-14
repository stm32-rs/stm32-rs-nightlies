///Register `DMAC1CATXDR` reader
pub type R = crate::R<DMAC1CATXDRrs>;
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
        f.debug_struct("DMAC1CATXDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
/**Channel 1 current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac1catxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:DMAC1CATXDR)*/
pub struct DMAC1CATXDRrs;
impl crate::RegisterSpec for DMAC1CATXDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1catxdr::R`](R) reader structure
impl crate::Readable for DMAC1CATXDRrs {}
///`reset()` method sets DMAC1CATXDR to value 0
impl crate::Resettable for DMAC1CATXDRrs {}
