///Register `DMACCARxDR` reader
pub type R = crate::R<DMACCARX_DRrs>;
///Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Receive Descriptor Address Pointer
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCARxDR")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
/**Channel current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccarx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#Ethernet_DMA:DMACCARxDR)*/
pub struct DMACCARX_DRrs;
impl crate::RegisterSpec for DMACCARX_DRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarx_dr::R`](R) reader structure
impl crate::Readable for DMACCARX_DRrs {}
///`reset()` method sets DMACCARxDR to value 0
impl crate::Resettable for DMACCARX_DRrs {}
