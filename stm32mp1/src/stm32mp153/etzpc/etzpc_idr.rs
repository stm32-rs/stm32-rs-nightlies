///Register `ETZPC_IDR` reader
pub type R = crate::R<ETZPC_IDRrs>;
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
        f.debug_struct("ETZPC_IDR").field("id", &self.id()).finish()
    }
}
/**ETZPC IP version register

You can [`read`](crate::Reg::read) this register and get [`etzpc_idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETZPC:ETZPC_IDR)*/
pub struct ETZPC_IDRrs;
impl crate::RegisterSpec for ETZPC_IDRrs {
    type Ux = u32;
}
///`read()` method returns [`etzpc_idr::R`](R) reader structure
impl crate::Readable for ETZPC_IDRrs {}
///`reset()` method sets ETZPC_IDR to value 0x0010_0061
impl crate::Resettable for ETZPC_IDRrs {
    const RESET_VALUE: u32 = 0x0010_0061;
}
