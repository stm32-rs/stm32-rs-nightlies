///Register `DDRPERFM_SID` reader
pub type R = crate::R<DDRPERFM_SIDrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_SID")
            .field("sid", &self.sid())
            .finish()
    }
}
/**DDRPERFM magic ID register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_sid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:DDRPERFM_SID)*/
pub struct DDRPERFM_SIDrs;
impl crate::RegisterSpec for DDRPERFM_SIDrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_sid::R`](R) reader structure
impl crate::Readable for DDRPERFM_SIDrs {}
///`reset()` method sets DDRPERFM_SID to value 0xa3c5_dd01
impl crate::Resettable for DDRPERFM_SIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
