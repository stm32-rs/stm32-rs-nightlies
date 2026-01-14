///Register `DMAC1CARXBR` reader
pub type R = crate::R<DMAC1CARXBRrs>;
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
        f.debug_struct("DMAC1CARXBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
/**Channel 1 current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac1carxbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ETH:DMAC1CARXBR)*/
pub struct DMAC1CARXBRrs;
impl crate::RegisterSpec for DMAC1CARXBRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1carxbr::R`](R) reader structure
impl crate::Readable for DMAC1CARXBRrs {}
///`reset()` method sets DMAC1CARXBR to value 0
impl crate::Resettable for DMAC1CARXBRrs {}
