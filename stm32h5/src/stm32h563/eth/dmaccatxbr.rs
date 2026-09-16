///Register `DMACCATXBR` reader
pub type R = crate::R<DMACCATXBRrs>;
///Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCATXBR")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
/**Channel current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccatxbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:DMACCATXBR)*/
pub struct DMACCATXBRrs;
impl crate::RegisterSpec for DMACCATXBRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccatxbr::R`](R) reader structure
impl crate::Readable for DMACCATXBRrs {}
///`reset()` method sets DMACCATXBR to value 0
impl crate::Resettable for DMACCATXBRrs {}
