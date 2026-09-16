///Register `DMACCARXDR` reader
pub type R = crate::R<DMACCARXDRrs>;
///Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACCARXDR")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
/**Channel current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmaccarxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:DMACCARXDR)*/
pub struct DMACCARXDRrs;
impl crate::RegisterSpec for DMACCARXDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaccarxdr::R`](R) reader structure
impl crate::Readable for DMACCARXDRrs {}
///`reset()` method sets DMACCARXDR to value 0
impl crate::Resettable for DMACCARXDRrs {}
