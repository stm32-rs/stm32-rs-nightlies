///Register `DMAC0CATXDR` reader
pub type R = crate::R<DMAC0CATXDRrs>;
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
        f.debug_struct("DMAC0CATXDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
/**Channel 0 current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac0catxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAC0CATXDR)*/
pub struct DMAC0CATXDRrs;
impl crate::RegisterSpec for DMAC0CATXDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0catxdr::R`](R) reader structure
impl crate::Readable for DMAC0CATXDRrs {}
///`reset()` method sets DMAC0CATXDR to value 0
impl crate::Resettable for DMAC0CATXDRrs {}
