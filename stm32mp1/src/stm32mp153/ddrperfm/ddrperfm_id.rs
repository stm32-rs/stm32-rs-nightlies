///Register `DDRPERFM_ID` reader
pub type R = crate::R<DDRPERFM_IDrs>;
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
        f.debug_struct("DDRPERFM_ID")
            .field("id", &self.id())
            .finish()
    }
}
/**DDRPERFM ID register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:DDRPERFM_ID)*/
pub struct DDRPERFM_IDrs;
impl crate::RegisterSpec for DDRPERFM_IDrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_id::R`](R) reader structure
impl crate::Readable for DDRPERFM_IDrs {}
///`reset()` method sets DDRPERFM_ID to value 0x0014_0061
impl crate::Resettable for DDRPERFM_IDrs {
    const RESET_VALUE: u32 = 0x0014_0061;
}
