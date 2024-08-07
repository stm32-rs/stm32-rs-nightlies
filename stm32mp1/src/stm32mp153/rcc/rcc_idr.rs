///Register `RCC_IDR` reader
pub type R = crate::R<RCC_IDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IDR").field("id", &self.id()).finish()
    }
}
/**This register gives the unique identifier of the RCC

You can [`read`](crate::Reg::read) this register and get [`rcc_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_IDR)*/
pub struct RCC_IDRrs;
impl crate::RegisterSpec for RCC_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_idr::R`](R) reader structure
impl crate::Readable for RCC_IDRrs {}
///`reset()` method sets RCC_IDR to value 0x01
impl crate::Resettable for RCC_IDRrs {
    const RESET_VALUE: u32 = 0x01;
}
