///Register `DDRPERFM_CNT2` reader
pub type R = crate::R<DDRPERFM_CNT2rs>;
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
        f.debug_struct("DDRPERFM_CNT2")
            .field("cnt", &self.cnt())
            .finish()
    }
}
/**DDRPERFM event counter 2 register

You can [`read`](crate::Reg::read) this register and get [`ddrperfm_cnt2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:DDRPERFM_CNT2)*/
pub struct DDRPERFM_CNT2rs;
impl crate::RegisterSpec for DDRPERFM_CNT2rs {
    type Ux = u32;
}
///`read()` method returns [`ddrperfm_cnt2::R`](R) reader structure
impl crate::Readable for DDRPERFM_CNT2rs {}
///`reset()` method sets DDRPERFM_CNT2 to value 0
impl crate::Resettable for DDRPERFM_CNT2rs {
    const RESET_VALUE: u32 = 0;
}
