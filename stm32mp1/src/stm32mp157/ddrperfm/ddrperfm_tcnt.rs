///Register `DDRPERFM_TCNT` reader
pub type R = crate::R<DDRPERFM_TCNTrs>;
///Field `CNT` reader - CNT
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CNT
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRPERFM_TCNT")
            .field("cnt", &self.cnt())
            .finish()
    }
}
/**DDRPERFM time counter register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_tcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_TCNT)*/
pub struct DDRPERFM_TCNTrs;
impl crate::RegisterSpec for DDRPERFM_TCNTrs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_tcnt::R`](R) reader structure
impl crate::Readable for DDRPERFM_TCNTrs {}
///`reset()` method sets DDRPERFM_TCNT to value 0
impl crate::Resettable for DDRPERFM_TCNTrs {
    const RESET_VALUE: u32 = 0;
}
