///Register `TCNT` reader
pub type R = crate::R<TCNTrs>;
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
        f.debug_struct("TCNT").field("cnt", &self.cnt()).finish()
    }
}
/**DDRPERFM time counter register

You can [`read`](crate::Reg::read) this register and get [`tcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:TCNT)*/
pub struct TCNTrs;
impl crate::RegisterSpec for TCNTrs {
    type Ux = u32;
}
///`read()` method returns [`tcnt::R`](R) reader structure
impl crate::Readable for TCNTrs {}
///`reset()` method sets TCNT to value 0
impl crate::Resettable for TCNTrs {}
