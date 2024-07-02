///Register `DDRPERFM_CNT1` reader
pub type R = crate::R<DDRPERFM_CNT1rs>;
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
        f.debug_struct("DDRPERFM_CNT1")
            .field("cnt", &self.cnt())
            .finish()
    }
}
/**DDRPERFM event counter 1 register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_cnt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_CNT1)*/
pub struct DDRPERFM_CNT1rs;
impl crate::RegisterSpec for DDRPERFM_CNT1rs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_cnt1::R`](R) reader structure
impl crate::Readable for DDRPERFM_CNT1rs {}
///`reset()` method sets DDRPERFM_CNT1 to value 0
impl crate::Resettable for DDRPERFM_CNT1rs {
    const RESET_VALUE: u32 = 0;
}
