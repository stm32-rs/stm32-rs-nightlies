///Register `DMACCARXBR` reader
pub type R = crate::R<DMACCARXBRrs>;
///Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCARXBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
/**Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccarxbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#ETH:DMACCARXBR)*/
pub struct DMACCARXBRrs;
impl crate::RegisterSpec for DMACCARXBRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarxbr::R`](R) reader structure
impl crate::Readable for DMACCARXBRrs {}
///`reset()` method sets DMACCARXBR to value 0
impl crate::Resettable for DMACCARXBRrs {}
