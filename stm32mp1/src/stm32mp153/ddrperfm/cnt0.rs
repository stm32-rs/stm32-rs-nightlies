///Register `CNT0` reader
pub type R = crate::R<CNT0rs>;
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
        f.debug_struct("CNT0").field("cnt", &self.cnt()).finish()
    }
}
/**DDRPERFM event counter 0 register

You can [`read`](crate::Reg::read) this register and get [`cnt0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRPERFM:CNT0)*/
pub struct CNT0rs;
impl crate::RegisterSpec for CNT0rs {
    type Ux = u32;
}
///`read()` method returns [`cnt0::R`](R) reader structure
impl crate::Readable for CNT0rs {}
///`reset()` method sets CNT0 to value 0
impl crate::Resettable for CNT0rs {}
