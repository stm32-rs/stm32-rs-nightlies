///Register `DMAC1CARXDR` reader
pub type R = crate::R<DMAC1CARXDRrs>;
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
        f.debug_struct("DMAC1CARXDR")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
/**Channel 1 current application receive descriptor register

You can [`read`](crate::Reg::read) this register and get [`dmac1carxdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:DMAC1CARXDR)*/
pub struct DMAC1CARXDRrs;
impl crate::RegisterSpec for DMAC1CARXDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1carxdr::R`](R) reader structure
impl crate::Readable for DMAC1CARXDRrs {}
///`reset()` method sets DMAC1CARXDR to value 0
impl crate::Resettable for DMAC1CARXDRrs {}
