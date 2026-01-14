///Register `CNT3` reader
pub type R = crate::R<CNT3rs>;
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
        f.debug_struct("CNT3").field("cnt", &self.cnt()).finish()
    }
}
/**DDRPERFM event counter 3 register

You can [`read`](crate::Reg::read) this register and get [`cnt3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPERFM:CNT3)*/
pub struct CNT3rs;
impl crate::RegisterSpec for CNT3rs {
    type Ux = u32;
}
///`read()` method returns [`cnt3::R`](R) reader structure
impl crate::Readable for CNT3rs {}
///`reset()` method sets CNT3 to value 0
impl crate::Resettable for CNT3rs {}
