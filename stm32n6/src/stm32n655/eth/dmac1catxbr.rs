///Register `DMAC1CATXBR` reader
pub type R = crate::R<DMAC1CATXBRrs>;
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
        f.debug_struct("DMAC1CATXBR")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
/**Channel 1 current application transmit buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1catxbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1CATXBR)*/
pub struct DMAC1CATXBRrs;
impl crate::RegisterSpec for DMAC1CATXBRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1catxbr::R`](R) reader structure
impl crate::Readable for DMAC1CATXBRrs {}
///`reset()` method sets DMAC1CATXBR to value 0
impl crate::Resettable for DMAC1CATXBRrs {}
