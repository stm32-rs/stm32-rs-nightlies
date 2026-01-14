///Register `DMACCATXDR` reader
pub type R = crate::R<DMACCATXDRrs>;
///Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCATXDR")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
/**Channel current application transmit descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccatxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:DMACCATXDR)*/
pub struct DMACCATXDRrs;
impl crate::RegisterSpec for DMACCATXDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccatxdr::R`](R) reader structure
impl crate::Readable for DMACCATXDRrs {}
///`reset()` method sets DMACCATXDR to value 0
impl crate::Resettable for DMACCATXDRrs {}
