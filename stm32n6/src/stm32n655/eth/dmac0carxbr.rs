///Register `DMAC0CARXBR` reader
pub type R = crate::R<DMAC0CARXBRrs>;
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
        f.debug_struct("DMAC0CARXBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
/**Channel 0 current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmac0carxbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC0CARXBR)*/
pub struct DMAC0CARXBRrs;
impl crate::RegisterSpec for DMAC0CARXBRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0carxbr::R`](R) reader structure
impl crate::Readable for DMAC0CARXBRrs {}
///`reset()` method sets DMAC0CARXBR to value 0
impl crate::Resettable for DMAC0CARXBRrs {}
