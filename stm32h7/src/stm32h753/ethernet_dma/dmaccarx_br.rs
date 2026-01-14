///Register `DMACCARxBR` reader
pub type R = crate::R<DMACCARX_BRrs>;
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
        f.debug_struct("DMACCARxBR")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
/**Channel current application receive buffer register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#Ethernet_DMA:DMACCARxBR)*/
pub struct DMACCARX_BRrs;
impl crate::RegisterSpec for DMACCARX_BRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarx_br::R`](R) reader structure
impl crate::Readable for DMACCARX_BRrs {}
///`reset()` method sets DMACCARxBR to value 0
impl crate::Resettable for DMACCARX_BRrs {}
